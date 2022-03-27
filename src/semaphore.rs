use std::sync::{Mutex, Condvar};
use std::ops::{DerefMut, Deref};

/// This is the original solution I came up with.
///
/// I don't remember why I thought there wa a need for the extra boolean. It doesn't seem
/// to be needed.
///
pub struct Semaphore {
    mutex: Mutex<(u8, bool)>,
    condvar: Condvar
}

impl Semaphore {
    pub fn new(u: u8) -> Semaphore {
        Semaphore {
            mutex: Mutex::new((u, if u <= 0 {false} else {true})),
            condvar: Condvar::new()
        }
    }

    pub fn increment(&self) -> () {
        let mut mutex_guard= self.mutex.lock().expect("Can't lock");
        let (counter, proceed) = mutex_guard.deref_mut();
        *counter = *counter + 1;
        *proceed = true;
        self.condvar.notify_one();
    }

    pub fn decrement(&self) -> () {
        let mut mutex_guard = self.mutex.lock().expect("Can't lock");
        while !(mutex_guard.deref().1) {  // guard against spurious awakes.
            mutex_guard = self.condvar.wait(mutex_guard).expect("Can't wait.");
        }
        let (counter, bool) = mutex_guard.deref_mut();
        *counter = *counter - 1;
        if *counter == 0 {
            *bool = false;
        }
    }
}