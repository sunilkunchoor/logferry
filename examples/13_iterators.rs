//! # 13 - Iterators
//!
//! An iterator produces a sequence of values one at a time. Iterator chains
//! are idiomatic Rust and compile to loops as fast as hand-written ones
//! ("zero-cost abstraction").
//!
//! ## What you will learn
//! - The `Iterator` trait and `next()`
//! - `iter()`, `iter_mut()`, `into_iter()` (borrow, mutably borrow, consume)
//! - Adapters (lazy): `map`, `filter`, `enumerate`, `zip`, `take`, `skip`,
//!   `chain`, `flat_map`, `rev`, `windows`, `chunks`
//! - Consumers (run it): `collect`, `sum`, `fold`, `count`, `any`, `all`,
//!   `find`, `position`, `min`, `max`
//! - Writing your own iterator
//!
//! ## Key ideas in plain English
//! **Adapters are lazy**: `v.iter().map(f)` does *nothing* until a consumer
//! such as `collect()` or a `for` loop pulls values through it. Think of a
//! factory assembly line that only runs when someone waits at the end.
//!
//! | Method | Items you get |
//! |--------|---------------|
//! | `.iter()` | `&T` |
//! | `.iter_mut()` | `&mut T` |
//! | `.into_iter()` | `T` (consumes the collection) |
//!
//! ## Common beginner mistakes
//! - Forgetting to `collect()` (the compiler warns "iterators are lazy").
//! - `collect()` without a type: `let v: Vec<_> = ...` or `collect::<Vec<_>>()`.
//! - Using the collection after `into_iter()` consumed it.
//!
//! ## How to run
//! ```text
//! cargo run --example 13_iterators
//! ```
//!
//! ## Try it yourself
//! Sum the squares of all odd numbers below 100 with one chain.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch13-02-iterators.html>
#![allow(rustdoc::private_intra_doc_links)]


/// A custom iterator yielding the Fibonacci numbers.
struct Fibonacci {
    /// Current value.
    current: u64,
    /// Next value.
    next: u64,
}

impl Fibonacci {
    /// Starts at 0, 1, 1, 2, 3, ...
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

/// Implement `next` and you get **all** the adapter methods for free.
impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let out = self.current;
        self.current = self.next;
        self.next += out;
        Some(out) // infinite: never returns None
    }
}

fn basics() {
    println!("--- basics ---");
    let v = vec![1, 2, 3];
    let mut it = v.iter();
    println!("{:?} {:?} {:?} {:?}", it.next(), it.next(), it.next(), it.next());

    let mut m = vec![1, 2, 3];
    for x in m.iter_mut() {
        *x += 10;
    }
    println!("{m:?}");
    let owned: Vec<String> = vec!["a".to_string(), "b".to_string()];
    for s in owned {
        // consumes `owned`
        print!("{s} ");
    }
    println!();
}

fn adapters() {
    println!("--- adapters ---");
    let nums = [1, 2, 3, 4, 5, 6];
    let squares: Vec<i32> = nums.iter().map(|x| x * x).collect();
    let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
    println!("{squares:?} {evens:?}");

    let pairs: Vec<(i32, char)> = nums.iter().copied().zip("abc".chars()).collect(); // stops at shortest
    println!("{pairs:?}");
    let window_sums: Vec<i32> = nums.windows(3).map(|w| w.iter().sum()).collect();
    println!("windows -> {window_sums:?}");
    println!("chunks  -> {:?}", nums.chunks(4).collect::<Vec<_>>());
    println!("skip/take -> {:?}", nums.iter().skip(1).take(3).collect::<Vec<_>>());
    println!("chain/rev -> {:?}", (1..3).chain(7..9).rev().collect::<Vec<_>>());

    let nested = vec![vec![1, 2], vec![3], vec![]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten -> {flat:?}");
    let words = ["hi there", "rust rocks"];
    let all: Vec<&str> = words.iter().flat_map(|s| s.split(' ')).collect();
    println!("flat_map -> {all:?}");

    // filter_map: map + drop the Nones in one go
    let parsed: Vec<i32> = ["1", "x", "3"].iter().filter_map(|s| s.parse().ok()).collect();
    println!("filter_map -> {parsed:?}");
}

fn consumers() {
    println!("--- consumers ---");
    let nums = vec![4, 8, 15, 16, 23, 42];
    println!("sum={} product(1..5)={}", nums.iter().sum::<i32>(), (1..=5).product::<i32>());
    println!("count={} min={:?} max={:?}", nums.iter().count(), nums.iter().min(), nums.iter().max());
    println!("any>40: {}  all>3: {}", nums.iter().any(|&n| n > 40), nums.iter().all(|&n| n > 3));
    println!("find>10: {:?}  position(16): {:?}", nums.iter().find(|&&n| n > 10), nums.iter().position(|&n| n == 16));
    let folded = nums.iter().fold(String::new(), |acc, n| format!("{acc}{n};"));
    println!("fold -> {folded}");
    let (small, big): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&&n| n < 16);
    println!("partition -> {small:?} {big:?}");
    println!("max_by_key -> {:?}", ["aa", "b", "cccc"].iter().max_by_key(|s| s.len()));

    // collect into other types
    let s: String = vec!['r', 'u', 's', 't'].into_iter().collect();
    println!("{s}");
    let r: Result<Vec<i32>, _> = ["1", "2", "z"].iter().map(|s| s.parse::<i32>()).collect();
    println!("collect Result -> {}", if r.is_err() { "Err (stopped at first failure)" } else { "Ok" });
}

fn custom() {
    println!("--- custom iterator ---");
    let first10: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{first10:?}");
    let sum_even: u64 = Fibonacci::new().take_while(|&n| n < 100).filter(|n| n % 2 == 0).sum();
    println!("sum of even fibs below 100 = {sum_even}");
}

fn main() {
    basics();
    adapters();
    consumers();
    custom();
}
