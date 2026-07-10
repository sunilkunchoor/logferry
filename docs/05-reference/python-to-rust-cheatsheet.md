# Python → Rust Cheatsheet

Side-by-side quick lookup for Python developers. Each row shows the Python idiom and its idiomatic Rust equivalent.

---

## Key Concepts

| Concept | Python | Rust |
|---|---|---|
| Memory management | GC / refcount | Ownership + borrow checker |
| Nullability | `None` on any variable | `Option<T>` — explicit, enforced |
| Error handling | `raise` / `try/except` | `Result<T, E>` / `?` operator |
| Type checking | Runtime (+ optional hints) | Compile-time, always |
| Mutability | Mutable by default | Immutable by default (`mut` opt-in) |
| String types | `str` (one type) | `String` (owned) + `&str` (borrowed) |
| Iteration | `for x in iterable` | `for x in iter` (lazy by default) |
| Interfaces | `Protocol` / `ABC` | `trait` |
| Generics | `TypeVar` (hints only) | `<T>` (compile-time, zero cost) |
| Concurrency | GIL limits CPU parallelism | No GIL; `Send`/`Sync` prove safety |
| Integer overflow | Silent widening | Panic (debug) / wrapping (release) |
| Import | `import`, `from … import` | `use`, `pub use` |

---

## Variables and Types

| Python | Rust |
|---|---|
| `x = 10` | `let x = 10;` |
| `x = 10; x = 20` (rebind) | `let mut x = 10; x = 20;` |
| `MY_CONST = 42` | `const MAX: u32 = 42;` |
| `x = "42"; x = int(x)` | `let x = "42"; let x: i64 = x.parse().unwrap();` (shadowing) |
| `x: int = 42` | `let x: i32 = 42;` |
| `x: float = 3.14` | `let x: f64 = 3.14;` |
| `x: bool = True` | `let x: bool = true;` |
| `x: str = "hello"` | `let x: &str = "hello";` (literal) or `let x = String::from("hello");` (owned) |

---

## Functions

| Python | Rust |
|---|---|
| `def add(a: int, b: int) -> int: return a + b` | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| `return a + b` | `a + b` (implicit return — last expression without `;`) |
| `def f(x, y=4): ...` | `fn f(x: i32, y: i32) -> ...` + `#[pyo3(signature = (x, y=4))]` |
| `return a, b` | `(a, b)` (tuple) |
| `a, b = f()` | `let (a, b) = f();` |
| `*args` | No direct equivalent; use `Vec<T>` or slices |

---

## Strings

| Python | Rust |
|---|---|
| `s.strip()` | `s.trim()` |
| `s.upper()` | `s.to_uppercase()` |
| `"sub" in s` | `s.contains("sub")` |
| `s.replace("a", "b")` | `s.replace("a", "b")` |
| `s.split(",")` | `s.split(',').collect::<Vec<_>>()` |
| `",".join(v)` | `v.join(",")` |
| `f"hello {name}"` | `format!("hello {name}")` |
| `len(s)` | `s.len()` |
| `str(x)` | `x.to_string()` |
| `int(s)` | `s.parse::<i64>()` → `Result` |

---

## Control Flow

| Python | Rust |
|---|---|
| `if x > 0: ...` | `if x > 0 { ... }` |
| `x if cond else y` | `if cond { x } else { y }` (expression) |
| `while q: q.pop()` | `while !q.is_empty() { q.pop(); }` |
| `for i in range(5): ...` | `for i in 0..5 { ... }` |
| `for i in range(1, 6): ...` | `for i in 1..=5 { ... }` |
| `for x in lst: ...` | `for x in &vec { ... }` |
| `for i, x in enumerate(lst): ...` | `for (i, x) in vec.iter().enumerate() { ... }` |
| `break` | `break` |
| `continue` | `continue` |
| `match x: case ...: ...` | `match x { ... => ..., }` (exhaustive) |
| `if isinstance(x, T): ...` | `if let T::Variant { .. } = x { ... }` |

---

## Collections

| Python | Rust |
|---|---|
| `lst = []` | `let mut v: Vec<i32> = Vec::new();` |
| `lst.append(x)` | `v.push(x);` |
| `lst.pop()` | `v.pop()` → `Option<T>` |
| `lst[0]` | `v[0]` (panics) or `v.get(0)` → `Option<&T>` |
| `len(lst)` | `v.len()` |
| `x in lst` | `v.contains(&x)` |
| `lst.sort()` | `v.sort();` |
| `[x*2 for x in lst]` | `v.iter().map(\|x\| x * 2).collect::<Vec<_>>()` |
| `[x for x in lst if x > 0]` | `v.iter().filter(\|&&x\| x > 0).collect::<Vec<_>>()` |
| `d = {}` | `let mut m: HashMap<K, V> = HashMap::new();` |
| `d[k] = v` | `m.insert(k, v);` |
| `d.get(k)` | `m.get(&k)` → `Option<&V>` |
| `d.get(k, default)` | `m.get(&k).copied().unwrap_or(default)` |
| `k in d` | `m.contains_key(&k)` |
| `d.items()` | `m.iter()` |
| `d[k] = d.get(k, 0) + 1` | `*m.entry(k).or_insert(0) += 1;` |
| `s = set()` | `let mut s: HashSet<T> = HashSet::new();` |
| `s.add(x)` | `s.insert(x);` |
| `x in s` | `s.contains(&x)` |
| `a & b` | `a.intersection(&b).collect()` |
| `a \| b` | `a.union(&b).collect()` |

---

## Error Handling

| Python | Rust |
|---|---|
| `raise ValueError("msg")` | `return Err(MyError::BadInput("msg".into()))` |
| `try: ... except E as e: ...` | `match result { Ok(v) => ..., Err(e) => ... }` |
| `x = f() or default` | `let x = f().unwrap_or(default);` |
| Automatic exception propagation | `?` operator (explicit propagation) |
| `Optional[T]` | `Option<T>` |
| `None` | `None` (in `Option<T>` context) |
| `x is not None` | `x.is_some()` |
| `x if x is not None else d` | `x.unwrap_or(d)` |

---

## Classes and OOP

| Python | Rust |
|---|---|
| `class Foo:` | `struct Foo { ... }` |
| `def __init__(self, x): self.x = x` | `fn new(x: i32) -> Self { Foo { x } }` (in `impl Foo`) |
| `def method(self): ...` | `fn method(&self) { ... }` (in `impl Foo`) |
| `def mut_method(self): self.x = 1` | `fn mut_method(&mut self) { self.x = 1; }` |
| `class ABC: @abstractmethod def f(self): ...` | `trait MyTrait { fn f(&self); }` |
| `class Foo(MyProtocol): def f(self): ...` | `impl MyTrait for Foo { fn f(&self) { ... } }` |
| `isinstance(x, Foo)` | type system (no runtime check needed in most cases) |
| `@dataclass class Foo: x: int` | `#[derive(Default)] struct Foo { x: i32 }` |
| `copy.deepcopy(x)` | `x.clone()` |

---

## Concurrency

| Python | Rust |
|---|---|
| `threading.Thread(target=f)` | `thread::spawn(move \|\| f())` |
| CPU-limited by GIL | No GIL — truly parallel |
| `multiprocessing.Pool` | `thread::scope` + work-stealing |
| `threading.Lock()` | `Mutex<T>` |
| `threading.RLock()` | No equivalent (design around it) |
| Share data via `queue.Queue` | Pass by ownership or `Arc<Mutex<T>>` |
| `concurrent.futures.ThreadPoolExecutor` | `rayon::ThreadPool` (crate) |

---

## Python Libraries to Rust Crates

| Python | Rust crate |
|---|---|
| `json` | `serde_json` |
| `pydantic` | `serde` + derive |
| `requests` | `reqwest` |
| `asyncio` | `tokio` |
| `click` / `argparse` | `clap` |
| `logging` | `tracing` |
| `datetime` | `chrono` |
| `pathlib` | `std::path::PathBuf` |
| `re` | `regex` |
| `uuid` | `uuid` |
| `python-dotenv` | `dotenvy` |
| `psycopg2` | `sqlx` |
| `redis-py` | `redis` |
| `pytest` | built-in `#[test]` + `cargo test` |
