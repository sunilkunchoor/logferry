# PyO3 Basics — a tutorial for Python engineers learning Rust

PyO3 is the bridge between Rust and Python. It lets you write a Rust library
and import it from Python as if it were a normal `.so` extension — complete
with proper Python types, exceptions, default argument values, and docstrings.
This tutorial walks through every concept used in `logferry`, starting from
project setup and ending at distribution.

---

## Prerequisites

You should be comfortable with Python. No prior Rust experience is assumed,
but having read a short introduction to ownership (e.g., the first few
chapters of [The Rust Book](https://doc.rust-lang.org/book/)) will make the
examples land faster.

Install the toolchain once:

```bash
# Install Rust (rustup manages the compiler)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# maturin is the build tool that wires Rust → Python
pip install maturin
```

---

## 1. Project setup

### Scaffold with maturin

```bash
maturin new my_extension --bindings pyo3
cd my_extension
```

This produces:

```
my_extension/
├── Cargo.toml
├── pyproject.toml
└── src/
    └── lib.rs
```

### What `Cargo.toml` must contain

```toml
[package]
name = "my_extension"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_extension"
# cdylib  → the .so Python imports
# rlib    → lets `cargo test` work without a Python interpreter
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
```

`extension-module` tells PyO3 not to statically link `libpython`. Python
provides its own symbols when it loads your `.so` at runtime, so linking
them twice would cause crashes. Always include this feature.

### First build

```bash
maturin develop          # debug build, installs into active venv
maturin develop --release  # optimised build
```

After this, `import my_extension` works from Python.

---

## 2. Exposing a function — `#[pyfunction]`

The simplest thing PyO3 can do: take a Rust function and make it callable
from Python.

### Python you want to write

```python
import my_extension
result = my_extension.add(3, 4)  # → 7
```

### Rust that produces it

```rust
use pyo3::prelude::*;

#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[pymodule]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
```

**What the macro does.** `#[pyfunction]` generates a hidden wrapper that
PyO3 registers with the Python runtime. The wrapper handles type-checking
and conversion; `add` itself stays a plain Rust function you can still call
directly from `cargo test`.

### Type mapping cheat sheet

| Python type | Rust type |
|---|---|
| `int` | `i64`, `i32`, `usize`, … |
| `float` | `f64`, `f32` |
| `str` | `&str` (borrow) or `String` (own) |
| `bool` | `bool` |
| `list[T]` | `Vec<T>` |
| `dict[K, V]` | `HashMap<K, V>` |
| `T \| None` | `Option<T>` |
| `tuple[A, B]` | `(A, B)` |
| `bytes` | `&[u8]` (borrow) or `Vec<u8>` (own) |

### Default arguments

```rust
#[pyfunction]
#[pyo3(signature = (text, repeat=1))]
fn shout(text: &str, repeat: usize) -> String {
    text.to_uppercase().repeat(repeat)
}
```

```python
my_extension.shout("hello")        # → "HELLO"
my_extension.shout("hello", 3)     # → "HELLOHELLOHELLO"
my_extension.shout("hi", repeat=2) # → "HIHI"
```

---

## 3. Exposing a class — `#[pyclass]`

### Python you want to write

```python
counter = my_extension.Counter(start=10)
counter.increment()
counter.increment()
print(counter.value)   # → 12
```

### Rust that produces it

```rust
use pyo3::prelude::*;

#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(start: i64) -> Self {
        Counter { value: start }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    #[getter]
    fn value(&self) -> i64 {
        self.value
    }
}
```

### Read-only attributes with `#[pyo3(get)]`

For structs you only return from Rust (never construct from Python), the
`#[pyo3(get)]` field attribute is less ceremony than `#[getter]` methods:

```rust
#[pyclass]
#[derive(Default, Clone)]
struct Stats {
    #[pyo3(get)]
    total: usize,
    #[pyo3(get)]
    errors: usize,
}
```

```python
stats = compute_stats(data)
print(stats.total, stats.errors)   # read-only; no setter generated
```

Field types still need to be Python-convertible (`usize` → `int`,
`HashMap<String, u64>` → `dict`, `Option<f64>` → `float | None`, etc.).

### `__repr__` and `__str__`

```rust
#[pymethods]
impl Counter {
    fn __repr__(&self) -> String {
        format!("Counter(value={})", self.value)
    }
}
```

---

## 4. Error handling — `Result` becomes `ValueError`

### The Python mental model

```python
def parse(line: str) -> LogRecord:
    try:
        return LogRecord(**json.loads(line))
    except json.JSONDecodeError as e:
        raise ValueError(f"bad JSON: {e}") from e
```

### The Rust mental model

Functions return `PyResult<T>` (= `Result<T, PyErr>`). Any `Err(...)` you
return (or that propagates with `?`) becomes a Python exception. Nothing is
raised implicitly; the compiler forces you to handle or propagate every error.

```rust
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn parse_int(s: &str) -> PyResult<i64> {
    s.parse::<i64>()
        .map_err(|e| PyValueError::new_err(format!("not an int: {e}")))
}
```

```python
my_extension.parse_int("42")    # → 42
my_extension.parse_int("oops")  # raises ValueError: not an int: ...
```

### The `?` operator and `From` conversions

`?` is Rust's equivalent of `raise` — it unwraps `Ok(v)` or returns the
`Err` early. You can make it automatically convert your custom error into a
`PyErr` by implementing `From`:

```rust
#[derive(Debug)]
enum MyError {
    BadInput(String),
    NotFound(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::BadInput(s)  => write!(f, "bad input: {s}"),
            MyError::NotFound(s)  => write!(f, "not found: {s}"),
        }
    }
}

impl std::error::Error for MyError {}

// This is the bridge: MyError → Python ValueError
impl From<MyError> for PyErr {
    fn from(err: MyError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

// Now `?` on a Result<_, MyError> inside PyResult<_> works automatically
#[pyfunction]
fn process(input: &str) -> PyResult<String> {
    if input.is_empty() {
        return Err(MyError::BadInput("empty string".into()).into());
    }
    let result = do_work(input)?;   // ? converts MyError → PyErr for free
    Ok(result)
}
```

### Other built-in exception types

```rust
use pyo3::exceptions::{
    PyValueError,       // ValueError
    PyTypeError,        // TypeError
    PyKeyError,         // KeyError
    PyRuntimeError,     // RuntimeError
    PyFileNotFoundError,// FileNotFoundError
    PyOSError,          // OSError
};
```

---

## 5. Accepting Python objects directly

Sometimes you need to inspect the Python object itself rather than having
PyO3 convert it first.

### Receiving a Python list

```rust
use pyo3::types::PyList;

#[pyfunction]
fn sum_list(py: Python<'_>, items: &PyList) -> PyResult<f64> {
    let mut total = 0.0_f64;
    for item in items.iter() {
        total += item.extract::<f64>()?;
    }
    Ok(total)
}
```

`extract::<T>()` converts a `PyAny` to a Rust type, raising `TypeError` if
the runtime type doesn't match.

### `PyAny` — the escape hatch

```rust
#[pyfunction]
fn describe(obj: &PyAny) -> String {
    if obj.is_none() {
        "None".into()
    } else if let Ok(n) = obj.extract::<i64>() {
        format!("int({})", n)
    } else if let Ok(s) = obj.extract::<&str>() {
        format!("str(\"{}\")", s)
    } else {
        format!("object of type {}", obj.get_type().name().unwrap_or("?"))
    }
}
```

---

## 6. Traits — the `Send + Sync` story

In Python, a `Protocol` or `ABC` defines an interface. Rust traits do the
same, with one extra layer: **auto-trait markers** that tell the compiler
whether a type is safe to share across threads.

```python
from typing import Protocol

class Validator(Protocol):
    def validate(self, record: dict) -> None: ...   # raises on failure
```

```rust
// The supertrait bounds Send + Sync are a compile-time promise:
// any type implementing Validator is safe to move into a thread (Send)
// and safe to share by reference across threads (Sync).
trait Validator: Send + Sync {
    fn validate(&self, record: &LogRecord) -> Result<(), MyError>;
}
```

Why does this matter for PyO3? When you pass a `Vec<Box<dyn Validator>>`
by reference into multiple worker threads (as `logferry` does), the compiler
needs proof that the contained values can't cause data races. `Send + Sync`
supertraits provide that proof — enforced at compile time, not at runtime.
Delete either bound and the code that spawns threads stops compiling.

---

## 7. Multi-threaded extensions and the GIL

### The GIL is Rust's responsibility, not Python's

Python's GIL serialises Python-level work. When your Rust code runs, it can
release the GIL so other Python threads progress concurrently. PyO3 handles
this boundary automatically when you use `py.allow_threads`:

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

### `thread::scope` — borrowing across threads without `Arc`

The standard way to parallelise in Python is `multiprocessing` (separate
processes, serialised data) or `concurrent.futures.ThreadPoolExecutor` (true
threads, but GIL-bound for CPU work).

In Rust, `std::thread::scope` lets you spawn threads that *borrow* data from
the calling function — no `Arc`, no `Mutex`, no clone:

```rust
use std::thread;

fn process_in_parallel(lines: &[String]) -> Vec<usize> {
    // lines is borrowed, not moved.
    // The compiler proves the borrow is valid for the entire scope block.
    thread::scope(|scope| {
        let mid = lines.len() / 2;
        let left  = scope.spawn(|| lines[..mid].iter().map(|l| l.len()).sum::<usize>());
        let right = scope.spawn(|| lines[mid..].iter().map(|l| l.len()).sum::<usize>());
        vec![left.join().unwrap(), right.join().unwrap()]
    })
}
```

Compare with the `Arc<Mutex<>>` approach you'd use with `thread::spawn`
(which requires `'static` bounds):

```rust
// Verbose alternative — needed when threads must outlive the calling function
use std::sync::{Arc, Mutex};

let lines = Arc::new(vec!["a".to_string(), "b".to_string()]);
let lines_clone = Arc::clone(&lines);
let handle = std::thread::spawn(move || {
    lines_clone.iter().map(|l| l.len()).sum::<usize>()
});
handle.join().unwrap();
```

`thread::scope` is the right choice when the threads are short-lived helpers
inside a single function — which is almost always true for batch-processing
work like log ingestion.

---

## 8. Putting it all together — a complete module

Here is a minimal but complete PyO3 module using everything covered so far:

```rust
use std::collections::HashMap;
use std::thread;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::Deserialize;

// --- Data model ---

#[derive(Deserialize)]
struct Event {
    kind: String,
    value: f64,
}

// --- Error type with automatic Python conversion ---

#[derive(Debug)]
enum AnalysisError {
    BadJson { reason: String },
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::BadJson { reason } => write!(f, "invalid JSON: {reason}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

impl From<AnalysisError> for PyErr {
    fn from(err: AnalysisError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

// --- Result object visible from Python ---

#[pyclass]
#[derive(Default, Clone)]
struct Summary {
    #[pyo3(get)]
    count: usize,
    #[pyo3(get)]
    by_kind: HashMap<String, u64>,
    #[pyo3(get)]
    avg_value: Option<f64>,
}

impl Summary {
    fn merge(mut self, other: Summary) -> Self {
        self.count += other.count;
        for (k, v) in other.by_kind {
            *self.by_kind.entry(k).or_insert(0) += v;
        }
        self
    }
}

// --- Processing logic ---

fn process_chunk(lines: &[String]) -> Summary {
    let mut s = Summary::default();
    let mut total = 0.0;
    let mut n = 0u64;
    for line in lines {
        if let Ok(ev) = serde_json::from_str::<Event>(line) {
            s.count += 1;
            *s.by_kind.entry(ev.kind).or_insert(0) += 1;
            total += ev.value;
            n += 1;
        }
    }
    s.avg_value = if n > 0 { Some(total / n as f64) } else { None };
    s
}

// --- Public Python API ---

#[pyfunction]
#[pyo3(signature = (lines, num_threads=4))]
fn analyse(py: Python<'_>, lines: Vec<String>, num_threads: usize) -> PyResult<Summary> {
    let chunk_size = (lines.len() + num_threads - 1) / num_threads;
    let result = py.allow_threads(|| {
        thread::scope(|scope| {
            let handles: Vec<_> = lines
                .chunks(chunk_size.max(1))
                .map(|chunk| scope.spawn(|| process_chunk(chunk)))
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().expect("thread panicked"))
                .fold(Summary::default(), Summary::merge)
        })
    });
    Ok(result)
}

// --- Module definition ---

#[pymodule]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyse, m)?)?;
    m.add_class::<Summary>()?;
    Ok(())
}
```

```python
import json, my_extension

lines = [
    json.dumps({"kind": "click", "value": 1.0}),
    json.dumps({"kind": "view",  "value": 3.5}),
    json.dumps({"kind": "click", "value": 2.0}),
]
s = my_extension.analyse(lines, num_threads=2)
print(s.count, s.by_kind, s.avg_value)
# → 3  {'click': 2, 'view': 1}  2.1666...
```

---

## 9. Testing

### Pure-Rust unit tests (preferred)

Keep as much logic as possible in plain Rust functions, tested with
`#[test]` — no Python interpreter needed:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn ev(kind: &str, value: f64) -> String {
        format!(r#"{{"kind":"{kind}","value":{value}}}"#)
    }

    #[test]
    fn counts_by_kind() {
        let lines = vec![ev("click", 1.0), ev("view", 2.0), ev("click", 3.0)];
        let s = process_chunk(&lines);
        assert_eq!(s.count, 3);
        assert_eq!(*s.by_kind.get("click").unwrap(), 2);
    }

    #[test]
    fn ignores_bad_json_gracefully() {
        let lines = vec!["not json".to_string(), ev("click", 1.0)];
        let s = process_chunk(&lines);
        assert_eq!(s.count, 1);
    }
}
```

Run with:

```bash
cargo test
cargo test counts_by_kind   # run one test by name
cargo test -- --nocapture   # show println! output
```

### Python-level integration tests

After `maturin develop`, use pytest like any other extension:

```python
# tests/test_extension.py
import json, pytest
import my_extension

def line(kind, value):
    return json.dumps({"kind": kind, "value": value})

def test_basic():
    s = my_extension.analyse([line("a", 1.0), line("b", 2.0)])
    assert s.count == 2
    assert s.by_kind == {"a": 1, "b": 1}

def test_empty():
    s = my_extension.analyse([])
    assert s.count == 0
    assert s.avg_value is None
```

```bash
pytest tests/
```

---

## 10. Writing type stubs for mypy / Pylance

PyO3 doesn't generate `.pyi` stubs automatically. Without them, type
checkers see your extension as `Any`. Write a stub file alongside the
compiled module:

```python
# my_extension.pyi
from typing import Optional

class Summary:
    count: int
    by_kind: dict[str, int]
    avg_value: Optional[float]

def analyse(lines: list[str], num_threads: int = ...) -> Summary: ...
```

Place it in the same directory as `my_extension.so` (or in the package's
`python/` directory for mixed layouts), and add a `py.typed` marker file
to signal full typing support.

---

## 11. Distributing

### Single-platform wheel

```bash
maturin build --release
# dist/my_extension-0.1.0-cp311-cp311-linux_x86_64.whl
pip install dist/my_extension-*.whl
```

The wheel filename encodes the Python version and platform. It is not
portable across platforms by default.

### Cross-platform wheels with GitHub Actions

The standard setup: run `maturin build` on three runners (ubuntu-latest,
macos-latest, windows-latest) and publish all three wheels to PyPI. The
[maturin-action](https://github.com/PyO3/maturin-action) GitHub Action
handles this in ~10 lines of YAML.

### Embedding inside a larger package

See **EXPLAINER.md → "Using it from a Python package"** for the mixed
layout (a `python/` directory alongside `src/`, a single `pyproject.toml`
with `[tool.maturin]`, and re-export from `__init__.py`).

---

## 12. Common gotchas

**`extension-module` feature missing.** Forgetting it causes a double-link
of `libpython` and a segfault or import error on some platforms. Always
include it.

**Only `cdylib` in `crate-type`.** Cargo can't build a test harness from a
`cdylib`-only crate. Add `"rlib"` alongside `"cdylib"` so `cargo test`
works.

**`#[pymodule]` function name must match the crate name.** If `Cargo.toml`
has `name = "my_extension"` and the function is named `fn my_module(...)`,
Python can't find the init symbol. Keep them in sync, or use
`[lib] name = "..."` in `Cargo.toml` to set the name explicitly.

**Returning `String` vs `&str`.** If you borrow from a local variable and
return a `&str`, the borrow checker rejects it (the local is dropped). When
in doubt, return `String` — PyO3 converts it to a Python `str` either way.

**`PyErr` construction outside the GIL.** Some `PyErr::new_err(...)` calls
need the GIL to allocate Python objects. If you're constructing them inside
`py.allow_threads(...)`, move the error construction outside the block or
use lazy `PyErr` construction.

**No `unwrap()` in production code.** Use `?` and proper `Result` types.
`unwrap()` on an error in a PyO3 function panics the Python interpreter —
on some builds this becomes a hard crash, not a catchable exception.

---

## Further reading

- [PyO3 user guide](https://pyo3.rs/latest/) — comprehensive, well-maintained
- [maturin docs](https://maturin.rs) — build system details, CI recipes, workspace setups
- [The Rust Book ch. 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — threads, channels, `Arc<Mutex<>>`
- [Rust std::thread::scope](https://doc.rust-lang.org/std/thread/fn.scope.html) — stable since 1.63
- [serde docs](https://serde.rs) — the derive-based JSON serialisation framework used in `logferry`
