use crate::core::game_ctx::state_structs::*;
use crate::core::game_ctx::GameCtx;
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
}

///// LEGACY
#[derive(Default, Debug, Clone)]
pub struct GameStats {
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}
