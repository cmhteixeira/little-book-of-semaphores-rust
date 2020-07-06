use std::sync::{Mutex, Condvar};
use std::ops::{DerefMut, Deref};

pub struct Semaphore {
    counter: Mutex<u8>,
    condvar: Condvar
}

impl Semaphore {
    pub fn new(u: u8) -> Semaphore {
        Semaphore {
            counter: Mutex::new(u),
            condvar: Condvar::new()
        }
    }

    pub fn increment(&self) -> () {
        let mut mutex_guard = self.counter.lock().expect("Can't lock");
        *mutex_guard = *mutex_guard + 1;
        if *mutex_guard >= 1 {
            self.condvar.notify_all();
        }
    }

    pub fn decrement(&self) -> () {
        let mut mutex_guard = self.counter.lock().expect("Can't lock");
        while ! (mutex_guard.ge(&1)) {  // guard against spurious awakes.
            mutex_guard = self.condvar.wait(mutex_guard).expect("Can't wait.");
        }
        let mut counter = mutex_guard.deref_mut();
        *counter = *counter - 1;
    }
}