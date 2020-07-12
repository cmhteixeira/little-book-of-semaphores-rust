use std::sync::{Mutex, Condvar};
use std::ops::{DerefMut, Deref};

pub struct Semaphore {
    counter: Mutex<(u8, bool)>,
    condvar: Condvar
}

impl Semaphore {
    pub fn new(u: u8) -> Semaphore {
        Semaphore {
            counter: Mutex::new((u, if u <= 0 {false} else {true})),
            condvar: Condvar::new()
        }
    }

    pub fn increment(&self) -> () {
        let mut mutex_guard= self.counter.lock().expect("Can't lock");
        let (mut counter, mut bool) = mutex_guard.deref_mut();
        bool = true;
        counter = counter + 1;
        self.condvar.notify_one();
    }

    pub fn decrement(&self) -> () {
        let mut mutex_guard = self.counter.lock().expect("Can't lock");
        let (mut counter, mut bool) = mutex_guard.deref_mut();
        while !bool {  // guard against spurious awakes.
            mutex_guard = self.condvar.wait(mutex_guard).expect("Can't wait.");
        }
        counter = counter - 1;
        if counter == 0 {
            bool = false;
        }
    }
}