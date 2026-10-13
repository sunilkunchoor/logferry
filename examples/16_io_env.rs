//! # 16 - Input, Output, Files and Environment
//!
//! How a program talks to the outside world.
//!
//! ## What you will learn
//! - Command-line arguments (`std::env::args`)
//! - Environment variables (`std::env::var`)
//! - Reading standard input (only if input is piped in)
//! - Writing and reading files (`std::fs`, `BufReader`, `BufWriter`)
//! - Paths with `std::path::Path`/`PathBuf`
//! - Exit codes
//!
//! ## Key ideas in plain English
//! Nearly every I/O call returns `std::io::Result<T>` because the outside
//! world can fail (missing file, no permission, disk full). Use `?` to bubble
//! those up, which is why `main` here returns `Result`.
//!
//! For big files, wrap the file in a **buffer** (`BufReader`/`BufWriter`) so
//! you do not make one system call per line.
//!
//! ## Common beginner mistakes
//! - Using `String` where a `Path` is clearer for file names.
//! - Forgetting `use std::io::Write` / `BufRead` (the methods are *trait*
//!   methods, so the trait must be in scope).
//! - Reading a huge file into one `String` with `read_to_string`.
//!
//! ## How to run
//! ```text
//! cargo run --example 16_io_env
//! cargo run --example 16_io_env -- first second          # arguments
//! echo some piped text | cargo run --example 16_io_env   # stdin (bash)
//! ```
//!
//! ## Try it yourself
//! Make the program take a file name as `args[1]` and print its line count.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, IsTerminal, Read, Write};
use std::path::PathBuf;

/// Prints the arguments passed after `--`.
fn arguments() {
    println!("--- args ---");
    let args: Vec<String> = env::args().collect();
    println!("program = {}", args[0]); // args[0] is the executable
    println!("extra   = {:?}", &args[1..]);
}

/// Reads environment variables, with a default.
fn environment() {
    println!("--- env ---");
    match env::var("LOGFERRY_LEVEL") {
        Ok(v) => println!("LOGFERRY_LEVEL = {v}"),
        Err(_) => println!("LOGFERRY_LEVEL not set (try setting it)"),
    }
    let level = env::var("LOGFERRY_LEVEL").unwrap_or_else(|_| "info".to_string());
    println!("effective level = {level}");
    println!("current dir = {:?}", env::current_dir().ok());
}

/// Reads stdin only when something is piped in, so the example never hangs.
fn standard_input() -> io::Result<()> {
    println!("--- stdin ---");
    let stdin = io::stdin();
    if stdin.is_terminal() {
        println!("(stdin is a terminal; pipe some text in to see it echoed)");
        return Ok(());
    }
    let mut text = String::new();
    stdin.lock().read_to_string(&mut text)?;
    println!("read {} bytes, {} lines", text.len(), text.lines().count());
    Ok(())
}

/// Writes then reads a temporary file, line by line.
///
/// # Errors
/// Any [`io::Error`] from creating, writing, reading or deleting the file.
fn files() -> io::Result<()> {
    println!("--- files ---");
    let mut path: PathBuf = env::temp_dir();
    path.push("rust_basics_example.txt"); // portable path building

    // 1) write the whole file at once
    fs::write(&path, "alpha\nbeta\n")?;

    // 2) append using OpenOptions
    let mut f = OpenOptions::new().append(true).open(&path)?;
    writeln!(f, "gamma")?;
    drop(f);

    // 3) buffered writer for many small writes
    {
        let mut w = BufWriter::new(OpenOptions::new().append(true).open(&path)?);
        for i in 1..=2 {
            writeln!(w, "line {i}")?;
        }
        w.flush()?; // also happens on drop, but flushing explicitly lets you see errors
    }

    // 4) read the whole file
    let all = fs::read_to_string(&path)?;
    println!("whole file has {} lines", all.lines().count());

    // 5) stream line by line (best for big files)
    let reader = BufReader::new(File::open(&path)?);
    for (n, line) in reader.lines().enumerate() {
        println!("{:>2}: {}", n + 1, line?);
    }

    // 6) metadata + cleanup
    println!("size = {} bytes, exists = {}", fs::metadata(&path)?.len(), path.exists());
    fs::remove_file(&path)?;
    println!("deleted; exists = {}", path.exists());

    // Missing files give a typed error you can inspect:
    match File::open("definitely/not/here.txt") {
        Ok(_) => println!("?!"),
        Err(e) if e.kind() == io::ErrorKind::NotFound => println!("not found, as expected"),
        Err(e) => println!("other error: {e}"),
    }
    Ok(())
}

/// Shows stdout/stderr and `print!` flushing.
fn output_streams() -> io::Result<()> {
    println!("--- stdout/stderr ---");
    print!("no newline, so flush: ");
    io::stdout().flush()?;
    println!("done");
    eprintln!("this goes to stderr (diagnostics)");
    Ok(())
}

/// `main` can return `Result`; an `Err` prints the error and exits with code 1.
fn main() -> io::Result<()> {
    arguments();
    environment();
    standard_input()?;
    files()?;
    output_streams()?;
    // std::process::exit(2); // would exit immediately with code 2
    Ok(())
}
