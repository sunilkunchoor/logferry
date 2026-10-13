# Rust Basics - Runnable Examples

A guided tour of the Rust fundamentals. Each file is a small, self-contained
program with heavy comments and documentation, written for beginners. Read the
file, run it, change it, break it, and read the compiler's error message.

> `demo.rs` in this folder is the logferry-specific demo. Everything numbered
> `01`-`18` is general Rust and uses only the standard library.

## Prerequisites

- Rust toolchain: <https://rustup.rs> (check with `cargo --version`)
- This project depends on PyO3, which needs a Python 3 interpreter at build
  time. If `cargo` says `no Python 3.x interpreter found`, point it at one:

  ```bash
  # bash
  export PYO3_PYTHON="$PWD/.venv/Scripts/python.exe"
  ```
  ```powershell
  # PowerShell
  $env:PYO3_PYTHON = "$PWD\.venv\Scripts\python.exe"
  ```

## How to run

```bash
cargo run --example 01_variables_types          # run one example
cargo run --example 16_io_env -- one two        # pass arguments after --
cargo test --example 17_testing                 # run the tests inside 17
cargo build --examples                          # compile them all
```

Run them all (bash):

```bash
for f in examples/[0-9]*.rs; do
  n=$(basename "$f" .rs); echo "=== $n"; cargo run -q --example "$n"
done
```

Run them all (PowerShell):

```powershell
Get-ChildItem examples\[0-9]*.rs | ForEach-Object {
  Write-Host "=== $($_.BaseName)"; cargo run -q --example $_.BaseName
}
```

## Generate HTML documentation

Every file has `//!` module docs and `///` item docs. Cargo does not document
examples by default, so pass `--examples`:

```bash
cargo doc --examples --no-deps --document-private-items --open
```

You get one page per example (`01_variables_types`, `02_control_flow`, ...)
with the explanations, tables and function docs rendered.

## The examples

Suggested order - each builds on the previous ones.

| # | File | Concepts |
|---|------|----------|
| 01 | [01_variables_types.rs](01_variables_types.rs) | `let`, `mut`, shadowing, `const`/`static`, integer/float/bool/char, tuples, arrays, `as` casts, overflow |
| 02 | [02_control_flow.rs](02_control_flow.rs) | `if` as an expression, `loop`/`while`/`for`, ranges, `break` with value, labels |
| 03 | [03_functions.rs](03_functions.rs) | parameters, return values, expressions vs statements, closures, `move`, function pointers, `impl Fn` |
| 04 | [04_ownership.rs](04_ownership.rs) | ownership rules, move/clone/`Copy`, `&` and `&mut` borrowing, slices |
| 05 | [05_lifetimes.rs](05_lifetimes.rs) | `'a` annotations, elision rules, structs holding references, `'static` |
| 06 | [06_structs_enums.rs](06_structs_enums.rs) | structs, `impl`, methods vs associated functions, enums with data, `Option` |
| 07 | [07_pattern_matching.rs](07_pattern_matching.rs) | `match`, guards, `@`, destructuring, `if let`, `while let`, `let else` |
| 08 | [08_strings_text.rs](08_strings_text.rs) | `&str` vs `String`, UTF-8, `split`/`trim`/`parse`, formatting |
| 09 | [09_collections.rs](09_collections.rs) | `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `VecDeque`, `entry` API |
| 10 | [10_error_handling.rs](10_error_handling.rs) | `Result`, `?`, custom errors, `From`, `Box<dyn Error>`, `panic!` |
| 11 | [11_traits_generics.rs](11_traits_generics.rs) | traits, default methods, generics, bounds, `impl Trait`, `dyn Trait` |
| 12 | [12_std_traits.rs](12_std_traits.rs) | `derive`, `Display`, `From`/`TryFrom`, operator overloading, `Drop` |
| 13 | [13_iterators.rs](13_iterators.rs) | adapters, consumers, `collect`, writing your own `Iterator` |
| 14 | [14_modules_smart_pointers.rs](14_modules_smart_pointers.rs) | `mod`/`pub`/`use`, `Box`, `Rc`, `RefCell`, `Weak` |
| 15 | [15_concurrency.rs](15_concurrency.rs) | threads, channels, `Arc<Mutex<T>>`, `RwLock`, atomics, scoped threads |
| 16 | [16_io_env.rs](16_io_env.rs) | args, env vars, stdin, files, buffered I/O, paths |
| 17 | [17_testing.rs](17_testing.rs) | `#[test]`, `assert!` family, `#[should_panic]`, doc comments |
| 18 | [18_macros_unsafe.rs](18_macros_unsafe.rs) | built-in macros, `macro_rules!`, `cfg`, raw pointers, `unsafe` |

## The big ideas in two minutes

### 1. Immutable by default
```rust
let x = 5;        // cannot change
let mut y = 5;    // can change
y += 1;
```

### 2. Ownership: one owner, freed when it goes out of scope
```rust
let a = String::from("hi");
let b = a;            // ownership MOVES to b; `a` is now unusable
let c = b.clone();    // explicit deep copy; both usable
```

### 3. Borrowing: look without owning
```rust
fn len(s: &String) -> usize { s.len() }     // shared borrow: many allowed
fn push(s: &mut String) { s.push('!') }     // exclusive borrow: only one
```
Rule: **many readers OR one writer**, never both at once.

### 4. No null, no exceptions
```rust
let maybe: Option<i32> = None;               // absence is a type
let parsed: Result<i32, _> = "x".parse::<i32>(); // failure is a value
```
Handle them with `match`, `if let`, `?`, `unwrap_or`...

### 5. Enums + `match` are exhaustive
```rust
enum Shape { Circle(f64), Square(f64) }
match shape {
    Shape::Circle(r) => 3.14 * r * r,
    Shape::Square(s) => s * s,
}   // forget a variant -> compile error
```

### 6. Traits describe behaviour; generics reuse code
```rust
fn largest<T: PartialOrd>(items: &[T]) -> &T { /* ... */ }
```

### 7. Iterators are lazy and fast
```rust
let sum: i32 = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).sum();
```

### 8. Fearless concurrency
The compiler rejects data races. Share with `Arc`, protect with `Mutex`, or
pass messages through channels.

## Reading compiler errors

Rust's errors are teachers. Typical ones you will meet in these files:

| Error | Meaning | Usual fix |
|-------|---------|-----------|
| `E0384` cannot assign twice to immutable variable | missing `mut` | add `mut` |
| `E0382` use of moved value | value was moved | borrow `&x` or `.clone()` |
| `E0499` / `E0502` borrow conflicts | readers and a writer overlap | shorten borrows, restructure |
| `E0106` missing lifetime specifier | compiler cannot link output to input | add `'a` |
| `E0004` non-exhaustive patterns | `match` misses a case | add an arm or `_` |
| `E0277` trait bound not satisfied | type lacks a needed trait | `derive`/implement it |

Run `rustc --explain E0382` for a full explanation of any code.

## Where next?

- Async Rust (`async`/`await`, Tokio)
- Cargo features, workspaces, and publishing crates
- FFI and Python bindings with PyO3 - see the main logferry
  [README](../README.md) and `src/`
- Crates worth knowing: `serde`, `clap`, `anyhow`/`thiserror`, `rayon`, `tokio`

## Learn more

- The Rust Book: <https://doc.rust-lang.org/book/>
- Rust by Example: <https://doc.rust-lang.org/rust-by-example/>
- Rustlings (exercises): <https://github.com/rust-lang/rustlings>
- Standard library docs: <https://doc.rust-lang.org/std/>
