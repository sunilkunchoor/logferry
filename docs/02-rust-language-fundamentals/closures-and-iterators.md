# Closures and Iterators

> **Prerequisites:** [Collections](collections.md)  
> **Next:** [Traits and Generics](traits-and-generics.md)

Rust closures are like Python lambdas but without the one-expression limit. Rust iterators are lazy (like Python generators) and chain without intermediate allocations. Together they form the idiomatic way to transform data.

---

## Closures

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

### Closures Capture Their Environment

```python
threshold = 0.9
good = list(filter(lambda s: s > threshold, scores))  # captures threshold
```

```rust
let threshold = 0.9_f64;
let good: Vec<_> = scores.iter().filter(|&&s| s > threshold).collect();
// threshold is captured by reference automatically
```

### `move` Closures — Take Ownership of Captured Values

```rust
let prefix = String::from("svc");
let label_fn = move |name: &str| format!("{prefix}:{name}");
// prefix is moved into the closure; caller can no longer use prefix
```

Use `move` when the closure must outlive the scope where the captured variable was defined — for example, when passing a closure into a spawned thread.

### Multi-Line Closures

```rust
let process = |line: &str| -> Option<u64> {
    let parsed: serde_json::Value = serde_json::from_str(line).ok()?;
    let level = parsed["level"].as_str()?;
    if level == "ERROR" { Some(1) } else { None }
};
```

---

## Iterators

Rust iterators are lazy — no work happens until you call a consuming method like `.collect()`, `.sum()`, or `.for_each()`. Intermediate adapters like `.filter()` and `.map()` just build up a description of the pipeline.

```python
scores = [0.91, 0.62, 0.87, 0.45, 0.93]

# filter → map → sum in one expression
total = sum(
    s * 1.1
    for s in scores
    if s > 0.7
)

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

// zip
for (name, score) in services.iter().zip(scores.iter()) {
    println!("{name} {score}");
}
```

### Common Iterator Adapters

```rust
let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

// map — transform each element
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();

// filter — keep elements matching a predicate
let big: Vec<&i32> = v.iter().filter(|&&x| x > 4).collect();

// filter_map — filter and transform in one step
let parsed: Vec<i32> = ["1", "two", "3"]
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())  // drops Err, keeps Ok
    .collect();

// fold — like Python's functools.reduce
let product: i64 = v.iter().fold(1, |acc, &x| acc * x as i64);

// any / all — like Python's any() and all()
let any_large = v.iter().any(|&x| x > 8);
let all_pos   = v.iter().all(|&x| x > 0);

// find — returns Option<&T>
let first_big = v.iter().find(|&&x| x > 4);

// count / sum / max / min
let n       = v.iter().count();
let total   = v.iter().sum::<i32>();
let biggest = v.iter().max();

// flat_map — like itertools.chain.from_iterable
let nested = vec![vec![1, 2], vec![3, 4]];
let flat: Vec<i32> = nested.iter().flat_map(|v| v.iter().copied()).collect();

// take / skip — like islice
let first_three: Vec<_> = v.iter().take(3).collect();
let skip_two:    Vec<_> = v.iter().skip(2).collect();

// enumerate — like Python's enumerate()
for (i, x) in v.iter().enumerate() {
    println!("{i}: {x}");
}

// chain — concatenate iterators
let a = vec![1, 2];
let b = vec![3, 4];
let combined: Vec<_> = a.iter().chain(b.iter()).collect();
```

### `iter()` vs `into_iter()` vs `iter_mut()`

| Method | Gives you | Ownership |
|---|---|---|
| `.iter()` | `&T` | borrows the collection |
| `.into_iter()` | `T` | consumes the collection |
| `.iter_mut()` | `&mut T` | mutably borrows the collection |

```rust
let names = vec!["alice", "bob", "carol"];

// iter() — borrow
for name in names.iter() { println!("{name}"); }   // names still valid

// into_iter() — consume (use when you want to move values out)
for name in names.into_iter() { println!("{name}"); }
// names is now invalid (moved into the iterator)
```

---

## Iterators in `logferry`

`logferry`'s parallel merge uses `fold` on an iterator of thread results:

```rust
handles
    .into_iter()
    .map(|h| h.join().expect("worker panicked"))
    .fold(IngestStats::default(), IngestStats::merge)
```

This is idiomatic Rust: chain `.map()` to extract results, then `.fold()` to reduce them into one value — all without a mutable accumulator variable.

---

## See Also

- [Traits and Generics](traits-and-generics.md) — the `Iterator` trait and how to implement it
- [Collections](collections.md) — `.collect()` materialises iterators into concrete collections
- [logferry Walkthrough](../04-pyo3-and-logferry/logferry-walkthrough.md) — `fold` used for merging thread results
