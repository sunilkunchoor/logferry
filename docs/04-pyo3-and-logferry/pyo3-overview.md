# PyO3 Overview

> **Prerequisites:** [Modules and Crates](../02-rust-language-fundamentals/modules-and-crates.md)  
> **Next:** [Exposing Functions](exposing-functions.md)

PyO3 is the bridge between Rust and Python. It lets you write a Rust library and import it from Python as if it were a normal `.so` extension — complete with proper Python types, exceptions, default argument values, and docstrings.

---

## What PyO3 Does

```
Python caller
     │
     │  import logferry
     ▼
logferry.so / logferry.pyd       ← compiled Rust extension
     │
     │  PyO3 type bridge          ← macros generate marshalling code
     ▼
Rust core logic                   ← ownership, threads, serde
```

PyO3 macros (`#[pyfunction]`, `#[pyclass]`, `#[pymodule]`) generate the boilerplate C-extension code that would otherwise require hand-written `ctypes` or `cffi` bindings. You write normal, type-checked Rust; the macros handle the Python ↔ Rust conversion.

---

## Project Setup

### Create a New PyO3 Project

```bash
# maturin scaffolds the correct Cargo.toml and pyproject.toml
pip install maturin
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

### Required `Cargo.toml` Settings

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

**`extension-module` feature is mandatory.** It tells PyO3 not to statically link `libpython`. Python provides its own symbols when it loads your `.so` at runtime; linking them twice causes crashes. Always include this feature.

**Both `cdylib` and `rlib` are needed.** `cdylib` produces the `.so`; `rlib` enables `cargo test` to build a test harness without a Python interpreter.

### Building and Installing

```bash
maturin develop          # debug build — installs into active venv
maturin develop --release  # optimised build

# After this, `import my_extension` works from Python
python -c "import my_extension; print(dir(my_extension))"
```

---

## Type Mapping

PyO3 automatically converts between Rust types and Python types at the boundary:

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

### `logferry`'s Type Bridge

| Rust | Python |
|---|---|
| `#[pyclass] struct IngestStats { #[pyo3(get)] total_lines: usize }` | `stats.total_lines` → plain `int` |
| `HashMap<String, u64>` field | `dict[str, int]` |
| `Option<f64>` field | `float \| None` |
| `Result<T, IngestError>` (via `From<IngestError> for PyErr`) | raises `ValueError` |
| `#[pyfunction] fn ingest_logs(lines: Vec<String>, num_threads: usize)` | `logferry.ingest_logs(lines, num_threads=4)` |

This table is the whole value proposition of PyO3: you write normal, checked Rust; the macros generate the marshalling code that would otherwise be a hand-written C extension.

---

## The Minimal Module

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

```python
import my_extension
result = my_extension.add(3, 4)  # → 7
```

---

## Working with `logferry`

```bash
# Clone the repo
git clone <repo-url>
cd logferry

# Create a virtual environment
python -m venv .venv
source .venv/bin/activate        # Linux / macOS
.venv\Scripts\activate           # Windows PowerShell

# Install maturin
pip install maturin

# Compile and install
maturin develop --release

# Verify
python -c "import logferry; print(dir(logferry))"
```

---

## See Also

- [Exposing Functions](exposing-functions.md) — `#[pyfunction]` in detail
- [Exposing Classes](exposing-classes.md) — `#[pyclass]` and `#[pymethods]`
- [Error Handling in PyO3](error-handling-pyo3.md) — `PyResult` and Python exceptions
- [Distribution](distribution.md) — building distributable wheels
