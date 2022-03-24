use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};
use std::thread::sleep;

pub struct Semaphore {
    size: u8,
    mutex: Mutex<(i8, u8)>, // first element is counter, second is passes
    cond_var: Condvar,
}

impl Semaphore {
    pub fn new(size: u8) -> Semaphore {
        if size == 0 {
            panic!("Semaphore size must be greater than 0.")
        }
        Semaphore {
            size,
            mutex: Mutex::new((size as i8, 0)),
            cond_var: Condvar::new(),
        }
    }

    pub fn decrement(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
        let f = (*mutex_guard.deref()).1;
        let (counter, _) = mutex_guard.deref_mut();
        *counter -= 1;

        if *counter < 0 {
            while (*mutex_guard.deref()).1 == 0 {
                mutex_guard = self.cond_var.wait(mutex_guard).unwrap();
            }
            let (_, passes) = mutex_guard.deref_mut();
            *passes -= 1;
        }
    }

    pub fn increment(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
        let (counter, passes) = mutex_guard.deref_mut();

        if *counter < 0 {
            *counter += 1;
            *passes += 1;
            self.cond_var.notify_one();
        } else if *counter == self.size as i8 {
            // do nothing
        } else {
            *counter += 1;
        }
    }

    pub fn wait(&self) {
        self.decrement()
    }

    pub fn signal(&self) {
        self.increment()
    }
}


