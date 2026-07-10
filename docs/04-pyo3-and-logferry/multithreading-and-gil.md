# Multithreading and the GIL

> **Prerequisites:** [Error Handling in PyO3](error-handling-pyo3.md)  
> **Next:** [logferry Walkthrough](logferry-walkthrough.md)

`logferry`'s primary value proposition is parallel log parsing with no GIL contention and no `Arc<Mutex<>>`. This article explains how that works.

---

## The GIL and CPU Parallelism

Python's Global Interpreter Lock (GIL) serialises Python bytecode execution. Threads are useful for I/O-bound work (network calls, disk reads) but cannot run Python code simultaneously on multiple cores.

```python
# Python: threads share `lines` by reference.
# Safe, but CPU-bound parsing here won't actually run in parallel —
# the GIL serialises it.
import threading

results = []
def worker(chunk):
    results.append(parse_and_count(chunk))

threads = [threading.Thread(target=worker, args=(c,)) for c in chunks]
```

Rust has no GIL. The compiler must prove sharing is safe before it lets you compile. This is a stronger guarantee — not just "no crash", but "provably no data race".

---

## Releasing the GIL — `py.allow_threads`

When your Rust code does CPU-bound work, release the GIL so other Python threads can run concurrently:

```rust
#[pyfunction]
fn heavy_compute(py: Python<'_>, data: Vec<f64>) -> f64 {
    // Release the GIL while we crunch numbers.
    // Other Python threads can run during this block.
    py.allow_threads(|| {
        data.iter().map(|x| x * x).sum::<f64>().sqrt()
    })
}
```

`logferry` uses this pattern in `ingest_logs`:

```rust
#[pyfunction]
#[pyo3(signature = (lines, num_threads=4))]
pub fn ingest_logs(py: Python<'_>, lines: Vec<String>, num_threads: usize) -> PyResult<IngestStats> {
    let chunk_size = (lines.len() + num_threads - 1) / num_threads;

    let merged = py.allow_threads(|| {
        thread::scope(|scope| {
            let handles: Vec<_> = lines
                .chunks(chunk_size.max(1))
                .map(|chunk| scope.spawn(|| ingest_chunk(chunk, &validators)))
                .collect();

            handles
                .into_iter()
                .map(|h| h.join().expect("worker thread panicked"))
                .fold(IngestStats::default(), IngestStats::merge)
        })
    });

    Ok(merged)
}
```

Inside `py.allow_threads(|| { ... })` the GIL is released. Python threads can run. Your Rust threads run truly in parallel.

---

## `thread::scope` — Borrowing Across Threads Without `Arc`

The standard Python approach to CPU parallelism uses `multiprocessing` (separate processes, serialised data). Rust's `std::thread::scope` is cheaper — threads share the same memory, and the borrow checker proves it is safe:

```rust
use std::thread;

fn process_in_parallel(lines: &[String]) -> Vec<usize> {
    // `lines` is borrowed, not moved. The compiler proves the borrow
    // is valid for the entire `scope` block.
    thread::scope(|scope| {
        let mid = lines.len() / 2;
        let left  = scope.spawn(|| lines[..mid].iter().map(|l| l.len()).sum::<usize>());
        let right = scope.spawn(|| lines[mid..].iter().map(|l| l.len()).sum::<usize>());
        vec![left.join().unwrap(), right.join().unwrap()]
    })
}
```

### Why No `Arc<Mutex<>>`?

`thread::scope` guarantees all spawned threads finish before the block exits. Because the compiler can see this, it knows the borrows are valid for the full duration of every thread. No heap-allocated shared pointer (`Arc`) or lock (`Mutex`) is needed:

```rust
// With thread::scope — clean, no Arc
let result = thread::scope(|scope| {
    scope.spawn(|| process(&lines))   // borrows lines
        .join().unwrap()
});

// Without thread::scope (using thread::spawn, which requires 'static) — verbose
let lines = Arc::new(vec!["a".to_string()]);
let lines_clone = Arc::clone(&lines);
let handle = std::thread::spawn(move || {
    lines_clone.iter().map(|l| l.len()).sum::<usize>()
});
handle.join().unwrap();
```

---

## `Send` and `Sync` — Thread-Safety at the Type Level

`thread::scope` is only allowed when the data you share across threads implements `Send` and `Sync`:

- **`Send`**: the value can be *moved* into another thread.
- **`Sync`**: the value can be *shared by reference* (`&T`) across threads.

Most types are `Send + Sync` automatically. Types that are not include `Rc<T>` (reference-counted pointer without atomics) and `RefCell<T>` (interior mutability without locks).

The `Validator` trait in `logferry` requires `Send + Sync` explicitly:

```rust
trait Validator: Send + Sync {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError>;
}
```

This is a compile-time promise: any type that implements `Validator` is safe to share across threads. Delete `Send + Sync` and the thread::scope code that passes `&validators` to multiple threads stops compiling — the compiler cannot prove it is safe.

---

## The Full Parallel Architecture of `logferry`

```
Python call: logferry.ingest_logs(lines, num_threads=8)
     │
     │ GIL released via py.allow_threads(|| { ... })
     ▼
thread::scope
     │
     ├── thread 1 ── ingest_chunk(lines[0..chunk]) ──► IngestStats
     ├── thread 2 ── ingest_chunk(lines[chunk..2*chunk]) ──► IngestStats
     ├── ...
     └── thread N ── ingest_chunk(lines[(N-1)*chunk..]) ──► IngestStats
                                                    │
                              fold(IngestStats::default(), IngestStats::merge)
                                                    │
                                               merged IngestStats
     │
     │ GIL re-acquired; IngestStats returned to Python
     ▼
Python: stats.by_level  stats.parsed_ok  stats.avg_latency_ms
```

Key properties:
- `lines` is borrowed into each thread — no cloning.
- `validators` is borrowed into each thread — no `Arc`.
- Each thread produces its own `IngestStats` — no shared mutable state.
- Merging happens single-threadedly after all threads finish — no `Mutex`.

---

## See Also

- [logferry Walkthrough](logferry-walkthrough.md) — the full source code with annotations
- [Ownership](../02-rust-language-fundamentals/ownership.md) — why borrowing across threads is safe
- [Traits and Generics](../02-rust-language-fundamentals/traits-and-generics.md) — `Send` and `Sync` traits
