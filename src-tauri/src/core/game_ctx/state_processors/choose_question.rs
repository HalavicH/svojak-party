use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{ChooseQuestion, DisplayQuestion};
use crate::core::game_entities::GameplayError;

impl GameCtx<ChooseQuestion> {
    pub fn choose_question(
        &self,
        topic: &str,
        price: i32,
    ) -> Result<GameCtx<DisplayQuestion>, GameplayError> {
        let mut game: GameCtx<DisplayQuestion> = self.transition();
        let ctx = &mut game.data;
        ctx.current_question = ctx
            .current_round
            .pop_question(topic, price)
            .ok_or(GameplayError::PackElementNotPresent)?;
        log::info!("Picked question! Topic: {}, price: {}", topic, price);
        Ok(game)
    }
}
