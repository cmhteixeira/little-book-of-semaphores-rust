use std::sync::{Condvar, Mutex};

/// This is the simplest semaphore implementation there is.
///
/// The counter, protected by the mutex, is never lower than 0. Some definitions of a semaphore
/// describe a counter value lower than zero as representing the number of threads waiting. That
/// is not the case here.
///
/// ### See also
///
/// 1. Sub-chapter `31.7` of the book `Operating Systems: Three Easy Pieces` by
/// `Remzi Arpachi-Dusseau` and `Andrea Arpaci-Dusseau`.
///
/// 2. Rust's official [implementation](https://doc.rust-lang.org/1.8.0/src/std/up/src/libstd/sync/semaphore.rs.html)
/// of a Semaphore for versions <= 1.8. Now deprecated.
///
pub struct Semaphore {
    mutex: Mutex<u16>,
    cond_var: Condvar,
}

impl Semaphore {
    pub fn new(initial_size: u16) -> Semaphore {
        Semaphore {
            mutex: Mutex::new(initial_size),
            cond_var: Condvar::new(),
        }
    }

    pub fn decrement(&self) {
        let mut guard = self.mutex.lock().unwrap();

        while *guard == 0 {
            guard = self.cond_var.wait(guard).unwrap();
        }
        *guard -= 1;
    }

    pub fn increment(&self) {
        let mut guard = self.mutex.lock().unwrap();
        *guard += 1;
        self.cond_var.notify_one();
    }
}