use std::sync::{
    RwLock,
    RwLockReadGuard,
    atomic::spin_loop_hint
};

/// Aquire a RwLock read lock, passing the lock to the lambda.
/// The lock is released when the lambda finishes.
/// This function returns whatever the lambda returns,
/// allowing you to extract data from a lock without having
/// to worry about releasing the lock.
/// ```ignore
/// let extracted = padlock::rw_read_lock(&arc_rwlock_var, |lock| lock.clone());
/// ```
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

/// Get the RwLockReadGuard directly, aquired with a spinlock.
///
/// Important: Don't forget to drop the lock! Locks release
/// themselfs when they go out of scope but the faster you
/// drop it, the faster other threads get access.
pub fn get_rw_read_lock<T>(l: &RwLock<T>) -> RwLockReadGuard<'_, T> {

    loop {

        match l.try_read() {

            Ok(lock) => return lock,

            Err(_) => spin_loop_hint()

        }

    }

}
