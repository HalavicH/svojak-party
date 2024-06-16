use crate::core::game_ctx::game_ctx::GameCtx;
use crate::core::game_ctx::state_structs::{ChooseQuestion, PickFirstQuestionChooser, SetupAndLoading, StartNextRound};
use crate::core::game_entities::GameplayError;

impl GameCtx<StartNextRound> {
    pub fn init_next_round(&mut self) -> Result<GameCtx<PickFirstQuestionChooser>, GameplayError> {
        let game = &mut self.data;
        game.set_next_round();
        Ok(self.transition())
    }
}