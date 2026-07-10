# Modules and Crates

> **Prerequisites:** [Traits and Generics](traits-and-generics.md)  
> **Next:** [Cargo In Depth](../03-project-setup-and-tooling/cargo-in-depth.md)

Rust organises code into *modules* (within a crate) and *crates* (the top-level compilation unit). Understanding this system is key to reading `logferry`'s `src/lib.rs` and adding your own code to it.

---

## Python Packages vs Rust Crates and Modules

```
Python               Rust
────────────────     ────────────────
package (dir)    →   crate
module (.py)     →   module (mod block or file)
import           →   use
__init__.py      →   mod.rs or lib.rs
```

---

## Defining Modules

### File-Based Modules

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
pub mod metrics;            // tells Rust to load src/metrics.rs

pub use metrics::precision; // re-export at crate root (like __init__.py)
```

### Inline Modules

```rust
mod config {
    pub struct Settings {
        pub batch_size: usize,
    }

    pub fn defaults() -> Settings {
        Settings { batch_size: 32 }
    }
}

// Use from outside the module
let s = config::defaults();
use config::Settings;      // bring into scope (like `from config import Settings`)
```

---

## Visibility

```python
# Python: convention only; _ prefix = private
_internal = "private by convention"
public    = "public"
```

```rust
// Rust: enforced by compiler
fn private_fn() { }           // private by default
pub fn public_fn() { }        // public (visible outside the crate)
pub(crate) fn crate_fn() { }  // visible within this crate only
pub(super) fn parent_fn() { } // visible to parent module only
```

`logferry` uses this to keep implementation details private. `ingest_chunk` is not `pub` — it is only callable from within the crate. `ingest_logs` and `validate_line` are `pub` because they are exposed to Python via PyO3.

---

## `use` — Bringing Names Into Scope

```python
from collections import HashMap    # brings HashMap into scope
from serde import Deserialize, Serialize
```

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use pyo3::prelude::*;              // import everything from pyo3::prelude
```

---

## Using External Crates

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

## `logferry`'s Module Structure

`logferry` uses a single `src/lib.rs` file rather than splitting into multiple modules. This is a legitimate choice for a small crate. Here is how you would split it if it grew:

```
src/
├── lib.rs          # crate root; declares sub-modules and the #[pymodule]
├── model.rs        # LogRecord, IngestStats structs
├── error.rs        # IngestError enum + From<IngestError> for PyErr
├── validator.rs    # Validator trait + NonEmptyMessage impl
└── ingest.rs       # ingest_chunk, ingest_logs
```

```rust
// src/lib.rs
mod model;
mod error;
mod validator;
mod ingest;

use pyo3::prelude::*;
use ingest::{ingest_logs, validate_line};
use model::IngestStats;

#[pymodule]
fn logferry(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ingest_logs, m)?)?;
    m.add_function(wrap_pyfunction!(validate_line, m)?)?;
    m.add_class::<IngestStats>()?;
    Ok(())
}
```

---

## The Test Module Convention

Rust unit tests live in the same file they test, inside a `#[cfg(test)]` module:

```rust
// src/lib.rs or any other file

fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;   // import everything from the parent module

    #[test]
    fn addition_works() {
        assert_eq!(add(2, 3), 5);
    }
}
```

`#[cfg(test)]` means the module is only compiled when running `cargo test` — it does not appear in the release binary. See [Testing](../03-project-setup-and-tooling/testing.md) for the full story.

---

## See Also

- [Cargo In Depth](../03-project-setup-and-tooling/cargo-in-depth.md) — Cargo.toml, dependency features, build profiles
- [Dependencies](../03-project-setup-and-tooling/dependencies.md) — finding and adding crates
- [Testing](../03-project-setup-and-tooling/testing.md) — unit and integration tests
