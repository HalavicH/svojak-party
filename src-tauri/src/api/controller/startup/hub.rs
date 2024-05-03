use tauri::command;
use crate::api::dto::PlayerSetupDto;
use crate::api::mapper::{map_players_to_players_setup_dto};

use crate::core::game_entities::{game, HubStatus};
use crate::hub_comm::common::hub_api::HubType;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;

/// Set hub type to web or serial
#[command]
pub fn set_hub_type(hub_type: HubType) {
    log::debug!("Got request to set hub type: {:?}", hub_type);
    let mut game = game();
    game.select_hub_type(hub_type);
}

/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) -> Result<HubStatus, HubManagerError> {
    let guard = game();
    let result = guard.get_locked_hub_mut().probe(&path);
    match result {
        Ok(status) => {
            log::info!("Hub status: {:?}", status);
            Ok(status)
        }
        Err(error_stack) => {
            log::error!("Can't open port: {:?}", error_stack);
            Err(error_stack.current_context().clone())
        }
    }
}

/// Calls HUB to get all available players
#[command]
pub fn discover_players() -> Result<Vec<PlayerSetupDto>, HubManagerError> {
    log::info!("Discovering terminals");
    let guard = game();
    let mut hub_guard = guard.get_locked_hub_mut();

    let players = hub_guard.discover_players().map_err(|e| {
        log::error!("{:#?}", e);
        e.current_context().clone()
    })?;
    Ok(map_players_to_players_setup_dto(&players))
}