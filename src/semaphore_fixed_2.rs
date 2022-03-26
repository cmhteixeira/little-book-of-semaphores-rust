use std::sync::{Mutex, Condvar};
use std::ops::{Deref, DerefMut};

/// State protected by the mutex that controls the behaviour of the semaphore.
///
///
///
/// ## Parameters
///
/// `passes`: One pass represents a sort of permit to enter the semaphore. It is not possible
///  for this parameter to be negative (i.e., < 0).
///
/// `counter`: Represents the number of available spots in the Semaphore. It takes into account
/// the number of passes "not redeemed".
/// If `counter` is greater than 0, then it represents how many threads would be able to enter
/// the semaphore without blocking. Additional threads might also be able to enter, depending on
/// the value of `passes`.
/// A `counter` of 0 means any new thread will block when trying to enter, and no threads are
/// waiting to enter *without* a pass. But there might be threads waiting to enter with `passes`.
/// If `counter` is lower than 0, then it represents how many threads are already blocked waiting
/// to enter. There might also be additional threads blocked waiting to enter, which are represented
/// by a positive value of `passes`.
///
///
struct State {
    counter: i16,
    passes: u16,
}

impl State {
    fn new(counter: i16, passes: u16) -> State {
        State {
            counter,
            passes,
        }
    }
}

pub struct Semaphore {
    mutex: Mutex<State>,
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
impl Semaphore {
    pub fn new(size: u16) -> Semaphore {
        if size == 0 {
            panic!("Semaphore size must be greater than 0.")
        }
        Semaphore {
            mutex: Mutex::new(State::new(size as i16, 0)),
            cond_var: Condvar::new(),
        }
    }

    pub fn decrement(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
        let State { counter, .. } = mutex_guard.deref_mut();
        *counter -= 1;

        if *counter < 0 {
            while (*mutex_guard.deref()).passes == 0 {
                mutex_guard = self.cond_var.wait(mutex_guard).unwrap();
            }
            let State { passes, .. } = mutex_guard.deref_mut();
            *passes -= 1;
        }
    }

    pub fn increment(&self) {
        let mut mutex_guard = self.mutex.lock().unwrap();
        let State { counter, passes } = mutex_guard.deref_mut();
        *counter += 1;

        if *counter <= 0 {
            *passes += 1;
            self.cond_var.notify_one();
        }
    }
}


