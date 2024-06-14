use error_stack::Report;
use crate::api::events::emit_error;
use crate::core::app_context::app_mut;
use crate::core::game_entities::GameplayError;
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
    let mut app = app_mut();
    app.start_new_game().map_err(map_game_error)?;
    app.pick_first_question_chooser().map_err(map_game_error)?;
    Ok(())
}

/// Select question to be played
#[command]
pub fn select_question(topic: String, price: i32) -> Result<(), GameplayError> {
    let mut app = app_mut();

    app.select_question(&topic, price)?;
    Ok(())
}

/// Allows events from players to be processed
#[command]
pub fn allow_answer() -> Result<(), GameplayError> {
    let mut guard = app_mut();
    guard.allow_answer().map_err(map_game_error)?;
    guard.wait_for_quickest_player_to_click().map_err(map_game_error)
}

/// Provide answer to active question
#[command]
pub fn answer_question(answered_correctly: bool) -> Result<(), GameplayError> {
    log::debug!("Answered correctly: {answered_correctly}");

    app_mut().answer_question(answered_correctly).map_err(map_game_error)
}

/// Finished current question and set's state to 'show answer'
#[command]
pub fn stop_asking_and_show_answer() {
    todo!("Rework");
    app_mut().stop_asking_and_show_answer().map_err(map_game_error);
}

/// Finished current question and set's state to 'show answer'
#[command]
pub fn finish_question() {
    app_mut().finish_question().map_err(map_game_error);
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    log::debug!("Victim id is: {}", victim_id);
}
