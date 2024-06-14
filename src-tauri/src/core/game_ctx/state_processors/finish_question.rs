use crate::api::events::emit_players_by_players_map;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, CheckEndOfRound, EndQuestion, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};

impl GameCtx<EndQuestion> {
    pub fn finish_question(
        &mut self,
    ) -> Result<GameCtx<CheckEndOfRound>, GameplayError> {
        // Handler for additional things that need to be done when the question is finished
        self.current_player_mut().state = PlayerState::QuestionChooser;
        Ok(self.transition())
    }
}
