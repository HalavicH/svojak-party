#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]

use serde::Serialize;
use crate::core::app_context::{app};
use std::sync::{Arc, Mutex, MutexGuard};
use tauri::Window;

pub enum Event {
    Message,
    Error,
    GameConfig,
}

/// Impl enum to &str conversion
impl<'a> From<Event> for &'a str {
    fn from(val: Event) -> Self {
        match val {
            Event::Message => "message",
            Event::Error => "error",
            Event::GameConfig => "GameConfig",
        }
    }
}

pub fn emit_message<S: Serialize + Clone>(message: S) {
    emit(Event::Message, message);
}

pub fn emit<S: Serialize + Clone>(event: Event, message: S) {
    if let Some(window) = window().as_ref() {
        window
            .emit(event.into(), message)
            .map_err(|e| format!("Failed to send message: {}", e))
            .expect("Expected to send message to the front-end")
    }
}

lazy_static::lazy_static! {
    static ref WINDOW: Arc<Mutex<Option<Window>>> = Arc::new(Mutex::new(Option::default()));
}

pub fn window() -> MutexGuard<'static, Option<Window>> {
    WINDOW.lock()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub fn set_window(window: Window) {
    let mut guard = WINDOW.lock().expect("Mutex is poisoned");
    *guard = Some(window);
}
