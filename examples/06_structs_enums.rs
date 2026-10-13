//! # 06 - Structs and Enums
//!
//! Structs and enums let you create your **own types**.
//!
//! ## What you will learn
//! - Named-field structs, tuple structs, unit structs
//! - `impl` blocks: methods and associated functions
//! - Enums, including variants that carry data
//! - `Option<T>`: Rust's replacement for `null`
//! - Struct update syntax
//!
//! ## Key ideas in plain English
//! - A **struct** is an *AND* type: a `User` has a name **and** an age.
//! - An **enum** is an *OR* type: a `Shape` is a circle **or** a rectangle.
//!
//! Methods take `self` in one of three forms:
//! | Form | Meaning |
//! |------|---------|
//! | `&self` | read-only borrow |
//! | `&mut self` | modify in place |
//! | `self` | consume the value |
//!
//! Functions in an `impl` block *without* `self` (like `new`) are called
//! with `Type::name()` and are named **associated functions**.
//!
//! Rust has **no null**. A value that may be missing is `Option<T>`: either
//! `Some(value)` or `None`. The compiler forces you to handle `None`.
//!
//! ## Common beginner mistakes
//! - Forgetting that fields are private across modules unless marked `pub`.
//! - Calling `.unwrap()` on `None` (panics). Prefer `match`/`if let`/`unwrap_or`.
//!
//! ## How to run
//! ```text
//! cargo run --example 06_structs_enums
//! ```
//!
//! ## Try it yourself
//! Add a `Triangle { base, height }` variant to [`Shape`] and let the
//! compiler show you every `match` that needs updating.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch05-00-structs.html> and
//! <https://doc.rust-lang.org/book/ch06-00-enums.html>
#![allow(rustdoc::private_intra_doc_links)]


/// A struct with named fields. `derive(Debug)` lets us print it with `{:?}`.
#[derive(Debug, Clone)]
struct User {
    /// Display name.
    name: String,
    /// Age in years.
    age: u32,
    /// Whether the account is enabled.
    active: bool,
}

impl User {
    /// Associated function acting as a constructor (no `self`).
    fn new(name: &str, age: u32) -> Self {
        Self { name: name.to_string(), age, active: true } // field init shorthand: `age`
    }

    /// Read-only method.
    fn describe(&self) -> String {
        format!("{} ({}), active={}", self.name, self.age, self.active)
    }

    /// Mutating method.
    fn birthday(&mut self) {
        self.age += 1;
    }

    /// Consuming method: the user is moved in and a new value comes out.
    fn deactivate(mut self) -> Self {
        self.active = false;
        self
    }
}

/// A **tuple struct**: fields are positional. Handy for "newtype" wrappers
/// that give meaning to a plain number.
#[derive(Debug)]
struct Meters(f64);

/// A **unit struct** has no data; useful as a marker.
#[derive(Debug)]
struct Marker;

/// An enum whose variants hold different data.
#[derive(Debug)]
enum Shape {
    /// A circle with a radius.
    Circle(f64),
    /// A rectangle with named fields.
    Rectangle {
        /// Horizontal size.
        width: f64,
        /// Vertical size.
        height: f64,
    },
    /// A shape with no size.
    Point,
}

impl Shape {
    /// Area of the shape. `match` must cover **every** variant.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Point => 0.0,
        }
    }
}

/// Simple C-like enum with explicit discriminants.
#[derive(Debug, Clone, Copy)]
enum Level {
    /// Verbose messages.
    Debug = 10,
    /// Normal messages.
    Info = 20,
    /// Serious problems.
    Error = 40,
}

/// Finds a user's age, which may not exist.
fn find_age(users: &[User], name: &str) -> Option<u32> {
    for u in users {
        if u.name == name {
            return Some(u.age);
        }
    }
    None
}

fn structs() {
    println!("--- structs ---");
    let mut ada = User::new("Ada", 36);
    println!("{}", ada.describe());
    ada.birthday();
    println!("{ada:?}");

    // Struct update syntax: copy remaining fields from another instance.
    let grace = User { name: "Grace".to_string(), ..ada.clone() };
    println!("{}", grace.describe());

    let retired = ada.deactivate(); // `ada` is consumed
    println!("{}", retired.describe());

    println!("{:?} {:?}", Meters(3.5), Marker);
    let Meters(inner) = Meters(9.0); // destructure a tuple struct
    println!("inner = {inner}");
}

fn enums() {
    println!("--- enums ---");
    let shapes = [Shape::Circle(1.0), Shape::Rectangle { width: 2.0, height: 3.0 }, Shape::Point];
    for s in &shapes {
        println!("{s:?} area = {:.2}", s.area());
    }
    println!("Level::Error as i32 = {}", Level::Error as i32);
    let _ = (Level::Debug, Level::Info);
}

fn options() {
    println!("--- Option ---");
    let users = vec![User::new("Ada", 36), User::new("Linus", 54)];
    match find_age(&users, "Linus") {
        Some(age) => println!("Linus is {age}"),
        None => println!("not found"),
    }
    if let Some(age) = find_age(&users, "Ada") {
        println!("Ada is {age}");
    }
    println!("missing -> {}", find_age(&users, "Bob").unwrap_or(0));
    // Combinators:
    let doubled = find_age(&users, "Ada").map(|a| a * 2);
    println!("{doubled:?}");
}

fn main() {
    structs();
    enums();
    options();
}
