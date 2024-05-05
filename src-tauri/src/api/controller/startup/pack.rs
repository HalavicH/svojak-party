use error_stack::Report;
use tauri::command;
use crate::api::dto::{AppContextDto, PackErrorData, PlayerSetupDto};
use crate::api::events::emit_pack_info;
use crate::api::mapper::{get_app_context_dto, map_package_to_pack_info_dto, update_players};
use crate::core::app_context::{app, app_mut};
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use crate::game_pack::game_pack_loader::{GamePackLoadingError, load_game_pack};

/// Provide saved game configuration
#[command]
pub fn fetch_configuration() -> AppContextDto {
    log::info!("Fetching config");

    let config = get_app_context_dto();
    log::info!("Config: {:#?}", config);

    config
}


/// Load game pack into the game
#[command]
pub fn init_game_pack(path: String) -> Result<(), PackErrorData> {
    log::info!("Obtained package path: {}", path);

    let result = load_game_pack(path.as_str());

    match result {
        Ok(pack) => {
            app_mut().set_game_pack(pack);

            let package = &app().game_pack.content;
            log::info!("Pack content: {:#?}", package);
            let pack_info_dto = map_package_to_pack_info_dto(package);
            log::info!("Pack info: {:#?}", pack_info_dto);
            emit_pack_info(pack_info_dto);
            Ok(())
        }
        Err(err) => handle_pack_info_error(path, err),
    }
}

fn handle_pack_info_error(
    path: String,
    err: Report<GamePackLoadingError>,
) -> Result<(), PackErrorData> {
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
