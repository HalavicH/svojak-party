use crate::api::dto::{AppContextDto, PackErrorData, PackInfoDto};
use crate::api::mapper::{get_app_context_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use error_stack::Report;
use tauri::{command, Window};

use crate::api::dto::PlayerSetupDto;
use crate::api::events::{emit, emit_app_context, Event, set_window};
use crate::core::app_context::{app, app_mut};

use crate::game_pack::game_pack_loader::{load_game_pack, GamePackLoadingError};

pub mod hub;
pub mod hw_hub;
pub mod context;

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> AppContextDto {
    log::info!("Fetching config");

    let config = get_app_context_dto();
    log::info!("Config: {:#?}", config);

    config
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

/// Load game pack into the game
#[command]
pub fn get_pack_info(path: String) -> Result<PackInfoDto, PackErrorData> {
    log::info!("Obtained package path: {}", path);

    let result = load_game_pack(path.as_str());

    match result {
        Ok(pack) => {
            app_mut().set_game_pack(pack);

            let pack_info_dto = map_package_to_pack_info_dto(&app().game.pack_content);
            log::info!("Pack info: {:#?}", pack_info_dto);
            Ok(pack_info_dto)
        }
        Err(err) => handle_pack_info_error(path, err),
    }
}

fn handle_pack_info_error(
    path: String,
    err: Report<GamePackLoadingError>,
) -> Result<PackInfoDto, PackErrorData> {
    log::error!("\n{err:?}");

    let stack_trace = format!("{:?}", err);
    let split = stack_trace
        .split("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
        .collect::<Vec<&str>>();
    let &details = split.first().unwrap_or(&"");
    let html_details = ansi_to_html::convert_escaped(details).unwrap_or_else(|e| {
        log::error!("Can't map ASNI to HTML for {}\nError {}", details, e);
        details.to_string()
    });

    let data = PackErrorData {
        path,
        cause: err.current_context().to_string(),
        details: html_details,
    };

    Err(data)
}

#[command]
pub fn save_round_duration(round_minutes: i32) {
    log::info!("Round duration is {round_minutes}");
}

#[command]
pub fn start_the_game() -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    app_mut().start_the_game().map_err(|e| {
        log::error!("{:#?}", e);
        e.current_context().clone()
    })
}
