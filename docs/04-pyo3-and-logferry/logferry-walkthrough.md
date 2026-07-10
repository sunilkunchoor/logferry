# logferry Walkthrough

> **Prerequisites:** All articles in Chapters 1–4 up to this point  
> **Next:** [Distribution](distribution.md)

This article walks through every part of `src/lib.rs` — the complete `logferry` source code — explaining each decision in context. Read this after absorbing the individual concept articles.

---

## What `logferry` Does

`logferry` takes a batch of JSON log lines (the kind an inference server would emit), validates them, splits the work across OS threads, and hands Python back a plain object with the results:

```python
import logferry

lines = [
    '{"level":"INFO","service":"ranker","message":"scored 128 candidates","latency_ms":42.1}',
    '{"level":"ERROR","service":"ranker","message":"model file not found","latency_ms":1.2}',
    '{"broken":',  # malformed — won't crash, just counts as a parse error
]

stats = logferry.ingest_logs(lines, num_threads=4)
print(stats.parsed_ok, stats.parse_errors, stats.by_level, stats.avg_latency_ms)
```

---

## The Data Model

### `LogRecord` — The Input Shape

```rust
#[derive(Deserialize)]
pub struct LogRecord {
    pub level:      String,
    pub service:    String,
    pub message:    String,
    pub latency_ms: Option<f64>,   // optional field
}
```

`#[derive(Deserialize)]` comes from `serde`. It generates the deserialization code that converts JSON bytes into a Rust struct at compile time. Fields not present in `LogRecord` are silently ignored; missing required fields produce a deserialization error.

### `IngestStats` — The Output Shape

```rust
#[pyclass]
#[derive(Default, Clone)]
pub struct IngestStats {
    #[pyo3(get)] pub total_lines:       usize,
    #[pyo3(get)] pub parsed_ok:         usize,
    #[pyo3(get)] pub parse_errors:      usize,
    #[pyo3(get)] pub validation_errors: usize,
    #[pyo3(get)] pub by_level:          HashMap<String, u64>,
    #[pyo3(get)] pub avg_latency_ms:    Option<f64>,
    #[pyo3(get)] pub sample_errors:     Vec<String>,
}
```

- `#[pyclass]` makes this struct a Python class.
- `#[pyo3(get)]` on each field generates a read-only Python property.
- `#[derive(Default)]` lets `IngestStats::default()` construct a zeroed instance — used at the start of each worker thread.
- `#[derive(Clone)]` is needed internally for the merge operation.

The type conversions happen automatically at the PyO3 boundary:
- `usize` → Python `int`
- `HashMap<String, u64>` → Python `dict[str, int]`
- `Option<f64>` → Python `float | None`
- `Vec<String>` → Python `list[str]`

---

## The Validator Trait

```rust
pub trait Validator: Send + Sync {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError>;
}

pub struct NonEmptyMessage;

impl Validator for NonEmptyMessage {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError> {
        if record.message.trim().is_empty() {
            Err(IngestError::EmptyMessage { service: record.service.clone() })
        } else {
            Ok(())
        }
    }
}
```

**Why `Send + Sync`?** Validators are shared by reference (`&[Box<dyn Validator>]`) across multiple worker threads. `Send + Sync` is a compile-time proof that any type implementing `Validator` is safe to do so. If you add a validator that contains a non-thread-safe type (like `Rc<T>`), the compiler rejects it at the `impl Validator` site — before it can cause a data race.

**Why `Box<dyn Validator>`?** This is Rust's dynamic dispatch. It allows a `Vec` to hold different concrete validator types while calling through a common trait interface — equivalent to a Python list of Protocol implementations. See [Traits and Generics](../02-rust-language-fundamentals/traits-and-generics.md) for the full picture.

---

## The Error Type

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

impl From<IngestError> for PyErr {
    fn from(err: IngestError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
```

The `From<IngestError> for PyErr` impl is the "type bridge" for errors. Once it exists, any `?` on an `IngestError` inside a `PyResult`-returning function produces a Python `ValueError` — no further boilerplate needed.

---

## `ingest_chunk` — The Worker

```rust
fn ingest_chunk(
    lines:      &[String],
    validators: &[Box<dyn Validator>],
) -> IngestStats {
    let mut stats = IngestStats::default();
    let mut latency_sum = 0.0_f64;
    let mut latency_count = 0_u64;

    for (i, line) in lines.iter().enumerate() {
        stats.total_lines += 1;

        // Step 1: JSON parse
        let record: LogRecord = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                stats.parse_errors += 1;
                if stats.sample_errors.len() < 5 {
                    stats.sample_errors.push(format!("line {i}: invalid JSON ({e})"));
                }
                continue;   // bad line: skip to next, don't abort the batch
            }
        };

        // Step 2: run all validators
        let mut valid = true;
        for v in validators {
            if let Err(e) = v.validate(&record) {
                stats.validation_errors += 1;
                if stats.sample_errors.len() < 5 {
                    stats.sample_errors.push(format!("line {i}: {e}"));
                }
                valid = false;
                break;
            }
        }

        if !valid { continue; }

        // Step 3: accumulate stats
        stats.parsed_ok += 1;
        *stats.by_level.entry(record.level.clone()).or_insert(0) += 1;

        if let Some(ms) = record.latency_ms {
            latency_sum += ms;
            latency_count += 1;
        }
    }

    if latency_count > 0 {
        stats.avg_latency_ms = Some(latency_sum / latency_count as f64);
    }

    stats
}
```

**Key design decisions:**
- Errors are *counted and sampled*, not propagated. One bad line in a 200,000-line batch should not abort the whole job.
- `stats.sample_errors.len() < 5` caps the error sample to avoid unbounded memory growth.
- `ingest_chunk` is a plain Rust function with no PyO3 involvement — it can be called directly from `#[test]`.

---

## `IngestStats::merge` — Combining Thread Results

```rust
impl IngestStats {
    pub fn merge(mut self, other: IngestStats) -> IngestStats {
        self.total_lines       += other.total_lines;
        self.parsed_ok         += other.parsed_ok;
        self.parse_errors      += other.parse_errors;
        self.validation_errors += other.validation_errors;

        for (level, count) in other.by_level {
            *self.by_level.entry(level).or_insert(0) += count;
        }

        // Weighted average of latency
        match (self.avg_latency_ms, other.avg_latency_ms) {
            (Some(a), Some(b)) => {
                let total = self.parsed_ok as f64 * a + other.parsed_ok as f64 * b;
                let count = (self.parsed_ok + other.parsed_ok) as f64;
                self.avg_latency_ms = Some(total / count);
            }
            (None, Some(b)) => self.avg_latency_ms = Some(b),
            _ => {}
        }

        self.sample_errors.extend(other.sample_errors);
        self.sample_errors.truncate(5); // keep only 5 across all threads

        self
    }
}
```

`merge` takes ownership of both `self` and `other`, combines them, and returns a new `IngestStats`. This is passed to `.fold()` on the iterator of thread results.

---

## `ingest_logs` — The Public API

```rust
#[pyfunction]
#[pyo3(signature = (lines, num_threads=4))]
pub fn ingest_logs(
    py: Python<'_>,
    lines: Vec<String>,
    num_threads: usize,
) -> PyResult<IngestStats> {
    let validators: Vec<Box<dyn Validator>> = vec![Box::new(NonEmptyMessage)];
    let num_threads = num_threads.max(1).min(lines.len().max(1));
    let chunk_size = (lines.len() + num_threads - 1) / num_threads;

    let merged = py.allow_threads(|| {
        thread::scope(|scope| {
            let handles: Vec<_> = lines
                .chunks(chunk_size)
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

**Flow:**
1. Build the validators list (extensible — add new ones here).
2. Clamp `num_threads` to a sensible range.
3. Release the GIL via `py.allow_threads`.
4. Spawn threads via `thread::scope`; each thread borrows a chunk of `lines` and `&validators`.
5. Collect all `IngestStats` results and fold them into one.
6. Re-acquire the GIL and return `Ok(merged)` to Python.

---

## The Module Entry Point

```rust
#[pymodule]
fn logferry(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ingest_logs, m)?)?;
    m.add_function(wrap_pyfunction!(validate_line, m)?)?;
    m.add_class::<IngestStats>()?;
    Ok(())
}
```

The function name `logferry` must match the `name` in `Cargo.toml`'s `[lib]` section.

---

## Extending `logferry`

To add a new validation rule:

1. Add a new struct and implement `Validator`:

```rust
struct LatencyInRange {
    max_ms: f64,
}

impl Validator for LatencyInRange {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError> {
        if let Some(ms) = record.latency_ms {
            if ms > self.max_ms {
                return Err(IngestError::EmptyMessage { service: record.service.clone() });
            }
        }
        Ok(())
    }
}
```

2. Push it into the `validators` vec in `ingest_logs`:

```rust
let validators: Vec<Box<dyn Validator>> = vec![
    Box::new(NonEmptyMessage),
    Box::new(LatencyInRange { max_ms: 60_000.0 }),
];
```

3. Add a `#[test]` and run `cargo test`.

Because `LatencyInRange` contains only `f64` (which is `Send + Sync`), the compiler automatically considers it thread-safe. No additional annotation needed.

---

## See Also

- [Distribution](distribution.md) — build and publish the wheel
- [Multithreading and the GIL](multithreading-and-gil.md) — the parallel architecture in detail
- [Testing](../03-project-setup-and-tooling/testing.md) — testing the pure-Rust layer
