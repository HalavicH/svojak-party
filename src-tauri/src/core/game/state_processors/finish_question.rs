use crate::core::game::game_ctx::GameCtx;
use crate::core::game::state_structs::{CheckEndOfRound, EndQuestion};
use crate::core::game_entities::{GameplayError, PlayerState};

impl GameCtx<EndQuestion> {
    pub fn finish_question(&mut self) -> Result<GameCtx<CheckEndOfRound>, GameplayError> {
        // Handler for additional things that need to be done when the question is finished
        self.data.set_active_player_state(PlayerState::Idle);
        Ok(self.transition())
    }
}
