use crate::api::events::{emit_app_context, set_window};
use crate::api::mapper::{get_app_context_dto, update_players};
use tauri::{command, Window};
use crate::api::dto::PlayerSetupDto;
use crate::core::game_entities::{Player, PlayerState};

#[command]
pub fn init_window_handle(window: Window) {
    set_window(window);
    // emit_app_config();
}

#[command]
pub fn request_context_update() {
    emit_app_context(get_app_context_dto());
}


/// Saves configuration to game context
#[command]
pub fn save_players(players: Vec<PlayerSetupDto>) {
    log::debug!("Updating game context with new config: {players:#?}");

    let player_entities: Vec<Player> = players
        .iter()
        .map(|player| Player {
            icon: player.icon.clone(),
            name: player.name.clone(),
            term_id: player.termId,
            is_used: player.isUsed,
            state: PlayerState::Idle,
            stats: Default::default(),
        })
        .collect();

    log::info!("Converted players: {:#?}", player_entities);

    update_players(&player_entities)
}