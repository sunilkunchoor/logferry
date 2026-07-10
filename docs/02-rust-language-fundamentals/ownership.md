# Ownership

> **Prerequisites:** [Functions and Control Flow](functions-and-control-flow.md)  
> **Next:** [Borrowing and Slices](borrowing-and-slices.md)

Ownership is Rust's most distinctive feature — it has no direct Python equivalent. Understanding it is the key to understanding everything else in Rust, including why `logferry` can safely parse logs in parallel with no locks.

---

## The Problem Ownership Solves

Python manages memory with reference counting plus a garbage collector. You never think about who "owns" an object. Rust has no GC — instead, the *compiler* tracks exactly one owner per value and automatically frees memory when the owner goes out of scope. Zero runtime overhead, zero GC pauses.

---

## The Three Rules

1. Every value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is freed.

```rust
{
    let data = String::from("model weights"); // `data` is the owner
    // use data ...
}   // `data` goes out of scope → memory freed automatically, no GC needed
```

---

## Move Semantics

```python
# Python: assignment copies the reference; both names point to same object
a = [1, 2, 3]
b = a           # b and a point to the same list
b.append(4)
print(a)        # [1, 2, 3, 4] — a is affected too
```

```rust
let a = vec![1, 2, 3];
let b = a;          // ownership MOVES from a to b
// println!("{:?}", a);  // compile error: value moved, a is invalid
println!("{:?}", b); // [1, 2, 3]
```

**Move** transfers ownership. After a move, the original variable is invalid and the compiler rejects any use of it. This prevents double-free bugs and use-after-free bugs at compile time.

---

## Copy Types — Stack-Only Values Are Copied, Not Moved

```rust
let x = 42;
let y = x;          // integers implement Copy; x is still valid
println!("{x} {y}"); // 42 42
```

Types that live entirely on the stack and are cheap to duplicate (integers, floats, booleans, `char`, tuples of `Copy` types) implement the `Copy` trait. Assignment copies them. Heap-allocated types (`String`, `Vec`, etc.) do not implement `Copy` — they move.

---

## Cloning — Explicit Deep Copy

```python
import copy
b = copy.deepcopy(a)  # explicit deep copy
```

```rust
let a = vec![1, 2, 3];
let b = a.clone();    // explicit deep copy; a is still valid
println!("{:?} {:?}", a, b);
```

`clone()` is the Rust equivalent of `deepcopy`. It is explicit — you opt in to the allocation cost, rather than being surprised by it.

---

## Ownership Across Threads — `thread::scope`

This is where ownership pays dividends for Python developers. In Python, sharing a list across threads is "free" because the GIL serialises access — but that also means you don't get real parallelism for CPU-bound work.

Rust has no GIL, so the compiler must prove sharing is safe before it lets you compile. `std::thread::scope` is the tool:

```rust
use std::thread;

let lines = vec![
    String::from("line 1"),
    String::from("line 2"),
    String::from("line 3"),
];

let merged = thread::scope(|scope| {
    let handles: Vec<_> = lines
        .chunks(1)
        .map(|chunk| scope.spawn(|| process(chunk)))  // chunk is *borrowed*
        .collect();

    handles.into_iter()
        .map(|h| h.join().expect("worker panicked"))
        .fold(0, |acc, x| acc + x)
});
```

`chunk` and any shared data are *borrowed* (`&[String]`), not cloned, not wrapped in `Arc`. `thread::scope` guarantees every spawned thread finishes before the block exits, so the compiler can prove the borrows never outlive the data they point to.

```
┌─────────────────────────── thread::scope ───────────────────────────┐
│                                                                      │
│   lines: Vec<String>  (owned by caller, never moved)                │
│        │                                                            │
│        ├─ chunk[0] ──borrow──▶ thread 1 ─▶ Stats ─┐               │
│        ├─ chunk[1] ──borrow──▶ thread 2 ─▶ Stats ─┤               │
│        └─ chunk[2] ──borrow──▶ thread 3 ─▶ Stats ─┤               │
│                                                    ▼               │
│                                        fold(..., merge)            │
└──────────────────────────────────────────────────────────────────────┘
```

This is exactly how `logferry` achieves parallel JSON parsing. No mutex, no `Arc`, no clone. See [Multithreading and the GIL](../04-pyo3-and-logferry/multithreading-and-gil.md) for the full walkthrough.

---

## Common Ownership Patterns

### Returning Ownership From a Function

```rust
// The function creates and returns a String — caller gets ownership
fn build_label(service: &str, version: u32) -> String {
    format!("{service}-v{version}")
}

let label = build_label("ranker", 3);  // label owns the String
```

### Passing Into and Out of Functions

```rust
fn process(data: Vec<String>) -> Vec<String> {
    // `data` moves in; caller loses ownership
    // process it, return it to give ownership back
    data
}

let lines = vec![String::from("a")];
let lines = process(lines);  // ownership returned via shadowing
```

In practice, you usually use *borrowing* (the next article) to avoid giving up ownership when you just need to read data.

---

## See Also

- [Borrowing and Slices](borrowing-and-slices.md) — how to lend access without transferring ownership
- [Multithreading and the GIL](../04-pyo3-and-logferry/multithreading-and-gil.md) — ownership + threads in logferry
- [Traits and Generics](traits-and-generics.md) — the `Copy` and `Clone` traits
