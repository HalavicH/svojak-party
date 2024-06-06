use crate::api::events::emit_message;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{PickFirstQuestionChooser, SetupAndLoading};
use crate::core::game_entities::GameplayError;
use crate::game_pack::pack_content_entities::PackContent;
use crate::hub::hub_api::TermEvent;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

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
        event_rx: Receiver<TermEvent>,
    ) -> Result<GameCtx<PickFirstQuestionChooser>, GameplayError> {
        let mut game = self.transition();
        let ctx = &mut game.data;
        ctx.pack_content = pack_content;
        if ctx.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::NotEnoughPlayers);
        }

        ctx.current_round_index = 0;
        ctx.set_current_round_by_id(0);

        ctx.events = Some(Arc::new(Mutex::new(Box::new(event_rx))));
        Ok(game)
    }
}
