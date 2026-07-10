# Traits and Generics

> **Prerequisites:** [Closures and Iterators](closures-and-iterators.md)  
> **Next:** [Modules and Crates](modules-and-crates.md)

Traits define shared behaviour — like Python's `typing.Protocol` or `abc.ABC`, but enforced at compile time with zero runtime overhead. Generics allow one function or struct to work with many types. Together they are the backbone of Rust's type system and the reason `logferry`'s `Validator` trait works safely across threads.

---

## Traits

### Python Protocol → Rust Trait

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
println!("{}", card.short_summary()); // uses default impl
```

### Common Derived Traits

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

### Trait Objects — Runtime Polymorphism

```python
handlers: list[Summarizable] = [ModelCard(...), OtherThing(...)]
for h in handlers:
    h.summarize()      # dynamic dispatch at runtime
```

```rust
let handlers: Vec<Box<dyn Summarizable>> = vec![
    Box::new(ModelCard { name: "bert".to_string(), accuracy: 0.91 }),
];
for h in &handlers {
    println!("{}", h.summarize());   // dynamic dispatch through vtable
}
```

`Box<dyn Trait>` is the heap-allocated, dynamically-dispatched equivalent of a Python object that implements a protocol. Use it when you need a heterogeneous collection of types that share a trait.

---

## `Send` and `Sync` — Thread-Safety Supertraits

This is where traits become uniquely powerful for concurrent code.

```rust
// The Send + Sync supertraits are a compile-time promise:
// - Send: any type implementing Validator can be moved into another thread
// - Sync: any type implementing Validator can be shared by reference across threads
trait Validator: Send + Sync {
    fn validate(&self, record: &LogRecord) -> Result<(), IngestError>;
}
```

In `logferry`, validators are shared by reference across multiple worker threads:

```rust
let validators: Vec<Box<dyn Validator>> = vec![Box::new(NonEmptyMessage)];
// &validators is passed to every worker thread via thread::scope
```

This is legal only because `Validator` requires `Send + Sync`. Delete either bound and the code that spawns threads stops compiling — the compiler can no longer prove it is safe.

```python
# Python equivalent (but without the compile-time guarantee)
class Validator(Protocol):
    def validate(self, record: dict) -> None: ...   # raises on failure
# Python has no way to guarantee thread-safety at the protocol level
```

---

## Generics

Generics let you write one function or struct that works with many types. Rust generates a concrete copy for each type at compile time (monomorphisation), so there is zero runtime overhead — unlike Python's runtime type dispatch.

```python
from typing import TypeVar

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

### Trait Bounds on Generics

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

### Generic Structs

```rust
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

## Lifetimes (Brief Introduction)

Lifetimes are how Rust tracks how long a reference is valid. The compiler infers them in the vast majority of cases. You only write explicit lifetime annotations when a function returns a reference and has multiple input references — the compiler cannot tell which input the output borrows from.

```rust
// 'a says: the output reference lives at least as long as
// whichever input reference lives shorter.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` is documentation for the compiler, not code that runs at runtime. You encounter explicit lifetime syntax mainly in library code, complex struct definitions with reference fields, and function signatures that return references.

---

## See Also

- [Modules and Crates](modules-and-crates.md) — next article
- [logferry Walkthrough](../04-pyo3-and-logferry/logferry-walkthrough.md) — see `Validator: Send + Sync` in context
- [Multithreading and the GIL](../04-pyo3-and-logferry/multithreading-and-gil.md) — how `Send + Sync` enables parallel validation
