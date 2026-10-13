//! # 02 - Control Flow
//!
//! Control flow decides *which* code runs and *how many times*.
//!
//! ## What you will learn
//! - `if` / `else if` / `else`, and `if` as an **expression** that yields a value
//! - `loop`, `while`, and `for`
//! - Ranges (`0..5`, `0..=5`)
//! - `break` with a value, `continue`, and labelled loops
//!
//! ## Key ideas in plain English
//! In Rust, *almost everything is an expression* - it produces a value. That
//! includes `if` and `match`. So you can write
//! `let size = if big { 10 } else { 1 };`. Both branches must have the
//! same type.
//!
//! Conditions must be real `bool`s. Unlike C or Python, `if 1 {}` or
//! `if "text" {}` is a compile error.
//!
//! `for` is the loop you will use most. It walks over anything *iterable*
//! (ranges, vectors, ...), which avoids index mistakes.
//!
//! ## Common beginner mistakes
//! - Putting a `;` after the last expression in a branch turns the value into
//!   `()` and causes a "mismatched types" error.
//! - Writing `for i in 0..=len` when you meant `0..len` (off-by-one).
//!
//! ## How to run
//! ```text
//! cargo run --example 02_control_flow
//! ```
//!
//! ## Try it yourself
//! Write a FizzBuzz for 1..=30 using [`classify`] as inspiration.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch03-05-control-flow.html>
#![allow(rustdoc::private_intra_doc_links)]


/// Classifies a number using `if` as an expression.
///
/// # Examples
/// ```text
/// // classify(-4) == "negative"
/// ```
fn classify(n: i32) -> &'static str {
    // The whole if/else chain is the function's return value (no `;`).
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n % 2 == 0 {
        "positive even"
    } else {
        "positive odd"
    }
}

/// `if` is an expression, so it can sit on the right of `let`.
fn if_expression() {
    println!("--- if ---");
    for n in [-4, 0, 7, 10] {
        println!("{n} is {}", classify(n));
    }
    let logged_in = true;
    let greeting = if logged_in { "welcome back" } else { "please sign in" };
    println!("{greeting}");
}

/// `loop` repeats forever until `break`. `break` can hand back a value.
fn loop_with_value() {
    println!("--- loop ---");
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts == 3 {
            break attempts * 10; // value of the whole `loop` expression
        }
    };
    println!("loop returned {result}"); // 30
}

/// `while` repeats as long as a condition holds.
fn while_loop() {
    println!("--- while ---");
    let mut n = 3;
    while n > 0 {
        println!("countdown: {n}");
        n -= 1;
    }
    println!("liftoff!");
}

/// `for` over ranges and collections.
///
/// - `a..b` excludes `b`; `a..=b` includes it.
/// - `.rev()` reverses; `.step_by(k)` skips.
/// - `.iter().enumerate()` gives `(index, item)` pairs.
fn for_loops() {
    println!("--- for ---");
    for i in 0..3 {
        print!("{i} "); // 0 1 2
    }
    println!();
    for i in (1..=3).rev() {
        print!("{i} "); // 3 2 1
    }
    println!();
    for i in (0..10).step_by(4) {
        print!("{i} "); // 0 4 8
    }
    println!();

    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{index}: {fruit}");
    }
}

/// `continue` skips to the next iteration; labels let `break`/`continue`
/// target an *outer* loop.
fn labels_and_continue() {
    println!("--- labels / continue ---");
    for n in 1..=6 {
        if n % 2 == 0 {
            continue; // skip evens
        }
        print!("{n} "); // 1 3 5
    }
    println!();

    'outer: for row in 0..3 {
        for col in 0..3 {
            if row == 1 && col == 1 {
                println!("found (1,1), leaving both loops");
                break 'outer;
            }
        }
    }
}

fn main() {
    if_expression();
    loop_with_value();
    while_loop();
    for_loops();
    labels_and_continue();
}
