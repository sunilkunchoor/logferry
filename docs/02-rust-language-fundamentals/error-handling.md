# Error Handling

> **Prerequisites:** [Structs and Enums](structs-and-enums.md)  
> **Next:** [Collections](collections.md)

Rust does not have exceptions. Instead, it has two built-in enum types — `Option<T>` for "maybe a value" and `Result<T, E>` for "success or failure" — and the `?` operator for propagating errors up the call stack. The compiler forces you to handle both cases, making error paths explicit and type-checked.

---

## `Option<T>` — Representing "Maybe a Value"

Python uses `None` to mean "no value." Rust wraps optional values in `Option<T>` — you can't accidentally use a `None` as if it were a real value because the type system prevents it.

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

### Common Option Methods

```rust
let t: Option<f64> = find_threshold("f1");

// unwrap_or — provide a default (like dict.get(key, default))
let value = t.unwrap_or(0.0);

// map — transform the inner value if it is Some
let adjusted = t.map(|v| v + 0.05);

// and_then — chain operations that might also return None
let result = t.and_then(|v| if v > 0.8 { Some(v) } else { None });

// is_some / is_none
if t.is_some() { println!("found"); }

// unwrap — panics if None; only use when None is truly impossible
let must_exist = t.unwrap();

// expect — panics with a custom message; better than unwrap in diagnostics
let must_exist = t.expect("threshold must be configured");
```

---

## `Result<T, E>` — Representing "Success or Failure"

Python uses exceptions. Rust uses `Result<T, E>` — success is `Ok(value)`, failure is `Err(error)`. The compiler forces you to handle both cases; you cannot ignore an error silently.

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

### The `?` Operator — Propagate Errors Up the Call Stack

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

`?` unwraps `Ok(v)` and continues, or returns `Err(e)` immediately from the current function — exactly what Python exceptions do implicitly, but explicit and type-checked.

### Common Result Methods

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

## `logferry`'s Approach to Errors

`logferry` uses `Result` in two distinct ways, depending on the context:

### Batch ingestion — count and continue

Inside `ingest_chunk`, a bad line should not abort a 200,000-line batch. So errors are *not* propagated with `?` — they are counted and sampled:

```rust
match serde_json::from_str(line) {
    Ok(r) => r,
    Err(e) => {
        // count it, keep going
        stats.parse_errors += 1;
        continue;
    }
};
```

### Single-line validation — fail loudly

`validate_line` *does* use `?` because a single bad line should raise a Python `ValueError`:

```rust
fn validate_line(line: &str) -> PyResult<bool> {
    let record: LogRecord = serde_json::from_str(line)
        .map_err(|e| PyValueError::new_err(format!("invalid JSON: {e}")))?;
    NonEmptyMessage.validate(&record)?;   // IngestError auto-converts to PyErr
    Ok(true)
}
```

The auto-conversion from `IngestError` to `PyErr` is one `impl` block:

```rust
impl From<IngestError> for PyErr {
    fn from(err: IngestError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
```

Once that exists, every `?` on an `IngestError` inside a `PyResult`-returning function becomes a Python `ValueError` automatically. See [Error Handling in PyO3](../04-pyo3-and-logferry/error-handling-pyo3.md) for the full pattern.

---

## See Also

- [Collections](collections.md) — next article
- [Error Handling in PyO3](../04-pyo3-and-logferry/error-handling-pyo3.md) — converting `Result` to Python exceptions
- [logferry Walkthrough](../04-pyo3-and-logferry/logferry-walkthrough.md) — see both error strategies in the real code
