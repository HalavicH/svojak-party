use crate::host_api::events::{emit_game_state_by_name, emit_players_by_game_data};
use crate::core::game::game_ctx::{GameCtx, INVALID_PLAYER_ID};
use crate::core::game::state_structs::{DisplayQuestion, EndQuestion, WaitingForAnswerRequests};
use crate::core::game_entities::GameplayError;
use crate::hub::hub_api::calc_current_epoch_ms;

impl GameCtx<DisplayQuestion> {
    pub fn allow_answer(&mut self) -> Result<GameCtx<WaitingForAnswerRequests>, GameplayError> {
        let game = self;

        let timestamp = calc_current_epoch_ms().expect("Expected to calc epoch successfully");
        game.data.allow_answer_timestamp = timestamp;
        log::info!("Current answer base timestamp: {}", timestamp);

        game.data.active_player_id = INVALID_PLAYER_ID;
        game.update_non_active_player_states("DisplayQuestion");
        emit_players_by_game_data(&game.data);
        game.data.answer_allowed = true;

        emit_game_state_by_name("WaitingForAnswerRequests");
        let game_ctx: GameCtx<WaitingForAnswerRequests> = game.transition();
        Ok(game_ctx)
    }

    pub fn finish_question_preemptively(&mut self) -> Result<GameCtx<EndQuestion>, GameplayError> {
        log::info!("Removing not answered question from the pack");
        self.data.remove_current_question();
        Ok(self.transition())
    }
}
