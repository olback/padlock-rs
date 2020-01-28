use std::sync::{
    Mutex,
    MutexGuard,
    atomic::spin_loop_hint
};

pub fn mutex_lock<T, F, R>(m: &Mutex<T>, f: F) -> R
    where F: FnOnce(&mut T) -> R {

    let r: R;

    loop {

        match m.try_lock().as_mut() {

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

pub fn get_mutex_lock<T>(m: &Mutex<T>) -> MutexGuard<T> {

    loop {

        match m.try_lock() {
            Ok(lock) => return lock,
            Err(_) => spin_loop_hint()
        }

    }

}
