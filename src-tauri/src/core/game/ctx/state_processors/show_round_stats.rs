use crate::core::game::ctx::game_ctx::GameCtx;
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
            log::info!("There's another round to play");
            return if self.data.alive_players_left() < 2 {
                log::info!("Less than 2 players left. Ending the game.");
                Ok(RoundStatsResult::EndTheGame(self.transition()))
            } else {
                log::info!("More than 2 players left. Starting the next round.");
                Ok(RoundStatsResult::StartNextRound(self.transition()))
            }
        } else {
            log::info!("No more rounds to play. Ending the game");
            Ok(RoundStatsResult::EndTheGame(self.transition()))
        }
    }
}
