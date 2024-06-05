use crate::api::events::emit_message;
use crate::core::game_ctx::game::Game;
use crate::core::game_ctx::state_structs::{ChooseQuestion, PickFirstQuestionChooser};
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use crate::core::term_event_processing::receive_fastest_click_from_hub;
use crate::hub::hub_api::calc_current_epoch_ms;
use error_stack::ResultExt;

impl Game<PickFirstQuestionChooser> {
    pub fn pick_first_question_chooser(mut self) -> Result<Game<ChooseQuestion>, GameplayError> {
        self.ctx.allow_answer_timestamp = calc_current_epoch_ms().expect("No epoch today");

        let term_id = match self.receive_fastest_click_player_id() {
            Ok(id) => id,
            Err(err) => Err(err.current_context().clone())?,
        };
        emit_message(format!("Fastest player with id: {}", term_id));
        self.ctx.set_active_player_by_id(term_id);
        self.ctx
            .set_active_player_state(PlayerState::QuestionChooser);
        Ok(self.transition())
    }

    fn receive_fastest_click_player_id(&mut self) -> error_stack::Result<u8, GameplayError> {
        let active_players = self.active_players_cnt();
        let active_players_cnt = active_players.len();

        if active_players_cnt == 0 {
            Err(GameplayError::NoActivePlayersLeft)?;
        } else if active_players_cnt == 1 {
            return Ok(active_players
                .first()
                .expect("Expected to have 1 user in list")
                .term_id);
        }

        let receiver = self
            .ctx
            .events
            .as_ref()
            .expect("Expected to have player event queue to be present at this point of game");

        let allow_answer_timestamp = self.ctx.allow_answer_timestamp;
        let fastest_player_id = receive_fastest_click_from_hub(
            receiver,
            allow_answer_timestamp,
            self.ctx.players_map_ref(),
        )
        .change_context(GameplayError::HubOperationError)?;

        log::info!("Fastest click from user: {}", fastest_player_id);

        Ok(fastest_player_id)
    }

    fn active_players_cnt(&mut self) -> Vec<Player> {
        self.ctx
            .players
            .values()
            .filter(|&p| p.allowed_to_click())
            .cloned()
            .collect()
    }
}
