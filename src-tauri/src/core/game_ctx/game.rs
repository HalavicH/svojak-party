use crate::core::game_ctx::state_structs::*;
use crate::core::game_ctx::GameCtx;
use crate::core::game_entities::PlayerState;
use std::any::type_name;
use std::marker::PhantomData;

pub const INVALID_PLAYER_ID: u8 = 0; // TODO: Consider using Option<u8> instead

#[derive(Debug, Clone)]
pub struct Game<State = SetupAndLoading> {
    pub(super) state: PhantomData<State>,
    pub(super) ctx: GameCtx,
}

impl Default for Game {
    fn default() -> Game<SetupAndLoading> {
        Self {
            state: PhantomData::<SetupAndLoading>,
            ctx: GameCtx::default(),
        }
    }
}

/// Common implementation for every state of the `GameContext`
impl<State> Game<State> {
    pub fn transition<T>(&self) -> Game<T> {
        let prev_state = Self::full_type_to_name(&format!("{:?}", self.state));
        let next_state = Self::full_type_to_name(type_name::<T>());
        log::debug!("Game transitions '{}' -> '{}'", prev_state, next_state);
        Game {
            state: PhantomData,
            ctx: self.ctx.clone(),
        }
    }

    pub fn new_with_game<T>(game: GameCtx) -> Game<T> {
        Game {
            state: PhantomData,
            ctx: game,
        }
    }
    pub fn game_mut(&mut self) -> &mut GameCtx {
        &mut self.ctx
    }

    pub fn game_ref(&self) -> &GameCtx {
        &self.ctx
    }

    fn full_type_to_name(next_state: &str) -> String {
        next_state
            .rsplit("::")
            .next()
            .expect("Expected to have type with :: in path")
            .replace(['"', '>'], "")
    }

    pub(super) fn update_non_active_player_states(&mut self, state_name: &str) {
        let game = &mut self.ctx;
        let active_id = game.active_player_id;

        game.players
            .iter_mut()
            .filter(|(&id, _)| id != active_id) // Filter out active player
            .for_each(|(id, p)| {
                // Logging for debugging purposes
                log::debug!(
                    "Game state: {:?}. Player: {}:{:?}",
                    state_name,
                    p.term_id,
                    p.state
                );

                if p.state == PlayerState::AnsweredWrong {
                    log::trace!("Player with id {} becomes inactive", id);
                    p.state = PlayerState::Inactive;
                }

                if p.state != PlayerState::Dead && p.state != PlayerState::Inactive {
                    log::trace!("Player with id {} becomes idle", id);
                    p.state = PlayerState::Idle;
                }
            });
    }
}

///// LEGACY
#[derive(Default, Debug, Clone)]
pub struct GameStats {
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}
