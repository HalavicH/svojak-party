use crate::host_api::events::{emit_players_by_players_map, emit_round_stats};
use crate::core::game::game_ctx::GameCtx;
use crate::core::game::state_structs::{CheckEndOfRound, ChooseQuestion, ShowRoundStats};
use crate::core::game_entities::{GameplayError, PlayerState};

pub enum CheckEndOfRoundResult {
    ChooseQuestion(GameCtx<ChooseQuestion>),
    ShowRoundStats(GameCtx<ShowRoundStats>),
}

impl GameCtx<CheckEndOfRound> {
    pub fn check_end_of_round(&mut self) -> Result<CheckEndOfRoundResult, GameplayError> {
        if self.data.current_round_ref().is_round_over() {
            log::info!("Round is over! Transitioning to CalcRoundStats");
            emit_round_stats(self.data.to_round_stats_dto());
            self.kill_players_with_negative_scores();
            Ok(CheckEndOfRoundResult::ShowRoundStats(self.transition()))
        } else {
            let questions_left = self.data.current_round_ref().questions_left;
            log::info!(
                "Round still has {} questions! Transitioning to ChooseQuestion",
                questions_left
            );
            log::debug!("Setting active player state to 'QuestionChooser'");
            self.data
                .set_active_player_state(PlayerState::QuestionChooser);
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
    fn kill_players_with_negative_scores(&mut self) {
        self.data.players.iter_mut().for_each(|(_, p)| {
            if p.stats.score < 0 {
                log::info!(
                    "Player {} has negative score of {}, killing",
                    p.term_id,
                    p.stats.score
                );
                p.state = PlayerState::Dead;
            }
        });
        emit_players_by_players_map(&self.data.players);
    }
}
