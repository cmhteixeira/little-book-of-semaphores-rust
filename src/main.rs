use std::cell::{UnsafeCell};


use std::sync::{Arc};
use std::thread;
use std::thread::{JoinHandle};







use little_book_semaphores_rust::semaphore_simplest::{Semaphore as SemaphoreSimplest};



struct SharedState {
    cell: UnsafeCell<u128>,
}

impl SharedState {
    fn new() -> SharedState {
        SharedState {
            cell: UnsafeCell::new(0)
        }
    }
}

unsafe impl Sync for SharedState {}

fn main() {
    let shared_state: Arc<SharedState> = Arc::new(SharedState::new());
    let sem = Arc::new(SemaphoreSimplest::new(1));


    fn create_thread(semaphore: Arc<SemaphoreSimplest>, shared_state: Arc<SharedState>, name: String, iterations: u32) -> JoinHandle<()> {
        thread::Builder::new().name(name.clone())
            .spawn(move || {
                let mut local_counter: u128 = 1;

                while local_counter <= iterations as u128 {
                    semaphore.decrement();
                    unsafe {
                        let old = *shared_state.cell.get();
                        let new = old + 1;
                        *shared_state.cell.get() = new;
                    }
                    semaphore.increment();
                    local_counter = local_counter + 1;
                }
            }).unwrap()
    }

    let num_threads = 28;
    let mut thread_handlers = vec![];

    for i in 0..num_threads {
        let shared_state_this = shared_state.clone();
        let semaphore = sem.clone();
        let thread_handle = create_thread(semaphore, shared_state_this, i.to_string(), 10000);
        thread_handlers.push(thread_handle);
    }

    println!("Joining ...");
    for handle in thread_handlers {
        handle.join();
    }
    println!("Leaving ...");
    let res: u128;
    unsafe {
        res = shared_state.cell.get().read();
    }
    println!("Result is {}", res)
}