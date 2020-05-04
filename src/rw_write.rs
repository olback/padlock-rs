use std::sync::{
    RwLock,
    RwLockWriteGuard,
    atomic::spin_loop_hint
};

/// Aquire a RwLock write lock, passing the lock to the lambda.
/// The lock is released when the lambda finishes.
/// This function returns whatever the lambda returns,
/// allowing you to extract data from a lock without having
/// to worry about releasing the lock.
///
/// Read:
/// ```ignore
/// let extracted = padlock::rw_write_lock(&arc_mutex_var, |lock| lock.clone());
/// ```
///
/// Write:
/// ```ignore
/// padlock::rw_write_lock(&arc_mutex_var, |lock| *lock = new_value);
/// ```
pub fn rw_write_lock<T, F, R>(l: &RwLock<T>, f: F) -> R
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

/// Get the RwLockWriteGuard directly, aquired with a spinlock.
///
/// Important: Don't forget to drop the lock! Locks release
/// themselfs when they go out of scope but the faster you
/// drop it, the faster other threads get access.
pub fn get_rw_write_lock<T>(l: &RwLock<T>) -> RwLockWriteGuard<'_, T> {

    loop {

        match l.try_write() {
            Ok(lock) => return lock,
            Err(_) => spin_loop_hint()
        }

    }

}
