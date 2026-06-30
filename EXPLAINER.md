# logferry: a multi-threaded log ingestor, from Python's side of the fence

`logferry` parses a batch of JSON log lines (the kind an inference server
would emit), validates them, splits the work across OS threads, and hands
Python back a plain object with the results. It exists to show four Rust
ideas in one realistic MLOps slice: ownership across threads, `Result`-based
error handling, traits as Python-ABC equivalents, and PyO3's type bridge.

## 1. Ownership across threads — no GIL, no `Arc<Mutex<>>`

In Python, sharing a list across threads is "free" because the GIL
serializes access — but that also means you don't get real parallelism for
CPU-bound work, only for I/O-bound work releasing the GIL.

```python
# Python: threads share `lines` by reference. Safe, but CPU-bound
# parsing here won't actually run in parallel — the GIL serializes it.
import threading
results = []
def worker(chunk):
    results.append(parse_and_count(chunk))
threads = [threading.Thread(target=worker, args=(c,)) for c in chunks]
```

Rust has no GIL, so the compiler has to prove sharing is safe *before* it
lets you compile. `std::thread::scope` is the tool for that:

```rust
let merged = thread::scope(|scope| {
    let handles: Vec<_> = lines
        .chunks(chunk_size)
        .map(|chunk| scope.spawn(|| ingest_chunk(chunk, &validators)))
        .collect();

    handles.into_iter()
        .map(|h| h.join().expect("worker thread panicked"))
        .fold(IngestStats::default(), IngestStats::merge)
});
```

`chunk` and `validators` are *borrowed* (`&[String]`, `&[Box<dyn Validator>]`),
not cloned, not wrapped in `Arc`. `thread::scope` guarantees every spawned
thread finishes before the block exits, so the compiler can prove the
borrows never outlive the data they point to. No mutex needed because no
thread ever writes to shared state — each thread computes its own
`IngestStats` and hands it back through `join()`; merging happens
afterward, single-threaded.

```
┌─────────────────────────── thread::scope ───────────────────────────┐
│                                                                      │
│   lines: Vec<String>  (owned by ingest_logs, never moved)            │
│        │                                                            │
│        ├─ chunk[0] ──borrow──▶ thread 1 ─▶ IngestStats ─┐            │
│        ├─ chunk[1] ──borrow──▶ thread 2 ─▶ IngestStats ─┤            │
│        └─ chunk[2] ──borrow──▶ thread 3 ─▶ IngestStats ─┤            │
│                                                          ▼            │
│                                            fold(..., IngestStats::merge)
└──────────────────────────────────────────────────────────────────────┘
```

## 2. `Result` instead of exceptions-as-control-flow

Python's `json.loads` raises on bad input; you find out what can fail by
reading docs or hitting a traceback in prod. Rust's `serde_json::from_str`
returns `Result<T, serde_json::Error>` — the failure mode is in the type
signature, and the compiler won't let you ignore it.

```python
# Python: nothing in the signature tells you this can throw
def parse(line: str) -> LogRecord:
    return LogRecord(**json.loads(line))
```

```rust
// Rust: the Result<_, _> *is* the contract
let record: LogRecord = match serde_json::from_str(line) {
    Ok(r) => r,
    Err(e) => { /* count it, keep going */ continue; }
};
```

`logferry` deliberately does **not** propagate parse errors with `?` inside
`ingest_chunk` — one bad line shouldn't abort a 200,000-line batch, so each
failure is counted and sampled instead. Contrast that with `validate_line`,
which *does* use `?` because there a single bad line should fail loudly:

```rust
fn validate_line(line: &str) -> PyResult<bool> {
    let record: LogRecord = serde_json::from_str(line)
        .map_err(|e| PyValueError::new_err(format!("invalid JSON: {e}")))?;
    NonEmptyMessage.validate(&record)?;   // ← IngestError auto-converts to PyErr
    Ok(true)
}
```

That auto-conversion is one `impl` away:

```rust
impl From<IngestError> for PyErr {
    fn from(err: IngestError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
```

Once that exists, every `?` on an `IngestError` inside a `PyResult`-returning
function becomes a Python `ValueError` for free — same role `raise` plays
in Python, but the conversion path is explicit and compiler-checked rather
than implicit.

## 3. A trait standing in for an ABC / Protocol

```python
from typing import Protocol

class Validator(Protocol):
    def validate(self, record: LogRecord) -> None: ...   # raises on failure

class NonEmptyMessage:
    def validate(self, record: LogRecord) -> None:
        if not record.message.strip():
            raise ValueError(f"service '{record.service}' sent an empty message")
```

```rust
trait Validator: Send + Sync {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError>;
}

struct NonEmptyMessage;

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

The `Send + Sync` supertraits are the interesting bit with no Python
equivalent: they're a compile-time promise that *any* type implementing
`Validator` is safe to share across threads. That promise is what makes
`&[Box<dyn Validator>]` legal to hand to multiple worker threads at once —
delete `Send + Sync` and the code that spawns threads stops compiling,
because the compiler can no longer prove it's safe.

## 4. PyO3: the type bridge

| Rust side | Python side |
|---|---|
| `#[pyclass] struct IngestStats { #[pyo3(get)] total_lines: usize, ... }` | `stats.total_lines` → plain `int` |
| `HashMap<String, u64>` field | `dict[str, int]` |
| `Option<f64>` field | `float` or `None` |
| `Result<T, IngestError>` (via `From<IngestError> for PyErr`) | raises `ValueError` |
| `#[pyfunction] fn ingest_logs(lines: Vec<String>, num_threads: usize)` | `logferry.ingest_logs(lines, num_threads=4)` |

This table is the whole value proposition of PyO3 for your stated goal:
you write normal, checked Rust; the macros generate the marshalling code
that would otherwise be a hand-written `ctypes` or `cffi` layer.

## Building it

This file was written in an environment with no Rust toolchain installed
(no `rustc`/`cargo`, and `rustup`'s installer is network-blocked here), so
the code below has been carefully hand-reviewed but **not compiled in this
session**. From a machine with Rust installed:

```bash
cd logferry
pip install maturin
maturin develop --release   # builds the extension, installs it into your venv
cargo test                  # runs the pure-Rust unit tests, no Python needed
python python_demo.py       # exercises it from Python
```

If `cargo build` or `cargo test` turns up an error, paste it back and we'll
fix it together — this is a from-scratch, unverified-by-compiler build, so
treat it as a strong first draft rather than a guarantee.

## Using it from a Python package

Two ways to ship this, depending on whether `logferry` is its own package
or lives inside a bigger one.

### Option A — standalone installable package

`maturin build` produces a wheel; `pip install` it like any other
dependency, in any venv, with no Rust toolchain needed on the consuming
machine:

```bash
cd logferry
maturin build --release        # writes dist/logferry-0.1.0-*.whl
pip install dist/logferry-0.1.0-*.whl
```

```python
import logferry
stats = logferry.ingest_logs(lines, num_threads=8)
```

Use this when `logferry` is a reusable tool other projects or teams will
depend on independently.

### Option B — embedded inside your own Python package

Maturin supports a "mixed" layout: a `python/` directory holding your real
package, plus the Rust extension compiled as a private submodule that your
`__init__.py` re-exports from. Example for a package called
`mlops_toolkit`:

```
mlops_toolkit/
├── Cargo.toml
├── pyproject.toml
├── src/
│   └── lib.rs              # the logferry code, module renamed to _logferry
└── python/
    └── mlops_toolkit/
        ├── __init__.py
        └── py.typed
```

`pyproject.toml`:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "mlops-toolkit"
version = "0.1.0"

[tool.maturin]
python-source = "python"
module-name = "mlops_toolkit._logferry"
```

Rename the `#[pymodule]` function in `lib.rs` to match (`fn _logferry(...)`),
then in `python/mlops_toolkit/__init__.py`:

```python
from mlops_toolkit._logferry import ingest_logs, validate_line, IngestStats

__all__ = ["ingest_logs", "validate_line", "IngestStats"]
```

`pip install -e .` (editable — rebuilds the Rust side via maturin on
install) or `maturin develop` during day-to-day development. Consumers of
`mlops_toolkit` just write `from mlops_toolkit import ingest_logs` and never
see that part of it is Rust.

One gap either way: PyO3 doesn't generate `.pyi` type stubs automatically,
so mypy/Pylance won't see types on `ingest_logs` or `IngestStats` unless you
hand-write a `_logferry.pyi` next to the compiled module — worth doing if
others will import this.

## Check your understanding

`ingest_chunk` currently treats a record with `latency_ms` greater than,
say, `60_000` (a full minute) as valid — clearly a bad sensor reading in a
real system. Add a new `Validator` called `LatencyInRange` that rejects
records with `latency_ms` over some threshold, wire it into the
`validators` vec in `ingest_logs`, and add a `#[test]` proving a too-high
latency shows up in `validation_errors`. Two things to watch for: where do
you put the threshold (a struct field vs. a constant), and does your new
validator need to be `Send + Sync` explicitly, or does it get that for free?
