//! # 15 - Concurrency
//!
//! Rust's ownership rules make many data races **compile errors**. Fearless
//! concurrency is the headline. This is also the theme of logferry, which
//! ingests logs on multiple threads.
//!
//! ## What you will learn
//! - Spawning threads and joining them
//! - `move` closures to give a thread its own data
//! - Message passing with `mpsc` channels
//! - Shared state with `Arc<Mutex<T>>` and `Arc<RwLock<T>>`
//! - Scoped threads that can borrow local data
//! - Atomics for lock-free counters
//!
//! ## Key ideas in plain English
//! - **Thread**: an independent line of execution.
//! - **Channel**: a pipe; one side sends, the other receives. "Do not
//!   communicate by sharing memory; share memory by communicating."
//! - **Mutex**: a lock so only one thread touches the data at a time.
//! - **Arc**: an atomically reference-counted pointer so several threads can
//!   own the same data. (`Rc` is *not* allowed across threads.)
//!
//! Two marker traits do the safety checking: `Send` (can move to another
//! thread) and `Sync` (can be shared by reference between threads).
//!
//! ## Common beginner mistakes
//! - Forgetting `move` on the closure: "closure may outlive the current function".
//! - Not calling `join()` so `main` exits before threads finish.
//! - Holding a `MutexGuard` for too long (it unlocks when dropped).
//!
//! ## How to run
//! ```text
//! cargo run --example 15_concurrency
//! ```
//!
//! ## Try it yourself
//! Make the channel example send `(id, result)` tuples from 4 workers.
//!
//! ## Further reading
//! <https://doc.rust-lang.org/book/ch16-00-concurrency.html>
#![allow(rustdoc::private_intra_doc_links)]


use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/// Spawn threads and wait for them with `join`.
fn spawn_and_join() {
    println!("--- spawn & join ---");
    let handle = thread::spawn(|| {
        let mut total = 0;
        for i in 1..=100 {
            total += i;
        }
        total // the closure's return value is delivered by join()
    });
    let result = handle.join().expect("thread panicked");
    println!("thread computed {result}");

    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // `move`: the thread now owns `data`
        println!("thread sees {data:?}");
    });
    handle.join().unwrap();
}

/// Producer/consumer using a multi-producer, single-consumer channel.
fn channels() {
    println!("--- channels ---");
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for id in 0..3 {
        let tx = tx.clone(); // each worker gets its own sender
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(10 * id));
            tx.send(format!("worker {id} done")).unwrap();
        }));
    }
    drop(tx); // close our own sender, otherwise the receive loop never ends
    for msg in rx {
        // iterates until all senders are dropped
        println!("got: {msg}");
    }
    for h in handles {
        h.join().unwrap();
    }
}

/// Shared mutable counter: `Arc` to share, `Mutex` to guard.
fn mutex_counter() {
    println!("--- Arc<Mutex> ---");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter.lock().unwrap(); // blocks until available
                *guard += 1;
            } // guard dropped -> unlocked
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("final = {} (expected 8000)", *counter.lock().unwrap());
}

/// `RwLock`: many readers OR one writer.
fn rwlock() {
    println!("--- RwLock ---");
    let config = Arc::new(RwLock::new(String::from("v1")));
    let readers: Vec<_> = (0..3)
        .map(|i| {
            let c = Arc::clone(&config);
            thread::spawn(move || format!("reader {i} saw {}", c.read().unwrap()))
        })
        .collect();
    for r in readers {
        println!("{}", r.join().unwrap());
    }
    *config.write().unwrap() = "v2".to_string();
    println!("after write: {}", config.read().unwrap());
}

/// Atomics are the cheapest way to share a simple counter.
fn atomics() {
    println!("--- atomics ---");
    let hits = Arc::new(AtomicUsize::new(0));
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let hits = Arc::clone(&hits);
            thread::spawn(move || {
                for _ in 0..500 {
                    hits.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    println!("hits = {}", hits.load(Ordering::SeqCst)); // 2000
}

/// Scoped threads may borrow from the stack because they must finish before
/// the scope ends. No `Arc`/`move` required.
fn scoped() {
    println!("--- scoped threads ---");
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = numbers.split_at(3);
    let (a, b) = thread::scope(|s| {
        let h1 = s.spawn(|| left.iter().sum::<i32>());
        let h2 = s.spawn(|| right.iter().sum::<i32>());
        (h1.join().unwrap(), h2.join().unwrap())
    });
    println!("left={a} right={b} total={}", a + b);
}

fn main() {
    spawn_and_join();
    channels();
    mutex_counter();
    rwlock();
    atomics();
    scoped();
}
