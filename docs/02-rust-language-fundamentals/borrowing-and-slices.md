# Borrowing and Slices

> **Prerequisites:** [Ownership](ownership.md)  
> **Next:** [Structs and Enums](structs-and-enums.md)

Ownership would be painful if you had to move everything into every function. *Borrowing* lets you pass a reference to a value without transferring ownership. *Slices* let you take a zero-copy view into a sequence.

---

## Immutable Borrow — `&T`

```python
def print_length(items: list) -> None:
    print(len(items))
    # caller's list is unchanged

data = [1, 2, 3]
print_length(data)   # data is still usable after the call
print(data)
```

```rust
fn print_length(items: &Vec<i32>) {   // & = borrow, not move
    println!("{}", items.len());
}   // borrow ends here; nothing is freed

let data = vec![1, 2, 3];
print_length(&data);   // lend a reference; data is still owned by caller
println!("{:?}", data); // still valid
```

---

## Mutable Borrow — `&mut T`

```python
def append_zero(items: list) -> None:
    items.append(0)   # Python: silently mutates in place

data = [1, 2, 3]
append_zero(data)
print(data)  # [1, 2, 3, 0]
```

```rust
fn append_zero(items: &mut Vec<i32>) { // &mut = mutable borrow
    items.push(0);
}

let mut data = vec![1, 2, 3];
append_zero(&mut data);   // explicit: "I am passing a mutable reference"
println!("{:?}", data);   // [1, 2, 3, 0]
```

---

## The Borrowing Rules (Enforced at Compile Time)

```
Rule 1: any number of immutable borrows at the same time — OK
Rule 2: exactly one mutable borrow at a time — OK
Rule 3: immutable and mutable borrows cannot coexist
```

```rust
let mut v = vec![1, 2, 3];

// Multiple immutable borrows — fine
let r1 = &v;
let r2 = &v;
println!("{:?} {:?}", r1, r2);

// One mutable borrow — fine (after immutable borrows are done)
let r3 = &mut v;
r3.push(4);

// Simultaneous mutable + immutable — compile error
// let r4 = &v;     // can't do this while r3 is alive
```

These rules prevent the entire class of data-race bugs at compile time — no locks, no runtime checks, no sanitizers needed.

---

## Slices

A slice is a borrowed view into a contiguous sequence — like a `memoryview` or NumPy array slice, but for any type.

```python
data = [10, 20, 30, 40, 50]
chunk = data[1:4]   # [20, 30, 40] — creates a new list in Python
```

```rust
let data = vec![10, 20, 30, 40, 50];
let chunk: &[i32] = &data[1..4];     // [20, 30, 40] — zero-copy view
println!("{:?}", chunk);
```

String slices work the same way:

```rust
let s = String::from("inference-server");
let service: &str = &s[10..];   // "server" — a view, not a copy
```

---

## Prefer Slices Over `&Vec<T>` and `&String` in Function Signatures

```rust
// Less flexible: only accepts Vec<i32>
fn process(data: &Vec<i32>) { ... }

// More flexible: accepts Vec<i32>, arrays, or any contiguous slice
fn process(data: &[i32]) { ... }

// Same for strings: &str accepts both string literals and &String
fn greet(name: &str) { ... }   // works with "Alice" or &my_string
```

This pattern appears throughout `logferry`. For example, `ingest_chunk` takes `&[String]` — it works on any contiguous slice of strings, whether from a `Vec` or a sub-slice.

---

## How Borrowing and Threads Interact

The borrowing rules extend to threads. `thread::scope` works precisely because the compiler tracks that:
1. The spawned threads only hold *immutable* borrows of `lines` and `validators`.
2. No thread holds a *mutable* borrow — so multiple threads can read the same data simultaneously.
3. All threads finish before `scope` exits — so the borrows are always valid.

Delete the `Send + Sync` bound from the `Validator` trait and the compiler immediately tells you that the reference cannot be shared across threads. This is borrow-checking applied to concurrency.

See [Multithreading and the GIL](../04-pyo3-and-logferry/multithreading-and-gil.md) for the full story.

---

## See Also

- [Ownership](ownership.md) — why borrowing is necessary
- [Structs and Enums](structs-and-enums.md) — next article
- [Traits and Generics](traits-and-generics.md) — `Send` and `Sync` auto-traits
