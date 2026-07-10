# Your First Project

> **Prerequisites:** [Installation](installation.md)  
> **Next:** [IDE Setup](ide-setup.md)

Cargo is Rust's all-in-one build tool and package manager. This article walks you through creating, building, and testing your first Rust project — and then a realistic second project that reads JSON log lines, to match the problem `logferry` solves.

---

## Creating a New Project

```bash
# Create a new binary project (produces an executable, like a Python script)
cargo new hello_rust
cd hello_rust
```

This creates:

```
hello_rust/
├── Cargo.toml    # manifest (like pyproject.toml / setup.cfg)
├── Cargo.lock    # exact dependency versions (auto-generated)
└── src/
    └── main.rs   # entry point
```

`src/main.rs` starts with:

```rust
fn main() {
    println!("Hello, world!");
}
```

Build and run in one step:

```bash
cargo run
# Compiling hello_rust v0.1.0
# Finished dev [unoptimized + debuginfo] target(s) in 0.42s
# Running `target/debug/hello_rust`
# Hello, world!
```

---

## Essential Cargo Commands

```bash
cargo build           # compile (debug mode — fast compile, slow binary)
cargo build --release # compile (optimised — slow compile, fast binary)
cargo check           # type-check only — fastest feedback, no binary produced
cargo run             # build (debug) and run
cargo test            # run all tests
cargo fmt             # format code (like Black)
cargo clippy          # lint (like flake8 but with fix suggestions)
cargo doc --open      # generate and open HTML documentation
cargo clean           # delete build artifacts (like __pycache__ cleanup)
```

> **Tip:** Use `cargo check` constantly while writing code. It is much faster than `cargo build` and catches almost all errors.

---

## Project Structure Deep Dive

```
my_project/
├── Cargo.toml          # package metadata and dependencies
├── Cargo.lock          # exact locked versions
├── rust-toolchain.toml # pin the compiler version (optional but recommended)
├── src/
│   ├── main.rs         # binary entry point (fn main)
│   ├── lib.rs          # library root (if it is also a library)
│   └── metrics/        # sub-module as a directory
│       ├── mod.rs      # module root
│       ├── precision.rs
│       └── recall.rs
├── tests/
│   └── integration_test.rs  # integration tests (separate from unit tests)
├── examples/
│   └── demo.rs         # runnable examples: `cargo run --example demo`
└── benches/
    └── throughput.rs   # benchmarks: `cargo bench`
```

---

## First Real Program — JSON Log Counter

Let's build something realistic: a CLI tool that reads JSON log lines from stdin and prints a count-by-level summary. This mirrors what `logferry` does, but as a standalone binary.

```bash
cargo new log_counter
cd log_counter
cargo add serde --features derive
cargo add serde_json
```

Edit `src/main.rs`:

```rust
use std::collections::HashMap;
use std::io::{self, BufRead};

use serde::Deserialize;

/// One JSON log line from an ML service.
/// Fields not in this struct are ignored (serde's default behaviour).
#[derive(Deserialize)]
struct LogLine {
    level: String,
    #[serde(default)]
    service: Option<String>,
}

fn main() {
    let stdin = io::stdin();
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut total: u64 = 0;
    let mut errors: u64 = 0;

    for line in stdin.lock().lines() {
        let raw = match line {
            Ok(l) => l,
            Err(e) => { eprintln!("read error: {e}"); continue; }
        };

        total += 1;

        match serde_json::from_str::<LogLine>(&raw) {
            Ok(log) => *counts.entry(log.level).or_insert(0) += 1,
            Err(_)  => errors += 1,
        }
    }

    println!("── log summary ──────────────");
    println!("total lines : {total}");
    println!("parse errors: {errors}");
    println!("by level:");

    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1)); // sort by count descending

    for (level, count) in sorted {
        println!("  {level:<10} {count}");
    }
}
```

Build and test it:

```bash
cargo build --release

echo '{"level":"INFO","service":"ranker","message":"ok"}
{"level":"ERROR","service":"ranker","message":"fail"}
{"level":"INFO","service":"featurizer","message":"ok"}
{"level":"WARN","service":"ranker","message":"slow"}
not valid json' | cargo run
```

Expected output:

```
── log summary ──────────────
total lines : 5
parse errors: 1
by level:
  INFO       2
  ERROR      1
  WARN       1
```

---

## Adding Unit Tests

Add a `#[cfg(test)]` block at the bottom of `src/main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialises_valid_line() {
        let raw = r#"{"level":"INFO","service":"ranker","message":"ok"}"#;
        let log: LogLine = serde_json::from_str(raw).unwrap();
        assert_eq!(log.level, "INFO");
        assert_eq!(log.service.as_deref(), Some("ranker"));
    }

    #[test]
    fn missing_optional_field_is_none() {
        let raw = r#"{"level":"WARN"}"#;
        let log: LogLine = serde_json::from_str(raw).unwrap();
        assert!(log.service.is_none());
    }

    #[test]
    fn rejects_missing_required_field() {
        let raw = r#"{"service":"ranker"}"#; // no "level"
        assert!(serde_json::from_str::<LogLine>(raw).is_err());
    }
}
```

```bash
cargo test
# running 3 tests
# test tests::deserialises_valid_line ... ok
# test tests::missing_optional_field_is_none ... ok
# test tests::rejects_missing_required_field ... ok
# test result: ok. 3 passed; 0 failed
```

---

## First Library — a Reusable Module

Libraries use `src/lib.rs` as the root instead of `src/main.rs`. They are imported by other Rust crates — or, via PyO3, by Python.

```bash
cargo new metrics_lib --lib
cd metrics_lib
```

`src/lib.rs`:

```rust
//! Classification metrics for MLOps pipelines.

/// Computes precision: TP / (TP + FP).
///
/// Returns `None` if both TP and FP are zero (undefined precision).
pub fn precision(tp: f64, fp: f64) -> Option<f64> {
    let denom = tp + fp;
    if denom == 0.0 { None } else { Some(tp / denom) }
}

/// Computes recall: TP / (TP + FN).
pub fn recall(tp: f64, fn_: f64) -> Option<f64> {
    let denom = tp + fn_;
    if denom == 0.0 { None } else { Some(tp / denom) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precision_basic() {
        assert!((precision(8.0, 2.0).unwrap() - 0.8).abs() < 1e-9);
    }

    #[test]
    fn precision_undefined() {
        assert!(precision(0.0, 0.0).is_none());
    }
}
```

```bash
cargo test
cargo doc --open   # generates and opens HTML API docs (like Sphinx for Python)
```

---

## See Also

- [IDE Setup](ide-setup.md) — get autocompletion and inline errors in your editor
- [Cargo In Depth](../03-project-setup-and-tooling/cargo-in-depth.md) — `Cargo.toml` anatomy and profiles
- [Testing](../03-project-setup-and-tooling/testing.md) — integration tests and pytest integration
- [Variables and Types](../02-rust-language-fundamentals/variables-and-types.md) — understand what `let`, `mut`, and types mean
