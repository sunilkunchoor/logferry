# Rust Basics for Python Developers

This guide teaches Rust by mapping every concept to its Python equivalent.
Each section shows the Python way first, then the idiomatic Rust way, then
explains what changed and why. Examples lean on MLOps scenarios — log
parsing, config loading, data pipelines — so the context stays familiar.

---

## Table of Contents

1. [Hello, World](#1-hello-world)
2. [Variables and Mutability](#2-variables-and-mutability)
3. [Primitive Types](#3-primitive-types)
4. [Strings](#4-strings)
5. [Functions](#5-functions)
6. [Control Flow](#6-control-flow)
7. [Ownership](#7-ownership)
8. [Borrowing and References](#8-borrowing-and-references)
9. [Slices](#9-slices)
10. [Structs](#10-structs)
11. [Enums](#11-enums)
12. [Pattern Matching](#12-pattern-matching)
13. [Option — representing "maybe a value"](#13-option--representing-maybe-a-value)
14. [Result — representing "success or failure"](#14-result--representing-success-or-failure)
15. [Collections: Vec, HashMap, HashSet](#15-collections-vec-hashmap-hashset)
16. [Closures](#16-closures)
17. [Iterators](#17-iterators)
18. [Traits](#18-traits)
19. [Generics](#19-generics)
20. [Modules and Crates](#20-modules-and-crates)
21. [Lifetimes (intro)](#21-lifetimes-intro)
22. [Key Differences at a Glance](#22-key-differences-at-a-glance)

---

## 1. Hello, World

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
- `println!` is a *macro* (note the `!`), not a function. Macros are
  expanded at compile time and can accept variable numbers of arguments
  with format strings.
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

## 2. Variables and Mutability

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

**Why immutable by default?** It makes the compiler your ally: if you
accidentally mutate something you didn't mean to, you get an error instead
of a subtle bug. In a data pipeline you often want to pass a dataset around
and be sure nothing upstream is modifying it.

### Shadowing — re-use the same name with a different value or type

```python
# Python: same variable, but type can silently change
value = "42"
value = int(value)  # now it's an int
```

```rust
// Rust: shadowing creates a new variable that hides the old one
let value = "42";
let value = value.parse::<i64>().unwrap(); // new `value`, now an i64
println!("{value}"); // 42
```

Shadowing is useful for transformation chains: you keep the same logical
name without needing `mut`, and each step can have a different type.

---

## 3. Primitive Types

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

// Characters (Unicode scalar, 4 bytes — not the same as a byte)
let j: char = 'λ';
```

**Underscores in numeric literals** are just readability separators —
`1_000_000` is `1000000`. Same idea as Python.

**Type inference** — Rust infers types from usage, so you rarely write
annotations in practice:

```rust
let batch_size = 32;          // inferred i32
let learning_rate = 0.001;    // inferred f64
let enabled = false;          // inferred bool
```

### Arithmetic and casting

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

## 4. Strings

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

// String — an owned, growable heap string (like Python's str for mutation)
let mut owned = String::from("hello");
owned.push_str(" world");   // append in-place
println!("{}", owned.len()); // 11
```

### Common string operations

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

### Converting between `String` and `&str`

```rust
let owned: String = String::from("hello");
let borrowed: &str = &owned;      // String → &str (cheap borrow)
let back: String = borrowed.to_string(); // &str → String (allocates)
```

Think of `&str` as a read-only window into a `String` or a string literal.
Pass `&str` to functions when you don't need ownership; return `String`
when you're building a new value.

---

## 5. Functions

```python
def add(a: int, b: int) -> int:
    return a + b

result = add(3, 4)  # 7
```

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon = implicit return (expression value)
}

let result = add(3, 4); // 7
```

**Implicit return.** The last expression in a block without a `;` is the
return value. Adding `;` turns it into a statement that returns `()` (unit,
Rust's equivalent of `None`). Both are idiomatic; use `return` for early
exits.

### Multiple return values — tuple vs struct

```python
def model_metrics(preds, labels):
    precision = compute_precision(preds, labels)
    recall    = compute_recall(preds, labels)
    return precision, recall          # Python: implicit tuple

p, r = model_metrics(preds, labels)  # unpack
```

```rust
fn model_metrics(preds: &[f64], labels: &[f64]) -> (f64, f64) {
    let precision = compute_precision(preds, labels);
    let recall    = compute_recall(preds, labels);
    (precision, recall)               // explicit tuple
}

let (p, r) = model_metrics(&preds, &labels); // destructure
```

For more than two or three values, prefer a named struct over a tuple —
it documents what each field means.

### No default arguments (but workarounds exist)

```python
def ingest(lines, num_threads=4):
    ...
```

In plain Rust there are no default argument values. Conventions:

```rust
// Option 1: separate function or constant
const DEFAULT_THREADS: usize = 4;

fn ingest(lines: &[String], num_threads: usize) { ... }

// Option 2: builder pattern (see EXPLAINER.md)

// Option 3: PyO3 #[pyo3(signature = (lines, num_threads=4))]
// — only relevant at the Python boundary
```

---

## 6. Control Flow

### if / else

```python
score = 0.87
if score > 0.9:
    print("excellent")
elif score > 0.7:
    print("good")
else:
    print("needs work")
```

```rust
let score = 0.87_f64;
if score > 0.9 {
    println!("excellent");
} else if score > 0.7 {
    println!("good");
} else {
    println!("needs work");
}
```

**`if` is an expression** — you can use it on the right side of `let`:

```python
label = "pass" if score > 0.7 else "fail"
```

```rust
let label = if score > 0.7 { "pass" } else { "fail" };
```

### loop

```rust
// `loop` = while True
let mut count = 0;
loop {
    count += 1;
    if count == 3 { break; }
}

// loop can return a value
let result = loop {
    count += 1;
    if count == 10 { break count * 2; } // break with value
};
```

### while

```python
while queue:
    item = queue.pop()
    process(item)
```

```rust
while !queue.is_empty() {
    let item = queue.pop().unwrap();
    process(item);
}
```

### for — iterating over ranges and collections

```python
for i in range(5):
    print(i)              # 0 1 2 3 4

for i in range(1, 6):
    print(i)              # 1 2 3 4 5

for item in my_list:
    print(item)

for i, item in enumerate(my_list):
    print(i, item)
```

```rust
for i in 0..5 {
    println!("{i}");      // 0 1 2 3 4
}

for i in 1..=5 {
    println!("{i}");      // 1 2 3 4 5  (..= is inclusive end)
}

for item in &my_vec {
    println!("{item}");
}

for (i, item) in my_vec.iter().enumerate() {
    println!("{i} {item}");
}
```

`0..5` is a `Range` — a value, not syntax sugar. You can store it, pass it
to functions, or chain iterator methods on it.

---

## 7. Ownership

This is Rust's most distinctive feature and has no Python equivalent.

### The problem ownership solves

Python manages memory with reference counting + a garbage collector.
You never think about who "owns" an object. Rust has no GC — instead, the
*compiler* tracks exactly one owner per value, and automatically frees memory
when the owner goes out of scope. Zero runtime overhead, zero GC pauses.

### The three rules

1. Every value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is freed.

```rust
{
    let data = String::from("model weights"); // `data` is the owner
    // use data ...
}   // `data` goes out of scope → memory freed automatically, no GC needed
```

### Move semantics

```python
# Python: assignment copies the reference; both names point to same object
a = [1, 2, 3]
b = a           # b and a point to the same list
b.append(4)
print(a)        # [1, 2, 3, 4] — a is affected too
```

```rust
let a = vec![1, 2, 3];
let b = a;          // ownership MOVES from a to b
// println!("{:?}", a);  // compile error: value moved, a is invalid
println!("{:?}", b); // [1, 2, 3]
```

**Move** transfers ownership. After a move, the original variable is invalid
and the compiler rejects any use of it. This prevents double-free bugs and
use-after-free bugs at compile time.

### Copy types — stack-only values are copied, not moved

```rust
let x = 42;
let y = x;          // integers implement Copy; x is still valid
println!("{x} {y}"); // 42 42
```

Types that live entirely on the stack and are cheap to duplicate (integers,
floats, booleans, `char`, tuples of Copy types) implement the `Copy` trait.
Assignment copies them. Heap-allocated types (`String`, `Vec`, etc.) do not
implement `Copy` — they move.

### Cloning — explicit deep copy

```python
import copy
b = copy.deepcopy(a)  # explicit deep copy
```

```rust
let a = vec![1, 2, 3];
let b = a.clone();    // explicit deep copy; a is still valid
println!("{:?} {:?}", a, b);
```

`clone()` is the Rust equivalent of `deepcopy`. It's explicit — you opt in
to the allocation cost, rather than being surprised by it.

---

## 8. Borrowing and References

Ownership would be painful if you had to move everything into every
function. *Borrowing* lets you pass a reference to a value without
transferring ownership.

### Immutable borrow — `&T`

```python
def print_length(items: list) -> None:
    print(len(items))
    # caller's list is unchanged

data = [1, 2, 3]
print_length(data)   # data is still usable after the call
print(data)
```

```rust
fn print_length(items: &Vec<i32>) {   // & = borrow, not move
    println!("{}", items.len());
}   // borrow ends here; nothing is freed

let data = vec![1, 2, 3];
print_length(&data);  // lend a reference; data is still owned by caller
println!("{:?}", data); // still valid
```

### Mutable borrow — `&mut T`

```python
def append_zero(items: list) -> None:
    items.append(0)   # Python: silently mutates in place

data = [1, 2, 3]
append_zero(data)
print(data)  # [1, 2, 3, 0]
```

```rust
fn append_zero(items: &mut Vec<i32>) { // &mut = mutable borrow
    items.push(0);
}

let mut data = vec![1, 2, 3];
append_zero(&mut data);   // explicit: "I am passing a mutable reference"
println!("{:?}", data);   // [1, 2, 3, 0]
```

### The borrowing rules (enforced at compile time)

```
Rule 1: any number of immutable borrows at the same time — OK
Rule 2: exactly one mutable borrow at a time — OK
Rule 3: immutable and mutable borrows cannot coexist
```

```rust
let mut v = vec![1, 2, 3];

// Multiple immutable borrows — fine
let r1 = &v;
let r2 = &v;
println!("{:?} {:?}", r1, r2);

// One mutable borrow — fine (after immutable borrows are done)
let r3 = &mut v;
r3.push(4);

// Simultaneous mutable + immutable — compile error
// let r4 = &v;     // can't do this while r3 is alive
```

These rules prevent the entire class of data-race bugs at compile time —
no locks, no runtime checks, no sanitizers needed.

---

## 9. Slices

A slice is a borrowed view into a contiguous sequence — like a `memoryview`
or NumPy array slice, but for any type.

```python
data = [10, 20, 30, 40, 50]
chunk = data[1:4]   # [20, 30, 40] — creates a new list in Python
```

```rust
let data = vec![10, 20, 30, 40, 50];
let chunk: &[i32] = &data[1..4];     // [20, 30, 40] — zero-copy view
println!("{:?}", chunk);
```

String slices work the same way:

```rust
let s = String::from("inference-server");
let service: &str = &s[10..];   // "server" — a view, not a copy
```

**Prefer slices over `&Vec<T>` and `&String` in function signatures:**

```rust
// Less flexible: only accepts Vec<i32>
fn process(data: &Vec<i32>) { ... }

// More flexible: accepts Vec<i32>, arrays, or any contiguous slice
fn process(data: &[i32]) { ... }

// Same for strings: &str accepts both &str literals and &String
fn greet(name: &str) { ... }   // can call with "Alice" or &my_string
```

---

## 10. Structs

### Python dataclass → Rust struct

```python
from dataclasses import dataclass
from typing import Optional

@dataclass
class ModelConfig:
    name: str
    version: int
    learning_rate: float
    dropout: Optional[float] = None

cfg = ModelConfig(name="bert", version=3, learning_rate=2e-5)
print(cfg.name, cfg.learning_rate)
```

```rust
#[derive(Debug)]                     // lets us use {:?} in println!
struct ModelConfig {
    name: String,
    version: u32,
    learning_rate: f64,
    dropout: Option<f64>,            // None equivalent: Option
}

let cfg = ModelConfig {
    name: String::from("bert"),
    version: 3,
    learning_rate: 2e-5,
    dropout: None,
};
println!("{:?}", cfg);
println!("{} {}", cfg.name, cfg.learning_rate);
```

### Methods — impl block

```python
@dataclass
class ModelConfig:
    name: str
    version: int

    def display_name(self) -> str:
        return f"{self.name}-v{self.version}"
```

```rust
struct ModelConfig {
    name: String,
    version: u32,
}

impl ModelConfig {
    // Associated function (like a classmethod / constructor)
    fn new(name: &str, version: u32) -> Self {
        ModelConfig {
            name: name.to_string(),
            version,               // shorthand when variable name == field name
        }
    }

    // Method — first parameter is &self (immutable) or &mut self (mutable)
    fn display_name(&self) -> String {
        format!("{}-v{}", self.name, self.version)
    }

    fn bump_version(&mut self) {
        self.version += 1;
    }
}

let mut cfg = ModelConfig::new("bert", 3);
println!("{}", cfg.display_name());  // "bert-v3"
cfg.bump_version();
println!("{}", cfg.display_name());  // "bert-v4"
```

### Struct update syntax

```python
import dataclasses
new_cfg = dataclasses.replace(cfg, version=4)  # copy with one field changed
```

```rust
let new_cfg = ModelConfig {
    version: 4,
    ..cfg          // copy remaining fields from cfg (cfg.name is moved here)
};
```

### Tuple structs

```python
class Meters(float): pass   # newtype pattern
class Seconds(float): pass

def speed(d: Meters, t: Seconds) -> float:
    return float(d) / float(t)
```

```rust
struct Meters(f64);
struct Seconds(f64);

fn speed(d: Meters, t: Seconds) -> f64 {
    d.0 / t.0      // .0 accesses the inner value
}

let v = speed(Meters(100.0), Seconds(9.8));
// speed(Seconds(9.8), Meters(100.0));  // compile error: wrong order
```

Type aliases (`Meters`, `Seconds`) make function signatures self-documenting
and let the compiler catch argument-swap bugs.

---

## 11. Enums

Python's `enum.Enum` gives named constants. Rust enums can also carry data —
they're more like sum types or tagged unions.

### Simple enum

```python
from enum import Enum

class LogLevel(Enum):
    INFO  = "INFO"
    WARN  = "WARN"
    ERROR = "ERROR"

level = LogLevel.INFO
```

```rust
#[derive(Debug, PartialEq)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

let level = LogLevel::Info;
```

### Enums with data — no Python equivalent, but think of it as a tagged union

```python
# Python: you'd use a dict or a class hierarchy
event = {"type": "metric", "name": "f1", "value": 0.92}
# or
event = {"type": "error", "code": 500, "message": "timeout"}
```

```rust
// Rust: each variant can carry different data
#[derive(Debug)]
enum Event {
    Metric { name: String, value: f64 },
    Error  { code: u32, message: String },
    Heartbeat,                             // variant with no data
}

let e1 = Event::Metric { name: "f1".to_string(), value: 0.92 };
let e2 = Event::Error  { code: 500, message: "timeout".to_string() };
let e3 = Event::Heartbeat;
```

This is the backbone of `Option<T>` (`Some(value)` or `None`) and
`Result<T, E>` (`Ok(value)` or `Err(error)`) — both are just enums.

---

## 12. Pattern Matching

`match` is like Python's `match`/`case` (3.10+), but more powerful and
exhaustive — the compiler forces you to handle every variant.

### Basic match

```python
match level:
    case LogLevel.INFO:
        print("informational")
    case LogLevel.WARN:
        print("warning")
    case LogLevel.ERROR:
        print("error")
    case _:
        print("unknown")
```

```rust
match level {
    LogLevel::Info  => println!("informational"),
    LogLevel::Warn  => println!("warning"),
    LogLevel::Error => println!("error"),
    // No `_` needed if all variants are covered — compiler checks this
}
```

**Exhaustiveness.** Remove any arm and the code won't compile. This means
when you add a new variant to an enum, the compiler points you to every
`match` that needs updating.

### Matching enum variants with data

```rust
match event {
    Event::Metric { name, value } => {
        println!("metric {name} = {value}");
    }
    Event::Error { code, message } => {
        eprintln!("error {code}: {message}");
    }
    Event::Heartbeat => {
        println!("alive");
    }
}
```

### Matching with guards

```python
match score:
    case s if s > 0.9:
        print("excellent")
    case s if s > 0.7:
        print("good")
    case _:
        print("needs work")
```

```rust
match score {
    s if s > 0.9 => println!("excellent"),
    s if s > 0.7 => println!("good"),
    _             => println!("needs work"),
}
```

### Matching multiple patterns

```rust
match day {
    "Mon" | "Tue" | "Wed" | "Thu" | "Fri" => println!("weekday"),
    "Sat" | "Sun"                          => println!("weekend"),
    _                                      => println!("unknown"),
}
```

### Ranges in match

```rust
match age {
    0..=17  => println!("minor"),
    18..=64 => println!("adult"),
    65..    => println!("senior"),
}
```

### `if let` — match one variant, ignore the rest

```python
if isinstance(event, MetricEvent):
    print(event.value)
```

```rust
if let Event::Metric { name, value } = &event {
    println!("{name} = {value}");
}
```

### `while let` — loop until a pattern stops matching

```rust
while let Some(item) = queue.pop() {
    process(item);
}
```

---

## 13. Option — representing "maybe a value"

Python uses `None` to mean "no value." Rust wraps optional values in
`Option<T>` — you can't accidentally use a `None` as if it were a value
because the type system prevents it.

```python
def find_threshold(name: str) -> float | None:
    thresholds = {"f1": 0.85, "precision": 0.90}
    return thresholds.get(name)   # returns None if not found

t = find_threshold("f1")
if t is not None:
    print(t + 0.05)   # Python won't stop you using None + 0.05 until runtime
```

```rust
fn find_threshold(name: &str) -> Option<f64> {
    let thresholds = std::collections::HashMap::from([
        ("f1", 0.85),
        ("precision", 0.90),
    ]);
    thresholds.get(name).copied()   // Option<f64>: Some(0.85) or None
}

// Must unwrap explicitly — compiler won't let you use Option<f64> as f64
match find_threshold("f1") {
    Some(t) => println!("{}", t + 0.05),
    None    => println!("not found"),
}
```

### Common Option methods

```rust
let t: Option<f64> = find_threshold("f1");

// unwrap_or — provide a default (like Python's dict.get(key, default))
let value = t.unwrap_or(0.0);

// map — transform the inner value if it's Some (like Python's optional chaining)
let adjusted = t.map(|v| v + 0.05);

// and_then — chain operations that might also fail
let result = t.and_then(|v| if v > 0.8 { Some(v) } else { None });

// is_some / is_none
if t.is_some() { println!("found"); }

// unwrap — panics if None; only use when None is truly impossible
let must_exist = t.unwrap();

// expect — panics with a custom message; better than unwrap in production
let must_exist = t.expect("threshold must be configured");
```

---

## 14. Result — representing "success or failure"

Python uses exceptions. Rust uses `Result<T, E>` — success is `Ok(value)`,
failure is `Err(error)`. The compiler forces you to handle both cases.

```python
def parse_int(s: str) -> int:
    return int(s)   # raises ValueError on bad input

try:
    n = parse_int("42")
except ValueError as e:
    print(f"failed: {e}")
```

```rust
fn parse_int(s: &str) -> Result<i64, std::num::ParseIntError> {
    s.parse::<i64>()  // returns Ok(42) or Err(ParseIntError)
}

match parse_int("42") {
    Ok(n)  => println!("got {n}"),
    Err(e) => println!("failed: {e}"),
}
```

### The `?` operator — propagate errors up the call stack

```python
def load_config(path: str) -> dict:
    with open(path) as f:       # raises FileNotFoundError
        return json.load(f)     # raises json.JSONDecodeError
    # exceptions bubble up automatically
```

```rust
use std::fs;

fn load_config(path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;   // ? returns Err early if it fails
    let cfg  = serde_json::from_str(&text)?; // same
    Ok(cfg)
}
```

`?` unwraps `Ok(v)` and continues, or returns `Err(e)` immediately from the
current function — exactly what Python exceptions do implicitly, but explicit
and type-checked.

### Common Result methods

```rust
let r: Result<i64, _> = "42".parse();

// unwrap_or — provide a default on error
let n = r.unwrap_or(0);

// map — transform the Ok value
let doubled = r.map(|n| n * 2);

// map_err — transform the Err value (e.g. to convert error types)
let r2 = r.map_err(|e| format!("parse failed: {e}"));

// is_ok / is_err
if r.is_ok() { println!("success"); }

// unwrap / expect — panic on Err (use only when Err is truly impossible)
let n = r.expect("hard-coded string must parse");
```

---

## 15. Collections: Vec, HashMap, HashSet

### Vec — Python's list

```python
items = []
items.append(1)
items.append(2)
items.append(3)
print(items[0])      # 1
print(len(items))    # 3
items.pop()          # removes last element
for x in items:
    print(x)
```

```rust
let mut items: Vec<i32> = Vec::new();
items.push(1);
items.push(2);
items.push(3);
println!("{}", items[0]);       // 1  (panics if out of bounds)
println!("{}", items.len());    // 3
items.pop();                    // returns Option<i32>
for x in &items {
    println!("{x}");
}

// Literal initialisation
let scores = vec![0.91, 0.87, 0.93];

// Safe indexed access
match items.get(0) {
    Some(v) => println!("{v}"),
    None    => println!("empty"),
}
```

### HashMap — Python's dict

```python
counts = {}
counts["INFO"]  = counts.get("INFO", 0) + 1
counts["ERROR"] = counts.get("ERROR", 0) + 1
print(counts.get("WARN", 0))

for level, n in counts.items():
    print(f"{level}: {n}")
```

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, u64> = HashMap::new();
*counts.entry("INFO".to_string()).or_insert(0)  += 1;
*counts.entry("ERROR".to_string()).or_insert(0) += 1;

println!("{}", counts.get("WARN").unwrap_or(&0));

for (level, n) in &counts {
    println!("{level}: {n}");
}

// Literal initialisation (Rust 1.56+)
let thresholds = HashMap::from([
    ("f1",        0.85),
    ("precision", 0.90),
]);
```

### HashSet — Python's set

```python
seen = set()
seen.add("ranker")
seen.add("featurizer")
print("ranker" in seen)          # True
print(seen & {"ranker", "other"}) # intersection
```

```rust
use std::collections::HashSet;

let mut seen: HashSet<String> = HashSet::new();
seen.insert("ranker".to_string());
seen.insert("featurizer".to_string());
println!("{}", seen.contains("ranker"));   // true

let other: HashSet<_> = ["ranker", "other"].iter().map(|s| s.to_string()).collect();
let intersection: HashSet<_> = seen.intersection(&other).collect();
```

---

## 16. Closures

Python lambdas, but without the one-expression limit.

```python
double = lambda x: x * 2
add    = lambda x, y: x + y

nums = [1, 2, 3, 4, 5]
evens  = list(filter(lambda x: x % 2 == 0, nums))
doubled = list(map(lambda x: x * 2, nums))
```

```rust
let double = |x: i64| x * 2;
let add    = |x: i64, y: i64| x + y;

let nums = vec![1, 2, 3, 4, 5];
let evens:   Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
let doubled: Vec<_> = nums.iter().map(|&x| x * 2).collect();
```

### Closures capture their environment

```python
threshold = 0.9
good = list(filter(lambda s: s > threshold, scores))  # captures threshold
```

```rust
let threshold = 0.9_f64;
let good: Vec<_> = scores.iter().filter(|&&s| s > threshold).collect(); // captures threshold
```

Rust closures capture by reference by default. Use `move` to take ownership:

```rust
let prefix = String::from("svc");
let label_fn = move |name: &str| format!("{prefix}:{name}"); // prefix is moved in
```

---

## 17. Iterators

Rust iterators are lazy (like Python generators) and chain without
intermediate allocations.

```python
scores = [0.91, 0.62, 0.87, 0.45, 0.93]

# filter → map → sum in one expression
total = sum(
    s * 1.1
    for s in scores
    if s > 0.7
)
print(total)

# zip
for name, score in zip(services, scores):
    print(name, score)
```

```rust
let scores = vec![0.91, 0.62, 0.87, 0.45, 0.93];

// filter → map → sum, no intermediate Vec allocated
let total: f64 = scores.iter()
    .filter(|&&s| s > 0.7)
    .map(|&s| s * 1.1)
    .sum();
println!("{total:.4}");

// zip
for (name, score) in services.iter().zip(scores.iter()) {
    println!("{name} {score}");
}
```

### Common iterator adapters

```rust
let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

// map
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();

// filter
let big: Vec<&i32> = v.iter().filter(|&&x| x > 4).collect();

// filter_map — filter and transform in one step
let parsed: Vec<i32> = ["1", "two", "3"]
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())  // drops Err, keeps Ok
    .collect();

// fold — like Python's functools.reduce
let product: i64 = v.iter().fold(1, |acc, &x| acc * x as i64);

// any / all
let any_large = v.iter().any(|&x| x > 8);   // like Python's any()
let all_pos   = v.iter().all(|&x| x > 0);   // like Python's all()

// find
let first_big = v.iter().find(|&&x| x > 4); // returns Option<&i32>

// count / sum / max / min
let n       = v.iter().count();
let total   = v.iter().sum::<i32>();
let biggest = v.iter().max();

// flat_map — like Python's itertools.chain.from_iterable
let nested = vec![vec![1, 2], vec![3, 4]];
let flat: Vec<i32> = nested.iter().flat_map(|v| v.iter().copied()).collect();

// take / skip — like Python's islice
let first_three: Vec<_> = v.iter().take(3).collect();
let skip_two:    Vec<_> = v.iter().skip(2).collect();
```

---

## 18. Traits

Traits define shared behaviour — think Python's `typing.Protocol` or
`abc.ABC`, but enforced at compile time.

```python
from typing import Protocol

class Summarizable(Protocol):
    def summarize(self) -> str: ...

class ModelCard:
    def __init__(self, name: str, accuracy: float):
        self.name = name
        self.accuracy = accuracy

    def summarize(self) -> str:
        return f"{self.name} acc={self.accuracy:.2f}"

def print_summary(item: Summarizable) -> None:
    print(item.summarize())
```

```rust
trait Summarizable {
    fn summarize(&self) -> String;

    // Default implementation — like a mixin method in Python
    fn short_summary(&self) -> String {
        format!("[{}]", self.summarize())
    }
}

struct ModelCard {
    name: String,
    accuracy: f64,
}

impl Summarizable for ModelCard {
    fn summarize(&self) -> String {
        format!("{} acc={:.2}", self.name, self.accuracy)
    }
}

fn print_summary(item: &impl Summarizable) {
    println!("{}", item.summarize());
}

let card = ModelCard { name: "bert".to_string(), accuracy: 0.91 };
print_summary(&card);
println!("{}", card.short_summary()); // default impl
```

### Common derived traits

These come for free with `#[derive(...)]`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct ServiceId {
    name: String,
    version: u32,
}
```

| Trait | Python equivalent | What it gives you |
|---|---|---|
| `Debug` | `__repr__` | `{:?}` formatting |
| `Display` | `__str__` | `{}` formatting (must impl manually) |
| `Clone` | `copy.deepcopy` | `.clone()` method |
| `Copy` | (automatic for primitives) | copy on assign |
| `PartialEq` / `Eq` | `__eq__` | `==` and `!=` operators |
| `PartialOrd` / `Ord` | `__lt__` etc. | `<` `>` and `.sort()` |
| `Hash` | `__hash__` | use as `HashMap` key |
| `Default` | `__init__` with defaults | `MyType::default()` |

### Trait objects — runtime polymorphism

```python
handlers: list[Summarizable] = [ModelCard(...), OtherThing(...)]
for h in handlers:
    h.summarize()      # dynamic dispatch at runtime
```

```rust
let handlers: Vec<Box<dyn Summarizable>> = vec![
    Box::new(ModelCard { name: "bert".to_string(), accuracy: 0.91 }),
    // Box::new(OtherThing { ... }),
];
for h in &handlers {
    println!("{}", h.summarize());   // dynamic dispatch through vtable
}
```

`Box<dyn Trait>` is the heap-allocated, dynamically-dispatched equivalent of
a Python object with that protocol. Use it when you need a heterogeneous
collection of types that share a trait.

---

## 19. Generics

Generics let you write one function or struct that works with many types —
like Python's type hints, but enforced at compile time, with zero runtime
overhead (Rust generates a concrete copy for each type at compile time).

```python
from typing import TypeVar, Generic

T = TypeVar("T")

def first(items: list[T]) -> T | None:
    return items[0] if items else None
```

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

let n = first(&[1, 2, 3]);        // Option<&i32>
let s = first(&["a", "b", "c"]); // Option<&&str>
```

### Trait bounds on generics

```python
from typing import Protocol, TypeVar

class Printable(Protocol):
    def __str__(self) -> str: ...

T = TypeVar("T", bound=Printable)

def display_all(items: list[T]) -> None:
    for item in items:
        print(item)
```

```rust
use std::fmt::Display;

fn display_all<T: Display>(items: &[T]) {
    for item in items {
        println!("{item}");
    }
}

display_all(&[1, 2, 3]);           // i32 implements Display
display_all(&["hello", "world"]);  // &str implements Display
```

Multiple bounds with `+`:

```rust
fn log_and_store<T: Display + Clone>(item: T) {
    println!("storing: {item}");
    let copy = item.clone();
    // ...
}
```

Complex bounds with `where`:

```rust
fn process<T, E>(result: Result<T, E>)
where
    T: Display + Clone,
    E: std::error::Error,
{
    match result {
        Ok(v)  => println!("ok: {v}"),
        Err(e) => eprintln!("err: {e}"),
    }
}
```

### Generic structs

```rust
// A simple wrapper that works for any type
struct Cache<T> {
    value: Option<T>,
}

impl<T: Clone> Cache<T> {
    fn new() -> Self {
        Cache { value: None }
    }

    fn set(&mut self, v: T) {
        self.value = Some(v);
    }

    fn get(&self) -> Option<T> {
        self.value.clone()
    }
}

let mut c: Cache<f64> = Cache::new();
c.set(0.92);
println!("{:?}", c.get()); // Some(0.92)
```

---

## 20. Modules and Crates

### Python packages vs Rust crates and modules

```
Python               Rust
────────────────     ────────────────
package (dir)    →   crate
module (.py)     →   module (mod block or file)
import           →   use
__init__.py      →   mod.rs or lib.rs
```

### Defining modules

```python
# metrics.py
def precision(tp, fp): return tp / (tp + fp)
def recall(tp, fn):    return tp / (tp + fn)
```

```rust
// src/metrics.rs
pub fn precision(tp: f64, fp: f64) -> f64 { tp / (tp + fp) }
pub fn recall(tp: f64, fn_: f64)   -> f64 { tp / (tp + fn_) }
```

```rust
// src/lib.rs — declare and re-export
pub mod metrics;           // tells Rust to load src/metrics.rs

pub use metrics::precision; // re-export at crate root (like __init__.py)
```

### Inline modules

```rust
mod config {
    pub struct Settings {
        pub batch_size: usize,
    }

    pub fn defaults() -> Settings {
        Settings { batch_size: 32 }
    }
}

// Use from outside
let s = config::defaults();
use config::Settings;      // bring into scope
```

### Visibility

```python
# Python: convention only; _ prefix = private
_internal = "private by convention"
public    = "public"
```

```rust
// Rust: enforced by compiler
fn private_fn() { }          // private by default
pub fn public_fn() { }       // public

pub(crate) fn crate_fn() { } // visible within this crate only
pub(super) fn parent_fn() { } // visible to parent module only
```

### Using external crates

In `Cargo.toml`:

```toml
[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
```

In code:

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Config {
    batch_size: usize,
}

let json = serde_json::to_string(&Config { batch_size: 32 }).unwrap();
```

---

## 21. Lifetimes (intro)

Lifetimes are how Rust tracks how long a reference is valid. The compiler
infers them most of the time. You only write explicit lifetime annotations
when the compiler can't figure it out on its own — which usually means
a function returns a reference and has multiple input references.

### The problem they solve

```python
# Python: fine, GC keeps objects alive
def longest(a: str, b: str) -> str:
    return a if len(a) > len(b) else b
```

```rust
// Rust: which input does the output reference? Compiler needs to know
// to check the caller doesn't use the return value after either input is dropped.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let s1 = String::from("long string");
let result;
{
    let s2 = String::from("short");
    result = longest(&s1, &s2);
    println!("{result}"); // OK: both s1 and s2 are alive here
}
// println!("{result}"); // compile error: s2 is dropped; result might point to it
```

`'a` is a lifetime parameter — it says "the output reference lives at least
as long as whichever input reference lives shorter." The annotation is
documentation for the compiler, not code that runs at runtime.

**Good news:** The compiler infers lifetimes in the vast majority of cases.
You encounter explicit lifetime syntax mainly in library code, complex
struct definitions, and function signatures that return references.

---

## 22. Key Differences at a Glance

| Concept | Python | Rust |
|---|---|---|
| Memory management | GC / refcount | Ownership + borrow checker |
| Nullability | `None` (any variable) | `Option<T>` (explicit) |
| Error handling | `raise` / `try/except` | `Result<T, E>` / `?` |
| Type checking | Runtime + optional hints | Compile time, always |
| Mutability | Mutable by default | Immutable by default (`mut` opt-in) |
| String types | `str` (one type) | `String` (owned) + `&str` (borrowed) |
| Iteration | `for x in iterable` | `for x in iter` (lazy by default) |
| Interfaces | `Protocol` / `ABC` | `trait` |
| Generics | `TypeVar` (hints only) | `<T>` (compile-time, zero cost) |
| Concurrency | GIL limits CPU parallelism | No GIL; `Send`/`Sync` prove safety |
| Integer overflow | Silent widening | Panic (debug) / wrapping (release) |
| Import | `import`, `from … import` | `use`, `pub use` |
| None coalescing | `x or default` | `x.unwrap_or(default)` |
| Truthiness | Any object | `bool` only; no implicit conversion |

---

## Further Reading

- [The Rust Book](https://doc.rust-lang.org/book/) — the official free textbook; read in order
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — short, runnable code examples
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises, highly recommended
- [TUTORIALS.md](TUTORIALS.md) — PyO3-specific tutorial for exposing Rust to Python
- [EXPLAINER.md](EXPLAINER.md) — how logferry uses ownership, traits, and PyO3 together
