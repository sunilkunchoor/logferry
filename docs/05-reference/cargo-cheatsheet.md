# Cargo Cheatsheet

Full command reference for `cargo` and `maturin`.

---

## Project Creation

```bash
cargo new my_bin            # new binary project (has fn main)
cargo new my_lib --lib      # new library project (has lib.rs)
cargo init                  # initialise in existing directory

maturin new ext --bindings pyo3   # new PyO3 extension project
```

---

## Building

```bash
cargo build                 # debug build (fast compile, unoptimised binary)
cargo build --release       # release build (slow compile, optimised binary)
cargo check                 # type-check only — fastest, no binary produced
cargo build --workspace     # build all workspace members
```

---

## Running

```bash
cargo run                   # build (debug) and run
cargo run --release         # build (release) and run
cargo run -- --flag arg     # pass arguments to your binary after --
cargo run --example demo    # run examples/demo.rs
cargo run -p my_bin         # run a specific workspace member
```

---

## Testing

```bash
cargo test                  # run all tests
cargo test my_fn            # run tests whose name contains "my_fn"
cargo test -- --nocapture   # show println! output during tests
cargo test -- --test-threads 1  # run tests sequentially
cargo test -- --list        # list all test names without running
cargo test --test my_file   # run a specific integration test file
cargo test --workspace      # test all workspace members
cargo nextest run           # faster alternative (needs cargo-nextest)
```

---

## Code Quality

```bash
cargo fmt                   # format all files (like Black)
cargo fmt --check           # check formatting without changing files (CI)
cargo clippy                # lint (like flake8 + pylint, with fix suggestions)
cargo clippy -- -D warnings # lint; treat warnings as errors (CI)
cargo fix                   # automatically apply clippy suggestions
```

---

## Dependencies

```bash
cargo add serde             # add dependency
cargo add serde --features derive  # add with specific features
cargo add tokio --features full    # add multiple features
cargo remove serde          # remove dependency
cargo update                # update all to latest compatible versions
cargo update -p serde       # update one specific dependency
cargo tree                  # show full dependency tree
cargo audit                 # check for known security vulnerabilities
cargo outdated              # show which deps have newer versions (needs cargo-outdated)
```

---

## Documentation

```bash
cargo doc                   # generate docs for your crate
cargo doc --open            # generate and open in browser
cargo doc --no-deps         # skip docs for dependencies
cargo doc --workspace       # generate docs for all workspace members
```

---

## Profiling and Analysis

```bash
cargo build --release && cargo flamegraph  # flame graph (needs cargo-flamegraph)
cargo expand                               # show macro expansions (needs cargo-expand)
```

---

## Cleanup

```bash
cargo clean                 # delete target/ directory
```

---

## PyO3 / Maturin

```bash
# Development
maturin develop             # build extension (debug) and install into active venv
maturin develop --release   # build extension (optimised) and install

# Distribution
maturin build --release     # build distributable wheel for current platform
maturin build --release --out dist  # specify output directory

# Publishing
maturin publish             # build and publish to PyPI
maturin publish --repository testpypi  # publish to TestPyPI first

# Scaffolding
maturin new ext --bindings pyo3   # create a new PyO3 project
maturin init --bindings pyo3      # initialise PyO3 in existing directory
```

---

## One-Off Tools (Install Once)

```bash
cargo install cargo-watch     # re-run commands on file changes
cargo install cargo-expand    # show what macros expand to
cargo install cargo-audit     # check dependencies for known vulnerabilities
cargo install cargo-outdated  # show which deps have newer versions
cargo install cargo-flamegraph # generate flame graphs for profiling
cargo install cargo-nextest   # faster test runner with better output
cargo install cargo-bloat     # analyse binary size
```

---

## Useful Compound Commands

```bash
# Run check on every file save
cargo watch -x check

# Run tests on every file save
cargo watch -x test

# Run the binary on every file save (with args)
cargo watch -x "run -- --verbose"

# Full pre-commit check
cargo fmt && cargo clippy -- -D warnings && cargo test && cargo build --release
```

---

## rustup Commands

```bash
rustup show                             # show installed toolchains
rustup update                           # update everything
rustup default stable                   # switch to latest stable
rustup override set 1.75.0              # pin one directory to a specific version
rustup component add rustfmt clippy rust-analyzer rust-src
rustup target add aarch64-unknown-linux-gnu    # cross-compilation target
```
