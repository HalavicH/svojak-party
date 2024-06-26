use crate::core::game::game_data::GameMode;
use crate::core::game_controller::game_mut;
use crate::core::game_entities::GameplayError;
use crate::host_api::events::emit_error;
use error_stack::Report;
use std::time::Duration;
use tauri::command;

fn map_game_error(e: Report<GameplayError>) -> GameplayError {
    emit_error(e.to_string());
    log::error!("{:#?}", e);
    e.current_context().clone()
}

/// Start the game with selected players and game pack
#[command]
pub async fn start_new_game(
    round_duration_min: i32,
    is_qcaf_mode: bool,
) -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    let mut app = game_mut();
    let game_mode = GameMode {
        round_duration: Duration::from_secs(round_duration_min as u64 * 60),
        question_chooser_answers_first: is_qcaf_mode,
        pig_in_poke_enabled: false,
    };
    app.start_new_game(game_mode).map_err(map_game_error)?;
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

#[command]
pub async fn reset_game() {
    let mut app = game_mut();
    app.reset_the_game();
}

#[command]
pub async fn edit_player_score(player_id: i32, score: i32) -> Result<(), GameplayError>{
    game_mut().edit_player_score(player_id, score).map_err(map_game_error)
}