use crate::host_api::events::emit_message;
use crate::core::game::game_ctx::GameCtx;
use crate::core::game::state_structs::{ChooseQuestion, PickFirstQuestionChooser};
use crate::core::game_entities::{GameplayError, PlayerState};
use crate::hub::hub_api::calc_current_epoch_ms;

impl GameCtx<PickFirstQuestionChooser> {
    pub fn pick_first_question_chooser(
        &mut self,
    ) -> Result<GameCtx<ChooseQuestion>, GameplayError> {
        self.data.allow_answer_timestamp = calc_current_epoch_ms().expect("No epoch today");

        let term_id = match self.get_fastest_click_player_id() {
            Ok(id) => id,
            Err(err) => Err(err.current_context().clone())?,
        };

        emit_message(format!("Fastest player with id: {}", term_id));
        self.data.set_active_player_by_id(term_id);
        self.data
            .set_active_player_state(PlayerState::QuestionChooser);
        Ok(self.transition())
    }
}
