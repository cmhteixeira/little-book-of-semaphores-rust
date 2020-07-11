use little_book_semaphores_rust::Semaphore;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use little_book_semaphores_rust::rendezvous::*;
use little_book_semaphores_rust::mutex::{monkey, MyCustomMutex};
use std::ops::Deref;

fn main() {
   let counter = MyCustomMutex::new(vec![]);
   let arc_counter = Arc::new(counter);

   let res = monkey(10, arc_counter.clone());

   for handle in res {
      handle.join().expect("asdsad");
   }

   println!("{:?}", arc_counter.deref().lock());
   arc_counter.deref().unlock();
   let res: (u128, bool) = arc_counter.deref().lock().iter().fold((0, true), |(a, b), t|{
      if b {
         (t.clone(), t.clone()  == a + 1)
      } else {
         (a, b)
      }
   });

   println!("{:?}", res)
}
