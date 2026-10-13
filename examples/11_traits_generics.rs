//! # 11 - Traits and Generics
//!
//! Generics let you write code once for *many types*. Traits describe *what a
//! type can do*. Together they are Rust's tool for abstraction.
//!
//! ## What you will learn
//! - Defining and implementing a **trait**
//! - Default methods
//! - **Generic** functions and structs (`<T>`)
//! - Trait bounds (`T: Trait`) and `where` clauses
//! - `impl Trait` in argument and return position
//! - **Trait objects** (`dyn Trait`) for runtime polymorphism
//!
//! ## Key ideas in plain English
//! - A **trait** is like an interface: "anything that is `Summary` has a
//!   `summarize` method."
//! - A **generic** is a placeholder type: `Vec<T>` works for any `T`.
//! - A **bound** restricts the placeholder: "any `T` that can be compared".
//!
//! Two ways to be polymorphic:
//! | | Generics (`T: Trait`) | Trait objects (`Box<dyn Trait>`) |
//! |-|-|-|
//! | Dispatch | static (compile time, monomorphised) | dynamic (vtable lookup) |
//! | Speed | fastest, may grow binary | tiny runtime cost |
//! | Mixed types in one Vec | no | yes |
//!
//! ## Common beginner mistakes
//! - `error[E0277]: the trait bound ... is not satisfied` - add the bound or
//!   implement the trait.
//! - Returning two different concrete types from `-> impl Trait` (not allowed;
//!   use `Box<dyn Trait>`).
//!
//! ## How to run
//! ```text
//! cargo run --example 11_traits_generics
//! ```
//!
//! ## Try it yourself
//! Implement `Summary` for a new `Tweet` type and add it to the `Vec<Box<dyn Summary>>`.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch10-00-generics.html> and
//! <https://doc.rust-lang.org/book/ch17-02-trait-objects.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::fmt::{Debug, Display};

/// Behaviour shared by things that can describe themselves.
trait Summary {
    /// Required: each type must provide an author.
    fn author(&self) -> String;

    /// Provided: has a default body that types may override.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.author())
    }
}

/// A blog post.
struct Post {
    /// Writer of the post.
    writer: String,
    /// Post title.
    title: String,
}

/// A short message.
struct Tweet {
    /// Handle of the user.
    handle: String,
    /// Message body.
    text: String,
}

impl Summary for Post {
    fn author(&self) -> String {
        self.writer.clone()
    }
    // uses the default `summarize`
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.handle)
    }
    // overrides the default
    fn summarize(&self) -> String {
        format!("{}: {}", self.author(), self.text)
    }
}

/// Generic function: works for any type `T` that can be ordered.
///
/// `T: PartialOrd` is the *bound*. `&[T]` is a borrowed slice of Ts.
fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}

/// Same idea with multiple bounds; a `where` clause keeps the signature tidy.
fn show_pair<A, B>(a: A, b: B) -> String
where
    A: Display,
    B: Debug,
{
    format!("{a} / {b:?}")
}

/// `impl Trait` in argument position is sugar for a generic with a bound.
fn notify(item: &impl Summary) -> String {
    format!("Breaking! {}", item.summarize())
}

/// `impl Trait` in return position: "some type that implements `Summary`".
fn make_tweet() -> impl Summary {
    Tweet { handle: "rustlang".into(), text: "Rust 2024 is here".into() }
}

/// A generic struct. `T` can be anything.
#[derive(Debug)]
struct Wrapper<T> {
    /// The wrapped value.
    value: T,
}

impl<T> Wrapper<T> {
    /// Creates a wrapper.
    fn new(value: T) -> Self {
        Self { value }
    }
}

/// Methods only available when `T: Display` (conditional implementation).
impl<T: Display> Wrapper<T> {
    /// Prints with brackets.
    fn show(&self) -> String {
        format!("[{}]", self.value)
    }
}

/// A trait with a required associated function & trait objects for shapes.
trait Area {
    /// Area of the shape.
    fn area(&self) -> f64;
    /// Name of the shape.
    fn name(&self) -> &'static str;
}

/// Square shape.
struct Square(f64);
/// Circle shape.
struct Circle(f64);

impl Area for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn name(&self) -> &'static str {
        "square"
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
    fn name(&self) -> &'static str {
        "circle"
    }
}

fn main() {
    println!("--- traits ---");
    let post = Post { writer: "Ada".into(), title: "Hello".into() };
    let tweet = Tweet { handle: "ferris".into(), text: "crab noises".into() };
    println!("{} (title: {})", post.summarize(), post.title);
    println!("{}", tweet.summarize());
    println!("{}", notify(&post));
    println!("{}", make_tweet().summarize());

    println!("--- generics ---");
    println!("largest int  : {}", largest(&[3, 9, 2]));
    println!("largest float: {}", largest(&[1.5, 0.2]));
    println!("largest char : {}", largest(&['x', 'b', 'z']));
    println!("{}", show_pair("left", vec![1, 2]));
    let w = Wrapper::new(42);
    println!("{} {:?}", w.show(), Wrapper::new("hi"));

    println!("--- trait objects ---");
    // Different concrete types, one Vec, via `Box<dyn Trait>` (heap + vtable).
    let shapes: Vec<Box<dyn Area>> = vec![Box::new(Square(2.0)), Box::new(Circle(1.0))];
    for s in &shapes {
        println!("{} area = {:.3}", s.name(), s.area());
    }
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total = {total:.3}");
}
