# Chapter 2 — Rust Language Fundamentals

This chapter teaches the Rust language by mapping every concept to its Python equivalent. Each article shows the Python way first, then the idiomatic Rust way, then explains what changed and why.

Examples lean on MLOps scenarios — log parsing, config loading, data pipelines — so the context stays familiar.

## Articles

| Article | Topics covered |
|---|---|
| [Variables and Types](variables-and-types.md) | `let`, `mut`, shadowing, primitives, `String` vs `&str`, formatting |
| [Functions and Control Flow](functions-and-control-flow.md) | `fn`, implicit return, `if`/`else`, `loop`, `while`, `for`, ranges |
| [Ownership](ownership.md) | The three ownership rules, move semantics, `Copy`, `clone()` |
| [Borrowing and Slices](borrowing-and-slices.md) | `&T`, `&mut T`, the borrow rules, slices |
| [Structs and Enums](structs-and-enums.md) | `struct`, `impl`, methods, enums with data, `match` |
| [Error Handling](error-handling.md) | `Option<T>`, `Result<T,E>`, the `?` operator |
| [Collections](collections.md) | `Vec`, `HashMap`, `HashSet` |
| [Closures and Iterators](closures-and-iterators.md) | Closures, `move`, lazy iterators, adapters |
| [Traits and Generics](traits-and-generics.md) | `trait`, `impl Trait`, `Box<dyn Trait>`, `Send + Sync`, generics, lifetimes |
| [Modules and Crates](modules-and-crates.md) | `mod`, `pub`, `use`, external crates, `Cargo.toml` |

## Prerequisites

Chapters 1 — [Getting Started](../01-getting-started/) should be complete. You should have Rust installed and be able to run `cargo new` and `cargo build`.

## Next

After completing this chapter, move on to [Chapter 3 — Project Setup and Tooling](../03-project-setup-and-tooling/) or jump straight to [Chapter 4 — PyO3 and logferry](../04-pyo3-and-logferry/) if you are eager to see how Rust talks to Python.
