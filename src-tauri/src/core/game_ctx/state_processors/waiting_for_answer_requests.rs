use crate::core::game_ctx::game::Game;
use crate::core::game_ctx::state_structs::{AnswerAttemptReceived, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};

impl Game<WaitingForAnswerRequests> {
    pub fn request_answer_by_player_id(
        &mut self,
        player_id: u8,
    ) -> Result<Game<AnswerAttemptReceived>, GameplayError> {
        let game = self;
        let player = game
            .ctx
            .players
            .get_mut(&player_id)
            .ok_or(GameplayError::PlayerNotPresent(player_id))?;
        player.state = PlayerState::Answering;
        Ok(game.transition())
    }
}
