use crate::core::game::ctx::game_ctx::GameCtx;
use crate::core::game::game_data::GameMode;
use crate::core::game::state_structs::{SetupAndLoading, StartNextRound};
use crate::core::game_entities::GameplayError;
use crate::core::game_pack::pack_content_entities::PackContent;
use crate::host_api::events::emit_message;

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
        game_mode: GameMode,
    ) -> Result<GameCtx<StartNextRound>, GameplayError> {
        let mut ctx = self.transition();
        let game = &mut ctx.data;
        game.set_pack_content(pack_content);
        game.game_mode = game_mode;
        if game.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::NotEnoughPlayers);
        }

        Ok(ctx)
    }
}
