use crate::core::game_ctx::game_ctx::GameCtx;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};

impl GameCtx<WaitingForAnswerRequests> {
    pub fn request_answer_by_player_id(
        &mut self,
        player_id: u8,
    ) -> Result<GameCtx<AnswerAttemptReceived>, GameplayError> {
        self.data.set_active_player_id(player_id);
        self.data.set_active_player_state(PlayerState::Answering);
        self.data.answer_allowed = false;
        Ok(self.transition())
    }
}
