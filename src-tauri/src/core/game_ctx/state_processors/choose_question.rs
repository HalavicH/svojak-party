use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{ChooseQuestion, DisplayQuestion};
use crate::core::game_entities::{GameplayError, PlayerState};

impl GameCtx<ChooseQuestion> {
    pub fn choose_question(
        &self,
        topic: &str,
        price: i32,
    ) -> Result<GameCtx<DisplayQuestion>, GameplayError> {
        let mut game: GameCtx<DisplayQuestion> = self.transition();
        let data = &mut game.data;
        let question = data
            .current_round
            .pop_question(topic, price)
            .ok_or(GameplayError::PackElementNotPresent)?;
        data.set_current_question(question);

        data.set_active_player_state(PlayerState::Idle);

        log::info!("Picked question! Topic: {}, price: {}", topic, price);
        Ok(game)
    }
}
