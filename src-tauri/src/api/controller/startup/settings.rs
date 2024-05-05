use crate::api::dto::PlayerDto;
use crate::api::events::{emit_app_context, set_window};
use crate::api::mapper::{get_app_context_dto, update_players};
use crate::core::game_entities::{Player, PlayerState};
use tauri::{command, Window};
use crate::core::app_context::app_mut;

/// Dirty hack to capture window handle
#[command]
pub fn init_window_handle(window: Window) {
    set_window(window);
}

/// To get initial app context
#[command]
pub fn request_context_update() {
    emit_app_context(get_app_context_dto());
}

/// Saves configuration to game context
#[command]
pub fn save_players(players: Vec<PlayerDto>) {
    log::debug!("Updating game context with new config: {players:#?}");

    let player_entities: Vec<Player> = players
        .iter()
        .map(|player| Player {
            icon: player.iconPath.clone(),
            name: player.name.clone(),
            term_id: player.id as u8,
            is_used: player.isUsed,
            state: PlayerState::Idle,
            stats: Default::default(),
        })
        .collect();

    log::info!("Converted players: {:#?}", player_entities);

    update_players(&player_entities)
}

/// Store round duration
#[command]
pub fn save_round_duration(round_minutes: i32) {
    log::info!("Round duration is {round_minutes}");
    app_mut().save_round_duration(round_minutes)
}
