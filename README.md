# logferry

A multi-threaded JSON log ingestor exposed to Python via [PyO3](https://pyo3.rs).
Parses, validates, and aggregates MLOps service logs across OS threads — no GIL
contention, no `Arc<Mutex<>>`, no marshalling boilerplate.

```python
import logferry

stats = logferry.ingest_logs(lines, num_threads=8)
print(stats.parsed_ok, stats.by_level, stats.avg_latency_ms)
```

---

## Features

- Parallel JSON parsing via `std::thread::scope` — borrows log lines across threads with zero clones
- Pluggable validation through a `Validator` trait (add your own rules without touching the core)
- Structured error reporting: per-batch counts + sampled error messages surfaced to Python
- Idiomatic error handling: Rust `Result<T, IngestError>` maps to Python `ValueError` automatically
- Zero-dependency Python API — consumers `pip install` the wheel; no Rust toolchain required

---

## Requirements

| Tool | Version | Purpose |
|---|---|---|
| Rust | ≥ 1.73 | Compiler (`rustup.rs`) |
| Python | ≥ 3.8 | Runtime and venv |
| maturin | ≥ 1.0 | Build backend that links Rust → Python |

---

## Installation

### Development (editable)

```bash
# 1. Clone and enter the project
git clone <your-repo>
cd logferry

# 2. Create and activate a virtual environment
python -m venv .venv
source .venv/bin/activate      # Windows: .venv\Scripts\activate

# 3. Install maturin, then build and install the extension in-place
pip install maturin
maturin develop --release
```

`maturin develop` compiles the Rust crate and installs `logferry` directly
into your active venv. Re-run it whenever you change `src/lib.rs`.

### Build a distributable wheel

```bash
maturin build --release
pip install dist/logferry-*.whl
```

The resulting `.whl` is a self-contained binary — no Rust toolchain needed
on the machine that installs it.

---

## Quick start

```python
import json
import logferry

# Build some log lines (in practice, read from Kafka, a file, etc.)
lines = [
    json.dumps({"level": "INFO",  "service": "ranker", "message": "scored 128 candidates", "latency_ms": 42.1}),
    json.dumps({"level": "ERROR", "service": "ranker", "message": "model file not found",  "latency_ms": 1.2}),
    json.dumps({"level": "WARN",  "service": "featurizer", "message": "cache miss",        "latency_ms": 88.0}),
    '{"broken":',  # malformed — won't crash, just increments parse_errors
]

stats = logferry.ingest_logs(lines, num_threads=4)

print(f"total     : {stats.total_lines}")
print(f"ok        : {stats.parsed_ok}")
print(f"parse err : {stats.parse_errors}")
print(f"val err   : {stats.validation_errors}")
print(f"by level  : {stats.by_level}")
print(f"avg ms    : {stats.avg_latency_ms:.1f}")
print(f"errors    : {stats.sample_errors}")
```

Expected output:

```
total     : 4
ok        : 3
parse err : 1
val err   : 0
by level  : {'INFO': 1, 'ERROR': 1, 'WARN': 1}
avg ms    : 43.8
errors    : ['line 0: invalid JSON (...)']
```

---

## API reference

### `logferry.ingest_logs(lines, num_threads=4) -> IngestStats`

Parses and validates a batch of JSON log lines in parallel.

| Parameter | Type | Default | Description |
|---|---|---|---|
| `lines` | `list[str]` | — | JSON-encoded log lines |
| `num_threads` | `int` | `4` | Worker thread count; clamped to `[1, len(lines)]` |

Raises `ValueError` only for programming errors (e.g., invalid arguments).
Malformed lines and failed validation are counted in `IngestStats`, not raised.

### `logferry.validate_line(line) -> bool`

Parses and validates a single JSON log line. Raises `ValueError` on any
failure — useful for unit-testing validation rules in isolation.

| Parameter | Type | Description |
|---|---|---|
| `line` | `str` | One JSON-encoded log line |

### `logferry.IngestStats`

Read-only result object returned by `ingest_logs`.

| Attribute | Type | Description |
|---|---|---|
| `total_lines` | `int` | Total lines received |
| `parsed_ok` | `int` | Lines that parsed and passed validation |
| `parse_errors` | `int` | Lines that were not valid JSON |
| `validation_errors` | `int` | Lines that failed a validation rule |
| `by_level` | `dict[str, int]` | Count of successfully parsed lines per log level |
| `avg_latency_ms` | `float \| None` | Mean `latency_ms` across all valid records, or `None` if none carried that field |
| `sample_errors` | `list[str]` | Up to 5 representative error messages (parse + validation combined) |

---

## Expected log line schema

Each line must be a JSON object with at least these fields:

```json
{
  "level":      "INFO",
  "service":    "inference-server",
  "message":    "handled request 42",
  "latency_ms": 18.4
}
```

`latency_ms` is optional; all other fields are required. Lines missing required
fields produce a `parse_errors` increment (serde rejects them during
deserialization). Lines with an empty `message` produce a `validation_errors`
increment.

---

## Running tests

```bash
# Pure-Rust unit tests — no Python interpreter needed
cargo test

# End-to-end demo via Python (requires maturin develop first)
python scripts/python_demo.py
```

`cargo test` exercises `ingest_chunk` and the `ingest_logs` merge logic
directly, without going through PyO3. This is intentional: the pure-Rust
layer has the real logic; the PyO3 layer is a thin boundary.

---

## Project structure

```
logferry/
├── Cargo.toml          # Rust package manifest and dependencies
├── pyproject.toml      # (optional) for maturin-managed distribution
├── src/
│   └── lib.rs          # All Rust source: data model, validators, ingestion, PyO3 bindings
├── scripts/
│   └── python_demo.py  # End-to-end Python usage demo
├── README.md           # This file
```

---

## Using logferry inside your own Python package

See **EXPLAINER.md → "Using it from a Python package"** for two approaches:

- **Standalone wheel** — build and distribute `logferry` independently; other projects `pip install` it.
- **Embedded mixed layout** — compile the extension as `mlops_toolkit._logferry`, re-export from `__init__.py`, and ship a single `mlops-toolkit` wheel.

---

## Adding a custom validator

1. Add a new struct to `src/lib.rs` and implement the `Validator` trait:

```rust
struct LatencyInRange {
    max_ms: f64,
}

impl Validator for LatencyInRange {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError> {
        if let Some(ms) = record.latency_ms {
            if ms > self.max_ms {
                return Err(IngestError::EmptyMessage {   // reuse or add a new variant
                    service: record.service.clone(),
                });
            }
        }
        Ok(())
    }
}
```

2. Push it into the `validators` vec inside `ingest_logs`:

```rust
let validators: Vec<Box<dyn Validator>> = vec![
    Box::new(NonEmptyMessage),
    Box::new(LatencyInRange { max_ms: 60_000.0 }),
];
```

3. Add a `#[test]` in the `tests` module and run `cargo test`.

---

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `pyo3` | 0.20 | Python ↔ Rust type bridge and extension-module scaffold |
| `serde` | 1 | Derive-based serialization framework |
| `serde_json` | 1 | JSON deserialization (used by serde) |

All three are pure-Rust; no C libraries, no system packages beyond Python itself.
