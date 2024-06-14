use crate::api::events::emit_players_by_players_map;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, CalcRoundStats, CheckEndOfRound, ChooseQuestion, EndQuestion, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};


pub enum CheckEndOfRoundResult {
    ChooseQuestion(GameCtx<ChooseQuestion>),
    CalcRoundStats(GameCtx<CalcRoundStats>),
}

impl GameCtx<CheckEndOfRound> {
    pub fn check_end_of_round(
        &mut self,
    ) -> Result<CheckEndOfRoundResult, GameplayError> {
        if self.data.current_round.is_over() {
            log::info!("Round is over! Transitioning to CalcRoundStats");
            Ok(CheckEndOfRoundResult::CalcRoundStats(self.transition()))
        } else {
            let questions_left = self.data.current_round.questions_left;
            log::info!("Round still has {} questions! Transitioning to ChooseQuestion", questions_left);
            Ok(CheckEndOfRoundResult::ChooseQuestion(self.transition()))
        }
    }
}
