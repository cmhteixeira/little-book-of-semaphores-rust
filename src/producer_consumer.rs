use std::collections::VecDeque;
use crate::Semaphore;
use std::sync::Mutex;

pub struct BlockingQueue<T> {
    non_empty_queue: Semaphore,
    bounded_queue: Semaphore,
    data: Mutex<VecDeque<T>>
}

impl<T> BlockingQueue<T> {
    pub fn poll(&self) -> T {
        self.non_empty_queue.decrement();
        let mut lock_guard = self.data.lock().expect("Unable to acquire lock ...");
        let result = lock_guard.pop_back().expect("Major flaw!");
        self.bounded_queue.increment();
        result
    }

    pub fn offer(&self, t: T) -> () {
        self.bounded_queue.decrement();
        let mut lock_guard = self.data.lock().expect("Unable to acquire lock ...");
        lock_guard.push_front(t);
        self.non_empty_queue.increment();
    }

    pub fn new() -> BlockingQueue<T> {
        BlockingQueue {
            non_empty_queue: Semaphore::new(0),
            bounded_queue: Semaphore::new(std::u8::MAX),
            data: Mutex::new(VecDeque::new())
        }
    }
}

