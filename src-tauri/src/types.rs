use std::sync::{Arc, Mutex, RwLock};

pub type ArcRwBox<T> = Arc<RwLock<Box<T>>>;
pub fn new_arc_rw_box<T>(t: T) -> ArcRwBox<T> {
    Arc::new(RwLock::new(Box::new(t)))
}
pub type ArcMutBox<T> = Arc<Mutex<Box<T>>>;
pub fn new_arc_mut_box<T>(t: T) -> ArcMutBox<T> {
    Arc::new(Mutex::new(Box::new(t)))
}
