//! # 04 - Ownership, Borrowing and Slices
//!
//! Ownership is Rust's signature feature. It gives memory safety **without**
//! a garbage collector.
//!
//! ## What you will learn
//! - The three ownership rules
//! - Moves versus copies versus clones
//! - Borrowing with `&` (shared) and `&mut` (exclusive)
//! - Slices (`&str`, `&[T]`)
//!
//! ## The three rules
//! 1. Every value has exactly **one owner** (a variable).
//! 2. There can only be one owner at a time.
//! 3. When the owner goes out of scope, the value is **dropped** (freed).
//!
//! ## Analogy
//! Ownership is like owning a physical book. If you *give* it to a friend
//! (a **move**), you no longer have it. You can *photocopy* it (a **clone**),
//! or *lend* it (a **borrow**). While it is lent out for editing (`&mut`),
//! nobody else may read it; many people may read it at once if nobody edits
//! (`&`).
//!
//! ## Borrowing rules
//! At any moment you may have **either**
//! - any number of shared references `&T`, **or**
//! - exactly one mutable reference `&mut T`.
//!
//! and references must never outlive the data (see `05_lifetimes`).
//!
//! ## Common beginner mistakes
//! - `error[E0382]: borrow of moved value` - you used a value after moving it.
//!   Fix: borrow (`&x`) or `.clone()`.
//! - `error[E0499]: cannot borrow as mutable more than once`.
//!
//! ## How to run
//! ```text
//! cargo run --example 04_ownership
//! ```
//!
//! ## Try it yourself
//! Uncomment the broken lines marked `<- error` one at a time and read each message.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>
#![allow(rustdoc::private_intra_doc_links)]


/// Heap types like `String` are **moved** on assignment.
fn moves() {
    println!("--- move ---");
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2
    // println!("{s1}"); // <- error: s1 no longer valid
    println!("s2 = {s2}");
}

/// `.clone()` makes an independent deep copy (costs time/memory).
fn clones() {
    println!("--- clone ---");
    let a = String::from("data");
    let b = a.clone();
    println!("a = {a}, b = {b}"); // both usable
}

/// Simple fixed-size types implement `Copy`: assignment duplicates bits.
///
/// Integers, floats, `bool`, `char`, and tuples/arrays of `Copy` types.
fn copies() {
    println!("--- copy ---");
    let x = 5;
    let y = x; // copy
    println!("x = {x}, y = {y}");
}

/// Takes ownership: the caller's `String` is gone after the call.
fn consume(s: String) {
    println!("consumed '{s}'");
} // `s` dropped here

/// Takes and gives back ownership (awkward - borrowing is nicer).
fn take_and_return(s: String) -> String {
    s
}

/// Ownership and functions.
fn ownership_and_functions() {
    println!("--- ownership & functions ---");
    let s = String::from("moved into fn");
    consume(s);
    // println!("{s}"); // <- error: moved
    let t = take_and_return(String::from("round trip"));
    println!("{t}");
}

/// Borrows immutably: reads without taking ownership.
fn length(s: &String) -> usize {
    s.len()
}

/// Borrows mutably: may modify the caller's value.
fn shout(s: &mut String) {
    s.push_str("!!!");
}

/// References let functions *look at* data without owning it.
fn borrowing() {
    println!("--- borrowing ---");
    let mut word = String::from("hey");
    println!("len = {}", length(&word)); // shared borrow
    shout(&mut word); // exclusive borrow
    println!("word = {word}");

    let r1 = &word;
    let r2 = &word; // many shared borrows are fine
    println!("{r1} {r2}");
    // Non-lexical lifetimes: r1/r2 are no longer used, so a &mut is allowed now.
    let r3 = &mut word;
    r3.push('?');
    println!("{r3}");
}

/// Returns the first word as a slice borrowed from the input.
///
/// Taking `&str` (not `&String`) is more flexible: it accepts both.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// Slices are views into contiguous data. They own nothing.
fn slices() {
    println!("--- slices ---");
    let sentence = String::from("hello brave new world");
    println!("first word: {}", first_word(&sentence));
    println!("literal too: {}", first_word("literal text"));

    let text = "abcdef";
    println!("{} {} {}", &text[..2], &text[2..4], &text[4..]); // ab cd ef

    let numbers = [1, 2, 3, 4, 5];
    let middle: &[i32] = &numbers[1..4];
    println!("{middle:?}, sum = {}", middle.iter().sum::<i32>());
}

fn main() {
    moves();
    clones();
    copies();
    ownership_and_functions();
    borrowing();
    slices();
}
