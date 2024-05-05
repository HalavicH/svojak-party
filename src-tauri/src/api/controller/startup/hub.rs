use crate::api::dto::PlayerSetupDto;
use crate::api::events::emit_message;
use crate::api::mapper::map_players_to_players_setup_dto;
use crate::core::app_context::{app, app_mut};
use tauri::command;

use crate::hub_comm::common::hub_api::HubType;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;

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
pub fn discover_hub(path: String) -> String {
    app_mut().discover_hub(path)
}

/// Calls HUB to get all available players
#[command]
pub fn discover_players() -> Result<Vec<PlayerSetupDto>, HubManagerError> {
    log::info!("Discovering terminals");
    let guard = app();
    let mut hub_guard = guard.get_locked_hub_mut();

    let players = hub_guard.discover_players().map_err(|e| {
        log::error!("{:#?}", e);
        e.current_context().clone()
    })?;
    Ok(map_players_to_players_setup_dto(&players))
}
