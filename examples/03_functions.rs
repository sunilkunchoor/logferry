//! # 03 - Functions and Closures
//!
//! Functions package reusable logic.
//!
//! ## What you will learn
//! - Defining functions, parameters, and return types
//! - *Statements* versus *expressions* (why there is often no `return`)
//! - Returning several values with tuples
//! - Closures (anonymous functions that capture variables)
//! - Function pointers and passing functions to functions
//!
//! ## Key ideas in plain English
//! Parameter types are **always required**: Rust wants function signatures to
//! be a clear contract. The return type follows `->`. If you omit it, the
//! function returns `()` ("unit", i.e. nothing).
//!
//! The last expression in a function body, *without* a semicolon, is the
//! return value. Adding `;` turns it into a statement and the function
//! returns `()` instead, a classic beginner error.
//!
//! A **closure** is written `|args| body`. Unlike a plain `fn`, it can
//! *capture* variables from the place where it was created.
//!
//! ## Common beginner mistakes
//! - `expected i32, found ()`: you added a trailing `;` on the last line.
//! - Forgetting to mark a closure `mut` when it modifies captured state.
//!
//! ## How to run
//! ```text
//! cargo run --example 03_functions
//! ```
//!
//! ## Try it yourself
//! Write `fn power(base: i64, exp: u32) -> i64` and test it.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch03-03-how-functions-work.html> and
//! <https://doc.rust-lang.org/book/ch13-01-closures.html>
#![allow(rustdoc::private_intra_doc_links)]


/// Adds two numbers. The last line has no `;`, so it is the return value.
///
/// # Examples
/// ```text
/// // add(2, 3) == 5
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Uses an early `return` for the error-ish case.
///
/// Returns `None` when dividing by zero (see `Option` in `06_structs_enums`).
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None; // early exit needs the `return` keyword
    }
    Some(a / b)
}

/// Returns multiple values by packing them in a tuple.
fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &v in values {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }
    (min, max)
}

/// Demonstrates statements vs expressions.
fn statements_vs_expressions() {
    println!("--- statements vs expressions ---");
    // A block `{ ... }` is an expression; its last line is its value.
    let y = {
        let x = 3;
        x + 1 // no semicolon -> value of the block
    };
    println!("y = {y}"); // 4
}

/// Closures: anonymous functions that can capture their environment.
fn closures() {
    println!("--- closures ---");
    let double = |x: i32| x * 2; // types can usually be inferred
    println!("double(21) = {}", double(21));

    let offset = 100;
    let add_offset = |x: i32| x + offset; // captures `offset` by reference
    println!("add_offset(5) = {}", add_offset(5));

    let mut total = 0;
    let mut accumulate = |x: i32| total += x; // mutates captured state
    accumulate(5);
    accumulate(7);
    println!("total = {total}"); // 12

    // `move` forces the closure to take ownership of what it captures.
    let name = String::from("Ferris");
    let greet = move || format!("Hello, {name}!");
    println!("{}", greet());
    // println!("{name}"); // <- error: `name` was moved into the closure
}

/// Accepts *any* function or closure taking an `i32` and returning `i32`.
///
/// `Fn(i32) -> i32` is a trait bound (see `11_traits_generics`).
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

/// Takes a plain function pointer type `fn(i32) -> i32`.
fn apply_pointer(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// A named function we can pass around as a value.
fn square(x: i32) -> i32 {
    x * x
}

/// Returns a closure. `impl Fn` hides the unnameable closure type.
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    println!("add(2,3) = {}", add(2, 3));
    println!("10/4 = {:?}, 1/0 = {:?}", safe_divide(10.0, 4.0), safe_divide(1.0, 0.0));
    let (lo, hi) = min_max(&[4, -2, 9, 0]);
    println!("min={lo} max={hi}");

    statements_vs_expressions();
    closures();

    println!("--- passing functions ---");
    println!("apply_twice(square, 3) = {}", apply_twice(square, 3)); // 81
    println!("apply_twice(|x| x+1, 3) = {}", apply_twice(|x| x + 1, 3)); // 5
    println!("apply_pointer(square, 5) = {}", apply_pointer(square, 5)); // 25
    let times3 = make_multiplier(3);
    println!("times3(7) = {}", times3(7)); // 21
}
