# Error Handling in PyO3

> **Prerequisites:** [Exposing Classes](exposing-classes.md)  
> **Next:** [Multithreading and the GIL](multithreading-and-gil.md)

PyO3 functions return `PyResult<T>` (which is `Result<T, PyErr>`). Any `Err(...)` you return becomes a Python exception. This article covers the patterns for producing Python-friendly errors from Rust code.

---

## The Python Mental Model

```python
def parse(line: str) -> LogRecord:
    try:
        return LogRecord(**json.loads(line))
    except json.JSONDecodeError as e:
        raise ValueError(f"bad JSON: {e}") from e
```

### The Rust Mental Model

Functions return `PyResult<T>`. Any `Err(...)` you return (or that propagates with `?`) becomes a Python exception. Nothing is raised implicitly; the compiler forces you to handle or propagate every error.

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

---

## The `?` Operator and `From` Conversions

`?` is Rust's equivalent of `raise` — it unwraps `Ok(v)` or returns the `Err` early. Make it automatically convert your custom error type into a `PyErr` by implementing `From`:

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
    // ...
    Ok(input.to_uppercase())
}
```

### `logferry`'s `IngestError`

```rust
#[derive(Debug)]
pub enum IngestError {
    EmptyMessage { service: String },
}

impl std::fmt::Display for IngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngestError::EmptyMessage { service } =>
                write!(f, "service '{service}' sent an empty message"),
        }
    }
}

impl std::error::Error for IngestError {}

// One impl — every `?` on IngestError inside PyResult becomes a ValueError
impl From<IngestError> for PyErr {
    fn from(err: IngestError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
```

Once that `From` impl exists, `validate_line` can use `?` on any `IngestError` and Python sees a `ValueError`:

```rust
#[pyfunction]
fn validate_line(line: &str) -> PyResult<bool> {
    let record: LogRecord = serde_json::from_str(line)
        .map_err(|e| PyValueError::new_err(format!("invalid JSON: {e}")))?;
    NonEmptyMessage.validate(&record)?;   // IngestError → PyErr via From
    Ok(true)
}
```

---

## Built-in Python Exception Types

```rust
use pyo3::exceptions::{
    PyValueError,        // ValueError
    PyTypeError,         // TypeError
    PyKeyError,          // KeyError
    PyRuntimeError,      // RuntimeError
    PyFileNotFoundError, // FileNotFoundError
    PyOSError,           // OSError
    PyAttributeError,    // AttributeError
    PyNotImplementedError, // NotImplementedError
};
```

Usage:

```rust
return Err(PyValueError::new_err("something went wrong"));
return Err(PyKeyError::new_err(format!("key not found: {key}")));
```

---

## Raising From an Inner Function

When an inner (non-PyO3) function produces an error that needs to reach Python:

```rust
// Inner function returns a Rust error
fn load_config(path: &str) -> Result<Config, ConfigError> { ... }

// PyO3 function converts it
#[pyfunction]
fn read_config(path: &str) -> PyResult<Config> {
    load_config(path).map_err(|e| PyValueError::new_err(e.to_string()))
}
```

Or implement `From<ConfigError> for PyErr` and use `?` directly:

```rust
#[pyfunction]
fn read_config(path: &str) -> PyResult<Config> {
    Ok(load_config(path)?)   // ? converts ConfigError → PyErr via From
}
```

---

## Common Gotchas

### No `unwrap()` in Production PyO3 Code

`unwrap()` on an `Err` inside a PyO3 function panics the Python interpreter — on some builds this becomes a hard crash, not a catchable exception. Always use `?` and proper `Result` types.

```rust
// ❌ Bad: panics if parse fails
let n: i64 = s.parse().unwrap();

// ✅ Good: converts the error to a Python ValueError
let n: i64 = s.parse().map_err(|e| PyValueError::new_err(format!("{e}")))?;
```

### `PyErr` Construction Outside the GIL

Some `PyErr::new_err(...)` calls need the GIL to allocate Python objects. If you are constructing errors inside `py.allow_threads(...)`, move the error construction outside the block.

---

## See Also

- [Multithreading and the GIL](multithreading-and-gil.md) — GIL release and error handling interaction
- [Error Handling](../02-rust-language-fundamentals/error-handling.md) — Rust-side Result and ? without PyO3
- [logferry Walkthrough](logferry-walkthrough.md) — both error strategies in the real code
