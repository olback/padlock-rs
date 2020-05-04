//! Aquire [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
//! and [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)s
//! safely.
//!
//! All methods in this crate will try to lock the passed Mutex or RwLock,
//! if the locking fails, [`spin_loop_hint`](https://doc.rust-lang.org/std/sync/atomic/fn.spin_loop_hint.html)
//! is called and we try again. This practice is called [spinlock](https://en.wikipedia.org/wiki/Spinlock).
//!
//! This means that all calls will block the current thread.
//!
//! Important: When using methods like `mutex_lock`, remember that the lock is
//! droped first when the lambda finishes running.
//!
//! Example:
//!```no_run
//! use std::{
//!     thread,
//!     sync::{Arc, Mutex},
//!     time::Duration
//! };
//!
//! #[derive(Debug)]
//! struct Person {
//!     age: u8,
//!     name: String
//! }
//!
//! fn main() {
//!
//!     let people = Arc::new(Mutex::new(Vec::<Person>::new()));
//!     let mut threads = Vec::<thread::JoinHandle<()>>::new();
//!
//!     // Write in one thread
//!     let people_clone = Arc::clone(&people);
//!     threads.push(thread::spawn(move || {
//!
//!         for i in 0..10 {
//!
//!             padlock::mutex_lock(&people_clone, |lock| {
//!
//!                 lock.push(Person {
//!                     age: i * 10,
//!                     name: format!("Name {}", i)
//!                 });
//!
//!             });
//!
//!             thread::sleep(Duration::from_millis(500));
//!
//!         }
//!
//!     }));
//!
//!     // Read from another
//!     let people_clone = Arc::clone(&people);
//!     threads.push(thread::spawn(move || {
//!
//!         for _ in 0..6 {
//!
//!             padlock::mutex_lock(&people_clone, |lock| {
//!
//!                 for person in lock {
//!                     println!("{:?}", person);
//!                 }
//!
//!             });
//!
//!             thread::sleep(Duration::from_secs(1));
//!
//!         }
//!
//!     }));
//!
//!     for t in threads {
//!         t.join();
//!     }
//!
//! }
//!```
//!

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod mutex;
mod rw_read;
mod rw_write;

pub use mutex::*;
pub use rw_read::*;
pub use rw_write::*;
