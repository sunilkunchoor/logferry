# What is Rust?

> **Prerequisites:** none  
> **Next:** [Installation](installation.md)

Rust is a systems programming language that focuses on three things: **speed**, **memory safety**, and **fearless concurrency** — all without a garbage collector. For Python developers, the most compelling use case is building native extension modules that run at C-level speed while remaining safe and maintainable.

---

## Why Rust for Python Developers?

Python is excellent for productivity and rapid iteration, but it has two hard constraints for CPU-bound work:

1. **The Global Interpreter Lock (GIL)** — only one thread can execute Python bytecode at a time. Threads don't give you true CPU parallelism.
2. **Runtime overhead** — dynamic typing, reference counting, and interpreter dispatch add cost that matters at scale.

Rust eliminates both constraints:

- **No GIL** — Rust threads run truly in parallel on multiple cores. The compiler verifies thread safety at compile time, so you never need a runtime lock for read-only shared data.
- **Zero-cost abstractions** — high-level constructs like iterators, closures, and generics compile down to the same machine code you would write by hand in C.

The bridge between the two worlds is [PyO3](https://pyo3.rs): write a Rust library, annotate it with a handful of macros, and Python imports it as a normal `.so` extension. The `logferry` project in this repository demonstrates this end-to-end.

---

## Memory Safety Without a Garbage Collector

Python uses reference counting plus a cycle-collecting GC to manage memory. You never think about allocation or deallocation.

Rust manages memory through **ownership** — a set of rules the compiler checks at compile time:

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is freed automatically.
3. You can lend references to values without transferring ownership, but the compiler ensures a reference never outlives the value it points to.

The result: no `malloc`/`free` to write, no use-after-free bugs, no dangling pointers — and no GC pauses.

```python
# Python: reference counting keeps objects alive
data = [1, 2, 3]
alias = data          # both point to the same list; refcount = 2
del data              # refcount drops to 1; memory is NOT freed yet
```

```rust
// Rust: one owner, freed when it goes out of scope
{
    let data = vec![1, 2, 3]; // `data` owns the allocation
    // use data...
}   // data goes out of scope → allocation freed immediately, no GC needed
```

---

## Fearless Concurrency

Python's GIL means threads are useful for I/O (network calls, file reads) but not for CPU-bound tasks like JSON parsing, matrix operations, or feature computation.

Rust's ownership model extends naturally to threads. If you can compile it, the borrow checker has proven it is free of data races:

```rust
use std::thread;

// These two chunks are borrowed (not copied) into separate threads.
// `thread::scope` guarantees they finish before the scope exits —
// so the compiler knows the borrows are safe.
let lines = vec!["line1".to_string(), "line2".to_string()];
let result = thread::scope(|scope| {
    let h1 = scope.spawn(|| lines[..1].len());
    let h2 = scope.spawn(|| lines[1..].len());
    h1.join().unwrap() + h2.join().unwrap()
});
```

`logferry` uses exactly this pattern to parse log lines across multiple OS threads with no `Arc<Mutex<>>` overhead. See [Ownership](../02-rust-language-fundamentals/ownership.md) and [Multithreading and the GIL](../04-pyo3-and-logferry/multithreading-and-gil.md) for the full explanation.

---

## How Rust Fits Into a Python Project

```
Python caller
     │
     │  import logferry          (normal Python import)
     ▼
logferry.so / logferry.pyd       (compiled Rust extension)
     │
     │  PyO3 type bridge         (macros handle marshalling)
     ▼
Rust core logic                  (ownership, threads, serde)
```

The Python caller never knows — or cares — that part of the code is Rust. It uses normal Python types: `list[str]`, `int`, `dict`, `float | None`.

---

## Key Differences at a Glance

| Topic | Python | Rust |
|---|---|---|
| Memory | GC / refcount | Ownership + borrow checker |
| Threads | GIL-limited | Truly parallel; compiler-verified safety |
| Types | Runtime (+ optional hints) | Compile-time, always |
| Nullability | `None` on any variable | `Option<T>` — explicit, enforced |
| Error handling | `raise` / `try/except` | `Result<T, E>` / `?` operator |
| Speed | Interpreter overhead | Near C-level |
| Distribution | `pip install wheel` | `pip install wheel` (maturin builds it) |

---

## Further Reading

- [Installation](installation.md) — set up the Rust toolchain next
- [The Rust Book](https://doc.rust-lang.org/book/) — the official free textbook
- [PyO3 Overview](../04-pyo3-and-logferry/pyo3-overview.md) — how Rust and Python connect
