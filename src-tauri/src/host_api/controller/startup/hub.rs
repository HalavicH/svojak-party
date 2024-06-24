use crate::core::game_controller::{game, game_mut};
use crate::hub::hub_api::HubType;
use tauri::command;

/// Set hub type to web or serial
#[command]
pub fn set_hub_type(hub_type: HubType) {
    log::debug!("Got request to set hub type: {:?}", hub_type);
    let mut app = game_mut();
    app.select_hub_type(hub_type);
}

/// Tries to detect hub at given serial port. If successful saves port name
#[command]
pub fn discover_hub(path: String) {
    let mut app = game_mut();
    app.discover_hub_and_players(path);
}

/// Calls HUB to set specific radio channel
#[command]
pub fn set_hw_hub_radio_channel(channel_id: i32) {
    log::info!("Got channel id: {channel_id}");
    game().set_hub_radio_channel(channel_id as u8);
}