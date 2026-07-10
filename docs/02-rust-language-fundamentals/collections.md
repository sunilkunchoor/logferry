# Collections

> **Prerequisites:** [Error Handling](error-handling.md)  
> **Next:** [Closures and Iterators](closures-and-iterators.md)

Rust's standard library provides three workhorses that cover the same ground as Python's `list`, `dict`, and `set`: `Vec<T>`, `HashMap<K, V>`, and `HashSet<T>`.

---

## `Vec<T>` — Python's `list`

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

// Safe indexed access — returns Option<&i32>
match items.get(0) {
    Some(v) => println!("{v}"),
    None    => println!("empty"),
}
```

### Common `Vec` Operations

```rust
let mut v = vec![3, 1, 4, 1, 5];

v.sort();                          // in-place sort
v.sort_by(|a, b| b.cmp(a));        // sort descending
v.dedup();                         // remove consecutive duplicates
v.retain(|&x| x > 2);             // keep only elements > 2 (like Python's filter)
v.extend([10, 11, 12]);            // append multiple elements
let sliced = &v[1..3];             // zero-copy slice
let contains = v.contains(&5);     // like Python's `5 in v`
```

---

## `HashMap<K, V>` — Python's `dict`

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

### The `entry` API — Python's `setdefault` Pattern

```python
# Python
counts[key] = counts.get(key, 0) + 1
```

```rust
// Rust
*counts.entry(key).or_insert(0) += 1;
```

`entry()` returns a reference to the existing value if the key exists, or inserts a default first. It avoids a double lookup and is the idiomatic way to update counts — exactly as used in `logferry`'s `by_level` accumulation.

### Common `HashMap` Operations

```rust
let mut map: HashMap<String, u64> = HashMap::new();

// Insert / update
map.insert("key".to_string(), 42);

// Get (returns Option<&V>)
let v = map.get("key");

// Contains
let exists = map.contains_key("key");

// Remove
map.remove("key");

// Iterate
for (k, v) in &map { println!("{k}: {v}"); }

// Collect from an iterator
let word_counts: HashMap<&str, usize> = words
    .iter()
    .map(|w| (*w, 1))
    .collect();
```

---

## `HashSet<T>` — Python's `set`

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

let other: HashSet<_> = ["ranker", "other"]
    .iter()
    .map(|s| s.to_string())
    .collect();

let intersection: HashSet<_> = seen.intersection(&other).collect();
let union:        HashSet<_> = seen.union(&other).collect();
let difference:   HashSet<_> = seen.difference(&other).collect();
```

---

## Collecting Into Collections

Rust's iterator pipeline typically ends with `.collect()` to materialise results into a concrete type. The compiler infers the target type from context:

```rust
// Collect filtered items into a Vec
let errors: Vec<&str> = lines
    .iter()
    .filter(|l| l.starts_with("ERROR"))
    .map(|l| l.as_str())
    .collect();

// Collect into a HashMap
let level_counts: HashMap<&str, usize> = vec!["INFO", "WARN", "INFO"]
    .into_iter()
    .fold(HashMap::new(), |mut map, level| {
        *map.entry(level).or_insert(0) += 1;
        map
    });
```

---

## See Also

- [Closures and Iterators](closures-and-iterators.md) — powerful patterns for transforming collections
- [logferry Walkthrough](../04-pyo3-and-logferry/logferry-walkthrough.md) — see `HashMap<String, u64>` for `by_level` counting
- [Python → Rust Cheatsheet](../05-reference/python-to-rust-cheatsheet.md) — quick collection method lookup
