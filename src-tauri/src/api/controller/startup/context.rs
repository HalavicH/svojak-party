use tauri::{command, Window};
use crate::api::events::{emit_app_context, set_window};
use crate::api::mapper::get_app_context_dto;

#[command]
pub fn init_window_handle(window: Window) {
    set_window(window);
    // emit_app_config();
}

#[command]
pub fn request_context_update() {
    emit_app_context(get_app_context_dto());
}

