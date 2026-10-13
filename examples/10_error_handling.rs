//! # 10 - Error Handling
//!
//! Rust has no exceptions. Failures are **values** you return and must deal
//! with.
//!
//! ## What you will learn
//! - `Result<T, E>`: success (`Ok`) or failure (`Err`)
//! - The `?` operator for propagating errors
//! - `unwrap`, `expect`, `unwrap_or`, `map_err`
//! - Writing your own error type and implementing `Display`/`Error`
//! - Converting errors automatically with `From`
//! - `Box<dyn Error>` for quick "any error" signatures
//! - When to `panic!`
//!
//! ## Key ideas in plain English
//! - **Recoverable** problems (file missing, bad input) -> return `Result`.
//! - **Bugs** (impossible state, broken invariant) -> `panic!`.
//!
//! `?` means: "if this is an `Err`, return it from the current function now;
//! otherwise give me the `Ok` value." It also converts the error type using
//! `From`, which is what makes custom error enums pleasant.
//!
//! `unwrap()` is fine for quick experiments and tests, but in real code
//! prefer `?`, `expect("reason")`, or explicit handling.
//!
//! ## Common beginner mistakes
//! - Using `?` in `main` that returns `()`: change it to
//!   `fn main() -> Result<(), Box<dyn Error>>`.
//! - Ignoring a `Result` (the compiler warns: `unused_must_use`).
//!
//! ## How to run
//! ```text
//! cargo run --example 10_error_handling
//! ```
//!
//! ## Try it yourself
//! Add a `TooLong(usize)` variant to [`ConfigError`] and enforce a max length.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch09-00-error-handling.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Our own error type: an enum listing everything that can go wrong.
#[derive(Debug)]
enum ConfigError {
    /// The input was empty.
    Empty,
    /// The text was not a valid integer. Wraps the underlying error.
    BadNumber(ParseIntError),
    /// The number was outside the allowed range.
    OutOfRange(i64),
}

/// `Display` is the human-readable message.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Empty => write!(f, "value was empty"),
            ConfigError::BadNumber(e) => write!(f, "not a number: {e}"),
            ConfigError::OutOfRange(n) => write!(f, "{n} is out of range 1..=100"),
        }
    }
}

/// Implementing `Error` lets it work with `Box<dyn Error>` and `source()`.
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::BadNumber(e) => Some(e),
            _ => None,
        }
    }
}

/// This `From` impl is what lets `?` turn a `ParseIntError` into a `ConfigError`.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

/// Parses a "port-like" setting in `1..=100`.
///
/// # Errors
/// Returns [`ConfigError`] if the text is empty, not a number, or out of range.
fn parse_setting(text: &str) -> Result<i64, ConfigError> {
    if text.trim().is_empty() {
        return Err(ConfigError::Empty);
    }
    let n: i64 = text.trim().parse()?; // `?` converts ParseIntError via From
    if !(1..=100).contains(&n) {
        return Err(ConfigError::OutOfRange(n));
    }
    Ok(n)
}

/// Chains several fallible steps; any failure exits early.
fn sum_settings(a: &str, b: &str) -> Result<i64, ConfigError> {
    Ok(parse_setting(a)? + parse_setting(b)?)
}

/// `Box<dyn Error>` accepts *any* error type: convenient in apps and examples.
fn read_number(s: &str) -> Result<i32, Box<dyn Error>> {
    let n: i32 = s.parse()?; // ParseIntError -> Box<dyn Error>
    if n < 0 {
        return Err("negative numbers not allowed".into()); // &str -> Box<dyn Error>
    }
    Ok(n)
}

/// The handy helper methods on `Result`/`Option`.
fn helpers() {
    println!("--- helpers ---");
    let ok: Result<i32, String> = Ok(5);
    let bad: Result<i32, String> = Err("boom".into());
    println!("unwrap_or : {} {}", ok.clone().unwrap_or(0), bad.clone().unwrap_or(0));
    println!("unwrap_or_else: {}", bad.clone().unwrap_or_else(|e| e.len() as i32));
    println!("map       : {:?}", ok.clone().map(|n| n * 2));
    println!("map_err   : {:?}", bad.clone().map_err(|e| e.to_uppercase()));
    println!("and_then  : {:?}", ok.and_then(|n| if n > 3 { Ok(n) } else { Err("small".to_string()) }));
    println!("ok()      : {:?}", bad.ok());
    // `expect` panics with your message - use for "can't happen" cases.
    let v = "7".parse::<i32>().expect("literal 7 is a valid integer");
    println!("expect    : {v}");
}

fn main() {
    println!("--- custom error + ? ---");
    for input in ["42", "", "abc", "500"] {
        match parse_setting(input) {
            Ok(n) => println!("{input:?} -> ok {n}"),
            Err(e) => println!("{input:?} -> error: {e}"),
        }
    }
    println!("sum: {:?}", sum_settings("10", "20").ok());
    println!("sum: {}", sum_settings("10", "x").unwrap_err());

    println!("--- error source chain ---");
    if let Err(e) = parse_setting("abc") {
        println!("error: {e}");
        if let Some(src) = e.source() {
            println!("caused by: {src}");
        }
    }

    println!("--- Box<dyn Error> ---");
    for s in ["5", "-1", "x"] {
        match read_number(s) {
            Ok(n) => println!("{s} -> {n}"),
            Err(e) => println!("{s} -> {e}"),
        }
    }

    helpers();

    println!("--- panic (caught) ---");
    // Panics unwind the stack. We catch one here only to show it; real
    // programs normally let a panic stop the thread.
    let result = std::panic::catch_unwind(|| {
        let v: Vec<i32> = Vec::new();
        v[0] // index out of bounds -> panic
    });
    println!("caught panic? {}", result.is_err());
}
