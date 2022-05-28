use std::thread;

use std::thread::JoinHandle;
use std::sync::atomic::{Ordering};
use crate::Semaphore;
use std::sync::Arc;



pub fn barrier(num_threads: u8) -> Vec<JoinHandle<()>> {
    let mut thread_handles = vec![];
    let counter = Arc::new(std::sync::atomic::AtomicU8::new(0));
    let semaphore = Arc::new(Semaphore::new(0));

    for i in 1..(num_threads + 1) {
        let semaphore = semaphore.clone();
        let counter = counter.clone();
        let handle =
            thread::Builder::new()
                .name(String::from(format!("Thread:{}/{}", i, num_threads)))
                .spawn(move || {
                    //rendezvous code
                    println!("Rendezvouz: Thread {}", thread::current().name().expect("kaboom"));
                    //rendezvous code
                    let number_reached_gate= counter.fetch_add(1, Ordering::SeqCst);
                    if number_reached_gate + 1 == num_threads {
                        println!("Entering ...");
                        for _ in 1..(num_threads + 1) {
                            semaphore.increment();
                        }
                    }
                    semaphore.decrement();
                    //critical section
                    println!("Critical section: Thread {}", thread::current().name().expect("kaboom"));
                    //critical section
                }).expect("Can't create thread ...");

        thread_handles.push(handle);
    }
    thread_handles
}