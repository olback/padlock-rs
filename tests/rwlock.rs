use padlock;
use std::sync::{Arc, RwLock};

#[test]
fn rw_lock() {

    let am = Arc::new(RwLock::new(0));

    padlock::rw_read_lock(&am, |lock| {
        assert_eq!(*lock, 0);
    });

    padlock::rw_write_lock(&am, |lock| {
        *lock = 10;
    });

    padlock::rw_read_lock(&am, |lock| {
        assert_eq!(*lock, 10);
    });

}

#[test]
fn get_rw_lock() {

    let am = Arc::new(RwLock::new(0));

    let r_lock = padlock::get_rw_read_lock(&am);
    assert_eq!(*r_lock, 0);
    drop(am.read().unwrap());
    drop(r_lock);

    let mut w_lock = padlock::get_rw_write_lock(&am);
    *w_lock = 42;
    drop(w_lock);

    let r_lock_1 = padlock::get_rw_read_lock(&am);
    let r_lock_2 = padlock::get_rw_read_lock(&am);
    let r_lock_3 = padlock::get_rw_read_lock(&am);

    assert_eq!(42, *r_lock_1);
    assert_eq!(*r_lock_2, *r_lock_3);

}
