use tauri::command;
use crate::core::game_controller::game;
use crate::player_server::player_server::ps;

pub mod game_ctx;
pub mod player_server;

/// To get initial app context
#[command]
pub fn request_context_update() {
    game().request_context_update();
    ps().request_context_update();
}

#[command]
pub fn is_debug_mode() -> bool {
    std::env::var("SVOJAK_DEBUG_MODE")
        .map(|v| v.to_lowercase().eq("true"))
        .unwrap_or(false)
}