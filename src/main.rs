use std::cell::RefMut;
use std::rc::Rc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};
use std::time;
use rand::Rng;

use little_book_semaphores_rust::readers_writers::ReadersWritersLock;
use little_book_semaphores_rust::semaphore_3::{Semaphore as SemaphoreFixed3};
use little_book_semaphores_rust::semaphore_2::{Semaphore as SemaphoreFixed2};
use little_book_semaphores_rust::semaphore_simplest::{Semaphore as SemaphoreSimplest};
use little_book_semaphores_rust::Semaphore;

fn main() {
    let s = Arc::new(SemaphoreSimplest::new(2));
    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);
    let s3 = Arc::clone(&s);
    // s3.increment();

    fn new_thread(s1: Arc<SemaphoreSimplest>, name: String, ttl_seconds: u64) -> JoinHandle<()> {
        thread::Builder::new().name(name.clone())
            .spawn(move || {
                s1.decrement();
                let begin = SystemTime::now();
                let mut elapsed = begin.elapsed().unwrap().as_secs();
                while elapsed < ttl_seconds {
                    thread::sleep(Duration::from_millis(500));
                    if (elapsed % 5) == 0 {
                        println!("Printing from '{}'", name)
                    }
                    elapsed = begin.elapsed().unwrap().as_secs();
                }
                s1.increment();
                println!("Leaving thread '{}'", name)
            }).unwrap()
    }

    let one = new_thread(s1, String::from("1"), 60);
    let _ = new_thread(s2, String::from("2"), 10);
    let _ = new_thread(s3, String::from("3"), 60);

    one.join();
}