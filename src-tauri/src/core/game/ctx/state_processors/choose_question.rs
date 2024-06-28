use crate::core::game::ctx::game_ctx::GameCtx;
use crate::core::game::state_structs::{
    AnswerAttemptReceived, ChooseQuestion, DisplayQuestion, WaitingForAnswerRequests,
};
use crate::core::game_entities::{GameplayError, PlayerState};

pub enum ChooseQuestionResult {
    DisplayQuestion(GameCtx<DisplayQuestion>),
    AnswerAttemptReceived(GameCtx<AnswerAttemptReceived>),
}

impl GameCtx<ChooseQuestion> {
    pub fn choose_question(
        &self,
        topic: &str,
        price: i32,
    ) -> Result<ChooseQuestionResult, GameplayError> {
        let mut ctx: GameCtx<DisplayQuestion> = self.transition();
        let data = &mut ctx.data;
        let question = data
            .get_question(topic, price)
            .map_err(Into::<GameplayError>::into)?;

        data.set_current_question(question.clone());

        log::info!("Picked question! Topic: {}, price: {}", topic, price);

        let player = data.active_player_id;
        if data.game_mode.question_chooser_answers_first {
            let mut ctx: GameCtx<WaitingForAnswerRequests> = ctx.transition();
            let ctx = ctx.request_answer_by_player_id(player)?;
            Ok(ChooseQuestionResult::AnswerAttemptReceived(ctx))
        } else {
            data.set_active_player_state(PlayerState::Idle);
            Ok(ChooseQuestionResult::DisplayQuestion(ctx))
        }
    }
}
