# Exposing Functions

> **Prerequisites:** [PyO3 Overview](pyo3-overview.md)  
> **Next:** [Exposing Classes](exposing-classes.md)

The `#[pyfunction]` attribute turns any Rust function into a Python callable. PyO3 generates the wrapper that handles type conversion; your function stays a plain, testable Rust function.

---

## A Simple Function

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

**What the macro does.** `#[pyfunction]` generates a hidden wrapper that PyO3 registers with the Python runtime. The wrapper handles type-checking and conversion; `add` itself stays a plain Rust function you can still call directly from `cargo test`.

---

## Type Mapping Quick Reference

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
| `bytes` | `&[u8]` or `Vec<u8>` |

---

## Default Arguments

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

`logferry` uses this for `num_threads`:

```rust
#[pyfunction]
#[pyo3(signature = (lines, num_threads=4))]
fn ingest_logs(py: Python<'_>, lines: Vec<String>, num_threads: usize) -> PyResult<IngestStats> {
    ...
}
```

---

## Returning `Option<T>` — Python `None`

```rust
#[pyfunction]
fn find_threshold(name: &str) -> Option<f64> {
    match name {
        "f1"        => Some(0.85),
        "precision" => Some(0.90),
        _           => None,
    }
}
```

```python
my_extension.find_threshold("f1")        # → 0.85
my_extension.find_threshold("unknown")   # → None
```

---

## Accepting a Python List

```rust
#[pyfunction]
fn sum_list(items: Vec<f64>) -> f64 {
    items.iter().sum()
}
```

```python
my_extension.sum_list([1.0, 2.0, 3.0])  # → 6.0
```

For more control, accept `&PyList` directly:

```rust
use pyo3::types::PyList;

#[pyfunction]
fn sum_list_raw(py: Python<'_>, items: &PyList) -> PyResult<f64> {
    let mut total = 0.0_f64;
    for item in items.iter() {
        total += item.extract::<f64>()?;
    }
    Ok(total)
}
```

`extract::<T>()` converts a `PyAny` to a Rust type, raising `TypeError` if the runtime type doesn't match.

---

## `PyAny` — The Escape Hatch

Use `&PyAny` when you need to inspect the Python object's type at runtime:

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

## Registering Multiple Functions

```rust
#[pymodule]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(shout, m)?)?;
    m.add_function(wrap_pyfunction!(find_threshold, m)?)?;
    m.add_function(wrap_pyfunction!(sum_list, m)?)?;
    Ok(())
}
```

---

## Testing Functions from `cargo test`

Because `#[pyfunction]` functions are plain Rust functions under the hood, you can test their core logic without PyO3 overhead:

```rust
// The public Python function calls an internal pure-Rust helper
fn compute_sum(items: &[f64]) -> f64 {
    items.iter().sum()
}

#[pyfunction]
fn sum_list(items: Vec<f64>) -> f64 {
    compute_sum(&items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_is_zero() {
        assert_eq!(compute_sum(&[]), 0.0);
    }

    #[test]
    fn sum_is_correct() {
        assert!((compute_sum(&[1.0, 2.0, 3.0]) - 6.0).abs() < 1e-10);
    }
}
```

This is exactly what `logferry` does: `ingest_chunk` is a pure-Rust function tested directly with `#[test]`, and `ingest_logs` is the thin PyO3 wrapper around it.

---

## See Also

- [Exposing Classes](exposing-classes.md) — expose Rust structs as Python objects
- [Error Handling in PyO3](error-handling-pyo3.md) — returning errors to Python
- [Testing](../03-project-setup-and-tooling/testing.md) — pure-Rust unit tests for PyO3 logic
