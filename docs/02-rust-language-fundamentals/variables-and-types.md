# Variables and Types

> **Prerequisites:** [Your First Project](../01-getting-started/your-first-project.md)  
> **Next:** [Functions and Control Flow](functions-and-control-flow.md)

This article covers Rust's foundational building blocks: how to declare variables, control mutability, understand the fixed-width numeric types, and work with the two string types.

---

## Hello, World

```python
# Python
print("Hello, world!")
```

```rust
// Rust
fn main() {
    println!("Hello, world!");
}
```

**What changed:**
- `println!` is a *macro* (note the `!`), not a function. Macros are expanded at compile time and can accept variable numbers of arguments with format strings.
- Every Rust executable needs a `fn main()` entry point.
- Statements end with `;`.

**Formatting:**

```python
name = "ranker"
score = 0.92
print(f"Service {name} scored {score:.2f}")
```

```rust
let name = "ranker";
let score = 0.92_f64;
println!("Service {name} scored {score:.2}");
```

Python's f-strings and Rust's format strings are nearly identical in syntax.

---

## Variables and Mutability

### Python — everything is mutable by default

```python
x = 10
x = 20          # rebind, no problem
x += 5          # mutate, no problem

MY_CONST = 42   # convention only; Python won't stop you from reassigning it
```

### Rust — immutable by default, explicit opt-in to mutation

```rust
let x = 10;
// x = 20;     // compile error: cannot assign twice to immutable variable

let mut x = 10; // `mut` opts in to mutation
x = 20;         // now fine
x += 5;

const MAX_RETRIES: u32 = 3; // true compile-time constant; type required
```

**Why immutable by default?** If you accidentally mutate something you didn't mean to, you get a compile error instead of a subtle runtime bug. In a data pipeline you often want to pass a dataset around and be sure nothing upstream is modifying it.

### Shadowing — re-use the same name with a different value or type

```python
# Python: same variable, type can silently change
value = "42"
value = int(value)  # now an int
```

```rust
// Rust: shadowing creates a new variable that hides the old one
let value = "42";
let value = value.parse::<i64>().unwrap(); // new `value`, now an i64
println!("{value}"); // 42
```

Shadowing is useful for transformation chains: you keep the same logical name without needing `mut`, and each step can have a different type.

---

## Primitive Types

### Python — types are inferred; ints are arbitrary precision

```python
x: int   = 42
y: float = 3.14
z: bool  = True
```

### Rust — types are fixed-width; you choose the size

```rust
// Integers
let a: i8   = -128;          // signed  8-bit  [-128, 127]
let b: i32  = -2_000_000;    // signed  32-bit (Python's default int range)
let c: i64  = 9_000_000_000; // signed  64-bit
let d: u8   = 255;           // unsigned 8-bit [0, 255]
let e: u64  = 18_000_000_000_000_000_000;
let f: usize = 42;           // pointer-sized; used for indices and lengths

// Floats
let g: f32 = 3.14_f32;       // 32-bit float
let h: f64 = 3.141_592_653;  // 64-bit float (Python's default)

// Booleans
let i: bool = true;           // lowercase, not True

// Characters (Unicode scalar, 4 bytes)
let j: char = 'λ';
```

**Underscores in numeric literals** are readability separators — `1_000_000` is `1000000`. Same idea as Python.

**Type inference** — Rust infers types from usage, so you rarely write annotations in practice:

```rust
let batch_size = 32;          // inferred i32
let learning_rate = 0.001;    // inferred f64
let enabled = false;          // inferred bool
```

### Arithmetic and Casting

```python
# Python: implicit widening, no overflow for int
result = 255 + 1        # → 256 (no overflow)
ratio  = 7 / 2          # → 3.5 (true division)
quot   = 7 // 2         # → 3   (floor division)
```

```rust
// Rust: explicit casting with `as`; integer overflow panics in debug mode
let result: u8 = 255_u8.wrapping_add(1); // → 0 (explicit wrap)
let ratio  = 7.0_f64 / 2.0;              // → 3.5
let quot   = 7 / 2;                      // → 3 (integer division, like //)

// Cast between types explicitly — no implicit conversion
let n: i32 = 42;
let f: f64 = n as f64;  // like Python's float(n)
```

---

## Strings

Python has one string type. Rust has two that serve different purposes.

| | Python | Rust `String` | Rust `&str` |
|---|---|---|---|
| Ownership | GC-managed | heap-allocated, owned | borrowed slice |
| Mutability | immutable (but re-bindable) | mutable if `mut` | always read-only |
| Analogy | `str` | `list` that holds chars | `memoryview` or slice |

```python
# Python — one type, zero friction
greeting = "hello"
greeting += " world"   # creates a new string, rebinds
print(len(greeting))
```

```rust
// &str — a borrowed view into string data (usually a string literal)
let greeting: &str = "hello";

// String — an owned, growable heap string
let mut owned = String::from("hello");
owned.push_str(" world");   // append in-place
println!("{}", owned.len()); // 11
```

### Common String Operations

```python
s = "  inference-server  "
print(s.strip())                      # "inference-server"
print(s.upper())                      # "  INFERENCE-SERVER  "
print("server" in s)                  # True
print(s.replace("server", "worker"))
parts = "a,b,c".split(",")            # ["a", "b", "c"]
joined = ",".join(["a", "b", "c"])    # "a,b,c"
print(f"batch={32}")
```

```rust
let s = "  inference-server  ";
println!("{}", s.trim());                         // "inference-server"
println!("{}", s.to_uppercase());                 // "  INFERENCE-SERVER  "
println!("{}", s.contains("server"));             // true
println!("{}", s.replace("server", "worker"));
let parts: Vec<&str> = "a,b,c".split(',').collect();
let joined = ["a", "b", "c"].join(",");           // "a,b,c"
println!("batch={}", 32);
```

### Converting Between `String` and `&str`

```rust
let owned: String = String::from("hello");
let borrowed: &str = &owned;       // String → &str (cheap borrow)
let back: String = borrowed.to_string(); // &str → String (allocates)
```

Think of `&str` as a read-only window into a `String` or a string literal. Pass `&str` to functions when you don't need ownership; return `String` when you're building a new value.

---

## See Also

- [Ownership](ownership.md) — why `String` and `&str` exist as separate types
- [Functions and Control Flow](functions-and-control-flow.md) — next article in this chapter
- [Python → Rust Cheatsheet](../05-reference/python-to-rust-cheatsheet.md) — quick reference table
