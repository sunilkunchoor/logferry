//! # 09 - Collections
//!
//! Collections store many values on the heap and can grow or shrink at runtime
//! (unlike arrays, whose size is fixed).
//!
//! ## What you will learn
//! - `Vec<T>`: a growable list (the one you will use most)
//! - `HashMap<K, V>`: key -> value lookup
//! - `HashSet<T>`: unique values
//! - `BTreeMap`: like `HashMap` but sorted by key
//! - `VecDeque`: a double-ended queue
//! - The `entry` API for "insert or update"
//!
//! ## Key ideas in plain English
//! | Need | Use |
//! |------|-----|
//! | ordered list, index access | `Vec` |
//! | fast lookup by key | `HashMap` |
//! | sorted keys / range queries | `BTreeMap` |
//! | "have I seen this?" | `HashSet` |
//! | push/pop at both ends | `VecDeque` |
//!
//! Collections **own** their elements. Pushing a `String` moves it in.
//!
//! ## Common beginner mistakes
//! - `v[10]` on a short vec panics; use `v.get(10)` which returns an `Option`.
//! - Holding a reference into a `Vec` and then `push`ing (it may reallocate!).
//!   The borrow checker stops you.
//! - Expecting `HashMap` iteration to be in insertion order. It is arbitrary.
//!
//! ## How to run
//! ```text
//! cargo run --example 09_collections
//! ```
//!
//! ## Try it yourself
//! Count how often each word appears in a sentence using [`word_count`].
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch08-00-common-collections.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

/// Everyday `Vec` operations.
fn vectors() {
    println!("--- Vec ---");
    let mut v = vec![3, 1, 2]; // vec! macro
    v.push(10);
    v.insert(0, 99);
    println!("{v:?} len={} first={:?}", v.len(), v.first());
    println!("pop -> {:?}", v.pop());
    println!("get(100) -> {:?}", v.get(100)); // safe access
    v.sort();
    v.dedup();
    println!("sorted {v:?}, contains 2? {}", v.contains(&2));
    v.retain(|&x| x != 99); // keep only matching
    println!("after retain {v:?}");

    for x in v.iter_mut() {
        *x *= 2; // modify in place through &mut
    }
    println!("doubled {v:?}");

    let mut empty: Vec<String> = Vec::new(); // type needed when empty
    empty.push("hello".to_string());
    println!("{empty:?}, capacity>=1: {}", empty.capacity() >= 1);
}

/// Counts words with the `entry` API: "insert if missing, then update".
fn word_count(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// `HashMap` basics.
fn hash_maps() {
    println!("--- HashMap ---");
    let mut ages: HashMap<&str, u32> = HashMap::new();
    ages.insert("Ada", 36);
    ages.insert("Linus", 54);
    ages.insert("Ada", 37); // overwrites
    println!("Ada = {:?}, Bob = {:?}", ages.get("Ada"), ages.get("Bob"));
    println!("contains Linus: {}", ages.contains_key("Linus"));
    ages.remove("Linus");
    println!("len = {}", ages.len());

    let counts = word_count("the cat and The dog and THE bird");
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0))); // by count desc, then key
    println!("{sorted:?}");
}

/// `HashSet` stores unique values and supports set algebra.
fn hash_sets() {
    println!("--- HashSet ---");
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let mut inter: Vec<_> = a.intersection(&b).copied().collect();
    inter.sort();
    let mut uni: Vec<_> = a.union(&b).copied().collect();
    uni.sort();
    println!("intersection {inter:?}, union {uni:?}");

    let mut seen = HashSet::new();
    for x in [1, 2, 2, 3, 1] {
        if !seen.insert(x) {
            println!("{x} is a duplicate");
        }
    }
}

/// `BTreeMap` keeps keys sorted; good for ordered output.
fn btree() {
    println!("--- BTreeMap ---");
    let mut m = BTreeMap::new();
    m.insert(3, "three");
    m.insert(1, "one");
    m.insert(2, "two");
    println!("{m:?}"); // always sorted
    for (k, v) in m.range(2..) {
        println!("{k} => {v}");
    }
}

/// `VecDeque` is a queue (FIFO) with efficient ends.
fn queue() {
    println!("--- VecDeque ---");
    let mut q = VecDeque::new();
    q.push_back("first");
    q.push_back("second");
    q.push_front("zeroth");
    while let Some(item) = q.pop_front() {
        print!("{item} ");
    }
    println!();
}

fn main() {
    vectors();
    hash_maps();
    hash_sets();
    btree();
    queue();
}
