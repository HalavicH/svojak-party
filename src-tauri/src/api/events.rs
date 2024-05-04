#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]

use serde::Serialize;
use tauri::Window;

pub enum Event {
    Message,
    Error,
    WebUsers,
}

/// Impl enum to &str conversion
impl<'a> From<Event> for &'a str {
    fn from(val: Event) -> Self {
        match val {
            Event::Message => "message",
            Event::Error => "error",
            Event::WebUsers => "web-users",
        }
    }
}

pub fn send_event<S: Serialize + Clone>(window: &Window, event: Event, message: S) {
    window
        .emit(event.into(), message)
        .map_err(|e| format!("Failed to send message: {}", e))
        .expect("Expected to send message to the front-end")
}

#[allow(dead_code)]
pub fn send_message(window: &Window, message: &str) {
    send_event(window, Event::Message, message)
}

#[allow(dead_code)]
pub fn send_error(window: &Window, message: &str) {
    send_event(window, Event::Error, message)
}
