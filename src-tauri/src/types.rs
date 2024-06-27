use rocket::serde::Serialize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

pub const GAME_SPEED_FACTOR: f64 = 1.0;

#[macro_export]
macro_rules! to_factored_ms {
    ($ms:expr) => {
        (($ms as f64 * crate::types::GAME_SPEED_FACTOR) as u64)
    };
}

pub type ArcRwBox<T> = Arc<RwLock<Box<T>>>;

pub fn new_arc_rw_box<T>(t: T) -> ArcRwBox<T> {
    Arc::new(RwLock::new(Box::new(t)))
}

pub type ArcMutBox<T> = Arc<Mutex<Box<T>>>;

pub fn new_arc_mut_box<T>(t: T) -> ArcMutBox<T> {
    Arc::new(Mutex::new(Box::new(t)))
}

/// Base64 encoded icon
#[derive(Debug, Serialize, Clone)]
pub struct Image {
    content: String,
    image_type: String,
}

impl TryFrom<PathBuf> for Image {
    type Error = String;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let image_type = path
            .extension()
            .ok_or_else(|| "No extension found".to_string())?
            .to_str()
            .ok_or_else(|| "Invalid extension".to_string())?
            .to_string();
        let content = std::fs::read(path).map_err(|e| format!("Failed to read file: {:?}", e))?;
        let base64 = base64::encode(content);
        Ok(Self {
            content: base64,
            image_type,
        })
    }
}

/// Enchants `Option` and `Result` with `expect_lazy` method to avoid using .unwrap_or_else(|| panic!())
pub trait LazyExpect<T> {
    fn expect_lazy(self, fun: impl FnOnce() -> String) -> T;
}

impl<T, E> LazyExpect<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
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

/// Adds `swap` method to `RwLock` to simplify value replacement
pub trait Swap<T> {
    fn swap(&self, new_value: T);
}

impl<T> Swap<T> for RwLock<T> {
    fn swap(&self, new_value: T) {
        let mut write_guard = self.write().unwrap();
        *write_guard = new_value;
    }
}
