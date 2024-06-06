use crate::api::events::emit_players_by_players_map;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};

impl GameCtx<WaitingForAnswerRequests> {
    pub fn request_answer_by_player_id(
        &mut self,
        player_id: u8,
    ) -> Result<GameCtx<AnswerAttemptReceived>, GameplayError> {
        let game = self;
        let player = game
            .data
            .players
            .get_mut(&player_id)
            .ok_or(GameplayError::PlayerNotPresent(player_id))?;
        player.state = PlayerState::Answering;
        emit_players_by_players_map(&game.data.players);
        Ok(game.transition())
    }
}
