use std::thread;
use std::sync::{Arc, Mutex};
use crate::Semaphore;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Output {
    pub thread_handles: (JoinHandle<()>, JoinHandle<()>),
    pub output: Arc<Mutex<Vec<String>>>
}

pub fn rendezvous() -> Output {
    let output = Arc::new(Mutex::new(vec![]));
    let a1_done = Arc::new(Semaphore::new(0));
    let b1_done = Arc::new(Semaphore::new(0));

    let a1_done_clone = a1_done.clone();
    let b1_done_clone = b1_done.clone();
    let output_clone = output.clone();
    let output_clone2 = output.clone();

    let handle_a =
        thread::Builder::new().name(String::from("Thread#A"))
        .spawn(move|| {
            output_clone2.lock().expect("Error-A1").push(String::from("a1"));
            a1_done.increment();

            b1_done.decrement();
            output_clone2.lock().expect("Error-A2").push(String::from("a2"));
            b1_done.increment();
        }).expect("Can't create thread ...");


    let handle_b =
        thread::Builder::new().name(String::from("Thread#B"))
        .spawn(move|| {
            output_clone.lock().expect("Error-B1").push(String::from("b1"));
            b1_done_clone.increment();

            a1_done_clone.decrement();
            output_clone.lock().expect("Error-B2").push(String::from("b2"));
            a1_done_clone.increment();
        }).expect("Can't create thread ...");


    Output {
        thread_handles: (handle_a, handle_b),
        output
    }
}