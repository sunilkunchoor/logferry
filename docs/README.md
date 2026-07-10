# Rust for the Python Programmer

You already know Python. This guide teaches you Rust — starting from what you know, mapping every concept side by side, and ending with a real working project: a multi-threaded log ingestor that Python imports like any other library.

**No prior Rust experience required.** Each article shows the Python way first, then the idiomatic Rust equivalent, then explains what changed and why.

> **Running example:** [`logferry`](https://github.com/sunilkunchoor/logferry) — a multi-threaded JSON log parser built with Rust and PyO3. Every chapter builds toward understanding how it works and how to extend it.

---

## How to Read This Guide

| Your goal | Where to start |
|---|---|
| Completely new to Rust | [Chapter 1 — Getting Started](01-getting-started/) → read in order |
| Know Rust basics, want the Python bridge | [Chapter 4 — PyO3 and logferry](04-pyo3-and-logferry/) |
| Quick syntax lookup | [Python → Rust Cheatsheet](05-reference/python-to-rust-cheatsheet.md) |
| Something broke | [Troubleshooting](05-reference/troubleshooting.md) |

---

## Chapters

### [01 — Getting Started](01-getting-started/)
Go from zero to a working Rust project in one sitting.

| Article | Description |
|---|---|
| [What is Rust?](01-getting-started/what-is-rust.md) | Why Python developers learn Rust, what the GIL costs you, and what Rust gives back |
| [Installation](01-getting-started/installation.md) | Install Rust on Windows, macOS, and Linux; verify your setup |
| [Your First Project](01-getting-started/your-first-project.md) | `cargo new`, hello world, a JSON log counter, unit tests |
| [IDE Setup](01-getting-started/ide-setup.md) | VS Code + rust-analyzer, RustRover, Neovim |

---

### [02 — Rust Language Fundamentals](02-rust-language-fundamentals/)
A Python-first tour of the Rust language. Every article shows Python code first, then the Rust equivalent.

| Article | Python concept mapped | Rust concept learned |
|---|---|---|
| [Variables and Types](02-rust-language-fundamentals/variables-and-types.md) | `x = 10`, type hints | `let`, `mut`, fixed-width integers, `String` vs `&str` |
| [Functions and Control Flow](02-rust-language-fundamentals/functions-and-control-flow.md) | `def`, `if/elif/else`, `for`, `match` | `fn`, implicit return, `loop`, ranges, exhaustive `match` |
| [Ownership](02-rust-language-fundamentals/ownership.md) | GC / refcount | The one-owner rule, move semantics, `Copy`, `clone()` |
| [Borrowing and Slices](02-rust-language-fundamentals/borrowing-and-slices.md) | Passing by reference | `&T`, `&mut T`, borrow rules, zero-copy slices |
| [Structs and Enums](02-rust-language-fundamentals/structs-and-enums.md) | `@dataclass`, `Enum` | `struct`, `impl`, enums with data, `match` destructuring |
| [Error Handling](02-rust-language-fundamentals/error-handling.md) | `raise` / `try/except` | `Option<T>`, `Result<T,E>`, the `?` operator |
| [Collections](02-rust-language-fundamentals/collections.md) | `list`, `dict`, `set` | `Vec`, `HashMap`, `HashSet`, the `entry` API |
| [Closures and Iterators](02-rust-language-fundamentals/closures-and-iterators.md) | `lambda`, list comprehensions, `map/filter` | Closures, `move`, lazy iterators, adapters |
| [Traits and Generics](02-rust-language-fundamentals/traits-and-generics.md) | `Protocol`, `ABC`, `TypeVar` | `trait`, `Send + Sync`, generics, lifetimes (intro) |
| [Modules and Crates](02-rust-language-fundamentals/modules-and-crates.md) | `import`, packages | `mod`, `pub`, `use`, crates, `Cargo.toml` |

---

### [03 — Project Setup and Tooling](03-project-setup-and-tooling/)
`cargo` is `pip` + `virtualenv` + `pytest` + `black` in one tool. This chapter explains how.

| Article | Description |
|---|---|
| [Cargo In Depth](03-project-setup-and-tooling/cargo-in-depth.md) | `Cargo.toml` anatomy, build profiles, the full command reference |
| [Dependencies](03-project-setup-and-tooling/dependencies.md) | `cargo add`, crates.io, Python-to-Rust library analogue table |
| [Testing](03-project-setup-and-tooling/testing.md) | `#[test]`, integration tests, and pytest for PyO3 modules |
| [Daily Workflow](03-project-setup-and-tooling/daily-workflow.md) | Inner loop, `cargo watch`, before-commit checklist, CI config |

---

### [04 — PyO3 and logferry](04-pyo3-and-logferry/)
Write Rust, import it from Python. This chapter covers the full bridge — from a single function to a published wheel.

| Article | Description |
|---|---|
| [PyO3 Overview](04-pyo3-and-logferry/pyo3-overview.md) | What PyO3 is, project setup, the type conversion table |
| [Exposing Functions](04-pyo3-and-logferry/exposing-functions.md) | `#[pyfunction]`, type mapping, default arguments |
| [Exposing Classes](04-pyo3-and-logferry/exposing-classes.md) | `#[pyclass]`, `#[pymethods]`, read-only properties, `__repr__` |
| [Error Handling in PyO3](04-pyo3-and-logferry/error-handling-pyo3.md) | `PyResult`, `From<MyError>` for automatic exception conversion |
| [Multithreading and the GIL](04-pyo3-and-logferry/multithreading-and-gil.md) | `py.allow_threads`, `thread::scope`, why no `Arc<Mutex<>>` needed |
| [logferry Walkthrough](04-pyo3-and-logferry/logferry-walkthrough.md) | Annotated end-to-end tour of `src/lib.rs` — every decision explained |
| [Distribution](04-pyo3-and-logferry/distribution.md) | `maturin build`, cross-platform wheels, type stubs, mixed layouts |

---

### [05 — Reference](05-reference/)
Quick-lookup tables for when you know what you want but need the exact syntax.

| Article | Description |
|---|---|
| [Cargo Cheatsheet](05-reference/cargo-cheatsheet.md) | Every `cargo` and `maturin` command in one place |
| [Python → Rust Cheatsheet](05-reference/python-to-rust-cheatsheet.md) | Side-by-side syntax and concept lookup |
| [Troubleshooting](05-reference/troubleshooting.md) | Common compiler errors, PyO3 pitfalls, platform gotchas |

---

## What is logferry?

`logferry` is the running example that ties this guide together. It is a multi-threaded JSON log ingestor exposed to Python via [PyO3](https://pyo3.rs) — small enough to read in one sitting, but real enough to demonstrate every important Rust concept:

| Concept | Where it appears in logferry |
|---|---|
| Ownership across threads | `thread::scope` borrows `lines` into workers with no `Arc` |
| `Result`-based error handling | Parse errors counted and sampled; validation errors raised to Python |
| Traits as interfaces | `Validator: Send + Sync` — extensible, thread-safe, zero overhead |
| PyO3 type bridge | `IngestStats` fields auto-convert to Python `int`, `dict`, `float\|None` |

See the [project README](https://github.com/sunilkunchoor/logferry/blob/main/README.md) for the quick-start API reference.
