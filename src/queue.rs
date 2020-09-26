use crate::Semaphore;
use std::cell::UnsafeCell;
use std::borrow::Borrow;
use std::sync::Mutex;
use std::ops::DerefMut;

pub struct BlockingOneElementQueue<A: Clone> {
    elem: UnsafeCell<A>,
    enqueue_semaphore: Semaphore,
    dequeue_semaphore: Semaphore,
}

impl<A: Clone> BlockingOneElementQueue<A> {
    pub fn new(a: A) -> BlockingOneElementQueue<A> {
        BlockingOneElementQueue {
            elem: UnsafeCell::new(a),
            enqueue_semaphore: Semaphore::new(0),
            dequeue_semaphore: Semaphore::new(1),
        }
    }

    pub fn enqueue(&self, a: A) -> () {
        self.enqueue_semaphore.decrement();
        unsafe {*self.elem.get() = a};
        self.dequeue_semaphore.increment();
    }

    pub fn dequeue(&self) -> A {
        self.dequeue_semaphore.decrement();
        let res = unsafe{(*self.elem.get()).clone()};
        self.enqueue_semaphore.increment();
        res
    }
}

unsafe impl<T: ?Sized + Send + Clone> Send for BlockingOneElementQueue<T> { }
unsafe impl<T: ?Sized + Send + Clone> Sync for BlockingOneElementQueue<T> { }