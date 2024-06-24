use crate::host_api::events::emit_error;
use crate::core::game_controller::game_mut;
use crate::core::game_entities::GameplayError;
use error_stack::Report;
use tauri::command;

fn map_game_error(e: Report<GameplayError>) -> GameplayError {
    emit_error(e.to_string());
    log::error!("{:#?}", e);
    e.current_context().clone()
}

/// Start the game with selected players and game pack
#[command]
pub async fn start_new_game() -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    let mut app = game_mut();
    app.start_new_game().map_err(map_game_error)?;
    Ok(())
}

/// Select question to be played
#[command]
pub async fn select_question(topic: String, price: i32) -> Result<(), GameplayError> {
    let mut app = game_mut();

    app.select_question(&topic, price)?;
    Ok(())
}

/// Allows events from players to be processed
#[command]
pub async fn allow_answer() -> Result<(), GameplayError> {
    let mut guard = game_mut();
    guard.allow_answer().map_err(map_game_error)?;
    guard
        .wait_for_quickest_player_to_click()
        .map_err(map_game_error)
}

/// Provide answer to active question
#[command]
pub async fn answer_question(answered_correctly: bool) -> Result<(), GameplayError> {
    log::debug!("Answered correctly: {answered_correctly}");

    game_mut()
        .answer_question(answered_correctly)
        .map_err(map_game_error)
}

/// Finished current question and set's state to 'show answer'
#[command]
pub async fn stop_asking_and_show_answer() -> Result<(), GameplayError> {
    game_mut()
        .stop_asking_and_show_answer()
        .map_err(map_game_error)
}

/// Finished current question and set's state to 'show answer'
#[command]
pub async fn finish_question() -> Result<(), GameplayError> {
    game_mut().finish_question().map_err(map_game_error)
}

/// Initiate next round
#[command]
pub async fn init_next_round() -> Result<(), GameplayError> {
    game_mut().process_end_of_round().map_err(map_game_error)
}

#[command]
pub async fn send_pip_victim(victim_id: i32) {
    log::debug!("Victim id is: {}", victim_id);
}

#[command]
pub async fn finish_game() -> Result<(), GameplayError> {
    game_mut().finish_game().map_err(map_game_error)
}
