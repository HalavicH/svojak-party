use crate::api::dto::PlayerDto;
use crate::api::events::*;
use crate::core::app_context::{app, app_mut};
use crate::core::game_entities::{Player, PlayerState};
use rocket::http::hyper::body::HttpBody;
use tauri::{command, Window};

/// Dirty hack to capture window handle
#[command]
pub fn init_window_handle(window: Window) {
    set_window(window);
}

/// To get initial app context
#[command]
pub fn request_context_update() {
    app().emit_game_config_locking_hub();
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

    app_mut().update_players(&player_entities)
}

/// Store round duration
#[command]
pub fn save_round_duration(round_minutes: i32) {
    log::info!("Round duration is {round_minutes}");
    app_mut().save_round_duration(round_minutes)
}
