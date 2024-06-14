use crate::api::events::emit_players_by_players_map;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};

impl GameCtx<WaitingForAnswerRequests> {
    pub fn request_answer_by_player_id(
        &mut self,
        player_id: u8,
    ) -> Result<GameCtx<AnswerAttemptReceived>, GameplayError> {
        self.data.current_player_id = player_id;
        let player = self.current_player_mut();
        player.state = PlayerState::Answering;
        emit_players_by_players_map(&self.data.players);
        Ok(self.transition())
    }
}
