//! # 12 - Common Standard-Library Traits
//!
//! The standard library defines traits that give your types "superpowers":
//! printing, comparing, copying, converting, and even using `+`.
//!
//! ## What you will learn
//! - `#[derive(...)]` for `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`,
//!   `Hash`, `PartialOrd`, `Ord`, `Default`
//! - Implementing `Display` by hand
//! - `From` / `Into` (and `TryFrom`)
//! - Operator overloading (`Add`, `Neg`, `Index`)
//! - `Drop`: running code when a value goes out of scope
//!
//! ## Key ideas in plain English
//! `derive` asks the compiler to write the obvious implementation for you.
//!
//! | Trait | Gives you |
//! |-------|-----------|
//! | `Debug` | `{:?}` printing for developers |
//! | `Display` | `{}` printing for users (manual only) |
//! | `Clone` | `.clone()` explicit copy |
//! | `Copy` | implicit bitwise copy (small, plain types) |
//! | `PartialEq`/`Eq` | `==` and `!=` |
//! | `PartialOrd`/`Ord` | `<`, `>`, `.sort()` |
//! | `Hash` | usable as `HashMap` key |
//! | `Default` | `T::default()` |
//!
//! Implement `From<A> for B` and you get `A.into()` -> `B` for free.
//!
//! ## Common beginner mistakes
//! - Deriving `Copy` on a type containing a `String` (not allowed).
//! - Floats are only `PartialEq`/`PartialOrd` (NaN != NaN), so they cannot
//!   derive `Eq`/`Ord`/`Hash`.
//!
//! ## How to run
//! ```text
//! cargo run --example 12_std_traits
//! ```
//!
//! ## Try it yourself
//! Implement `Sub` for [`Vec2`] and `Mul<f64>` to scale a vector.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/std/ops/index.html> and
//! <https://doc.rust-lang.org/std/convert/index.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, Index, Neg};

/// A 2-D vector that is cheap to copy, comparable and hashable-friendly.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Vec2 {
    /// X component.
    x: f64,
    /// Y component.
    y: f64,
}

/// Overloads `a + b`.
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

/// Overloads unary `-a`.
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

/// User-facing text with `{}`.
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

/// Integers-only type: can derive everything including `Eq`, `Hash`, `Ord`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Version {
    /// Major number. Fields compare in declaration order.
    major: u32,
    /// Minor number.
    minor: u32,
    /// Patch number.
    patch: u32,
}

/// Build a `Version` from a tuple. Gives `(1,2,3).into()` for free.
impl From<(u32, u32, u32)> for Version {
    fn from(t: (u32, u32, u32)) -> Self {
        Version { major: t.0, minor: t.1, patch: t.2 }
    }
}

/// Fallible conversion: `TryFrom` when it can fail.
impl TryFrom<&str> for Version {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let nums: Result<Vec<u32>, _> = s.split('.').map(str::parse::<u32>).collect();
        match nums.map_err(|e| e.to_string())?.as_slice() {
            [a, b, c] => Ok(Version { major: *a, minor: *b, patch: *c }),
            _ => Err(format!("'{s}' is not MAJOR.MINOR.PATCH")),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Wraps a list and makes `bag[i]` work through `Index`.
struct Bag(Vec<&'static str>);

impl Index<usize> for Bag {
    type Output = &'static str;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

/// A guard that announces when it is dropped (RAII).
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("  dropping guard '{}'", self.0);
    }
}

fn operators() {
    println!("--- operators ---");
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 0.5, y: 0.5 };
    println!("{a} + {b} = {}", a + b);
    println!("-a = {}", -a);
    println!("a == a? {}", a == a);
    println!("default = {:?}", Vec2::default()); // zeros
    let c = a; // Copy: `a` is still usable
    println!("{a} {c}");
    let bag = Bag(vec!["x", "y"]);
    println!("bag[1] = {}", bag[1]);
}

fn derived_traits() {
    println!("--- derive ---");
    let mut versions = vec![Version::from((1, 2, 0)), (1, 0, 5).into(), Version::default()];
    versions.sort(); // Ord
    let shown: Vec<String> = versions.iter().map(|v| v.to_string()).collect(); // Display -> to_string
    println!("{shown:?}");
    println!("max = {}", versions.iter().max().unwrap());

    let set: HashSet<Version> = versions.into_iter().collect(); // Hash + Eq
    println!("set has {} unique versions", set.len());

    println!("{:?}", Version::try_from("3.1.4"));
    println!("{:?}", Version::try_from("3.1"));
    let parsed: Result<Version, _> = "x.y.z".try_into();
    println!("{:?}", parsed);
}

fn drop_order() {
    println!("--- Drop ---");
    let _outer = Guard("outer");
    {
        let _inner = Guard("inner");
        println!("  inside block");
    } // inner dropped here
    let moved = Guard("moved early");
    drop(moved); // explicit early drop
    println!("  end of function (outer drops next)");
}

fn main() {
    operators();
    derived_traits();
    drop_order();
}
