use crate::api::events::emit_message;
use crate::core::game_ctx::game_ctx::GameCtx;
use crate::core::game_ctx::state_structs::{PickFirstQuestionChooser, SetupAndLoading, StartNextRound};
use crate::core::game_entities::GameplayError;
use crate::game_pack::pack_content_entities::PackContent;

impl GameCtx<SetupAndLoading> {
    pub fn set_round_duration(&mut self, round_duration_min: i32) {
        let ctx = &mut self.data;
        ctx.round_duration_min = round_duration_min;
        emit_message(format!(
            "Selected round duration of: {}",
            ctx.round_duration_min
        ));
    }

    pub fn start(
        &self,
        pack_content: PackContent,
    ) -> Result<GameCtx<StartNextRound>, GameplayError> {
        let mut ctx = self.transition();
        let game = &mut ctx.data;
        game.set_pack_content(pack_content);
        if game.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::NotEnoughPlayers);
        }

        Ok(ctx)
    }
}
