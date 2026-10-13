//! # 14 - Modules and Smart Pointers
//!
//! Two topics for organising code and managing memory.
//!
//! ## What you will learn
//! - Modules: `mod`, `pub`, `use`, `self`/`super`, nested paths
//! - `Box<T>`: heap allocation (and recursive types)
//! - `Rc<T>`: shared ownership (single thread)
//! - `RefCell<T>`: mutation through a shared reference ("interior mutability")
//! - `Weak<T>`: non-owning pointer to avoid reference cycles
//!
//! ## Key ideas in plain English
//! **Modules** are folders for code *inside* a file or across files. Everything
//! is private by default; mark items `pub` to expose them.
//!
//! **Smart pointers** are structs that act like pointers but also own data:
//! | Type | Meaning |
//! |------|---------|
//! | `Box<T>` | exactly one owner, data on the heap |
//! | `Rc<T>` | many owners, reference counted, not thread-safe |
//! | `Arc<T>` | like `Rc` but thread-safe (see `15_concurrency`) |
//! | `RefCell<T>` | borrow rules checked at **runtime** instead of compile time |
//!
//! `Rc<RefCell<T>>` = "many owners, each can mutate". If you break the borrow
//! rules at runtime, `RefCell` **panics**.
//!
//! ## Common beginner mistakes
//! - Calling `.borrow_mut()` while a `.borrow()` is still alive (panic).
//! - Creating `Rc` cycles (A -> B -> A) which leak memory; use `Weak` for the
//!   back-pointer.
//!
//! ## How to run
//! ```text
//! cargo run --example 14_modules_smart_pointers
//! ```
//!
//! ## Try it yourself
//! Add a `shapes::circle` module with an `area` function and call it from `main`.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>
//! and <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A module groups related items. In a real project this would be
/// `src/geometry.rs` or `src/geometry/mod.rs`.
mod geometry {
    /// Public constant.
    pub const UNIT: f64 = 1.0;

    /// A nested module.
    pub mod rect {
        /// Area of a rectangle.
        pub fn area(w: f64, h: f64) -> f64 {
            w * h * super::UNIT // `super` = the parent module
        }

        /// Private helper: invisible outside `rect`.
        #[allow(dead_code)]
        fn secret() {}
    }

    /// A struct with a private field, exposed only through methods.
    pub struct Counter {
        /// Not `pub`, so only this module can touch it.
        count: u32,
    }

    impl Counter {
        /// Constructor (required since the field is private).
        pub fn new() -> Self {
            Counter { count: 0 }
        }
        /// Increments and returns the new value.
        pub fn tick(&mut self) -> u32 {
            self.count += 1;
            self.count
        }
    }
}

use geometry::rect; // bring a path into scope
use geometry::Counter as Ticker; // rename on import

/// A recursive type needs `Box`: otherwise the size would be infinite.
#[derive(Debug)]
enum List {
    /// A node holding a value and the rest of the list.
    Cons(i32, Box<List>),
    /// End of list.
    Nil,
}

impl List {
    /// Sum all values recursively.
    fn sum(&self) -> i32 {
        match self {
            List::Cons(v, rest) => v + rest.sum(),
            List::Nil => 0,
        }
    }
}

/// A tree node: owns its children, weakly knows its parent.
#[derive(Debug)]
struct Node {
    /// Value stored in the node.
    value: i32,
    /// Back-pointer; `Weak` so parent <-> child does not leak.
    parent: RefCell<Weak<Node>>,
    /// Children are shared & mutable.
    children: RefCell<Vec<Rc<Node>>>,
}

fn modules() {
    println!("--- modules ---");
    println!("area = {}", rect::area(2.0, 3.0));
    println!("UNIT = {}", geometry::UNIT);
    let mut t = Ticker::new();
    t.tick();
    println!("tick = {}", t.tick());
    // t.count; // <- error: field `count` is private
}

fn boxes() {
    println!("--- Box ---");
    let b = Box::new(5); // 5 lives on the heap
    println!("b = {b}, b+1 = {}", *b + 1); // `*` dereferences
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{list:?} sum = {}", list.sum());
}

fn reference_counting() {
    println!("--- Rc ---");
    let shared = Rc::new(String::from("shared text"));
    println!("count = {}", Rc::strong_count(&shared)); // 1
    let a = Rc::clone(&shared); // cheap: bumps the counter, no deep copy
    {
        let _b = Rc::clone(&shared);
        println!("count = {}", Rc::strong_count(&shared)); // 3
    }
    println!("count = {}", Rc::strong_count(&shared)); // 2
    println!("{a}");
}

fn interior_mutability() {
    println!("--- RefCell ---");
    let cell = RefCell::new(vec![1, 2]);
    cell.borrow_mut().push(3); // mutate through a shared ref
    println!("{:?}", cell.borrow());
    println!("try_borrow_mut while reading: {}", {
        let _r = cell.borrow();
        cell.try_borrow_mut().is_err() // would panic with borrow_mut()
    });

    // Rc<RefCell<T>>: shared AND mutable.
    let shared = Rc::new(RefCell::new(0));
    let handle = Rc::clone(&shared);
    *handle.borrow_mut() += 10;
    *shared.borrow_mut() += 1;
    println!("shared = {}", shared.borrow()); // 11
}

fn trees_with_weak() {
    println!("--- Weak ---");
    let leaf = Rc::new(Node { value: 3, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    println!("branch strong={} weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("branch children = {}", branch.children.borrow().len());
}

fn main() {
    modules();
    boxes();
    reference_counting();
    interior_mutability();
    trees_with_weak();
}
