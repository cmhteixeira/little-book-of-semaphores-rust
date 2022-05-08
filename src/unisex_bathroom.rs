use crate::semaphore_simplest::Semaphore;
use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::sync::Mutex;

pub struct Bathroom {
    women_counter: UnsafeCell<u16>,
    men_counter: UnsafeCell<u16>,
    sw: Semaphore,
    sm: Semaphore,
    sw_c: Semaphore,
    sm_c: Semaphore,
    sg: Semaphore,
}

impl Bathroom {
    pub fn new(max_people: u16) -> Bathroom {
        Bathroom {
            women_counter: UnsafeCell::new(0),
            men_counter: UnsafeCell::new(0),
            sw: Semaphore::new(max_people),
            sm: Semaphore::new(max_people),
            sw_c: Semaphore::new(1),
            sm_c: Semaphore::new(1),
            sg: Semaphore::new(1),
        }
    }

    pub fn access_woman<T>(&self, f: T) -> () where T: Fn(()) -> () {
        self.sw.decrement();
        self.sw_c.decrement();
        unsafe {
            if *self.women_counter.get() == 0 {
                self.sg.decrement();
            }
            *self.women_counter.get() += 1;
        }
        self.sw_c.increment();
        f(());
        self.sw_c.decrement();
        unsafe {
            if *self.women_counter.get() == 1 {
                self.sg.increment();
            }
            *self.women_counter.get() -= 1;
        }
        self.sw_c.increment();
        self.sw.increment();
    }

    pub fn access_man<T>(&self, f: T) -> () where T: Fn(()) -> () {
        self.sm.decrement();
        self.sm_c.decrement();
        unsafe {
            if *self.men_counter.get() == 0 {
                self.sg.decrement();
            }
            *self.men_counter.get() += 1;
        }
        self.sm_c.increment();
        f(());
        self.sm_c.decrement();
        unsafe {
            if *self.men_counter.get() == 1 {
                self.sg.increment();
            }
            *self.men_counter.get() -= 1;
        }
        self.sm_c.increment();
        self.sm.increment();
    }
}

unsafe impl Sync for Bathroom {}
