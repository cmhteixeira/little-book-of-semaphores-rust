use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};

/// This implementation is not 100% correct. It does not account for spurious wakes.
///
/// Meaning of the counter:
///
/// 1. counter > 0: Number of threads that could enter the semaphore without blocking.
/// 2. counter = 0: No new threads could enter the semaphore without blocking. However,
/// there might be additional threads which are in process of waking up (and about to enter), but
/// haven't yet done so.
/// 3. counter < 0: Number of threads currently blocking, waiting to enter the semaphore.
/// This does not account for threads which might be blocked, but in the process of being woken up.
///
pub struct Semaphore {
    mutex: Mutex<i16>,
    cond_var: Condvar,
}

impl Semaphore {
    pub fn new(size: u16) -> Semaphore {
        if size == 0 {
            panic!("Semaphore size must be greater than 0.")
        }
        Semaphore {
            mutex: Mutex::new(size as i16),
            cond_var: Condvar::new(),
        }
    }

    pub fn decrement(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
        let counter = mutex_guard.deref_mut();
        *counter -= 1;

        if *mutex_guard.deref() < 0 {
            mutex_guard = self.cond_var.wait(mutex_guard).unwrap();
        }
    }

    pub fn increment(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
        let counter = mutex_guard.deref_mut();
        *counter += 1;

        if *mutex_guard.deref() <= 0 {
            self.cond_var.notify_one();
        }
    }
}


