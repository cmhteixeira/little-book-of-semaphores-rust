use little_book_semaphores_rust::Semaphore;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use little_book_semaphores_rust::rendezvous::*;
use little_book_semaphores_rust::mutex::{monkey, MyCustomMutex};
use std::ops::Deref;
use little_book_semaphores_rust::barrier::barrier;

fn main() {
   let barrier = barrier(200);

   for a in barrier {
      a.join();
   }
}
