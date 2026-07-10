# Testing

> **Prerequisites:** [Dependencies](dependencies.md)  
> **Next:** [Daily Workflow](daily-workflow.md)

Rust has first-class built-in testing — no separate test framework needed. Unit tests live alongside the code they test; integration tests live in a separate `tests/` directory. For PyO3 extensions, pytest covers the Python surface.

---

## Unit Tests

Add a `#[cfg(test)]` module at the bottom of any source file:

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;   // import everything from the parent module

    #[test]
    fn addition_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn subtraction_not_addition() {
        assert_ne!(add(2, 3), 4);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn division_by_zero_panics() {
        let _ = 10 / 0;
    }
}
```

`#[cfg(test)]` means the module is only compiled during `cargo test` — it does not appear in the release binary.

### Running Tests

```bash
cargo test                     # run all tests
cargo test addition            # run tests whose name contains "addition"
cargo test -- --nocapture      # show println! output during tests
cargo test -- --test-threads 1 # run tests sequentially (useful for debugging)
cargo test -- --list           # list all test names without running them
```

### Assertions

```rust
assert!(condition);                         // panics if false
assert_eq!(left, right);                    // panics if left != right
assert_ne!(left, right);                    // panics if left == right
assert!(condition, "message: {}", value);   // custom panic message
```

---

## Integration Tests

Integration tests live in the `tests/` directory. Each file is compiled as a separate crate that has access only to the public API of your crate — exactly like a real consumer would.

```
my_project/
├── src/
│   └── lib.rs
└── tests/
    └── ingestion_test.rs   ← integration test
```

```rust
// tests/ingestion_test.rs
use logferry;   // import the crate under test by name

#[test]
fn parses_valid_batch() {
    let lines = vec![
        r#"{"level":"INFO","service":"ranker","message":"ok","latency_ms":42.0}"#.to_string(),
    ];
    let stats = logferry::ingest_logs_raw(&lines, 1).unwrap();
    assert_eq!(stats.parsed_ok, 1);
    assert_eq!(stats.parse_errors, 0);
}
```

```bash
cargo test --test ingestion_test   # run only this integration test file
```

---

## `logferry`'s Test Strategy

`logferry` deliberately tests the pure-Rust layer directly — the PyO3 boundary is thin, so unit-testing the Rust functions provides maximum coverage with minimal test infrastructure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn make_line(level: &str, service: &str, message: &str, latency: f64) -> String {
        format!(
            r#"{{"level":"{level}","service":"{service}","message":"{message}","latency_ms":{latency}}}"#
        )
    }

    #[test]
    fn valid_lines_are_counted() {
        let lines = vec![
            make_line("INFO", "ranker", "scored", 42.0),
            make_line("ERROR", "ranker", "failed", 1.2),
        ];
        let stats = ingest_chunk(&lines, &[Box::new(NonEmptyMessage)]);
        assert_eq!(stats.parsed_ok, 2);
        assert_eq!(stats.parse_errors, 0);
    }

    #[test]
    fn malformed_json_increments_parse_errors() {
        let lines = vec!["not json".to_string()];
        let stats = ingest_chunk(&lines, &[Box::new(NonEmptyMessage)]);
        assert_eq!(stats.parse_errors, 1);
        assert_eq!(stats.parsed_ok, 0);
    }
}
```

---

## Python-Level Integration Tests with pytest

After running `maturin develop`, the extension is installed into your virtual environment and behaves like any other Python package:

```python
# tests/test_logferry.py
import json
import pytest
import logferry

def make_line(**fields):
    return json.dumps(fields)

def test_basic_ingestion():
    lines = [
        make_line(level="INFO", service="ranker", message="ok", latency_ms=42.0),
        make_line(level="ERROR", service="ranker", message="fail", latency_ms=1.2),
    ]
    stats = logferry.ingest_logs(lines, num_threads=2)
    assert stats.parsed_ok == 2
    assert stats.parse_errors == 0
    assert stats.by_level == {"INFO": 1, "ERROR": 1}

def test_malformed_json_does_not_raise():
    stats = logferry.ingest_logs(["{broken:"], num_threads=1)
    assert stats.parse_errors == 1
    assert stats.parsed_ok == 0

def test_validate_line_raises_on_bad_json():
    with pytest.raises(ValueError, match="invalid JSON"):
        logferry.validate_line("{not json}")

def test_empty_message_fails_validation():
    line = make_line(level="INFO", service="ranker", message="  ", latency_ms=1.0)
    with pytest.raises(ValueError):
        logferry.validate_line(line)
```

```bash
# Install pytest
pip install pytest

# Build the extension
maturin develop --release

# Run Python tests
pytest tests/
```

---

## Test Helpers and Fixtures

```rust
// A helper function — not a test itself, but used by multiple tests
fn valid_log(level: &str) -> String {
    format!(r#"{{"level":"{level}","service":"svc","message":"ok","latency_ms":1.0}}"#)
}

#[test]
fn by_level_counts_correctly() {
    let lines = vec![valid_log("INFO"), valid_log("INFO"), valid_log("WARN")];
    let stats = ingest_chunk(&lines, &[Box::new(NonEmptyMessage)]);
    assert_eq!(stats.by_level.get("INFO"), Some(&2));
    assert_eq!(stats.by_level.get("WARN"), Some(&1));
}
```

---

## Faster Tests with cargo-nextest

[cargo-nextest](https://nexte.st) is a drop-in replacement for `cargo test` with better output, parallel test execution, and retry support:

```bash
cargo install cargo-nextest
cargo nextest run
cargo nextest run --test-threads 8   # parallelism
```

---

## See Also

- [Daily Workflow](daily-workflow.md) — running tests as part of your development loop
- [Cargo Cheatsheet](../05-reference/cargo-cheatsheet.md) — full test command reference
- [logferry Walkthrough](../04-pyo3-and-logferry/logferry-walkthrough.md) — the full test module in context
