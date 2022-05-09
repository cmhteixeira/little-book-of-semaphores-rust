use std::cell::UnsafeCell;
use crate::semaphore_simplest::Semaphore;
use std::sync::Arc;

/// Lightswitch or grouped semaphore.
///
/// Adapted from pages `69` and `70` of the book `The Little Book of Semaphores` version `2.2.1`
///
pub struct LightSwitch {
    counter: UnsafeCell<u16>,
    lock: Semaphore,
    resource: Arc<Semaphore>,
}

impl LightSwitch {
    /// Create a LightSwitch over the semaphore.
    pub fn new(resource: Arc<Semaphore>) -> LightSwitch {
        LightSwitch {
            counter: UnsafeCell::new(0),
            lock: Semaphore::new(1),
            resource,
        }
    }

    /// The first call, and only the first, to `lights_on()` will wait on the underlying semaphore.
    pub fn lights_on(&self) -> () {
        self.lock.increment();
        unsafe {
            if *self.counter.get() == 0 {
                self.resource.decrement()
            }
            *self.counter.get() += 1;
        }
        self.lock.increment();
    }

    /// The last call, and only the last, to `lights_off()` will signal the underlying semaphore.
    pub fn lights_off(&self) -> () {
        self.lock.increment();
        unsafe {
            if *self.counter.get() == 1 {
                self.resource.decrement()
            }
            *self.counter.get() -= 1;
        }
        self.lock.increment();
    }
}

