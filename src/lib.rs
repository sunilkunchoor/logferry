//! logferry: a small multi-threaded JSON log ingestor, exposed to Python
//! through PyO3.
//!
//! The pitch: your ML services emit millions of JSON log lines a day
//! (inference latency, errors, request metadata). This crate parses,
//! validates, and aggregates them across threads, then hands back a
//! plain Python object with the results — no GIL-bound Python loop,
//! no extra serialization step.
//!
//! Build with `maturin develop --release`, then `import logferry` from
//! Python. See python_demo.py for usage.

use std::collections::HashMap;
use std::thread;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::Deserialize;

// ---------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------

/// One line of an MLOps service log, e.g. what an inference server would
/// write to stdout and a log shipper (Fluent Bit, Vector, ...) would
/// forward as JSON.
///
/// `serde`'s `Deserialize` derive is doing the same job `pydantic.BaseModel`
/// or `dataclasses` + `json.loads` would do in Python — except the parsing
/// is generated at compile time, not reflected over at runtime.
#[derive(Debug, Deserialize)]
struct LogRecord {
    level: String,
    service: String,
    message: String,
    #[serde(default)]
    latency_ms: Option<f64>,
}

// ---------------------------------------------------------------------
// Errors — Rust's Result<T, E>, not Python's exceptions-as-control-flow
// ---------------------------------------------------------------------

/// Every recoverable failure this crate produces, in one enum. There is
/// no `except Exception:` equivalent here — every call site that can
/// fail must say so in its return type, and the compiler checks you
/// handled it.
#[derive(Debug)]
enum IngestError {
    BadJson { line_no: usize, reason: String },
    EmptyMessage { service: String },
}

impl std::fmt::Display for IngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngestError::BadJson { line_no, reason } => {
                write!(f, "line {line_no}: invalid JSON ({reason})")
            }
            IngestError::EmptyMessage { service } => {
                write!(f, "service '{service}' emitted a record with an empty message")
            }
        }
    }
}

impl std::error::Error for IngestError {}

/// This is the bridge between Rust error handling and Python's. Any
/// function below that does `something_returning_ingest_error()?` inside
/// a `PyResult<T>`-returning function gets the conversion for free — the
/// caller in Python just sees a normal `ValueError`.
impl From<IngestError> for PyErr {
    fn from(err: IngestError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

// ---------------------------------------------------------------------
// Validation — a trait plays the role Python's ABC / Protocol would
// ---------------------------------------------------------------------

/// Anything that can inspect a parsed `LogRecord` and accept or reject it.
/// `Send + Sync` as supertraits mean: any type implementing `Validator`
/// is safe to share across threads — that's what lets us pass
/// `&[Box<dyn Validator>]` into multiple worker threads below without a
/// `Mutex` anywhere in sight.
trait Validator: Send + Sync {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError>;
}

/// Rejects records whose `message` is empty — a common "silent failure"
/// mode in hand-rolled logging code.
struct NonEmptyMessage;

impl Validator for NonEmptyMessage {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError> {
        if record.message.trim().is_empty() {
            Err(IngestError::EmptyMessage {
                service: record.service.clone(),
            })
        } else {
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------
// Stats — the type Python actually sees
// ---------------------------------------------------------------------

/// `#[pyclass]` turns this into a real Python type. Each `#[pyo3(get)]`
/// field becomes a read-only attribute on the Python side — no manual
/// `__getattr__`, no dict-wrapping. `stats.by_level` in Python comes back
/// as an honest `dict[str, int]`.
#[pyclass]
#[derive(Debug, Default, Clone)]
struct IngestStats {
    #[pyo3(get)]
    total_lines: usize,
    #[pyo3(get)]
    parsed_ok: usize,
    #[pyo3(get)]
    parse_errors: usize,
    #[pyo3(get)]
    validation_errors: usize,
    #[pyo3(get)]
    by_level: HashMap<String, u64>,
    #[pyo3(get)]
    avg_latency_ms: Option<f64>,
    #[pyo3(get)]
    sample_errors: Vec<String>,
}

const MAX_SAMPLE_ERRORS: usize = 5;

impl IngestStats {
    /// Combines this chunk's stats with another chunk's. Used to fold
    /// per-thread results back into one report.
    fn merge(mut self, other: IngestStats) -> Self {
        self.total_lines += other.total_lines;
        self.parsed_ok += other.parsed_ok;
        self.parse_errors += other.parse_errors;
        self.validation_errors += other.validation_errors;

        for (level, count) in other.by_level {
            *self.by_level.entry(level).or_insert(0) += count;
        }

        self.sample_errors.extend(other.sample_errors);
        self.sample_errors.truncate(MAX_SAMPLE_ERRORS);

        self
    }
}

// ---------------------------------------------------------------------
// The actual work
// ---------------------------------------------------------------------

/// Parses and validates one chunk of lines, sequentially, on whichever
/// thread calls it. Takes `&[String]` — a *borrow* — rather than an
/// owned `Vec<String>`, which is only legal across the thread boundary
/// because `thread::scope` (below) guarantees the borrow outlives every
/// thread it spawns. No `Arc`, no `clone()` of the log lines, no GIL.
fn ingest_chunk(lines: &[String], validators: &[Box<dyn Validator>]) -> IngestStats {
    let mut stats = IngestStats::default();
    let mut latency_sum = 0.0_f64;
    let mut latency_count = 0_u64;

    for (i, line) in lines.iter().enumerate() {
        stats.total_lines += 1;

        let record: LogRecord = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                stats.parse_errors += 1;
                let err = IngestError::BadJson {
                    line_no: i,
                    reason: e.to_string(),
                };
                if stats.sample_errors.len() < MAX_SAMPLE_ERRORS {
                    stats.sample_errors.push(err.to_string());
                }
                continue;
            }
        };

        if let Some(bad) = validators.iter().find_map(|v| v.validate(&record).err()) {
            stats.validation_errors += 1;
            if stats.sample_errors.len() < MAX_SAMPLE_ERRORS {
                stats.sample_errors.push(bad.to_string());
            }
            continue;
        }

        stats.parsed_ok += 1;
        *stats.by_level.entry(record.level.clone()).or_insert(0) += 1;

        if let Some(ms) = record.latency_ms {
            latency_sum += ms;
            latency_count += 1;
        }
    }

    stats.avg_latency_ms = if latency_count > 0 {
        Some(latency_sum / latency_count as f64)
    } else {
        None
    };

    stats
}

/// Splits `lines` into `num_threads` roughly-even chunks, ingests each
/// chunk on its own OS thread, and merges the results.
///
/// Exposed to Python as `logferry.ingest_logs(lines, num_threads=4)`.
#[pyfunction]
#[pyo3(signature = (lines, num_threads=4))]
fn ingest_logs(lines: Vec<String>, num_threads: usize) -> PyResult<IngestStats> {
    if lines.is_empty() {
        return Ok(IngestStats::default());
    }

    let num_threads = num_threads.max(1).min(lines.len());
    let chunk_size = (lines.len() + num_threads - 1) / num_threads;
    let validators: Vec<Box<dyn Validator>> = vec![Box::new(NonEmptyMessage)];

    // `thread::scope` is the key trick: it lets worker closures borrow
    // `lines` and `validators` by reference instead of needing `'static`
    // + `Arc` + `move`. The compiler proves every spawned thread joins
    // before `scope` returns, so the borrows can never outlive their data.
    let merged = thread::scope(|scope| {
        let handles: Vec<_> = lines
            .chunks(chunk_size)
            .map(|chunk| scope.spawn(|| ingest_chunk(chunk, &validators)))
            .collect();

        handles
            .into_iter()
            .map(|h| h.join().expect("worker thread panicked"))
            .fold(IngestStats::default(), IngestStats::merge)
    });

    Ok(merged)
}

/// Validates a single JSON log line, raising a Python `ValueError` if it's
/// malformed or fails validation. Lets Python code unit-test the rule
/// in isolation, without going through the batch pipeline.
///
/// This is the function to look at for how `?` + `From<IngestError> for
/// PyErr` turns a Rust `Result` into a Python exception automatically.
#[pyfunction]
fn validate_line(line: &str) -> PyResult<bool> {
    let record: LogRecord = serde_json::from_str(line)
        .map_err(|e| PyValueError::new_err(format!("invalid JSON: {e}")))?;
    NonEmptyMessage.validate(&record)?;
    Ok(true)
}

// ---------------------------------------------------------------------
// Module definition
// ---------------------------------------------------------------------

#[pymodule]
fn logferry(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ingest_logs, m)?)?;
    m.add_function(wrap_pyfunction!(validate_line, m)?)?;
    m.add_class::<IngestStats>()?;
    Ok(())
}

// ---------------------------------------------------------------------
// Tests — pure Rust, no Python interpreter needed (`cargo test`)
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn line(level: &str, msg: &str) -> String {
        format!(r#"{{"level":"{level}","service":"svc","message":"{msg}","latency_ms":12.5}}"#)
    }

    #[test]
    fn parses_valid_lines_and_counts_levels() {
        let lines = vec![
            line("INFO", "started"),
            line("ERROR", "boom"),
            line("INFO", "done"),
        ];
        let validators: Vec<Box<dyn Validator>> = vec![Box::new(NonEmptyMessage)];
        let stats = ingest_chunk(&lines, &validators);

        assert_eq!(stats.total_lines, 3);
        assert_eq!(stats.parsed_ok, 3);
        assert_eq!(stats.by_level.get("INFO"), Some(&2));
        assert_eq!(stats.by_level.get("ERROR"), Some(&1));
    }

    #[test]
    fn rejects_empty_messages() {
        let lines = vec![line("INFO", "")];
        let validators: Vec<Box<dyn Validator>> = vec![Box::new(NonEmptyMessage)];
        let stats = ingest_chunk(&lines, &validators);

        assert_eq!(stats.validation_errors, 1);
        assert_eq!(stats.parsed_ok, 0);
    }

    #[test]
    fn counts_malformed_json_without_panicking() {
        let lines = vec!["not json".to_string(), line("INFO", "ok")];
        let validators: Vec<Box<dyn Validator>> = vec![Box::new(NonEmptyMessage)];
        let stats = ingest_chunk(&lines, &validators);

        assert_eq!(stats.parse_errors, 1);
        assert_eq!(stats.parsed_ok, 1);
        assert_eq!(stats.sample_errors.len(), 1);
    }

    #[test]
    fn ingest_logs_merges_results_across_threads() {
        let lines: Vec<String> = (0..1000)
            .map(|i| line("INFO", &format!("message {i}")))
            .collect();

        let stats = ingest_logs(lines, 8).expect("all lines are valid");

        assert_eq!(stats.total_lines, 1000);
        assert_eq!(stats.parsed_ok, 1000);
        assert_eq!(*stats.by_level.get("INFO").unwrap(), 1000);
    }
}
