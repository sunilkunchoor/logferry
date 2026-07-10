# Chapter 4 — PyO3 and logferry

This chapter covers the PyO3 bridge between Rust and Python, using `logferry` as the concrete working example throughout.

## Articles

| Article | Topics covered |
|---|---|
| [PyO3 Overview](pyo3-overview.md) | What PyO3 is, project setup, type mapping table |
| [Exposing Functions](exposing-functions.md) | `#[pyfunction]`, signatures, default arguments |
| [Exposing Classes](exposing-classes.md) | `#[pyclass]`, `#[pymethods]`, getters, `__repr__` |
| [Error Handling in PyO3](error-handling-pyo3.md) | `PyResult`, `From<MyError>`, Python exception types |
| [Multithreading and the GIL](multithreading-and-gil.md) | `py.allow_threads`, `thread::scope`, `Send + Sync` |
| [logferry Walkthrough](logferry-walkthrough.md) | End-to-end walk through `src/lib.rs` |
| [Distribution](distribution.md) | `maturin build`, wheels, type stubs, mixed layouts |

## Prerequisites

[Chapter 2 — Rust Language Fundamentals](../02-rust-language-fundamentals/), especially [Error Handling](../02-rust-language-fundamentals/error-handling.md) and [Traits and Generics](../02-rust-language-fundamentals/traits-and-generics.md).

## Next

[Chapter 5 — Reference](../05-reference/) for quick-lookup tables and troubleshooting.
