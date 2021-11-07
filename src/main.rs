use little_book_semaphores_rust::producer_consumer::BlockingQueue;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use little_book_semaphores_rust::readers_writers::ReadersWritersLock;
use rand::Rng;

fn main() {
    let mut my_initial = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let rw_lock = ReadersWritersLock::new(&mut my_initial);
    let rw_lock = Arc::new(rw_lock);

    let mut threads = vec![];
    for i in 1..5 {
        let rw_lock = rw_lock.clone();
        let thread_handle = thread::Builder::new().name(String::from(format!("WriterThread:{}/100", i)))
            .spawn(move || unsafe {
                // let mut current_value: Vec<String> = vec![];
                let mut iter = 0;
                loop {
                    iter += 1;
                    println!("I am writer thread {}, outside", i);
                    thread::sleep(Duration::from_millis(100));
                    rw_lock.write(|a| {
                        println!("I am writer thread {}, currently on the critical section", i);
                        let num = rand::thread_rng().gen_range(20..30);
                        thread::sleep(Duration::from_millis(num * 10));
                        let new_value = a.into_iter().map(|k| format!("{}-{}", k, i)).collect();
                        new_value
                    });
                    if iter == 20 {break;}
                }
            }).expect("Can't create thread ...");
        threads.push(thread_handle);
    }

    for i in 1..4 {
        let rw_lock = rw_lock.clone();
        let thread_handle = thread::Builder::new().name(String::from(format!("ReaderThread:{}/100", i)))
            .spawn(move || {
                let mut iter = 0;
                loop {
                    iter += 1;
                    let res = rw_lock.read(|protected_resource| {
                        let num = rand::thread_rng().gen_range(1..5);
                        thread::sleep(Duration::from_millis(num * 1000));
                        protected_resource.clone()
                    });
                    println!("I am reader thread {}, currently on the critical section. The content is:\n{} ", i, res.join(" # "));
                    if iter == 20 {break;}
                }
            }).expect("Can't create thread ...");
        threads.push(thread_handle);
    }

    for i in threads {
        i.join();
    }
}
