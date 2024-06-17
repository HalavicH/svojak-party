use crate::core::game::game_ctx::GameCtx;
use crate::core::game::state_structs::{EndTheGame, ShowRoundStats, StartNextRound};
use crate::core::game_entities::GameplayError;

pub enum RoundStatsResult {
    StartNextRound(GameCtx<StartNextRound>),
    EndTheGame(GameCtx<EndTheGame>),
}

impl GameCtx<ShowRoundStats> {
    pub(crate) fn get_end_round_path(&self) -> Result<RoundStatsResult, GameplayError> {
        let game = &self.data;
        if game.has_next_round() {
            log::info!("There's another round to play. Starting next round");
            Ok(RoundStatsResult::StartNextRound(self.transition()))
        } else {
            log::info!("No more rounds to play. Ending the game");
            Ok(RoundStatsResult::EndTheGame(self.transition()))
        }
    }
}
