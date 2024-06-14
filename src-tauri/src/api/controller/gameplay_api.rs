use crate::api::events::emit_error;
use crate::core::app_context::app_mut;
use crate::core::game_entities::GameplayError;
use tauri::command;

/// Start the game with selected players and game pack
#[command]
pub async fn start_new_game() -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    let mut app = app_mut();
    app.start_new_game().map_err(|e| {
        emit_error(e.to_string());
        log::error!("{:#?}", e);
        e.current_context().clone()
    })?;

    app.pick_first_question_chooser().map_err(|e| {
        emit_error(e.to_string());
        log::error!("{:#?}", e);
        e.current_context().clone()
    })?;

    app.emit_game_config_locking_hub();
    app.emit_game_context();
    Ok(())
}

/// Select question to be played
#[command]
pub fn select_question(topic: String, price: i32) -> Result<(), GameplayError> {
    let mut app = app_mut();

    app.select_question(&topic, price)?;
    app.emit_game_context();
    Ok(())
}

/// Allows events from players to be processed
#[command]
pub fn allow_answer() -> Result<(), GameplayError> {
    let mut guard = app_mut();
    guard.allow_answer().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })?;

    guard.wait_for_quickest_player_to_click().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })
}

/// Provide answer to active question
#[command]
pub fn answer_question(answered_correctly: bool) -> Result<(), GameplayError> {
    log::debug!("Answered correctly: {answered_correctly}");

    app_mut().answer_question(answered_correctly).map_err(|e| {
        log::error!("Failed to answer question: {:?}", e);
        e.current_context().clone()
    })
}

/// Finished current question and set's state to 'show answer'
#[command]
pub fn stop_asking_and_show_answer() {
    todo!("Rework");
    app_mut().stop_asking_and_show_answer().map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    });
}

/// Finished current question and set's state to 'show answer'
#[command]
pub fn finish_question() {
    app_mut().finish_question().map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    });
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    log::debug!("Victim id is: {}", victim_id);
}
