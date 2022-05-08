use std::cell::RefMut;
use std::rc::Rc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::{JoinHandle, Thread};
use std::time::{Duration, SystemTime};
use std::time;
use rand::Rng;

use little_book_semaphores_rust::readers_writers::ReadersWritersLock;
use little_book_semaphores_rust::semaphore_3::{Semaphore as SemaphoreFixed3};
use little_book_semaphores_rust::semaphore_2::{Semaphore as SemaphoreFixed2};
use little_book_semaphores_rust::semaphore_simplest::{Semaphore as SemaphoreSimplest};
use little_book_semaphores_rust::Semaphore;
use little_book_semaphores_rust::unisex_bathroom::Bathroom;

fn main() {
    enum Person {
        Man,
        Woman,
    }

    let bathroom = Arc::new(Bathroom::new(5));
    let bathroom_c1 = Arc::clone(&bathroom);
    let bathroom_c2 = Arc::clone(&bathroom);
    let bathroom_c3 = Arc::clone(&bathroom);
    let bathroom_c4 = Arc::clone(&bathroom);

    fn person(bathroom: Arc<Bathroom>, name: String, ttl_seconds: u64, type_person: Person) -> JoinHandle<()> {
        thread::Builder::new().name(name.clone())
            .spawn(move || {
                match type_person {
                    Person::Man => {
                        bathroom.access_man(|_| {
                            println!("Man {} entered", name);
                            thread::sleep(Duration::from_millis(500));
                            println!("Man {} exiting", name);
                        })
                    }
                    Person::Woman => {
                        bathroom.access_woman(|_| {
                            println!("Woman {} entered", name);
                            thread::sleep(Duration::from_millis(500));
                            println!("Woman {} exiting", name);
                        })
                    }
                }
            }).unwrap()
    }

    let one = person(bathroom_c1, String::from("A"), 60, Person::Man);
    let _ = person(bathroom_c2, String::from("B"), 60, Person::Man);
    let _ = person(bathroom_c3, String::from("1"), 60, Person::Woman);
    let _ = person(bathroom_c4, String::from("2"), 60, Person::Woman);

    one.join();
    thread::sleep(Duration::from_secs(5));
}