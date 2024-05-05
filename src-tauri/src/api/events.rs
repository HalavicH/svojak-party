#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]

use crate::api::dto::{AppContextDto, PackInfoDto};
use crate::api::mapper::get_app_context_dto;
use crate::core::app_context::app;
use serde::Serialize;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard};
use tauri::Window;

pub enum Event {
    Message,
    Error,
    GameConfig,
    PackInfo,
}

/// Impl enum to &str conversion
impl<'a> From<Event> for &'a str {
    fn from(val: Event) -> Self {
        match val {
            Event::Message => "message",
            Event::Error => "error",
            Event::GameConfig => "GameConfig",
            Event::PackInfo => "PackInfo",
        }
    }
}

/// Game specific events
pub fn emit_app_context(config: AppContextDto) {
    log::debug!("Transmitting app context of: {:#?}", config);
    emit(Event::GameConfig, config);
}

pub fn emit_pack_info(pack_info: PackInfoDto) {
    emit(Event::PackInfo, pack_info);
}

/// Generic API
pub fn emit_message<S: Serialize + Clone>(message: S) {
    emit(Event::Message, message);
}

pub fn emit_error<S: Serialize + Clone>(message: S) {
    emit(Event::Error, message);
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
    static ref WINDOW: Arc<RwLock<Option<Window>>> = Arc::new(RwLock::new(Option::default()));
}

pub fn window() -> RwLockReadGuard<'static, Option<Window>> {
    WINDOW
        .read()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub fn set_window(window: Window) {
    let mut guard = WINDOW.write().expect("Mutex is poisoned");
    *guard = Some(window);
}
