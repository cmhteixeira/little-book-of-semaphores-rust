use little_book_semaphores_rust::Semaphore;
use std::sync::Arc;
use std::time::Duration;
use little_book_semaphores_rust::rendezvous::*;
use little_book_semaphores_rust::mutex::{monkey, MyCustomMutex};
use std::ops::Deref;
use little_book_semaphores_rust::barrier::barrier;
use little_book_semaphores_rust::queue::BlockingOneElementQueue;
use std::thread;
use std::thread::JoinHandle;

fn main() {
   let queue = Arc::new(BlockingOneElementQueue::new(33));
   let queue_thread1 = queue.clone();

   let handle_a =
       thread::Builder::new().name(String::from("Thread#A"))
           .spawn(move|| {
              let queue = queue_thread1.clone();
              for i in 1..10000 {
                 queue.enqueue(i)
              }
           }).expect("Can't create thread ...");

   let queue_thread2 = queue.clone();
   let mut acc = vec![];
   for i in 1..10000 {
      acc.push(queue_thread2.dequeue());
   }

   handle_a.join();
   for i in acc {
      println!("{}", i);
   }
}
