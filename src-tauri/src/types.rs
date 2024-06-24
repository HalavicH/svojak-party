use std::sync::{Arc, Mutex, RwLock};

pub type ArcRwBox<T> = Arc<RwLock<Box<T>>>;

pub fn new_arc_rw_box<T>(t: T) -> ArcRwBox<T> {
    Arc::new(RwLock::new(Box::new(t)))
}

pub type ArcMutBox<T> = Arc<Mutex<Box<T>>>;

pub fn new_arc_mut_box<T>(t: T) -> ArcMutBox<T> {
    Arc::new(Mutex::new(Box::new(t)))
}

pub trait LazyExpect<T> {
    fn expect_lazy(self, fun: impl FnOnce() -> String) -> T;
}

impl<T, E> LazyExpect<T> for Result<T, E>
    where E: std::fmt::Debug {
    fn expect_lazy(self, msg: impl FnOnce() -> String) -> T {
        match self {
            Ok(v) => v,
            Err(e) => panic!("{}: {:?}", msg(), e),
        }
    }
}

impl<T> LazyExpect<T> for Option<T> {
    fn expect_lazy(self, msg: impl FnOnce() -> String) -> T {
        match self {
            Some(v) => v,
            None => panic!("{}", msg()),
        }
    }
}
