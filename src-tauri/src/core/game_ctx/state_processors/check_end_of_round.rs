use crate::api::dto::{PlayerEndRoundStatsDto, RoundStatsDto};
use crate::api::events::{emit_players_by_players_map, emit_round_stats};
use crate::core::game_ctx::game::{GameCtx, RoundStats};
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
            emit_round_stats(self.to_round_stats_dto());
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

    fn to_round_stats_dto(&self) -> RoundStatsDto {
        // self.data.round_stats
        let stats = &self.data.round_stats;
        RoundStatsDto {
            roundName: stats.round_name.clone(),
            questionsPlayed: self.data.current_round.question_count,
            normalQuestionsPlayed: stats.normal_questions_played,
            pigInPokeQuestionPlayed: stats.pip_questions_played,
            totalCorrectAnswers: stats.total_correct_answers,
            totalWrongAnswers: stats.total_wrong_answers,
            totalTries: stats.total_tries,
            roundTimeSec: 666,
            players: self.data.players
                .iter()
                .map(|(_, p)| {
                    PlayerEndRoundStatsDto {
                        id: p.term_id as i32,
                        name: p.name.clone(),
                        score: p.stats.score,
                        playerIconPath: p.icon.clone(),
                        totalAnswers: p.stats.total_tries,
                        answeredCorrectly: p.stats.answered_correctly,
                        answeredWrong: p.stats.answered_wrong,
                    }
                })
                .collect(),
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
