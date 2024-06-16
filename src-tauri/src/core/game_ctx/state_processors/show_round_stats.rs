use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{EndTheGame, PickFirstQuestionChooser, ShowRoundStats, StartNextRound};
use crate::core::game_entities::GameplayError;

pub enum RoundStatsResult {
    PickFirstQuestionChooser(GameCtx<PickFirstQuestionChooser>),
    EndTheGame(GameCtx<EndTheGame>),
}

impl GameCtx<ShowRoundStats> {
    pub(crate) fn get_end_round_path(&self) -> Result<RoundStatsResult, GameplayError> {
        let game = &self.data;
        if game.has_next_round() {
            Ok(RoundStatsResult::PickFirstQuestionChooser(self.transition()))
        } else {
            Ok(RoundStatsResult::EndTheGame(self.transition()))
        }
    }
}
