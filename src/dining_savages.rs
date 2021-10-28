// use crate::Semaphore;
// use std::sync::atomic::{AtomicU16, Ordering};
// use std::sync::Arc;
// use std::rc::Rc;
//
// pub struct Pot {
//     size_pot: u8,
//     sem_1: Arc<Semaphore>,
//     pot_is_empty: Arc<Semaphore>,
//     access_to_pot: Arc<Semaphore>,
//     servings_left: Arc<AtomicU16>
// }
//
// pub struct Cook {
//     size_pot: u8,
//     sem_1: Arc<Semaphore>,
//     pot_is_empty: Arc<Semaphore>,
//     access_to_pot: Arc<Semaphore>,
//     servings_left: Arc<AtomicU16>
// }
//
// impl !Sync for Cook {}
//
// impl Cook {
//     pub fn put_servings_in_pot(&self) -> () {
//         self.pot_is_empty.decrement();
//         self.access_to_pot.decrement();
//         for _ in 1 .. self.size_pot + 1 {
//             self.sem_1.increment();
//         }
//         self.servings_left.store(self.size_pot.into(), Ordering::SeqCst);
//         self.access_to_pot.increment();
//     }
// }
//
// pub struct Missionary {}
//
//
// impl Pot {
//     pub fn get_serving(&self) -> Missionary {
//         self.sem_1.decrement();
//         self.access_to_pot.decrement();
//         let i = self.servings_left.fetch_sub(1, Ordering::SeqCst) - 1;
//         if i == 0 {
//             self.pot_is_empty.increment();
//         }
//         self.access_to_pot.increment();
//         Missionary {}
//     }
//
//     pub fn new(servings: u8) -> (Pot, Cook) {
//         let sem_1 = Arc::new(Semaphore::new(servings));
//         let pot_is_empty = Arc::new(Semaphore::new(0));
//         let access_to_pot = Arc::new(Semaphore::new(1));
//         let servings_left = Arc::new(AtomicU16::new(servings.into()));
//
//         let pot = Pot {
//             size_pot: servings,
//             sem_1: sem_1.clone(),
//             pot_is_empty: pot_is_empty.clone(),
//             access_to_pot: access_to_pot.clone(),
//             servings_left: servings_left.clone()
//         };
//
//         let cook = Cook {
//             size_pot: servings,
//             sem_1: sem_1.clone(),
//             pot_is_empty: pot_is_empty.clone(),
//             access_to_pot: access_to_pot.clone(),
//             servings_left: servings_left.clone()
//         };
//
//         (pot, cook)
//     }
// }