use crate::api::events::emit_message;
use crate::core::app_context::{app_mut};
use tauri::command;

use crate::hub_comm::common::hub_api::HubType;

/// Set hub type to web or serial
#[command]
pub fn set_hub_type(hub_type: HubType) {
    log::debug!("Got request to set hub type: {:?}", hub_type);
    emit_message(&format!("Set {:?}", hub_type));
    let mut app = app_mut();
    // send_message(&window, &format!("Set {:?}", hub_type));
    app.select_hub_type(hub_type);
}

/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) {
    app_mut().discover_hub(path)
}
