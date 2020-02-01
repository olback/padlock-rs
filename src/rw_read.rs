use std::sync::{
    RwLock,
    RwLockReadGuard,
    atomic::spin_loop_hint
};

pub fn rw_read_lock<T, F, R>(l: &RwLock<T>, f: F) -> R
    where F: FnOnce(&T) -> R {

    let r: R;

    loop {

        match l.try_read() {

            Ok(lock) => {
                r = f(&lock);
                drop(lock);
                break;
            },

            Err(_) => spin_loop_hint()

        }

    }

    r

}

pub fn get_rw_read_lock<T>(l: &RwLock<T>) -> RwLockReadGuard<T> {

    loop {

        match l.try_read() {

            Ok(lock) => return lock,

            Err(_) => spin_loop_hint()

        }

    }

}
