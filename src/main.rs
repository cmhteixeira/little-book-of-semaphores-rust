use little_book_semaphores_rust::producer_consumer::BlockingQueue;
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let blocking_queue = BlockingQueue::new();
    let blocking_queue = Arc::new(blocking_queue);

    let mut threads = vec![];
    for i in 1..4 {
        let blocking_queue = blocking_queue.clone();
        let thread_handle = thread::Builder::new().name(String::from(format!("ProducerThread:{}/100", i)))
            .spawn(move || {
                loop {
                    blocking_queue.offer(i);
                    thread::sleep(Duration::from_millis(100));
                }
            }).expect("Can't create thread ...");
        threads.push(thread_handle);
    }

    for i in 1..1000 {
        let blocking_queue = blocking_queue.clone();
        let thread_handle = thread::Builder::new().name(String::from(format!("ConsumerThread:{}/100", i)))
            .spawn(move || {
                loop {
                    println!("{}", blocking_queue.poll());
                }
            }).expect("Can't create thread ...");
    }

    for i in threads {
        i.join();
    }
}
