//! # 07 - Pattern Matching
//!
//! Patterns let you test the *shape* of a value and pull pieces out of it at
//! the same time.
//!
//! ## What you will learn
//! - `match` and exhaustiveness
//! - Literal, range, `|` (or), and wildcard `_` patterns
//! - Destructuring tuples, structs, enums, and references
//! - Match guards (`if`) and `@` bindings
//! - `if let`, `while let`, and `let else`
//!
//! ## Key ideas in plain English
//! `match` is like a very powerful `switch`. The compiler checks that you
//! covered **every** possibility ("exhaustive"). If you add an enum variant
//! later, every incomplete `match` becomes a compile error: great for safety.
//!
//! Patterns are tried top to bottom; the first match wins. So put specific
//! patterns before general ones.
//!
//! ## Common beginner mistakes
//! - `error[E0004]: non-exhaustive patterns` - add the missing arm or `_`.
//! - Using a variable name in a pattern expecting it to compare: a bare name
//!   *binds* a new variable (use a `const` or a guard to compare).
//!
//! ## How to run
//! ```text
//! cargo run --example 07_pattern_matching
//! ```
//!
//! ## Try it yourself
//! Add a `Command::Move` arm with a guard that rejects negative distances.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch18-00-patterns.html>
#![allow(rustdoc::private_intra_doc_links)]


/// A small command language to match against.
#[derive(Debug)]
enum Command {
    /// Stop everything.
    Quit,
    /// Move by x, y.
    Move {
        /// Horizontal delta.
        x: i32,
        /// Vertical delta.
        y: i32,
    },
    /// Say something.
    Say(String),
    /// Paint an RGB colour.
    Color(u8, u8, u8),
}

/// A 2-D point used for struct destructuring.
struct Point {
    /// X coordinate.
    x: i32,
    /// Y coordinate.
    y: i32,
}

/// Matches every kind of `Command`.
fn run(cmd: &Command) -> String {
    match cmd {
        Command::Quit => "bye".to_string(),
        Command::Move { x: 0, y: 0 } => "not moving".to_string(), // literal fields
        Command::Move { x, y: 0 } => format!("horizontal {x}"), // y must be 0
        Command::Move { x, y } => format!("move {x},{y}"),
        Command::Say(text) if text.is_empty() => "(silence)".to_string(), // guard
        Command::Say(text) => format!("says {text}"),
        Command::Color(r, _, _) if *r > 200 => "reddish".to_string(),
        Command::Color(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
    }
}

/// Literals, ranges, alternatives, and `@` bindings.
fn describe_number(n: i32) -> String {
    match n {
        0 => "zero".into(),
        1 | 2 | 3 => "small".into(), // or-pattern
        x @ 4..=9 => format!("medium ({x})"), // bind while matching a range
        i32::MIN..=-1 => "negative".into(),
        _ => "large".into(), // wildcard / catch-all
    }
}

/// Destructuring tuples, structs, arrays and nested data.
fn destructuring() {
    println!("--- destructuring ---");
    let (a, (b, c)) = (1, (2.5, "three"));
    println!("{a} {b} {c}");

    let p = Point { x: 3, y: -3 };
    let Point { x, y } = p;
    println!("x={x} y={y}");
    match p {
        Point { x, y: 0 } => println!("on x axis at {x}"),
        Point { x: 0, y } => println!("on y axis at {y}"),
        Point { x, y } if x == -y => println!("on anti-diagonal"),
        Point { .. } => println!("somewhere else"),
    }

    let arr = [1, 2, 3, 4, 5];
    // Fixed-size arrays always match, so a plain `let` works; slices need `if let`.
    let [first, .., last] = arr;
    println!("first={first} last={last}");
    if let [head, rest @ ..] = &arr[..] {
        println!("head={head} rest={rest:?}");
    }
}

/// `if let`, `while let` and `let else` are shorthand for one-arm matches.
fn conditional_patterns() {
    println!("--- if let / while let / let else ---");
    let maybe: Option<i32> = Some(7);
    if let Some(n) = maybe {
        println!("got {n}");
    } else {
        println!("nothing");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{top} "); // 3 2 1
    }
    println!();

    // `let else`: bind or diverge (return / break / panic).
    fn parse_or_zero(s: &str) -> i32 {
        let Ok(n) = s.parse::<i32>() else {
            return 0;
        };
        n
    }
    println!("{} {}", parse_or_zero("42"), parse_or_zero("nope"));
}

/// Matching on references and `Option<&T>`.
fn reference_patterns() {
    println!("--- references ---");
    let v = vec![10, 20, 30];
    for &n in &v {
        // `&n` pattern dereferences while destructuring
        print!("{} ", n + 1);
    }
    println!();
    match v.first() {
        Some(&n) if n > 5 => println!("first is big: {n}"),
        Some(n) => println!("first is {n}"),
        None => println!("empty"),
    }
}

fn main() {
    let cmds = [
        Command::Quit,
        Command::Move { x: 0, y: 0 },
        Command::Move { x: 5, y: 0 },
        Command::Move { x: 1, y: 2 },
        Command::Say(String::new()),
        Command::Say("hi".into()),
        Command::Color(255, 0, 0),
        Command::Color(10, 20, 30),
    ];
    println!("--- match on enum ---");
    for c in &cmds {
        println!("{c:?} => {}", run(c));
    }

    println!("--- numbers ---");
    for n in [0, 2, 7, -5, 100] {
        println!("{n}: {}", describe_number(n));
    }

    destructuring();
    conditional_patterns();
    reference_patterns();
}
