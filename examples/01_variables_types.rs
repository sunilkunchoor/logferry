//! # 01 - Variables and Types
//!
//! Every program stores data. This file shows *how* Rust stores it and what
//! kinds (types) of data exist.
//!
//! ## What you will learn
//! - Declaring variables with `let`, and why they are immutable by default
//! - Making a variable changeable with `mut`
//! - *Shadowing* (re-using a name with a new value or even a new type)
//! - Constants (`const`) and statics (`static`)
//! - Scalar types: integers, floats, `bool`, `char`
//! - Compound types: tuples and arrays
//! - Converting between types with `as`
//! - What happens when a number gets too big (overflow)
//!
//! ## Key ideas in plain English
//! Think of a variable as a *labelled box*. In most languages you can freely
//! swap what is inside. In Rust the lid is **glued shut** unless you say
//! `mut`. This helps the compiler (and you) know what can change, which
//! prevents a whole class of bugs.
//!
//! Rust is *statically typed*: every value has a type known at compile time.
//! You usually do not have to write the type because the compiler can
//! *infer* it, but you may always add it: `let x: i32 = 5;`.
//!
//! ## Common beginner mistakes
//! - `error[E0384]: cannot assign twice to immutable variable` -> add `mut`.
//! - Mixing types: `let a: i32 = 1; let b: i64 = 2; a + b` does not compile.
//!   Rust never converts numbers silently; use `as` or `.into()`.
//!
//! ## How to run
//! ```text
//! cargo run --example 01_variables_types
//! ```
//!
//! ## Try it yourself
//! 1. Remove `mut` from `counter` in [`mutability`] and read the error.
//! 2. Change `u8` in [`overflow`] to `u16` and see how the result changes.
//!
//! ## Further reading
//! The Rust Book, chapter 3: <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>
#![allow(rustdoc::private_intra_doc_links)]


/// A **constant**: always immutable, must have an explicit type, and is
/// evaluated at compile time. By convention names are `SCREAMING_SNAKE_CASE`.
const MAX_RETRIES: u32 = 3;

/// A **static**: one fixed location in memory for the whole program run.
/// Use `const` by default; `static` is for when you need a stable address.
static APP_NAME: &str = "logferry";

/// Shows `let` (immutable) versus `let mut` (mutable).
///
/// # Examples
/// ```
/// let x = 5;          // immutable: x = 6 would not compile
/// let mut y = 5;      // mutable
/// y += 1;
/// assert_eq!(y, 6);
/// # let _ = x;
/// ```
fn mutability() {
    println!("--- mutability ---");
    let language = "Rust"; // immutable
    println!("language = {language}");
    // language = "C"; // <- uncomment: error[E0384]

    let mut counter = 0; // mutable
    counter += 1;
    counter += 1;
    println!("counter = {counter}"); // 2
}

/// Shows **shadowing**: declaring a new variable with the same name.
///
/// Unlike `mut`, shadowing creates a *brand new* variable, so it may even
/// change the type. The old one becomes inaccessible.
fn shadowing() {
    println!("--- shadowing ---");
    let spaces = "   "; // a &str
    let spaces = spaces.len(); // now a usize: same name, new type!
    println!("spaces = {spaces}"); // 3

    let x = 5;
    let x = x * 2; // 10
    {
        let x = x + 100; // inner scope shadows only inside the braces
        println!("inner x = {x}"); // 110
    }
    println!("outer x = {x}"); // 10
}

/// A tour of the **scalar** (single value) types.
///
/// | Kind | Types |
/// |------|-------|
/// | signed integers | `i8 i16 i32 i64 i128 isize` |
/// | unsigned integers | `u8 u16 u32 u64 u128 usize` |
/// | floats | `f32 f64` |
/// | boolean | `bool` |
/// | Unicode character | `char` (4 bytes!) |
///
/// `i32` is the default integer and `f64` the default float.
fn scalar_types() {
    println!("--- scalar types ---");
    let small: i8 = -128; // smallest i8
    let big: u64 = 18_446_744_073_709_551_615; // `_` is only for readability
    let hex = 0xff; // 255
    let binary = 0b1010; // 10
    let byte = b'A'; // u8 with value 65
    println!("{small} {big} {hex} {binary} {byte}");

    let pi: f64 = 3.141_592_653_5;
    let half: f32 = 0.5;
    println!("{pi} {half}");

    let is_ready: bool = true;
    let heart: char = '♥'; // single quotes = char, double quotes = string
    println!("{is_ready} {heart}");

    // Integer division truncates; floats do not.
    println!("7 / 2 = {}, 7.0 / 2.0 = {}, 7 % 2 = {}", 7 / 2, 7.0 / 2.0, 7 % 2);
}

/// **Tuples** group values of *different* types; **arrays** hold a fixed
/// number of values of the *same* type.
///
/// Use `Vec<T>` (see `09_collections`) when the length must change.
fn compound_types() {
    println!("--- compound types ---");
    let person: (&str, u32, bool) = ("Ada", 36, true);
    let (name, age, _active) = person; // destructuring
    println!("{name} is {age}; first field via index: {}", person.0);

    let days = [31, 28, 31]; // [i32; 3]
    let zeros = [0u8; 5]; // five zeros
    println!("{} {:?} {:?}", days[0], days, zeros);
    println!("array length = {}", days.len());
    // days[10]; // would PANIC at runtime: index out of bounds (Rust checks!)
}

/// Converting between numeric types explicitly with `as`.
///
/// `as` may lose information (truncate), so be deliberate. For safe,
/// fallible conversions prefer `TryFrom` / `try_into()`.
fn casting() {
    println!("--- casting ---");
    let a: i32 = 300;
    let b = a as u8; // 300 doesn't fit in u8 -> wraps to 44
    let c = 3.99_f64 as i32; // truncates toward zero -> 3
    let d = -1_i32 as u32; // two's complement reinterpretation
    println!("{b} {c} {d}");

    let safe: Result<u8, _> = u8::try_from(300_i32);
    println!("try_from(300) -> {safe:?}"); // Err(...)
    let widened: i64 = i64::from(5_i32); // lossless conversions use From
    println!("widened = {widened}");
}

/// What happens when arithmetic exceeds a type's range.
///
/// - **Debug builds** panic on overflow.
/// - **Release builds** silently wrap.
///
/// Never rely on either; choose the explicit method you want.
fn overflow() {
    println!("--- overflow ---");
    let max = u8::MAX; // 255
    println!("wrapping_add : {}", max.wrapping_add(1)); // 0
    println!("checked_add  : {:?}", max.checked_add(1)); // None
    println!("saturating   : {}", max.saturating_add(1)); // 255
    println!("overflowing  : {:?}", max.overflowing_add(1)); // (0, true)
}

fn main() {
    println!("{APP_NAME} will retry up to {MAX_RETRIES} times\n");
    mutability();
    shadowing();
    scalar_types();
    compound_types();
    casting();
    overflow();
}
