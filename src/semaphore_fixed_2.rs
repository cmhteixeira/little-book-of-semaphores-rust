use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};
use std::thread::sleep;

pub struct Semaphore {
    size: u16,
    mutex: Mutex<(i16, u16)>, // first element is counter, second is passes
    cond_var: Condvar,
}


/// A thing that happen, although that does not seem to be a problem is a permit being
/// stolen by a new thread. That is, there are threads waiting, when a spot appears on the semaphore
/// because of a thread leaving. You might think that some of the CURRENTLY waiting threads will
/// enter the semaphore; not necessarily. A new thread, not currently waiting, might win contention
/// for the lock.
///
/// Initial                  X,X|X,X,X : counter=-2,passes=0
/// One thread leaves        X,X|O,X,X : counter=-1,passes=1
/// A NEW thread enters      X,X|X,X,X : counter=-2,passes=0
///
//

impl Semaphore {
    pub fn new(size: u16) -> Semaphore {
        if size == 0 {
            panic!("Semaphore size must be greater than 0.")
        }
        Semaphore {
            size,
            mutex: Mutex::new((size as i16, 0)),
            cond_var: Condvar::new(),
        }
    }

    pub fn decrement(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
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
        } else if *counter == self.size as i16 {
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


