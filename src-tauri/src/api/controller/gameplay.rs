use crate::api::dto::{PlayerGameDto, QuestionDataDto, QuestionType, RoundDto, RoundStatsDto};
use crate::api::mapper::*;
use crate::core::app_context::app;
use crate::core::game_entities::GameplayError;
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use tauri::command;

#[command]
pub fn fetch_players() -> Vec<PlayerGameDto> {
    let mut guard = app();
    let players = guard.fetch_players();
    let vec = map_players_to_player_game_dto(players);
    log::trace!("Players: {:#?}", vec);
    vec
}

#[command]
pub fn fetch_round() -> RoundDto {
    let round_dto = map_round_to_dto(app().game.get_current_round());
    log::trace!("{round_dto:#?}");
    round_dto
}

#[command]
pub fn get_question_data(topic: String, price: i32) -> Result<QuestionDataDto, GameplayError> {
    let (question, q_num) = app().get_pack_question(&topic, &price).map_err(|e| {
        log::error!("Can't get question data: {:#?}", e);
        e.current_context().clone()
    })?;

    Ok(map_question_to_question_dto(topic, question, q_num))
}

#[command]
pub fn allow_answer() -> Result<(), HubManagerError> {
    app().allow_answer().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn get_fastest_click() -> Result<i32, GameplayError> {
    let id = app().get_fastest_click_player_id().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })?;
    Ok(id as i32)
}

#[command]
pub fn answer_question(answered_correctly: bool) -> Result<bool, GameplayError> {
    log::debug!("Answered correctly: {answered_correctly}");

    app().answer_question(answered_correctly).map_err(|e| {
        log::error!("Failed to answer question: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn has_next_question() -> bool {
    app().has_next_question()
}

#[command]
pub fn finish_question_prematurely() -> Result<(), GameplayError> {
    app().finish_question_prematurely().map_err(|e| {
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn init_next_round() {
    app().game.load_next_round();
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    log::debug!("Victim id is: {}", victim_id);
}

#[command]
pub fn get_active_player_id() -> i32 {
    app().get_active_player_id() as i32
}

#[command]
pub fn is_allow_answer_required() -> bool {
    app().game.question_type == QuestionType::Normal
}

#[command]
pub fn fetch_round_stats() -> RoundStatsDto {
    app().fetch_round_stats()
}
