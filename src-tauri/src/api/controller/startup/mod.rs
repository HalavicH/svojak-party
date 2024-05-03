use crate::api::dto::{ConfigDto, PackErrorData, PackInfoDto};
use crate::api::mapper::{get_config_dto, map_package_to_pack_info_dto, update_players};
use crate::core::game_entities::{game, GameplayError, Player, PlayerState};
use tauri::command;

use crate::api::dto::PlayerSetupDto;

use crate::game_pack::game_pack_loader::{load_game_pack};

pub mod hub;
pub mod hw_hub;

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> ConfigDto {
    log::info!("Fetching config");

    let config = get_config_dto();
    log::info!("Config: {:#?}", config);

    config
}

/// Saves configuration to game context
#[command]
pub fn save_players(players: Vec<PlayerSetupDto>) {
    log::debug!("Updating game context with new config: {players:#?}");

    let player_entities = players
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
            game().game_pack = pack;

            let pack_info_dto = map_package_to_pack_info_dto(&game().game_pack.content);
            log::info!("Pack info: {:#?}", pack_info_dto);
            Ok(pack_info_dto)
        }
        Err(err) => {
            log::error!("\n{err:?}");

            let stack_trace = format!("{:?}", err);
            let split = stack_trace
                .split("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
                .collect::<Vec<&str>>();
            let &details = split.get(0).unwrap_or(&"");
            let html_details = ansi_to_html::convert_escaped(details)
                .unwrap_or_else(|e| {
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
    }
}

#[command]
pub fn save_round_duration(round_minutes: i32) {
    log::info!("Round duration is {round_minutes}");
}

#[command]
pub fn start_the_game() -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    game().start_the_game().map_err(|e| {
        log::error!("{:#?}", e);
        e.current_context().clone()
    })
}
