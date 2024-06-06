use crate::api::events::emit_message;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{ChooseQuestion, PickFirstQuestionChooser};
use crate::core::game_entities::{GameplayError, PlayerState};
use crate::hub::hub_api::calc_current_epoch_ms;
use std::time::{Duration, Instant};

struct FastestClickRequest {
    start_time: Instant,
    timeout: Duration,
}

impl FastestClickRequest {
    pub fn new(start_time: Instant, timeout: Duration) -> Self {
        Self {
            start_time,
            timeout,
        }
    }
    pub(crate) fn is_timed_out(&self) -> bool {
        self.start_time.elapsed() >= self.timeout
    }
}

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
