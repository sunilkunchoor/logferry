//! # 17 - Testing and Documentation
//!
//! Rust has testing built into the language and `cargo`.
//!
//! ## What you will learn
//! - Writing unit tests with `#[test]`
//! - The `#[cfg(test)]` module convention
//! - `assert!`, `assert_eq!`, `assert_ne!` with custom messages
//! - Testing for panics (`#[should_panic]`) and returning `Result` from tests
//! - Doc comments (`///` and `//!`) and doc-tests
//!
//! ## Key ideas in plain English
//! A test is just a function marked `#[test]`. It **passes** if it returns
//! normally and **fails** if it panics. `cargo test` builds the code with a
//! test harness, runs every test in parallel, and reports the result.
//!
//! `#[cfg(test)]` means "only compile this when testing", so test helpers do
//! not bloat your release build.
//!
//! In a normal library crate, tests also live in a top-level `tests/`
//! directory (integration tests) and doc comments containing code blocks are
//! compiled and run too (doc-tests). Doc-tests only run for **library**
//! crates, so the blocks in this file are for illustration.
//!
//! ## Common beginner mistakes
//! - `assert_eq!(a, b)` needs both sides to implement `Debug` and `PartialEq`.
//! - Comparing floats with `==`; compare within an epsilon instead.
//! - Expecting tests to print: output is captured unless you pass
//!   `-- --nocapture`.
//!
//! ## How to run
//! ```text
//! cargo run  --example 17_testing                    # runs main()
//! cargo test --example 17_testing                    # runs the tests
//! cargo test --example 17_testing -- --nocapture     # show println! output
//! cargo test --example 17_testing add                # only tests with "add" in the name
//! ```
//!
//! ## Try it yourself
//! Make `test_is_palindrome_ignores_case` pass by lower-casing the input in
//! [`is_palindrome`], then add a test with punctuation.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch11-00-testing.html>
#![allow(rustdoc::private_intra_doc_links)]


/// Adds two numbers.
///
/// # Examples
/// ```text
/// // In a library crate this code would be compiled and run by `cargo test`.
/// // assert_eq!(add(2, 3), 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides, returning an error for division by zero.
///
/// # Errors
/// Returns `Err("division by zero")` when `b == 0`.
///
/// # Panics
/// Never panics.
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Returns the element at `index`.
///
/// # Panics
/// Panics if `index >= items.len()`. A `# Panics` section is the convention
/// for documenting this.
fn element_at(items: &[i32], index: usize) -> i32 {
    items[index]
}

/// Checks whether text reads the same forwards and backwards (letters only,
/// case-insensitive).
fn is_palindrome(text: &str) -> bool {
    let cleaned: Vec<char> = text.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

fn main() {
    println!("add(2, 3)            = {}", add(2, 3));
    println!("divide(7, 2)         = {:?}", divide(7, 2));
    println!("divide(1, 0)         = {:?}", divide(1, 0));
    println!("element_at([1,2], 1) = {}", element_at(&[1, 2], 1));
    println!("palindrome 'Level'   = {}", is_palindrome("Level"));
    println!("\nNow run:  cargo test --example 17_testing");
}

/// Only compiled by `cargo test`.
#[cfg(test)]
mod tests {
    use super::*; // bring the parent module's items into scope

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_with_message() {
        // optional message after the values
        assert_eq!(add(-2, -3), -5, "adding negatives should work, got {}", add(-2, -3));
    }

    #[test]
    fn test_assert_variants() {
        assert!(add(1, 1) == 2);
        assert_ne!(add(1, 1), 3);
        assert!(divide(1, 0).is_err(), "expected an error");
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    /// Tests may return `Result`; `?` then works inside them.
    #[test]
    fn test_divide_with_question_mark() -> Result<(), String> {
        let v = divide(9, 3)?;
        assert_eq!(v, 3);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_element_at_panics() {
        element_at(&[1, 2, 3], 10);
    }

    #[test]
    fn test_is_palindrome_ignores_case() {
        assert!(is_palindrome("Level"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome("Rust"));
    }

    #[test]
    fn test_float_comparison() {
        let x: f64 = 0.1 + 0.2;
        assert!((x - 0.3).abs() < 1e-9, "floats: compare within epsilon");
    }

    #[test]
    #[ignore = "example of a slow test; run with --ignored"]
    fn test_slow() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
