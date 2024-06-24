use serde::Serialize;
use crate::api::dto::PlayerEndRoundStatsDto;
use crate::core::game::game_ctx::GameCtx;
use crate::core::game::state_structs::{EndTheGame, SetupAndLoading};
use crate::core::game_entities::GameplayError;

impl GameCtx<EndTheGame> {
    pub fn calculate_final_results(&self) {

    }
    pub fn end_game(&self) -> Result<GameCtx<SetupAndLoading>, GameplayError> {
        todo!()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FinalResultsDto {
    pub first: PlayerEndRoundStatsDto,
    pub second: PlayerEndRoundStatsDto,
    pub third: PlayerEndRoundStatsDto,
    pub theRest: Vec<PlayerEndRoundStatsDto>,
}
