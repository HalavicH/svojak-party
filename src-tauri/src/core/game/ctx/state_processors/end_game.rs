use crate::core::game::ctx::game_ctx::GameCtx;
use crate::core::game::ctx::state_processors::show_round_stats::EndGameReason;
use crate::core::game::game_data::GameData;
use crate::core::game::state_structs::{EndTheGame, SetupAndLoading};
use crate::core::game_entities::{GameplayError, Player, PlayerState, PlayerStats};
use crate::host_api::dto::{EndGameStatsDto, PlayerFinalStatsDto};
use crate::host_api::events::emit_final_results;

impl GameCtx<EndTheGame> {
    pub fn calculate_final_results(&self, reason: EndGameReason) {
        let mut players = self
            .data
            .players_map_ref()
            .values()
            .cloned()
            .collect::<Vec<Player>>();
        players.sort_by(|a, b| b.stats.score.cmp(&a.stats.score));
        let first = players.first().expect("Expected at least one player");
        let second = players.get(1).expect("Expected at least two players");
        let third_opt = players.get(2);
        let the_rest = players
            .iter()
            .skip(3)
            .map(Into::into)
            .collect::<Vec<PlayerFinalStatsDto>>();

        let final_stats = EndGameStatsDto {
            endGameReason: reason,
            first: first.into(),
            second: second.into(),
            third: third_opt.map(Into::into),
            theRest: the_rest,
        };
        emit_final_results(final_stats);
    }

    pub fn finish_game(&self) -> Result<GameCtx<SetupAndLoading>, GameplayError> {
        let mut ctx = self.transition();
        ctx.data = GameData::new(
            ctx.data
                .players_ref_as_vec()
                .iter()
                .map(|&p| Player {
                    state: PlayerState::default(),
                    stats: PlayerStats::default(),
                    ..p.clone()
                })
                .collect(),
            ctx.data.events.clone(),
        );
        Ok(ctx)
    }
}
