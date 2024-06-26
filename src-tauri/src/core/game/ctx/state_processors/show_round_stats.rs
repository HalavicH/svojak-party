use crate::core::game::ctx::game_ctx::GameCtx;
use crate::core::game::state_structs::{EndTheGame, ShowRoundStats, StartNextRound};
use crate::core::game_entities::GameplayError;

pub enum RoundStatsResult {
    StartNextRound(GameCtx<StartNextRound>),
    EndTheGame(GameCtx<EndTheGame>, EndGameReason),
}

#[derive(Clone, Debug)]
pub enum EndGameReason {
    OnePlayerLeft,
    NoPlayersLeft,
    AllRoundsPlayed,
}

impl GameCtx<ShowRoundStats> {
    pub(crate) fn get_end_round_path(&self) -> Result<RoundStatsResult, GameplayError> {
        let game = &self.data;
        if game.has_next_round() {
            log::info!("There's another round to play");
            let players_left = self.data.alive_players_left();
            return if players_left == 1 {
                log::info!("Only one players left. Ending the game.");
                Ok(RoundStatsResult::EndTheGame(self.transition(), EndGameReason::OnePlayerLeft))
            } else if players_left == 0 {
                log::info!("No players left. Ending the game.");
                Ok(RoundStatsResult::EndTheGame(self.transition(), EndGameReason::NoPlayersLeft))
            } else {
                log::info!("More than 2 players left. Starting the next round.");
                Ok(RoundStatsResult::StartNextRound(self.transition()))
            }
        } else {
            log::info!("No more rounds to play. Ending the game");
            Ok(RoundStatsResult::EndTheGame(self.transition(), EndGameReason::AllRoundsPlayed))
        }
    }
}
