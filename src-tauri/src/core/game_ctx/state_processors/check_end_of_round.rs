use crate::api::dto::{PlayerEndRoundStatsDto, RoundStatsDto};
use crate::api::events::{emit_players_by_players_map, emit_round_stats};
use crate::core::game_ctx::game_ctx::{GameCtx, RoundStats};
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, ShowRoundStats, CheckEndOfRound, ChooseQuestion, EndQuestion, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};


pub enum CheckEndOfRoundResult {
    ChooseQuestion(GameCtx<ChooseQuestion>),
    ShowRoundStats(GameCtx<ShowRoundStats>),
}

impl GameCtx<CheckEndOfRound> {
    pub fn check_end_of_round(
        &mut self,
    ) -> Result<CheckEndOfRoundResult, GameplayError> {
        if self.data.current_round.is_over() {
            log::info!("Round is over! Transitioning to CalcRoundStats");
            emit_round_stats(self.data.to_round_stats_dto());
            Ok(CheckEndOfRoundResult::ShowRoundStats(self.transition()))
        } else {
            let questions_left = self.data.current_round.questions_left;
            log::info!("Round still has {} questions! Transitioning to ChooseQuestion", questions_left);
            log::debug!("Setting active player state to 'QuestionChooser'");
            self.data.set_active_player_state(PlayerState::QuestionChooser);
            self.reactivate_inactive_players();
            Ok(CheckEndOfRoundResult::ChooseQuestion(self.transition()))
        }
    }


    fn reactivate_inactive_players(&mut self) {
        let game = &mut self.data;
        game.players.iter_mut().for_each(|(_, p)| {
            if p.state == PlayerState::Inactive {
                p.state = PlayerState::Idle;
            }
        });
        emit_players_by_players_map(&game.players);
    }
}
