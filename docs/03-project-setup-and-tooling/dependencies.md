# Dependencies

> **Prerequisites:** [Cargo In Depth](cargo-in-depth.md)  
> **Next:** [Testing](testing.md)

This article covers how to find, add, and manage Rust dependencies — the equivalent of `pip`, `PyPI`, and `requirements.txt`.

---

## Finding Packages

- **[crates.io](https://crates.io)** — the official Rust package registry (like PyPI)
- **[lib.rs](https://lib.rs)** — better search UI and category browsing
- **[docs.rs](https://docs.rs)** — auto-generated API docs for every published crate version

Search from the terminal:

```bash
cargo search "json parser"
```

---

## Adding Dependencies

```bash
# Add a dependency (updates Cargo.toml and fetches the crate)
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full

# Remove a dependency
cargo remove tokio

# Update all dependencies to latest compatible versions (respects semver)
cargo update

# Update one specific dependency
cargo update -p serde

# See the full dependency tree (like pipdeptree)
cargo tree

# Check for outdated dependencies (needs cargo-outdated)
cargo install cargo-outdated
cargo outdated

# Check for known security vulnerabilities
cargo install cargo-audit
cargo audit
```

---

## Python to Rust Crate Analogues

| Python library | Rust crate | Purpose |
|---|---|---|
| `json` | `serde_json` | JSON parse / serialise |
| `pydantic` | `serde` + derive | Struct ↔ JSON/YAML/etc. |
| `requests` | `reqwest` | HTTP client |
| `asyncio` | `tokio` | Async runtime |
| `click` / `argparse` | `clap` | CLI argument parsing |
| `logging` | `tracing` | Structured logging |
| `datetime` | `chrono` | Date and time |
| `pathlib` | `std::path::PathBuf` | File paths (stdlib) |
| `re` | `regex` | Regular expressions |
| `uuid` | `uuid` | UUID generation |
| `python-dotenv` | `dotenvy` | `.env` file loading |
| `psycopg2` | `sqlx` | PostgreSQL async |
| `redis-py` | `redis` | Redis client |
| `pytest` | built-in `#[test]` + `cargo test` | Testing |
| `PyO3` / `cffi` | `pyo3` | Python ↔ Rust bridge |

---

## Specifying Version Requirements

Rust uses semantic versioning. In `Cargo.toml`:

```toml
[dependencies]
serde = "1"          # >= 1.0.0, < 2.0.0  (compatible release)
serde = "1.0"        # >= 1.0.0, < 1.1.0  (more restrictive)
serde = "=1.0.195"   # exact version
serde = ">=1.0, <2"  # explicit range
serde = "*"          # any version (avoid — too permissive)
```

### Features

Some crates have optional features (like extras in Python):

```toml
[dependencies]
serde      = { version = "1", features = ["derive"] }   # enables #[derive(Serialize, Deserialize)]
tokio      = { version = "1", features = ["full"] }     # enables all tokio features
pyo3       = { version = "0.20", features = ["extension-module"] }
```

```bash
# Add a crate with a specific feature enabled
cargo add serde --features derive
```

---

## `Cargo.lock` — Exact Reproducible Builds

`Cargo.lock` records the exact version of every transitive dependency, like `pip freeze`. Cargo updates it whenever you run `cargo build` with new or changed dependencies.

**Convention:**
- **Binary projects** (executables): commit `Cargo.lock` so every developer and CI run uses identical versions.
- **Library crates** (no `fn main`): `.gitignore` `Cargo.lock` — let downstream users pick compatible versions within your declared ranges.

`logferry` is a library (from Cargo's perspective), but it has a Python-facing binary build. Committing `Cargo.lock` is reasonable either way.

---

## Dev and Build Dependencies

```toml
# Dev dependencies — used in tests and examples, not shipped to users
[dev-dependencies]
pretty_assertions = "1"   # nicer diff output in test failures
tempfile          = "3"   # temporary files and directories for tests

# Build dependencies — used by build.rs scripts only
[build-dependencies]
cc = "1"   # C compiler wrapper (for crates with C code)
```

---

## Private and Path Dependencies

```toml
[dependencies]
# Local path — useful for multi-crate workspaces or developing two crates together
metrics_lib = { path = "../metrics_lib" }

# Git dependency — pin to a specific commit or tag
my_crate = { git = "https://github.com/user/my_crate", tag = "v1.2.0" }
```

---

## Workspaces

For projects with multiple related crates (e.g. a binary + a library), use a Cargo workspace:

```toml
# Workspace-level Cargo.toml (at the repo root)
[workspace]
members = [
    "logferry",
    "logferry-cli",
    "metrics_lib",
]
```

```bash
# Build all workspace members
cargo build --workspace

# Test all workspace members
cargo test --workspace
```

---

## See Also

- [Testing](testing.md) — using dev dependencies in tests
- [Cargo Cheatsheet](../05-reference/cargo-cheatsheet.md) — all cargo commands
- [Python → Rust Cheatsheet](../05-reference/python-to-rust-cheatsheet.md) — full library analogy table
