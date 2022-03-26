use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};

/// This implementation is strikingly incorrect.
///
/// Imagine the following scenario:
///
/// 1. Initial                  X,X|X,X,X : counter=-2
/// 2. One thread leaves        X,X|O,X,X : counter=-1
/// 3. Because counter is negative, when woken up, the waiting threads will go back to waiting
/// state right after, because of the while condition trying to account for spurious wakes.
///
/// This would be fixed if we removed the while guard, but then we would be exposed to
/// spurious wakes.
///
///
pub struct Semaphore {
    size: u8,
    mutex_counter: Mutex<i8>,
    cond_var: Condvar,
}

impl Semaphore {
    pub fn new(size: u8) -> Semaphore {
        if size == 0 {
            panic!("Semaphore size must be greater than 0.")
        }
        Semaphore {
            size,
            mutex_counter: Mutex::new(size as i8),
            cond_var: Condvar::new(),
        }
    }

    pub fn decrement(&self) {
        let mut mutex_guard = self.mutex_counter.lock().unwrap();

        if *mutex_guard.deref() <= 0 {
            let counter = mutex_guard.deref_mut();
            *counter -= 1;

            while *mutex_guard.deref() <= 0 {
                mutex_guard = self.cond_var.wait(mutex_guard).unwrap();
            }
        } else {
            let counter = mutex_guard.deref_mut();
            *counter -= 1;
        }
    }

    pub fn increment(&self) {
        let mut mutex_guard = self.mutex_counter.lock().unwrap();

        if *mutex_guard.deref() < 0 {
            let counter = mutex_guard.deref_mut();
            *counter += 1;
            self.cond_var.notify_one();
        } else if *mutex_guard.deref() >= 0 && *mutex_guard.deref() < self.size as i8 {
            let counter = mutex_guard.deref_mut();
            *counter += 1;
        } else { // case where counter is N, semaphore is full
            // do nothing
        }
    }

    pub fn wait(&self) {
        self.decrement()
    }

    pub fn signal(&self) {
        self.increment()
    }
}


