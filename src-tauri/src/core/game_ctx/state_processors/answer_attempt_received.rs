use crate::core::game_ctx::game::Game;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, DisplayQuestion, EndQuestion};
use crate::core::game_entities::GameplayError;

pub enum AnswerQuestionResult {
    EndQuestion(Game<EndQuestion>),
    DisplayQuestion(Game<DisplayQuestion>),
}

impl Game<AnswerAttemptReceived> {
    pub fn answer_question(
        &mut self,
        answered_correctly: bool,
    ) -> Result<AnswerQuestionResult, GameplayError> {
        self.process_stats(answered_correctly)?;
        self.update_non_active_player_states("AnswerAttemptReceived");
        if answered_correctly || !self.no_players_to_answer_left() {
            log::info!("Removing question from the pack");
            self.remove_current_question();
            Ok(AnswerQuestionResult::EndQuestion(self.transition()))
        } else {
            Ok(AnswerQuestionResult::DisplayQuestion(self.transition()))
        }
    }

    fn process_stats(&mut self, answered_correctly: bool) -> Result<(), GameplayError> {
        let player_id = self.ctx.active_player_id;
        let player = self
            .ctx
            .players
            .get_mut(&player_id)
            .ok_or(GameplayError::PlayerNotPresent(player_id))?;
        if answered_correctly {
            player.answered_correctly(&self.ctx.current_question);
            self.ctx.round_stats.total_correct_answers += 1;
        } else {
            player.answered_wrong(&self.ctx.current_question);
            self.ctx.round_stats.total_wrong_answers += 1;
        }
        log::info!("Answered player stats: {:?}", player);
        self.ctx.round_stats.total_tries += 1;
        Ok(())
    }

    fn no_players_to_answer_left(&self) -> bool {
        self.ctx.players.iter().all(|(_, p)| !p.can_answer())
    }

    fn remove_current_question(&mut self) {
        let round = &mut self.ctx.current_round;
        round.pop_question(
            &self.ctx.current_question.topic,
            self.ctx.current_question.price,
        );
    }
}
