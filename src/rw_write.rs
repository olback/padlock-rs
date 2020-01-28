use std::sync::{
    RwLock,
    RwLockWriteGuard,
    atomic::spin_loop_hint
};

pub fn rw_write_lock<T, F, R>(l: RwLock<T>, f: F) -> R
    where F: FnOnce(&mut T) -> R {

    let r: R;

    loop {

        match l.try_write().as_mut() {

            Ok(mut lock) => {
                r = f(&mut lock);
                drop(lock);
                break;
            },

            Err(_) => spin_loop_hint()

        }

    }

    r

}

pub fn get_rw_write_lock<T>(l: &RwLock<T>) -> RwLockWriteGuard<T> {

    loop {

        match l.try_write() {
            Ok(lock) => return lock,
            Err(_) => spin_loop_hint()
        }

    }

}
