use crate::Semaphore;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::ops::{DerefMut, Deref};
use std::cell::{RefCell, UnsafeCell};
use std::time::Duration;

pub struct MyCustomMutex<A> {
    a: UnsafeCell<A>,
    semaphore: Semaphore
}

impl<T> MyCustomMutex<T> {
    pub fn new(a: T) -> MyCustomMutex<T> {
        MyCustomMutex {
            a: UnsafeCell::new(a),
            semaphore: Semaphore::new(1)
        }
    }

    pub fn lock(&self) -> &mut T {
        self.semaphore.decrement();
        unsafe {&mut *self.a.get()}
    }

    pub fn unlock(&self) -> () {
        self.semaphore.increment()
    }
}

unsafe impl<T: Send> Sync for MyCustomMutex<T>{}
unsafe impl<T: Send> Send for MyCustomMutex<T>{}

pub fn monkey(num_threads: u8, the_ref: Arc<MyCustomMutex<Vec<u128>>>) -> Vec<JoinHandle<()>> {
    let mut thread_handles = vec![];

    for i in 1.. num_threads {
        let arc = the_ref.clone();
        let handle = thread::Builder::new().name(String::from(format!("Thread:{}/{}", i, num_threads)))
            .spawn(move || {
                loop {
                    thread::sleep(Duration::new(0, 1000));
                    let mut lock_obtained = arc.lock();
                    if lock_obtained.len() >= 100 {
                        arc.unlock();
                        break;
                    }
                    let last = lock_obtained.last().unwrap_or(&0);
                    lock_obtained.push(last.clone() + 1);
                    arc.unlock();
                }
            }).expect("Can't create thread ...");

        thread_handles.push(handle);
    }
    thread_handles
}