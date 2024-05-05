use crate::api::dto::QuestionDataDto;
use crate::core::app_context::app_mut;
use crate::core::game_entities::GameplayError;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use tauri::command;

/// Start the game with selected players and game pack
#[command]
pub fn start_the_game() -> Result<(), GameplayError> {
    log::info!("Triggered the game start");
    app_mut().start_new_game().map_err(|e| {
        log::error!("{:#?}", e);
        e.current_context().clone()
    })
}

/// Select question to be played
#[command]
pub fn select_question(topic: String, price: i32) -> Result<QuestionDataDto, GameplayError> {
    todo!("Rework needed");
    // let (question, q_num) = app_mut().get_pack_question(&topic, &price).map_err(|e| {
    //     log::error!("Can't get question data: {:#?}", e);
    //     e.current_context().clone()
    // })?;
    //
    // Ok(map_question_to_question_dto(topic, question, q_num))
}

/// Allows events from players to be processed
#[command]
pub fn allow_answer() -> Result<(), HubManagerError> {
    app_mut().allow_answer().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })
}

/// Provide answer to active question
#[command]
pub fn answer_question(answered_correctly: bool) -> Result<bool, GameplayError> {
    log::debug!("Answered correctly: {answered_correctly}");

    app_mut().answer_question(answered_correctly).map_err(|e| {
        log::error!("Failed to answer question: {:?}", e);
        e.current_context().clone()
    })
}

/// Finished current question and set's state to 'show answer'
#[command]
pub fn finish_question_prematurely_and_show_answer() {
    todo!("Rework");
    app_mut().finish_question_prematurely().map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    });
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    log::debug!("Victim id is: {}", victim_id);
}
