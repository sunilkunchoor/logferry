# Daily Workflow

> **Prerequisites:** [Testing](testing.md)  
> **Next:** [PyO3 Overview](../04-pyo3-and-logferry/pyo3-overview.md)

This article describes the recommended day-to-day development loop for Rust, compared to Python, and lists the one-off tools that make it faster and more productive.

---

## Python vs Rust Workflow Comparison

```
Python workflow              Rust workflow
─────────────────────────    ─────────────────────────────
activate venv                (nothing to activate)
write code                   write code
python script.py             cargo run
pytest                       cargo test
black . && flake8 .          cargo fmt && cargo clippy
push to CI                   push to CI (cargo test + clippy)
```

The biggest shift: in Rust you get compile errors as you type (via rust-analyzer), so many bugs are caught before you ever run `cargo build`.

---

## The Recommended Inner Loop

### Step 1: Keep `cargo check` Running in the Background

```bash
# Install cargo-watch (do this once)
cargo install cargo-watch

# In one terminal — re-run `cargo check` on every file save
cargo watch -x check

# Alternatively, re-run tests on every save (slower but more thorough)
cargo watch -x test
```

`cargo check` does a full type-check in under 2 seconds on most projects. You see compiler errors immediately as you write code — similar to rust-analyzer in the IDE, but in the terminal.

### Step 2: Use `cargo check` — not `cargo build` — for Fast Feedback

```bash
cargo check    # type-check only, no binary; fastest feedback
cargo test     # run tests (also does a type-check)
cargo run      # build + run (only when you need to see actual output)
```

### Step 3: For PyO3 Extensions, Use `maturin develop`

```bash
# After editing src/lib.rs:
maturin develop --release   # recompile and reinstall (~5–30s)
python scripts/python_demo.py        # test the changes from Python
```

---

## Before Committing

Run all four commands before every commit:

```bash
cargo fmt                    # format code (non-negotiable)
cargo clippy -- -D warnings  # lint; -D warnings fails on any warning
cargo test                   # all tests must pass
cargo build --release        # verify the optimised build works too
```

These are idiomatic gate checks. Many projects run them in CI as well.

### One-Liner Pre-Commit Check

```bash
cargo fmt && cargo clippy -- -D warnings && cargo test && cargo build --release
```

---

## Useful One-Off Tools

Install with `cargo install` — these persist across projects:

```bash
cargo install cargo-watch     # re-run commands on file changes
cargo install cargo-expand    # show what macros expand to (debugging macros)
cargo install cargo-audit     # check dependencies for known vulnerabilities
cargo install cargo-outdated  # show which deps have newer versions
cargo install cargo-flamegraph # generate flame graphs for profiling
cargo install cargo-nextest   # faster test runner with better output
```

---

## Handling Slow Compile Times

Rust's incremental compiler caches intermediate results, so after the first build only changed files are recompiled. Two settings further reduce compile times for development:

```toml
# .cargo/config.toml (in your project root)
[build]
# Use the faster mold linker (Linux)
# Install: sudo apt-get install mold
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[profile.dev]
opt-level = 0   # disable optimisation in dev builds
debug = 1       # less debug info = faster link
```

---

## The `logferry` Development Loop

```bash
# Edit src/lib.rs in your IDE (rust-analyzer gives real-time errors)

# Quick type-check (fastest)
cargo check

# Run pure-Rust tests (no Python needed)
cargo test

# Rebuild and test from Python
maturin develop --release
python scripts/python_demo.py

# Lint before committing
cargo fmt && cargo clippy -- -D warnings
```

---

## CI Configuration (GitHub Actions Example)

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Format check
        run: cargo fmt --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Test
        run: cargo test

      - name: Release build
        run: cargo build --release
```

---

## See Also

- [Cargo Cheatsheet](../05-reference/cargo-cheatsheet.md) — full command reference
- [Troubleshooting](../05-reference/troubleshooting.md) — common errors
- [IDE Setup](../01-getting-started/ide-setup.md) — rust-analyzer for real-time feedback
