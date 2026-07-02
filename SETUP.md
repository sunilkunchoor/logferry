# Rust Developer Setup Guide

Complete setup from zero to a working Rust development environment on
Windows, Linux, and macOS — including IDE configuration, essential tools,
and first projects written from a Python developer's perspective.

---

## Table of Contents

1. [Installing Rust](#1-installing-rust)
   - [Linux](#linux)
   - [macOS](#macos)
   - [Windows](#windows)
2. [Verify the Installation](#2-verify-the-installation)
3. [Manage Your Toolchain with rustup](#3-manage-your-toolchain-with-rustup)
4. [Essential Components](#4-essential-components)
5. [IDE Setup](#5-ide-setup)
   - [VS Code (recommended)](#vs-code-recommended)
   - [RustRover / IntelliJ](#rustrover--intellij)
   - [Neovim](#neovim)
6. [Cargo — Rust's Package Manager](#6-cargo--rusts-package-manager)
7. [Your First Project](#7-your-first-project)
8. [Project Structure Deep Dive](#8-project-structure-deep-dive)
9. [Working with Dependencies](#9-working-with-dependencies)
10. [First Real Program — JSON Log Counter](#10-first-real-program--json-log-counter)
11. [First Library — a Reusable Module](#11-first-library--a-reusable-module)
12. [Setting up for PyO3 / logferry](#12-setting-up-for-pyo3--logferry)
13. [Daily Developer Workflow](#13-daily-developer-workflow)
14. [Useful cargo Commands Cheat Sheet](#14-useful-cargo-commands-cheat-sheet)

---

## 1. Installing Rust

Rust is installed and managed through **rustup** — the official toolchain
installer. Think of it as `pyenv` for Rust: it manages compiler versions,
lets you switch between stable/nightly channels, and installs extra
components like `rustfmt` and `clippy`.

---

### Linux

```bash
# Download and run the rustup installer script
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The installer walks you through options. Accept the defaults (press `1`).
When it finishes, reload your shell:

```bash
# Bash
source "$HOME/.cargo/env"

# Or add permanently to your shell profile
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc    # bash
echo 'source "$HOME/.cargo/env"' >> ~/.zshrc     # zsh
```

**What gets installed:**
- `~/.cargo/bin/` — cargo, rustc, rustup, and other tools
- `~/.rustup/` — toolchain files (compiler, standard library)

**Linux dependencies.** Some crates need a C linker. Install it once:

```bash
# Ubuntu / Debian
sudo apt-get update && sudo apt-get install -y build-essential pkg-config

# Fedora / RHEL
sudo dnf groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

---

### macOS

```bash
# Option 1: rustup (recommended — same as Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

```bash
# Option 2: Homebrew
brew install rust

# Note: Homebrew installs a fixed version; you won't get rustup's
# toolchain management. Prefer Option 1 for serious development.
```

macOS requires the Xcode Command Line Tools for the linker:

```bash
xcode-select --install
```

If you're on an Apple Silicon Mac (M1/M2/M3), the installer detects your
architecture automatically and installs the correct `aarch64-apple-darwin`
target.

---

### Windows

**Option 1: rustup-init.exe (recommended)**

1. Download the installer from [https://rustup.rs](https://rustup.rs)
2. Run `rustup-init.exe`
3. Accept the default installation when prompted

The installer adds `%USERPROFILE%\.cargo\bin` to your `PATH` automatically.
Open a new terminal after installation.

**Prerequisite — C++ Build Tools.** Rust needs the MSVC linker:

- Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- In the installer, select **"Desktop development with C++"**
- The minimum required components are: MSVC compiler, Windows SDK

Alternatively, if you prefer the GNU toolchain (MinGW):

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**Option 2: winget**

```powershell
winget install Rustlang.Rustup
```

**Option 3: Chocolatey**

```powershell
choco install rustup.install
```

**Option 4: Scoop**

```powershell
scoop install rustup
```

---

## 2. Verify the Installation

Run these in a new terminal (or after reloading your shell):

```bash
rustc --version
# rustc 1.78.0 (9b00956e5 2024-04-29)

cargo --version
# cargo 1.78.0 (54d8815d0 2024-04-09)

rustup --version
# rustup 1.27.0 (bbb9276d2 2024-03-08)
```

Compile and run a quick smoke test:

```bash
echo 'fn main() { println!("Rust is working!"); }' > /tmp/hello.rs
rustc /tmp/hello.rs -o /tmp/hello
/tmp/hello
# Rust is working!
```

---

## 3. Manage Your Toolchain with rustup

```bash
# Show what's installed
rustup show

# Update everything (compiler + components)
rustup update

# Install a specific version
rustup install 1.75.0

# Switch default toolchain
rustup default stable    # latest stable (recommended)
rustup default nightly   # nightly (needed for some experimental features)
rustup default 1.75.0    # pin to a specific version

# Override for one directory only (like pyenv's .python-version)
cd my-project
rustup override set 1.75.0

# Pin via file (committed to git — teammates get same version)
echo "1.75.0" > rust-toolchain.toml
# Or more explicit:
cat > rust-toolchain.toml << 'EOF'
[toolchain]
channel = "1.75.0"
components = ["rustfmt", "clippy"]
EOF

# Add a cross-compilation target
rustup target add aarch64-unknown-linux-gnu   # Linux ARM64 (e.g. AWS Graviton)
rustup target add x86_64-unknown-linux-musl   # static Linux binary
```

---

## 4. Essential Components

Install these once — they make day-to-day development much smoother:

```bash
# Code formatter (like Black for Python)
rustup component add rustfmt

# Linter with helpful suggestions (like flake8/pylint, but much smarter)
rustup component add clippy

# Language server — powers IDE autocompletion and inline errors
rustup component add rust-analyzer

# Source code for the standard library (enables "go to definition" into std)
rustup component add rust-src
```

Verify:

```bash
cargo fmt --version
cargo clippy --version
rust-analyzer --version
```

---

## 5. IDE Setup

### VS Code (recommended)

1. Install [VS Code](https://code.visualstudio.com/)
2. Install the **rust-analyzer** extension (Extension ID: `rust-lang.rust-analyzer`)
3. Optionally install **Even Better TOML** (`tamasfe.even-better-toml`) for
   Cargo.toml syntax highlighting

**Recommended VS Code settings** — add to `.vscode/settings.json` in your
project or to your user settings:

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

**What rust-analyzer gives you:**
- Inline type hints (like Pylance for Python)
- Autocompletion for methods, fields, and imports
- Red squiggles for compile errors as you type — no need to run `cargo build`
- Inline documentation on hover
- "Go to definition" / "Find all references" across your whole project
- Automatic import insertion
- Refactoring: rename, extract function, fill match arms

**Useful keyboard shortcuts (VS Code defaults):**

| Action | Mac | Windows / Linux |
|---|---|---|
| Go to definition | `F12` | `F12` |
| Show all references | `⇧F12` | `Shift+F12` |
| Rename symbol | `F2` | `F2` |
| Trigger suggestions | `⌃Space` | `Ctrl+Space` |
| Format document | `⇧⌥F` | `Shift+Alt+F` |
| Open Problems panel | `⇧⌘M` | `Ctrl+Shift+M` |

---

### RustRover / IntelliJ

JetBrains offers two options:

- **RustRover** — standalone IDE dedicated to Rust (free for non-commercial
  use). Download at [jetbrains.com/rust](https://www.jetbrains.com/rust/).
- **IntelliJ IDEA** + **Rust plugin** — if you already have IntelliJ.

RustRover has the same feature set as rust-analyzer but embedded in
JetBrains' polished UI, including integrated debugger (LLDB), profiler,
and test runner with coverage.

---

### Neovim

```bash
# Install rust-analyzer binary (if not already done via rustup)
rustup component add rust-analyzer

# In Neovim, using lazy.nvim + nvim-lspconfig:
# Add to your Lua config:
```

```lua
require("lspconfig").rust_analyzer.setup({
    settings = {
        ["rust-analyzer"] = {
            checkOnSave = { command = "clippy" },
            cargo = { features = "all" },
        },
    },
})
```

For a full Rust Neovim setup, the [rustaceanvim](https://github.com/mrcjkb/rustaceanvim)
plugin gives the richest experience.

---

## 6. Cargo — Rust's Package Manager

Cargo is `pip` + `virtualenv` + `setuptools` + `pytest` combined into one
tool. You'll use it for almost everything.

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

## 7. Your First Project

```bash
# Create a new binary project (like `python my_script.py`)
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

Other essential commands:

```bash
cargo build           # compile (debug mode, fast compile, slow binary)
cargo build --release # compile (optimised, slow compile, fast binary)
cargo check           # type-check without producing a binary (fastest feedback)
cargo test            # run all tests
cargo fmt             # format code (like Black)
cargo clippy          # lint (like flake8 but with fix suggestions)
cargo doc --open      # generate and open HTML documentation
cargo clean           # delete build artifacts (like __pycache__ cleanup)
```

---

## 8. Project Structure Deep Dive

```
my_project/
├── Cargo.toml          # package metadata and dependencies
├── Cargo.lock          # exact locked versions (commit this for binaries;
│                       #   .gitignore it for libraries)
├── rust-toolchain.toml # pin the compiler version (optional but recommended)
├── src/
│   ├── main.rs         # binary entry point (fn main)
│   ├── lib.rs          # library root (if it's also a library)
│   └── metrics/        # sub-module as a directory
│       ├── mod.rs      # module root (alternative: metrics.rs at src/ level)
│       ├── precision.rs
│       └── recall.rs
├── tests/
│   └── integration_test.rs  # integration tests (separate from unit tests)
├── examples/
│   └── demo.rs         # runnable examples: `cargo run --example demo`
└── benches/
    └── throughput.rs   # benchmarks: `cargo bench`
```

### Cargo.toml anatomy

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

---

## 9. Working with Dependencies

```bash
# Add a dependency (updates Cargo.toml and fetches it)
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full

# Remove a dependency
cargo remove tokio

# Update all dependencies to latest compatible versions
cargo update

# Update one specific dependency
cargo update -p serde

# See the full dependency tree (like pip show / pipdeptree)
cargo tree

# Search crates.io from the terminal
cargo search "json parser"

# Check for outdated dependencies (needs cargo-outdated)
cargo install cargo-outdated
cargo outdated
```

### Finding packages

- **[crates.io](https://crates.io)** — the official registry (like PyPI)
- **[lib.rs](https://lib.rs)** — better search and category browsing
- **[docs.rs](https://docs.rs)** — auto-generated API docs for every crate

**Commonly used crates by Python analogy:**

| Python | Rust crate | Purpose |
|---|---|---|
| `json` | `serde_json` | JSON parse / serialise |
| `pydantic` | `serde` + `derive` | Struct ↔ JSON/YAML/etc. |
| `requests` | `reqwest` | HTTP client |
| `asyncio` | `tokio` | Async runtime |
| `click` | `clap` | CLI argument parsing |
| `logging` | `tracing` | Structured logging |
| `datetime` | `chrono` | Date and time |
| `pathlib` | `std::path::PathBuf` | File paths (stdlib) |
| `re` | `regex` | Regular expressions |
| `uuid` | `uuid` | UUID generation |
| `dotenv` | `dotenvy` | `.env` file loading |
| `psycopg2` | `sqlx` | PostgreSQL async |
| `redis-py` | `redis` | Redis client |
| `pytest` | built-in `#[test]` + `cargo test` | Testing |

---

## 10. First Real Program — JSON Log Counter

Let's build something realistic: a CLI tool that reads JSON log lines from
stdin and prints a count-by-level summary. This mirrors what `logferry`
does, but as a standalone binary you can run from the shell.

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

/// One JSON log line coming from an ML service.
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

# Pipe some fake logs through it
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

Add unit tests at the bottom of `main.rs`:

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

## 11. First Library — a Reusable Module

Libraries use `src/lib.rs` as the root instead of `src/main.rs`. They're
imported by other Rust crates — or, via PyO3, by Python.

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
///
/// # Examples
///
/// ```
/// use metrics_lib::precision;
/// assert!((precision(8.0, 2.0).unwrap() - 0.8).abs() < 1e-9);
/// ```
pub fn precision(tp: f64, fp: f64) -> Option<f64> {
    let denom = tp + fp;
    if denom == 0.0 { None } else { Some(tp / denom) }
}

/// Computes recall: TP / (TP + FN).
pub fn recall(tp: f64, fn_: f64) -> Option<f64> {
    let denom = tp + fn_;
    if denom == 0.0 { None } else { Some(tp / denom) }
}

/// Computes F1 score from precision and recall.
pub fn f1(p: f64, r: f64) -> f64 {
    if p + r == 0.0 { return 0.0; }
    2.0 * p * r / (p + r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precision_basic() {
        assert!((precision(8.0, 2.0).unwrap() - 0.8).abs() < 1e-9);
    }

    #[test]
    fn precision_undefined_when_no_predictions() {
        assert!(precision(0.0, 0.0).is_none());
    }

    #[test]
    fn f1_is_harmonic_mean() {
        let p = precision(8.0, 2.0).unwrap();  // 0.8
        let r = recall(8.0, 2.0).unwrap();     // 0.8
        assert!((f1(p, r) - 0.8).abs() < 1e-9);
    }
}
```

```bash
cargo test
# test tests::f1_is_harmonic_mean ... ok
# test tests::precision_basic ... ok
# test tests::precision_undefined_when_no_predictions ... ok

# Generate and open HTML documentation
cargo doc --open
# The /// doc comments become rendered API docs, just like Python docstrings → Sphinx
```

Use it from a binary in the same workspace or as a dependency in `Cargo.toml`:

```toml
[dependencies]
metrics_lib = { path = "../metrics_lib" }
```

---

## 12. Setting up for PyO3 / logferry

PyO3 lets you call Rust from Python. The build tool that bridges the two is
**maturin**.

```bash
# Install maturin (do this once per Python environment)
pip install maturin

# Create a new mixed Rust+Python project
maturin new my_extension --bindings pyo3
cd my_extension
```

Or to work with the existing `logferry` project:

```bash
# Clone the repo
git clone <your-repo-url>
cd logferry

# Create a virtual environment
python -m venv .venv

# Activate it
source .venv/bin/activate        # Linux / macOS
.venv\Scripts\activate           # Windows PowerShell

# Install maturin into the venv
pip install maturin

# Compile Rust and install the extension into the active venv
maturin develop --release

# Verify it works
python -c "import logferry; print(dir(logferry))"
```

**Development loop:**

```bash
# After editing src/lib.rs:
maturin develop --release   # recompile and reinstall (takes ~5–30s)
python python_demo.py        # test the changes
```

**Checking Rust code without rebuilding the Python module:**

```bash
cargo check    # fast type-check — catches most errors in < 2s
cargo test     # run pure-Rust unit tests (no Python needed)
cargo clippy   # lint
cargo fmt      # format
```

---

## 13. Daily Developer Workflow

Here's what a typical Rust development session looks like, compared to
Python:

```
Python workflow              Rust workflow
─────────────────────────    ─────────────────────────────
activate venv                (nothing to activate)
write code                   write code
python script.py             cargo run
pytest                       cargo test
black . && flake8 .          cargo fmt && cargo clippy
push to CI                   push to CI (cargo test + clippy in CI)
```

### Recommended inner loop

```bash
# In one terminal — watch for changes and re-check continuously
# (install cargo-watch first: cargo install cargo-watch)
cargo watch -x check         # re-runs `cargo check` on every file save
cargo watch -x test          # re-runs tests on every save
cargo watch -x "run -- args" # re-runs the binary on every save
```

### Before committing

```bash
cargo fmt                    # format (non-negotiable)
cargo clippy -- -D warnings  # lint; -D warnings fails on any warning
cargo test                   # all tests must pass
cargo build --release        # verify the optimised build works too
```

### Useful one-off tools (install once with `cargo install`)

```bash
cargo install cargo-watch     # re-run commands on file changes
cargo install cargo-expand    # show what macros expand to (debugging macros)
cargo install cargo-audit     # check dependencies for known vulnerabilities
cargo install cargo-outdated  # show which deps have newer versions
cargo install cargo-flamegraph # generate flame graphs for profiling
cargo install cargo-nextest   # faster test runner with better output
```

---

## 14. Useful cargo Commands Cheat Sheet

```bash
# ── Project creation ────────────────────────────────────────
cargo new my_bin            # new binary project
cargo new my_lib --lib      # new library project
cargo init                  # initialise in existing directory
maturin new ext --bindings pyo3   # new PyO3 extension project

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
cargo nextest run           # faster alternative (needs cargo-nextest)

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
cargo update                # update all dependencies
cargo tree                  # show dependency tree
cargo audit                 # check for vulnerabilities

# ── Documentation ───────────────────────────────────────────
cargo doc                   # generate docs for your crate
cargo doc --open            # generate and open in browser
cargo doc --no-deps         # skip docs for dependencies

# ── Profiling / analysis ────────────────────────────────────
cargo build --release && cargo flamegraph   # flame graph (needs cargo-flamegraph)

# ── Cleanup ─────────────────────────────────────────────────
cargo clean                 # delete target/ directory

# ── PyO3 / maturin ──────────────────────────────────────────
maturin develop             # build extension (debug) and install into venv
maturin develop --release   # build extension (optimised) and install
maturin build --release     # build distributable wheel
maturin publish             # build and publish to PyPI
```

---

## Environment at a Glance

After setup, you should have:

```
~/.cargo/bin/
├── cargo        # build tool and package manager
├── rustc        # compiler
├── rustup       # toolchain manager
├── rustfmt      # formatter
├── cargo-clippy # linter (invoked as `cargo clippy`)
└── rust-analyzer # language server (used by your IDE)

~/.rustup/toolchains/stable-*/
└── ...          # compiler, standard library, docs
```

And in each project:

```
my_project/
├── target/      # build output (like __pycache__; add to .gitignore)
│   ├── debug/   # debug builds
│   └── release/ # release builds
└── Cargo.lock   # exact locked dependency versions
```

Add `target/` to `.gitignore`:

```bash
echo "target/" >> .gitignore
```

---

## Troubleshooting

**`cargo` not found after installation**

```bash
source "$HOME/.cargo/env"          # Linux / macOS
# Or restart your terminal
```

**`linker 'cc' not found` on Linux**

```bash
sudo apt-get install build-essential   # Ubuntu / Debian
sudo dnf groupinstall "Development Tools"  # Fedora
```

**`linker 'link.exe' not found` on Windows**

Install Visual Studio Build Tools with the "Desktop development with C++"
workload, or switch to the GNU toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**`maturin develop` fails with `python not found`**

Make sure your virtual environment is activated before running maturin:

```bash
source .venv/bin/activate
maturin develop --release
```

**Slow compile times**

Rust's compile times improve dramatically with a few settings. Create
`.cargo/config.toml` in your project:

```toml
[build]
# Use the faster mold linker (Linux) or lld (cross-platform)
# Install: sudo apt-get install mold   or   cargo install -f cargo-binutils
# Linux:
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]
# macOS / Windows with lld:
# rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[profile.dev]
# Reduce optimisation in dev builds for faster iteration
opt-level = 0
debug = 1   # less debug info = faster link
```

**rust-analyzer not working in VS Code**

1. Make sure the `rust-analyzer` component is installed: `rustup component add rust-analyzer`
2. Reload the VS Code window: `Ctrl+Shift+P` → "Developer: Reload Window"
3. Check the rust-analyzer output panel: `Ctrl+Shift+P` → "rust-analyzer: Show RA server logs"
