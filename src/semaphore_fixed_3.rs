use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};

/// This implementation is not 100% correct. It does not account for
///
/// 1. Spurious wakes.
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


