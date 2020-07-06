use little_book_semaphores_rust::Semaphore;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use little_book_semaphores_rust::rendezvous::*;

fn main() {
   let Output{thread_handles, output} = rendezvous();

   thread_handles.1.join().expect("Error joining thread ...");
   thread_handles.0.join().expect("Error joining thread ...");

   println!("{:?}", output.lock().expect("Error obtaining lock ...").to_vec());
}
