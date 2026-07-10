# Functions and Control Flow

> **Prerequisites:** [Variables and Types](variables-and-types.md)  
> **Next:** [Ownership](ownership.md)

This article covers how to define functions, use Rust's expression-based control flow, and iterate over ranges and collections.

---

## Functions

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

**Implicit return.** The last expression in a block without a `;` is the return value. Adding `;` turns it into a statement that returns `()` (unit — Rust's equivalent of `None`). Both are idiomatic; use `return` only for early exits.

### Multiple Return Values

```python
def model_metrics(preds, labels):
    return compute_precision(preds, labels), compute_recall(preds, labels)

p, r = model_metrics(preds, labels)
```

```rust
fn model_metrics(preds: &[f64], labels: &[f64]) -> (f64, f64) {
    let precision = compute_precision(preds, labels);
    let recall    = compute_recall(preds, labels);
    (precision, recall)
}

let (p, r) = model_metrics(&preds, &labels);
```

For more than two or three values, prefer a named struct over a tuple — it documents what each field means.

### No Default Arguments (But Workarounds Exist)

```python
def ingest(lines, num_threads=4):
    ...
```

In plain Rust there are no default argument values. Common conventions:

```rust
// Option 1: a constant
const DEFAULT_THREADS: usize = 4;
fn ingest(lines: &[String], num_threads: usize) { ... }

// Option 2: at the Python boundary via PyO3
#[pyo3(signature = (lines, num_threads=4))]
fn ingest(...) { ... }
```

---

## Control Flow

### `if` / `else`

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

### `loop`

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

### `while`

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

### `for` — Iterating Over Ranges and Collections

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

`0..5` is a `Range` value — not just syntax sugar. You can store it, pass it to functions, or chain iterator methods on it.

---

## Pattern Matching

`match` is like Python's `match`/`case` (3.10+), but exhaustive — the compiler forces you to handle every variant.

### Basic `match`

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

**Exhaustiveness.** Remove any arm and the code won't compile. When you add a new enum variant, the compiler points to every `match` that needs updating.

### Matching Enum Variants with Data

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

### Guards

```rust
match score {
    s if s > 0.9 => println!("excellent"),
    s if s > 0.7 => println!("good"),
    _             => println!("needs work"),
}
```

### Multiple Patterns

```rust
match day {
    "Mon" | "Tue" | "Wed" | "Thu" | "Fri" => println!("weekday"),
    "Sat" | "Sun"                          => println!("weekend"),
    _                                      => println!("unknown"),
}
```

### Ranges in `match`

```rust
match age {
    0..=17  => println!("minor"),
    18..=64 => println!("adult"),
    65..    => println!("senior"),
}
```

### `if let` — Match One Variant, Ignore the Rest

```python
if isinstance(event, MetricEvent):
    print(event.value)
```

```rust
if let Event::Metric { name, value } = &event {
    println!("{name} = {value}");
}
```

### `while let` — Loop Until a Pattern Stops Matching

```rust
while let Some(item) = queue.pop() {
    process(item);
}
```

---

## See Also

- [Ownership](ownership.md) — why you write `&my_vec` instead of `my_vec` in `for` loops
- [Structs and Enums](structs-and-enums.md) — define the enum types that `match` works on
- [Error Handling](error-handling.md) — `Option` and `Result` work naturally with `match`
