//! # 08 - Strings and Text
//!
//! Text handling surprises many newcomers because Rust strings are **UTF-8**.
//!
//! ## What you will learn
//! - `&str` (borrowed view) versus `String` (owned, growable)
//! - Building and modifying strings
//! - Why you cannot index a string with `s[0]`
//! - Iterating `chars()` and `bytes()`
//! - Splitting, trimming, searching, replacing
//! - Parsing text into numbers
//! - Formatting with `format!` / `println!`
//!
//! ## Key ideas in plain English
//! - `&str` is like a *window* onto text stored somewhere else (a literal or a
//!   `String`). It is read-only and has a fixed size.
//! - `String` *owns* its text on the heap and can grow. `&String` converts to
//!   `&str` automatically, so make function parameters `&str`.
//!
//! Text is UTF-8: a character like `é` or `♥` takes 2-4 bytes. So `s.len()`
//! counts **bytes**, not characters, and `s[0]` is disallowed because byte 0
//! may be only half a character.
//!
//! ## Common beginner mistakes
//! - Slicing in the middle of a multi-byte character panics.
//! - Using `+` with two `&str`s (`"a" + "b"` fails; start with a `String`).
//!
//! ## How to run
//! ```text
//! cargo run --example 08_strings_text
//! ```
//!
//! ## Try it yourself
//! Write a function that counts vowels in a string using `chars()`.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch08-02-strings.html>
#![allow(rustdoc::private_intra_doc_links)]


/// Creating and growing strings.
fn creating() {
    println!("--- creating ---");
    let literal: &str = "hello"; // &'static str
    let mut owned = String::from(literal); // or literal.to_string()
    owned.push(' '); // one char
    owned.push_str("world"); // a &str
    println!("{owned}");

    let a = String::from("foo");
    let b = String::from("bar");
    let joined = a + &b; // `a` is moved; `b` is borrowed
    println!("{joined}");

    let formatted = format!("{}-{}-{}", "x", 1, 2.5); // doesn't move anything
    println!("{formatted}");
    println!("{}", ["a", "b", "c"].join(", "));
    println!("{}", "ab".repeat(3));
}

/// Bytes versus characters.
fn unicode() {
    println!("--- unicode ---");
    let s = "héllo ♥";
    println!("bytes = {}, chars = {}", s.len(), s.chars().count()); // 10 vs 7
    for c in "añ♥".chars() {
        println!("char {c} is {} byte(s)", c.len_utf8());
    }
    println!("first byte = {}", s.as_bytes()[0]);
    println!("uppercase = {}", s.to_uppercase());
    // &s[0..2] would panic: 'é' occupies bytes 1..3
}

/// The everyday query / transform methods.
fn searching_and_transforming() {
    println!("--- searching & transforming ---");
    let line = "  2024-05-01, ERROR, disk full  ";
    let trimmed = line.trim();
    println!("[{trimmed}]");
    println!("starts_with 2024: {}", trimmed.starts_with("2024"));
    println!("contains ERROR : {}", trimmed.contains("ERROR"));
    println!("find ','       : {:?}", trimmed.find(','));
    println!("replace        : {}", trimmed.replace("ERROR", "WARN"));

    let parts: Vec<&str> = trimmed.split(',').map(|p| p.trim()).collect();
    println!("{parts:?}");
    println!("split_once     : {:?}", trimmed.split_once(", "));
    for (i, word) in "one two  three".split_whitespace().enumerate() {
        print!("{i}={word} ");
    }
    println!();
    for line in "a\nb\nc".lines() {
        print!("<{line}>");
    }
    println!();
}

/// Turning text into numbers (and handling failure).
fn parsing() {
    println!("--- parsing ---");
    let n: i32 = "42".parse().unwrap(); // type annotation drives parse::<i32>
    let f = "3.5".parse::<f64>().unwrap();
    println!("{n} {f}");
    match "abc".parse::<i32>() {
        Ok(v) => println!("parsed {v}"),
        Err(e) => println!("error: {e}"),
    }
    println!("{:?}", "true".parse::<bool>());
}

/// Formatting mini-language cheat sheet.
fn formatting() {
    println!("--- formatting ---");
    let name = "Ada";
    let pi = 3.14159265;
    println!("inline : {name} {pi:.2}"); // inline variables
    println!("width  : [{:>8}] [{:<8}] [{:^8}]", "r", "l", "c");
    println!("pad    : [{:05}] [{:+}]", 42, 7);
    println!("radix  : {:b} {:o} {:x} {:X} {:#x}", 10, 10, 255, 255, 255);
    println!("debug  : {:?} {:#?}", Some(1), (1, "two"));
    println!("named  : {a} {b} {a}", a = 1, b = 2);
    println!("braces : {{literal}}");
    eprintln!("(this line goes to stderr)");
}

fn main() {
    creating();
    unicode();
    searching_and_transforming();
    parsing();
    formatting();
}
