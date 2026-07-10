# Exposing Classes

> **Prerequisites:** [Exposing Functions](exposing-functions.md)  
> **Next:** [Error Handling in PyO3](error-handling-pyo3.md)

The `#[pyclass]` and `#[pymethods]` attributes expose Rust structs as Python classes. This is how `logferry` makes `IngestStats` available as a plain Python object.

---

## A Simple Class

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

Register it in the module:

```rust
#[pymodule]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Counter>()?;
    Ok(())
}
```

---

## Read-Only Attributes with `#[pyo3(get)]`

For structs you only return from Rust (never construct from Python), `#[pyo3(get)]` on a field generates a read-only property with less ceremony than `#[getter]` methods:

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

Field types must be Python-convertible: `usize` → `int`, `HashMap<String, u64>` → `dict`, `Option<f64>` → `float | None`.

### `logferry`'s `IngestStats`

```rust
#[pyclass]
#[derive(Default, Clone)]
pub struct IngestStats {
    #[pyo3(get)] pub total_lines:         usize,
    #[pyo3(get)] pub parsed_ok:           usize,
    #[pyo3(get)] pub parse_errors:        usize,
    #[pyo3(get)] pub validation_errors:   usize,
    #[pyo3(get)] pub by_level:            HashMap<String, u64>,
    #[pyo3(get)] pub avg_latency_ms:      Option<f64>,
    #[pyo3(get)] pub sample_errors:       Vec<String>,
}
```

```python
stats = logferry.ingest_logs(lines, num_threads=8)
print(stats.parsed_ok)           # int
print(stats.by_level)            # dict[str, int]
print(stats.avg_latency_ms)      # float | None
```

---

## `__repr__` and `__str__`

```rust
#[pymethods]
impl Counter {
    fn __repr__(&self) -> String {
        format!("Counter(value={})", self.value)
    }

    fn __str__(&self) -> String {
        self.value.to_string()
    }
}
```

```python
c = my_extension.Counter(start=5)
repr(c)   # → "Counter(value=5)"
str(c)    # → "5"
```

---

## Read-Write Attributes with `#[pyo3(get, set)]`

```rust
#[pyclass]
struct Config {
    #[pyo3(get, set)]
    batch_size: usize,
}
```

```python
cfg = my_extension.Config()
cfg.batch_size = 64
print(cfg.batch_size)  # → 64
```

---

## Class Methods and Static Methods

```rust
#[pymethods]
impl Counter {
    #[classmethod]
    fn from_zero(_cls: &PyType) -> Self {
        Counter { value: 0 }
    }

    #[staticmethod]
    fn max_value() -> i64 {
        i64::MAX
    }
}
```

```python
c = my_extension.Counter.from_zero()
print(my_extension.Counter.max_value())
```

---

## Returning Instances From Functions

A common pattern: a `#[pyfunction]` does the work and returns a `#[pyclass]` result to Python.

```rust
#[pyfunction]
#[pyo3(signature = (lines, num_threads=4))]
fn ingest_logs(py: Python<'_>, lines: Vec<String>, num_threads: usize) -> PyResult<IngestStats> {
    // ... do processing ...
    Ok(stats)   // IngestStats is a #[pyclass]; PyO3 boxes it for Python automatically
}
```

```python
stats = logferry.ingest_logs(lines, num_threads=8)
# stats is a Python object of type IngestStats
```

---

## Iterables and Container Protocols

You can implement Python's iterator protocol:

```rust
#[pyclass]
struct LogIterator {
    lines: Vec<String>,
    index: usize,
}

#[pymethods]
impl LogIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        if slf.index < slf.lines.len() {
            let line = slf.lines[slf.index].clone();
            slf.index += 1;
            Some(line)
        } else {
            None
        }
    }
}
```

```python
for line in my_extension.LogIterator(lines):
    print(line)
```

---

## See Also

- [Error Handling in PyO3](error-handling-pyo3.md) — returning errors from methods
- [logferry Walkthrough](logferry-walkthrough.md) — `IngestStats` in full context
- [Exposing Functions](exposing-functions.md) — returning class instances from functions
