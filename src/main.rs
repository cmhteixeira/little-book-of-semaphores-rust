use std::cell::RefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};
use std::time;
use rand::Rng;

use little_book_semaphores_rust::readers_writers::ReadersWritersLock;
use little_book_semaphores_rust::semaphore_fixed_2::{Semaphore as SemaphoreFixed2};
use little_book_semaphores_rust::semaphore_fixed::{Semaphore as SemaphoreFixed};
use little_book_semaphores_rust::Semaphore;

fn main() {
    let s = Arc::new(SemaphoreFixed2::new(2));
    let s1 = Arc::clone(&s);
    let s2 = Arc::clone(&s);
    let s3 = Arc::clone(&s);


    let r = thread::Builder::new().name(String::from("Thread-1"))
        .spawn(move || unsafe {
            s1.decrement();
            let being = SystemTime::now();
            loop {
                let time = being.elapsed().unwrap().as_secs();
                thread::sleep(Duration::from_millis(500));
                if (time % 5) == 0 {
                    println!("Printing from thread-1")
                }
            }
        }).unwrap();

    let t = thread::Builder::new().name(String::from("Thread-2"))
        .spawn(move || unsafe {
            s2.decrement();
            let being = SystemTime::now();
            let mut time = being.elapsed().unwrap().as_secs();
            while (time == 0 || time % 15 != 0 ) {
                thread::sleep(Duration::from_millis(500));
                if (time % 5) == 0 {
                    println!("Printing from thread-2")
                }
                time = being.elapsed().unwrap().as_secs();
                // println!("{}", time)
            }
            println!("Leaving thread-2");
            s2.increment()
        }).unwrap();

    let z = thread::Builder::new().name(String::from("Thread-3"))
        .spawn(move || unsafe {
            s3.decrement();
            let being = SystemTime::now();
            loop {
                let time = being.elapsed().unwrap().as_secs();
                thread::sleep(Duration::from_millis(500));
                if (time % 5) == 0 {
                    println!("Printing from thread-3")
                }
            }
        }).unwrap();


    r.join();
}