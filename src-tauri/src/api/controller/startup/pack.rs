use crate::api::dto::PackErrorData;
use crate::api::events::emit_pack_info;
use crate::core::app_context::app_mut;
use crate::game_pack::game_pack_loader::{load_game_pack, GamePackLoadingError};
use error_stack::Report;
use tauri::command;

/// Load game pack into the game
#[command]
pub fn init_game_pack(path: String) -> Result<(), PackErrorData> {
    log::info!("Obtained package path: {}", path);

    let result = load_game_pack(path.as_str());

    match result {
        Ok(pack) => {
            emit_pack_info((&pack.content).into());
            app_mut().set_game_pack(pack);
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
