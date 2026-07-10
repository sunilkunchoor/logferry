# Troubleshooting

Common errors, platform-specific gotchas, and PyO3 pitfalls.

---

## Installation and Toolchain

### `cargo` not found after installation

```bash
source "$HOME/.cargo/env"   # Linux / macOS
# Or restart your terminal (Windows)
```

### `linker 'cc' not found` on Linux

```bash
sudo apt-get install build-essential   # Ubuntu / Debian
sudo dnf groupinstall "Development Tools"  # Fedora / RHEL
sudo pacman -S base-devel              # Arch
```

### `linker 'link.exe' not found` on Windows

Install Visual Studio Build Tools with the "Desktop development with C++" workload. Or switch to the GNU toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### `rustup update` fails or hangs

```bash
# Force re-download of the toolchain
rustup self update   # update rustup itself first
rustup update stable
```

---

## Compile Errors

### `cannot assign twice to immutable variable`

You forgot `mut`:

```rust
// ❌
let x = 10;
x = 20;   // error

// ✅
let mut x = 10;
x = 20;
```

### `use of moved value`

The value was moved into a function or another variable. Either clone it or borrow it:

```rust
// ❌
let a = vec![1, 2, 3];
let b = a;           // a is moved
println!("{:?}", a); // error

// ✅ — borrow
let b = &a;
println!("{:?}", a); // still valid

// ✅ — clone
let b = a.clone();
println!("{:?}", a); // still valid
```

### `mismatched types` when mixing `String` and `&str`

```rust
// ❌
fn greet(name: String) { ... }
greet("alice");   // "alice" is &str, not String

// ✅
fn greet(name: &str) { ... }   // accept &str — more flexible
greet("alice");
greet(&my_string);   // &String coerces to &str
```

### `borrowed value does not live long enough`

A reference outlives the value it points to. The classic fix is to move the value into a binding that lives longer:

```rust
// ❌
let result;
{
    let s = String::from("hello");
    result = &s;   // s is dropped at end of block
}
println!("{result}");   // error: s is gone

// ✅
let s = String::from("hello");   // s lives in the outer scope
let result = &s;
println!("{result}");
```

### `the trait bound T: Send is not satisfied`

You are trying to share a non-`Send` type across threads. Common culprits:
- `Rc<T>` — use `Arc<T>` instead
- `RefCell<T>` — use `Mutex<T>` or `RwLock<T>` instead
- Raw pointers — wrap in a struct that implements `Send` manually (advanced)

---

## PyO3 / Maturin Errors

### `extension-module` feature missing — segfault or import error

```toml
# ✅ Required in Cargo.toml
[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
```

Forgetting `extension-module` causes a double-link of `libpython` and a segfault or import error on some platforms.

### `cargo test` fails with linker errors on a `cdylib`-only crate

```toml
# ✅ Required — add rlib so cargo test can build a test harness
[lib]
crate-type = ["cdylib", "rlib"]
```

### `#[pymodule]` function name does not match crate name

```toml
# Cargo.toml
[lib]
name = "logferry"   # the Python module name
```

```rust
// lib.rs — function name must match [lib] name
#[pymodule]
fn logferry(_py: Python<'_>, m: &PyModule) -> PyResult<()> { ... }
//  ^^^^^^^^ must match Cargo.toml [lib] name
```

Mismatch causes `ImportError: cannot import name 'logferry'`.

### `maturin develop` fails with `python not found`

Make sure your virtual environment is activated before running maturin:

```bash
source .venv/bin/activate        # Linux / macOS
.venv\Scripts\activate           # Windows PowerShell
maturin develop --release
```

### `unwrap()` panics the Python interpreter

Never use `unwrap()` in production PyO3 code. Use `?` and return `PyResult`:

```rust
// ❌ — panics the interpreter on error
let n: i64 = s.parse().unwrap();

// ✅ — converts to a Python ValueError
let n: i64 = s.parse().map_err(|e| PyValueError::new_err(format!("{e}")))?;
```

### Returning `&str` from a function that borrows a local variable

```rust
// ❌ — borrow checker rejects this
#[pyfunction]
fn greet(name: &str) -> &str {
    let result = format!("Hello, {name}!");
    &result   // result is a local — it will be dropped
}

// ✅ — return an owned String; PyO3 converts it to Python str
#[pyfunction]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

### `PyErr` construction inside `py.allow_threads`

Some `PyErr::new_err(...)` calls need the GIL. Construct errors before the `allow_threads` block or use lazy construction:

```rust
// ❌ — may panic inside allow_threads
let result = py.allow_threads(|| {
    if bad { return Err(PyValueError::new_err("bad")); }  // GIL not held
    Ok(42)
});

// ✅ — signal the error with a Rust type, convert after re-acquiring GIL
let result: Result<i64, String> = py.allow_threads(|| {
    if bad { return Err("bad".to_string()); }
    Ok(42)
});
result.map_err(|e| PyValueError::new_err(e))?;
```

---

## Performance

### Slow compile times

```toml
# .cargo/config.toml
[profile.dev]
opt-level = 0
debug = 1         # less debug info = faster link
```

On Linux, the `mold` linker dramatically reduces link times:

```bash
sudo apt-get install mold
```

```toml
# .cargo/config.toml
[build]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

### rust-analyzer not working in VS Code

1. `rustup component add rust-analyzer`
2. Reload the window: `Ctrl+Shift+P` → "Developer: Reload Window"
3. Check output: `Ctrl+Shift+P` → "rust-analyzer: Show RA server logs"

---

## See Also

- [Installation](../01-getting-started/installation.md) — setup guide with platform-specific steps
- [Daily Workflow](../03-project-setup-and-tooling/daily-workflow.md) — recommended inner loop
- [Cargo Cheatsheet](cargo-cheatsheet.md) — all cargo and maturin commands
