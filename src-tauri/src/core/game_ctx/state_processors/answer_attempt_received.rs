use crate::core::game_ctx::game_ctx::GameCtx;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, DisplayQuestion, EndQuestion};
use crate::core::game_entities::GameplayError;

pub enum AnswerQuestionResult {
    EndQuestion(GameCtx<EndQuestion>),
    DisplayQuestion(GameCtx<DisplayQuestion>),
}

impl GameCtx<AnswerAttemptReceived> {
    pub fn answer_question(
        &mut self,
        answered_correctly: bool,
    ) -> Result<AnswerQuestionResult, GameplayError> {
        self.process_stats(answered_correctly)?;
        self.update_non_active_player_states("AnswerAttemptReceived");
        let no_players_to_answer_left = self.no_players_to_answer_left();
        log::debug!(
            "Anwsered correctly: {}, players to answer left: {}",
            answered_correctly,
            no_players_to_answer_left
        );
        if answered_correctly || no_players_to_answer_left {
            log::info!("Removing correctly answered question from the pack");
            self.data.remove_current_question();
            Ok(AnswerQuestionResult::EndQuestion(self.transition()))
        } else {
            Ok(AnswerQuestionResult::DisplayQuestion(self.transition()))
        }
    }

    fn process_stats(&mut self, answered_correctly: bool) -> Result<(), GameplayError> {
        let player_id = self.data.active_player_id;
        let player = self
            .data
            .players
            .get_mut(&player_id)
            .ok_or(GameplayError::PlayerNotPresent(player_id))?;
        if answered_correctly {
            player.answered_correctly(&self.data.current_question);
            self.data.round_stats.total_correct_answers += 1;
        } else {
            player.answered_wrong(&self.data.current_question);
            self.data.round_stats.total_wrong_answers += 1;
        }
        log::info!("Answered player stats: {:?}", player);
        self.data.round_stats.total_tries += 1;
        Ok(())
    }

    fn no_players_to_answer_left(&self) -> bool {
        self.data.players.iter().all(|(_, p)| !p.can_answer())
    }
}
