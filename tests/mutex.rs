use padlock;
use std::sync::{Arc, Mutex};

#[test]
fn mutex_lock() {

    let am = Arc::new(Mutex::new(0));

    padlock::mutex_lock(&am, |lock| {
        assert_eq!(*lock, 0);
    });

    padlock::mutex_lock(&am, |lock| {
        *lock = 10;
    });

    padlock::mutex_lock(&am, |lock| {
        assert_eq!(*lock, 10);
    });

}

#[test]
fn get_mutex_lock() {

    let am = Arc::new(Mutex::new(0));

    let mut lock = padlock::get_mutex_lock(&am);
    assert_eq!(*lock, 0);
    *lock = 42;
    assert_eq!(*lock, 42);
    drop(lock);

    // Locking here does not work unless the previous lock is dropped!
    let lock = padlock::get_mutex_lock(&am);
    assert_eq!(*lock, 42);
    drop(lock);

}

#[test]
fn get_mutex_lock_panic() {

    let am = Arc::new(Mutex::new(0));
    let _lock = padlock::get_mutex_lock(&am);

    drop(am.try_lock().unwrap_err());

}
