use crate::Semaphore;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::Arc;

fn das_multiplex(n: u8, num_threads: u8) -> Vec<JoinHandle<()>>{
    let mut thread_handles = vec![];
    let semaphore = Semaphore::new(n);
    let semaphore = Arc::new(semaphore);

    for i in 1.. num_threads {
        let semaphore = semaphore.clone();
        let handle = thread::Builder::new().name(String::from(format!("Thread:{}/{}", i, num_threads)))
            .spawn(move || {
                semaphore.decrement();
                thread::sleep(Duration::new(1, 3000));
                semaphore.increment();
            }).expect("Can't create thread ...");

        thread_handles.push(handle);
    }
    thread_handles
}
