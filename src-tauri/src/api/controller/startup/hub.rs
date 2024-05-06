use std::ops::Deref;
use crate::api::events::{emit_hub_config, emit_message};
use crate::core::app_context::app_mut;
use crate::hub::hub_api::HubType;
use tauri::command;

/// Set hub type to web or serial
#[command]
pub fn set_hub_type(hub_type: HubType) {
    log::debug!("Got request to set hub type: {:?}", hub_type);
    emit_message(format!("Set {:?}", hub_type));
    let mut app = app_mut();
    emit_message(format!("Set {:?}", hub_type));
    app.select_hub_type(hub_type);
    emit_hub_config(app.hub().deref().into());
}

/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) {
    let mut app = app_mut();
    app.discover_hub_and_players(path);

    emit_hub_config(app.hub().deref().into());
    // app_mut().emit_game_config_locking_hub();
}
