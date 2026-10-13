//! # 18 - Macros and Unsafe Rust
//!
//! Two "advanced-looking" features you will meet early, explained gently.
//!
//! ## What you will learn
//! - What the `!` in `println!` means
//! - Common built-in macros (`vec!`, `format!`, `assert!`, `dbg!`, `todo!`, ...)
//! - Writing a simple declarative macro with `macro_rules!`
//! - Conditional compilation with `cfg!`/`#[cfg]`
//! - What `unsafe` does (and does not) mean
//! - Raw pointers and a safe wrapper around an `unsafe` block
//!
//! ## Key ideas in plain English
//! A **macro** is code that writes code at compile time. Functions have a
//! fixed number of typed arguments; macros can take a flexible number of
//! arguments (`println!("{} {}", a, b)`), which is why `println!` is a macro.
//!
//! `unsafe` does **not** turn off the borrow checker. It unlocks exactly five
//! extra powers:
//! 1. dereference a raw pointer
//! 2. call an `unsafe` function
//! 3. access or modify a mutable `static`
//! 4. implement an `unsafe` trait
//! 5. access fields of a `union`
//!
//! You take over responsibility for upholding the rules the compiler normally
//! checks. The standard practice is to keep `unsafe` blocks tiny and wrap them
//! in a safe API with a `// SAFETY:` comment explaining why it is sound.
//! (logferry's PyO3 bindings sit on top of this kind of FFI boundary.)
//!
//! ## Common beginner mistakes
//! - Reaching for `unsafe` to silence the borrow checker. Restructure the code
//!   or use `Rc`/`RefCell`/`Arc`/`Mutex` instead.
//! - Macro hygiene surprises: arguments are matched as syntax fragments
//!   (`expr`, `ident`, `ty`, ...), not values.
//!
//! ## How to run
//! ```text
//! cargo run --example 18_macros_unsafe
//! ```
//!
//! ## Try it yourself
//! Extend `square_all!` so it also works with no arguments (returning `vec![]`).
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch19-06-macros.html> and
//! <https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html>
#![allow(rustdoc::private_intra_doc_links)]


/// A declarative macro: takes any number of expressions and returns a `Vec`
/// of their squares.
///
/// The pattern `$( $x:expr ),*` means "zero or more comma-separated
/// expressions"; the body repeats once per match.
///
/// # Examples
/// ```text
/// // let v = square_all!(1, 2, 3); // vec![1, 4, 9]
/// ```
macro_rules! square_all {
    ( $( $x:expr ),* ) => {
        vec![ $( $x * $x ),* ]
    };
}

/// A macro with two arms (like `match` for syntax) that builds a function.
macro_rules! make_greeter {
    ($name:ident, $greeting:expr) => {
        fn $name(who: &str) -> String {
            format!("{}, {}!", $greeting, who)
        }
    };
}

make_greeter!(say_hello, "Hello");
make_greeter!(say_bonjour, "Bonjour");

/// Tour of the macros you will use all the time.
fn builtin_macros() {
    println!("--- built-in macros ---");
    let v = vec![1, 2, 3]; // build a Vec
    let s = format!("{v:?}"); // build a String
    assert!(s.starts_with('[')); // panic if false
    assert_eq!(v.len(), 3);
    debug_assert!(v.len() < 10); // only checked in debug builds
    let doubled = dbg!(v[0] * 2); // prints file:line + value to stderr, returns value
    println!("{s} {doubled}");
    println!("file={} line={}", file!(), line!());
    println!("stringify -> {}", stringify!(1 + 2));
    println!("concat    -> {}", concat!("a", 1, true));
    // todo!() / unimplemented!() / unreachable!() panic with helpful messages.
    #[allow(unreachable_code)]
    let _never = || -> i32 { todo!("write this later") };
}

/// Shows our own macros.
fn custom_macros() {
    println!("--- macro_rules! ---");
    println!("{:?}", square_all!(1, 2, 3, 4));
    println!("{:?}", square_all!(10 + 1)); // $x:expr keeps precedence: (10+1)*(10+1)
    println!("{}", say_hello("Rust"));
    println!("{}", say_bonjour("Ferris"));
}

/// Conditional compilation.
fn conditional_compilation() {
    println!("--- cfg ---");
    if cfg!(debug_assertions) {
        println!("this is a debug build");
    } else {
        println!("this is a release build");
    }
    #[cfg(target_os = "windows")]
    println!("compiled for Windows");
    #[cfg(not(target_os = "windows"))]
    println!("compiled for a non-Windows OS");
}

/// Safe API around raw pointers: swaps two elements of a slice.
///
/// # Panics
/// Panics if either index is out of range.
fn swap_elements(items: &mut [i32], a: usize, b: usize) {
    assert!(a < items.len() && b < items.len(), "index out of range");
    let ptr = items.as_mut_ptr();
    // SAFETY: both indices were bounds-checked above, so `ptr.add(a)` and
    // `ptr.add(b)` point inside the same allocation. `ptr::swap` supports
    // a == b because it handles overlapping pointers.
    unsafe {
        std::ptr::swap(ptr.add(a), ptr.add(b));
    }
}

/// Raw pointers: creating them is safe, dereferencing is `unsafe`.
fn raw_pointers() {
    println!("--- unsafe / raw pointers ---");
    let mut x = 10;
    let p1 = &x as *const i32; // read-only raw pointer
    let p2 = &mut x as *mut i32; // mutable raw pointer
    // SAFETY: both point to the live local `x`, and we do not use `x`
    // through any reference while these pointers are used.
    unsafe {
        *p2 += 5;
        println!("via raw pointers: {} {}", *p1, *p2);
    }

    let mut data = [1, 2, 3, 4];
    swap_elements(&mut data, 0, 3);
    println!("after safe wrapper swap: {data:?}"); // [4, 2, 3, 1]

    // An `unsafe fn` must be called inside `unsafe`.
    unsafe fn first_unchecked(v: &[i32]) -> i32 {
        // SAFETY: caller guarantees `v` is not empty.
        unsafe { *v.get_unchecked(0) }
    }
    // SAFETY: `data` has 4 elements.
    println!("first_unchecked = {}", unsafe { first_unchecked(&data) });
}

fn main() {
    builtin_macros();
    custom_macros();
    conditional_compilation();
    raw_pointers();
}
