use crate::api::dto::{PlayerGameDto, QuestionDataDto, QuestionType, RoundDto, RoundStatsDto};
use crate::api::mapper::*;
use crate::core::game_entities::{game, GameplayError};
use crate::hub_comm::hw::hw_hub_manager::HubManagerError;
use tauri::command;

#[command]
pub fn fetch_players() -> Vec<PlayerGameDto> {
    let vec = map_players_to_player_game_dto(game().fetch_players());
    log::trace!("Players: {:#?}", vec);
    vec
}

#[command]
pub fn fetch_round() -> RoundDto {
    let round_dto = map_round_to_dto(game().get_current_round());
    log::trace!("{round_dto:#?}");
    round_dto
}

#[command]
pub fn get_question_data(topic: String, price: i32) -> Result<QuestionDataDto, GameplayError> {
    let (question, q_num) = game().get_pack_question(&topic, &price).map_err(|e| {
        log::error!("Can't get question data: {:#?}", e);
        e.current_context().clone()
    })?;

    Ok(map_question_to_question_dto(topic, question, q_num))
}

#[command]
pub fn allow_answer() -> Result<(), HubManagerError> {
    game().allow_answer().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn get_fastest_click() -> Result<i32, GameplayError> {
    let id = game().get_fastest_click_player_id().map_err(|e| {
        log::error!("{:?}", e);
        e.current_context().clone()
    })?;
    Ok(id as i32)
}

#[command]
pub fn answer_question(answered_correctly: bool) -> Result<bool, GameplayError> {
    log::debug!("Answered correctly: {answered_correctly}");

    game().answer_question(answered_correctly).map_err(|e| {
        log::error!("Failed to answer question: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn has_next_question() -> bool {
    game().has_next_question()
}

#[command]
pub fn finish_question_prematurely() -> Result<(), GameplayError> {
    game().finish_question_prematurely().map_err(|e|{
        log::error!("Operation failed: {:?}", e);
        e.current_context().clone()
    })
}

#[command]
pub fn init_next_round() {
    game().init_next_round();
}

#[command]
pub fn send_pip_victim(victim_id: i32) {
    log::debug!("Victim id is: {}", victim_id);
}

#[command]
pub fn get_active_player_id() -> i32 {
    game().get_active_player_id() as i32
}

#[command]
pub fn is_allow_answer_required() -> bool {
    game().current.question_type == QuestionType::Normal
}

#[command]
pub fn fetch_round_stats() -> RoundStatsDto {
    game().fetch_round_stats()
}
