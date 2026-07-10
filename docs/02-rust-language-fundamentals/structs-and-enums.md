# Structs and Enums

> **Prerequisites:** [Borrowing and Slices](borrowing-and-slices.md)  
> **Next:** [Error Handling](error-handling.md)

Structs and enums are Rust's primary data-modelling tools. Structs bundle named fields together; enums represent a value that can be one of several distinct variants — and each variant can carry different data.

---

## Structs

### Python Dataclass → Rust Struct

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

### Methods — impl Block

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

### Struct Update Syntax

```python
import dataclasses
new_cfg = dataclasses.replace(cfg, version=4)  # copy with one field changed
```

```rust
let new_cfg = ModelConfig {
    version: 4,
    ..cfg          // copy remaining fields from cfg
};
```

### Tuple Structs — Newtype Pattern

```python
class Meters(float): pass
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

Tuple structs make function signatures self-documenting and let the compiler catch argument-swap bugs at compile time.

---

## Enums

Python's `enum.Enum` gives named constants. Rust enums can also carry data — they are more like sum types or tagged unions.

### Simple Enum

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

### Enums with Data

```python
# Python: you'd use a dict or a class hierarchy
event = {"type": "metric", "name": "f1", "value": 0.92}
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

This is the backbone of `Option<T>` (`Some(value)` or `None`) and `Result<T, E>` (`Ok(value)` or `Err(error)`) — both are just enums in the standard library.

### `logferry`'s `IngestError` Enum

The `logferry` codebase uses an enum to represent every way ingestion can fail:

```rust
#[derive(Debug)]
enum IngestError {
    EmptyMessage { service: String },
    // add new variants here as new validation rules are added
}
```

Adding a new variant and running `cargo build` immediately points to every `match` that needs a new arm. No test is required to find the gap.

---

## Pattern Matching on Structs and Enums

See [Functions and Control Flow](functions-and-control-flow.md) for `match` syntax. The key additions for structs and enums:

```rust
// Destructure a struct in a match arm
match cfg {
    ModelConfig { version: 1, .. } => println!("v1 model"),
    ModelConfig { version, .. }    => println!("model v{version}"),
}

// Destructure directly in a let binding
let ModelConfig { name, version, .. } = cfg;
println!("{name} v{version}");
```

---

## Common Derived Traits for Structs

```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct ServiceId {
    name: String,
    version: u32,
}
```

| Derive | What it gives you |
|---|---|
| `Debug` | `{:?}` formatting (like Python's `__repr__`) |
| `Clone` | `.clone()` method (like `copy.deepcopy`) |
| `PartialEq` | `==` and `!=` operators (like `__eq__`) |
| `Default` | `ServiceId::default()` with zero values |

---

## See Also

- [Error Handling](error-handling.md) — `Option<T>` and `Result<T,E>` are built-in enums
- [Traits and Generics](traits-and-generics.md) — implement traits for your structs
- [logferry Walkthrough](../04-pyo3-and-logferry/logferry-walkthrough.md) — see `LogRecord`, `IngestStats`, and `IngestError` in context
