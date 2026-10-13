//! # 05 - Lifetimes
//!
//! Lifetimes are how the compiler proves that **a reference never outlives
//! the data it points to** (no dangling pointers).
//!
//! ## What you will learn
//! - Why the compiler sometimes asks for `'a`
//! - Lifetime annotations on functions
//! - Lifetimes inside structs
//! - Lifetime elision (when you can leave them out)
//! - `'static`
//!
//! ## Key ideas in plain English
//! A lifetime annotation does **not** change how long anything lives. It only
//! *describes a relationship*: "the returned reference is valid for as long as
//! both of these inputs are". The compiler then checks callers obey it.
//!
//! `'a` is read "tick a". The names are arbitrary (`'a`, `'b`, `'input`).
//!
//! ## Elision rules (why you rarely write them)
//! 1. Each reference parameter gets its own lifetime.
//! 2. If there is exactly one input lifetime, it is given to all outputs.
//! 3. If one input is `&self`/`&mut self`, its lifetime is given to outputs.
//!
//! ## Common beginner mistakes
//! - `error[E0106]: missing lifetime specifier` - a function returns a
//!   reference but the compiler cannot tell which input it borrows from.
//! - Trying to return a reference to a local variable (`E0515`). Return the
//!   owned value (`String`) instead.
//!
//! ## How to run
//! ```text
//! cargo run --example 05_lifetimes
//! ```
//!
//! ## Try it yourself
//! In [`dangling_demo`], uncomment the block and read the error.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html>
#![allow(rustdoc::private_intra_doc_links)]


/// Returns the longer of two string slices.
///
/// Without `'a` the compiler cannot know whether the result borrows from
/// `x` or `y`. The annotation says: *the result lives no longer than the
/// shorter of the two inputs.*
///
/// # Examples
/// ```text
/// // longest("hello", "hi") == "hello"
/// ```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/// Elision at work: one input reference, so the output gets its lifetime.
/// Written out in full it would be `fn first_char<'a>(s: &'a str) -> &'a str`.
fn first_char(s: &str) -> &str {
    &s[..1]
}

/// A struct that **borrows** data must declare how long the borrow lasts.
///
/// The `Excerpt` cannot outlive the `String` it points into.
#[derive(Debug)]
struct Excerpt<'a> {
    /// A slice borrowed from some longer text.
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Returns the borrowed part (elision rule 3: tied to `&self`).
    fn part(&self) -> &str {
        self.part
    }
}

/// Shows two lifetimes: the result is tied to `x` only, not to `y`.
fn pick_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

/// `'static` means "lives for the whole program". String literals are baked
/// into the binary, so they are `&'static str`.
fn static_lifetime() -> &'static str {
    "I live forever"
}

/// Demonstrates the *dangling reference* the compiler prevents.
fn dangling_demo() {
    println!("--- dangling references ---");
    let outer;
    {
        let inner = 5;
        outer = inner; // copying is fine. Borrowing (`&inner`) would not be:
        // outer = &inner; // <- error[E0597]: `inner` does not live long enough
    }
    println!("outer = {outer}");
}

fn main() {
    println!("--- longest ---");
    let a = String::from("a long string");
    let result;
    {
        let b = String::from("short");
        result = longest(a.as_str(), b.as_str());
        println!("longest = {result}"); // must be used while `b` is alive
    }
    // Using `result` here would be an error: it might point to `b`.

    println!("--- elision ---");
    println!("first_char = {}", first_char("Rust"));

    println!("--- struct with lifetime ---");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("no '.' found");
    let excerpt = Excerpt { part: first_sentence };
    println!("{excerpt:?} -> {}", excerpt.part());

    println!("--- two lifetimes ---");
    println!("{}", pick_first("left", "right"));

    println!("--- static ---");
    println!("{}", static_lifetime());

    dangling_demo();
}
