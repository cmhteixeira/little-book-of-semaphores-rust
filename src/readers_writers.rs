use std::cell::UnsafeCell;
use crate::Semaphore;
use std::sync::atomic;
use std::sync::atomic::{AtomicU64, AtomicPtr, Ordering};
use std::fmt::{Display, Debug};
use rand::Rng;

pub struct ReadersWritersLock<T: Debug> {
    value: T,
    elem: AtomicPtr<T>,
    readers_counter: atomic::AtomicU64,
    readers_s: Semaphore,
    writers_s: Semaphore,
}

impl<T: Debug> ReadersWritersLock<T> {
    pub fn new(mut t: T) -> ReadersWritersLock<T> {
        // let mut foo = t;
        // let pointer = AtomicPtr::new(&mut t);

        ReadersWritersLock {
            elem: AtomicPtr::new(&mut t),
            value: t,
            readers_counter: AtomicU64::new(0),
            readers_s: Semaphore::new(1),
            writers_s: Semaphore::new(1),
        }
    }

    pub fn read<B, Res>(&self, read_op: B) -> Res
        where B: Fn(&T) -> Res {
        self.readers_s.decrement();
        if self.readers_counter.load(Ordering::SeqCst) == 0 {
            self.writers_s.decrement();
        }

        let current_readers_i = self.readers_counter.load(Ordering::SeqCst);
        self.readers_counter.store(current_readers_i + 1, Ordering::SeqCst);

        self.readers_s.increment();

        // Read critical section - Begin
        let result;
        unsafe {
            println!("Read..");
            let current_value = self.elem.load(Ordering::SeqCst);
            result = read_op(&*current_value);
        }
        // Read critical section - End

        self.readers_s.decrement();
        if self.readers_counter.load(Ordering::SeqCst) == 1 {
            self.writers_s.increment()
        }
        let current_readers_j = self.readers_counter.load(Ordering::SeqCst);
        self.readers_counter.store(current_readers_j - 1, Ordering::SeqCst);
        self.readers_s.increment();
        result
    }

    pub fn write<F>(&self, mut write_op: F) -> () where F: FnMut(&T) -> T {
        self.writers_s.decrement();

        // Writers critical section
        unsafe {
            let current_value = self.elem.load(Ordering::SeqCst);
            let mut new_value = write_op(&*current_value);
            *current_value = new_value;
            self.elem.store(current_value, Ordering::SeqCst);
        }
        // Writers critical section

        println!("this is");
        println!("This is the element {:?}", self.value);
        self.writers_s.increment();
    }
}


impl<T: Debug> Drop for ReadersWritersLock<T> {
   fn drop(&mut self) {
       println!("Dropping ...");
       // std::thread::sleep(std::time::Duration::from_millis(2000));
       println!("Dropping2 ...");
       println!("The readers counter is: {:?}", self.readers_counter.load(Ordering::SeqCst));
       unsafe {
           println!("The pointer is: {:?}", *self.elem.load(Ordering::SeqCst));
       }
       // println!("The value is: {:?}", self.value);
       println!("Dropping3 ...");
       println!("The reference is: {:?}", self.elem.load(Ordering::SeqCst));
    }
}