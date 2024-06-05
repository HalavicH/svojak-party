use crate::api::events::{emit_game_state_by_name, emit_players_by_game_ctx};
use crate::core::game_ctx::game::{Game, INVALID_PLAYER_ID};
use crate::core::game_ctx::state_structs::{DisplayQuestion, WaitingForAnswerRequests};
use crate::core::game_entities::{GameplayError, PlayerState};
use crate::hub::hub_api::calc_current_epoch_ms;

impl Game<DisplayQuestion> {
    pub fn allow_answer(&mut self) -> Result<Game<WaitingForAnswerRequests>, GameplayError> {
        let game = self;

        let timestamp = calc_current_epoch_ms().expect("Expected to calc epoch successfully");
        game.ctx.allow_answer_timestamp = timestamp;
        log::info!("Current answer base timestamp: {}", timestamp);

        game.ctx.active_player_id = INVALID_PLAYER_ID;
        game.update_non_active_player_states();
        emit_players_by_game_ctx(&game.ctx);
        game.ctx.answer_allowed = true;

        emit_game_state_by_name("WaitingForAnswerRequests");
        // game_ctx.game.set_active_player_state(PlayerState::Answering);
        let game_ctx: Game<WaitingForAnswerRequests> = game.transition();
        Ok(game_ctx)
    }

    /// For DisplayQuestion state
    fn update_non_active_player_states(&mut self) {
        let game_state = "DisplayQuestion";
        let game = &mut self.ctx;
        let active_id = game.active_player_id;

        game.players
            .iter_mut()
            .filter(|(&id, _)| id != active_id)
            .for_each(|(id, p)| {
                // Logging for debugging purposes
                log::debug!(
                    "Game state: {:?}. Player: {}:{:?}",
                    game_state,
                    p.term_id,
                    p.state
                );

                if p.state == PlayerState::AnsweredWrong {
                    log::trace!("Player with id {} becomes inactive", id);
                    p.state = PlayerState::Inactive;
                }

                if p.state != PlayerState::Dead && p.state != PlayerState::Inactive {
                    log::trace!("Player with id {} becomes idle", id);
                    p.state = PlayerState::Idle;
                }
            });
    }
}
