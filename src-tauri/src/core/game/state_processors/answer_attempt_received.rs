use crate::core::game::game_ctx::GameCtx;
use crate::core::game::state_structs::{AnswerAttemptReceived, DisplayQuestion, EndQuestion};
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
        let data = &mut self.data;

        let stats = data.current_round_stats_mut();
        stats.total_tries += 1;
        if answered_correctly {
            stats.total_correct_answers += 1;
        } else {
            stats.total_wrong_answers += 1;
        }

        let player_id = data.active_player_id;
        let player = data
            .players
            .get_mut(&player_id)
            .ok_or(GameplayError::PlayerNotPresent(player_id))?;
        if answered_correctly {
            player.answered_correctly(&data.current_question);
        } else {
            player.answered_wrong(&data.current_question);
        }

        log::info!("Answered player stats: {:?}", player);
        Ok(())
    }

    fn no_players_to_answer_left(&self) -> bool {
        self.data.players.iter().all(|(_, p)| !p.can_answer())
    }
}
