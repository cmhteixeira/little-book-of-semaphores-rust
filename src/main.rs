use little_book_semaphores_rust::Semaphore;
use std::sync::Arc;
use std::time::Duration;
use little_book_semaphores_rust::rendezvous::*;
use little_book_semaphores_rust::mutex::{monkey, MyCustomMutex};
use std::ops::Deref;
use little_book_semaphores_rust::barrier::barrier;
use little_book_semaphores_rust::queue::BlockingOneElementQueue;
use std::thread;
use std::thread::{JoinHandle, Thread};
use little_book_semaphores_rust::dining_savages::{Pot, Cook};

fn main() {
    let (pot, cook) = Pot::new(5);
    let pot = Arc::new(pot);

    let mut consumer_threads: Vec<JoinHandle<()>> = vec![];

    for i in 1..100 {
        let pot = pot.clone();
        let thread_handle = thread::Builder::new().name(String::from(format!("ConsumerThread:{}/100", i)))
            .spawn(move || {
                loop {
                    let serving = pot.get_serving();
                    println!("Thread: '{}'. Hmm.... This missionary is delicious! I want more.", thread::current().name().unwrap())
                }
            }).expect("Can't create thread ...");
        consumer_threads.push(thread_handle);
    }


    loop {
        cook.put_servings_in_pot();
        println!("##################I am the cook ... and I have just refilled the pot.########################");
        thread::sleep(Duration::new(4, 0))
    }

    for i in consumer_threads {
        i.join();
    }
}
