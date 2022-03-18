use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};

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


