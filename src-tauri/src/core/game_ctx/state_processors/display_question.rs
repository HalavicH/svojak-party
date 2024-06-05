use crate::api::events::{emit_game_state_by_name, emit_players_by_game_ctx};
use crate::core::game_ctx::game::{Game, INVALID_PLAYER_ID};
use crate::core::game_ctx::state_structs::{DisplayQuestion, WaitingForAnswerRequests};
use crate::core::game_entities::GameplayError;
use crate::hub::hub_api::calc_current_epoch_ms;

impl Game<DisplayQuestion> {
    pub fn allow_answer(&mut self) -> Result<Game<WaitingForAnswerRequests>, GameplayError> {
        let game = self;

        let timestamp = calc_current_epoch_ms().expect("Expected to calc epoch successfully");
        game.ctx.allow_answer_timestamp = timestamp;
        log::info!("Current answer base timestamp: {}", timestamp);

        game.ctx.active_player_id = INVALID_PLAYER_ID;
        game.update_non_active_player_states("DisplayQuestion");
        emit_players_by_game_ctx(&game.ctx);
        game.ctx.answer_allowed = true;

        emit_game_state_by_name("WaitingForAnswerRequests");
        // game_ctx.game.set_active_player_state(PlayerState::Answering);
        let game_ctx: Game<WaitingForAnswerRequests> = game.transition();
        Ok(game_ctx)
    }
}
