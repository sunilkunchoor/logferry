# Cargo In Depth

> **Prerequisites:** [Your First Project](../01-getting-started/your-first-project.md)  
> **Next:** [Dependencies](dependencies.md)

Cargo is `pip` + `virtualenv` + `setuptools` + `pytest` combined into one tool. This article covers the anatomy of `Cargo.toml`, build profiles, and all the commands you'll use daily.

---

## Cargo vs Python Tooling

```
Python                         Rust / Cargo equivalent
─────────────────────────────  ──────────────────────────────────
pip install pandas             cargo add pandas   (or edit Cargo.toml)
pip install -r requirements    cargo build        (reads Cargo.toml)
python setup.py build          cargo build --release
python my_script.py            cargo run
pytest                         cargo test
black .                        cargo fmt
flake8 / pylint                cargo clippy
pypi                           crates.io
requirements.txt               Cargo.toml [dependencies]
poetry.lock / pip freeze       Cargo.lock
```

---

## `Cargo.toml` Anatomy

```toml
[package]
name    = "my_project"
version = "0.1.0"
edition = "2021"          # always use 2021 for new projects
authors = ["Your Name <you@example.com>"]
description = "What this does"
license = "MIT"

# Dependencies (like requirements.txt)
[dependencies]
serde      = { version = "1",    features = ["derive"] }
serde_json = "1"
tokio      = { version = "1",    features = ["full"] }
reqwest    = { version = "0.12", features = ["json"] }
thiserror  = "1"

# Dev-only dependencies (like pytest, black — not shipped to users)
[dev-dependencies]
pretty_assertions = "1"
tempfile          = "3"

# Build script dependencies (rare)
[build-dependencies]
cc = "1"

# Optimisation profiles
[profile.dev]
opt-level = 0       # fast compile, slow binary

[profile.release]
opt-level = 3       # slow compile, fast binary
lto       = true    # link-time optimisation (smaller, faster binary)
strip     = true    # strip debug symbols (smaller binary)
```

### `logferry`'s `Cargo.toml`

```toml
[package]
name = "logferry"
version = "0.1.0"
edition = "2021"
description = "A multi-threaded JSON log ingestor, exposed to Python via PyO3."

[lib]
name = "logferry"
# cdylib  → the .so/.pyd Python actually imports
# rlib    → lets `cargo test` run the pure-Rust logic without a Python interpreter
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.20.3", features = ["extension-module"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[profile.release]
opt-level = 3
lto = true
```

Two things to notice:
- `crate-type = ["cdylib", "rlib"]` — both are required. `cdylib` produces the Python-importable `.so`; `rlib` allows `cargo test` to build a test harness without a Python interpreter.
- `features = ["extension-module"]` on `pyo3` — tells PyO3 not to statically link `libpython`. Python provides the symbols at import time. Omitting this causes crashes on some platforms.

---

## Build Profiles

Rust has two built-in profiles:

| Profile | Command | Compile speed | Binary speed | Use case |
|---|---|---|---|---|
| `dev` | `cargo build` | Fast | Slow (debug-checked) | Development, all tests |
| `release` | `cargo build --release` | Slow | Fast (fully optimised) | Benchmarks, distribution |

The dev profile also enables overflow checks and debug assertions, so bugs that would be silent in C are panics in Rust during development.

---

## Essential Commands Reference

```bash
# ── Project creation ────────────────────────────────────────
cargo new my_bin            # new binary project
cargo new my_lib --lib      # new library project
cargo init                  # initialise in existing directory

# ── Building ────────────────────────────────────────────────
cargo build                 # debug build (fast compile)
cargo build --release       # release build (optimised)
cargo check                 # type-check only (fastest — use this constantly)

# ── Running ─────────────────────────────────────────────────
cargo run                   # build (debug) and run
cargo run --release         # build (release) and run
cargo run -- --flag arg     # pass arguments to your binary
cargo run --example demo    # run examples/demo.rs

# ── Testing ─────────────────────────────────────────────────
cargo test                  # run all tests
cargo test my_fn            # run tests whose name contains "my_fn"
cargo test -- --nocapture   # show println! output during tests
cargo test -- --test-threads 1  # run tests sequentially

# ── Code quality ────────────────────────────────────────────
cargo fmt                   # format all files
cargo fmt --check           # check formatting without changing files (CI)
cargo clippy                # lint
cargo clippy -- -D warnings # lint; treat warnings as errors (CI)
cargo fix                   # automatically apply clippy suggestions

# ── Dependencies ────────────────────────────────────────────
cargo add serde             # add dependency
cargo add serde --features derive  # add with features
cargo remove serde          # remove dependency
cargo update                # update all to latest compatible versions
cargo tree                  # show dependency tree
cargo audit                 # check for known vulnerabilities

# ── Documentation ───────────────────────────────────────────
cargo doc                   # generate docs for your crate
cargo doc --open            # generate and open in browser
cargo doc --no-deps         # skip docs for dependencies

# ── Cleanup ─────────────────────────────────────────────────
cargo clean                 # delete target/ directory
```

---

## The `target/` Directory

```
target/
├── debug/       # output of `cargo build` (debug mode)
│   └── my_bin   # compiled binary
├── release/     # output of `cargo build --release`
│   └── my_bin   # optimised binary
└── .gitignore   # Cargo auto-creates this
```

Always add `target/` to your `.gitignore`:

```bash
echo "target/" >> .gitignore
```

The `target/` directory can reach several gigabytes for large projects. `cargo clean` deletes it. Cargo uses incremental compilation so rebuilds only what changed.

---

## See Also

- [Dependencies](dependencies.md) — adding and managing crates
- [Testing](testing.md) — `cargo test` in detail
- [Cargo Cheatsheet](../05-reference/cargo-cheatsheet.md) — full command reference including maturin
