use crate::api::events::emit_players_by_players_map;
use crate::core::game::game_data::GameData;
use crate::core::game::state_structs::*;
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use crate::hub::hub_api::{PlayerEvent, TermButtonState};
use error_stack::ResultExt;
use rocket::yansi::Paint;
use std::any::type_name;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::thread::sleep;
use std::time::Duration;

pub const INVALID_PLAYER_ID: u8 = 0; // TODO: Consider using Option<u8> instead
const FASTEST_CLICK_ITERATION_DUR: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct GameCtx<State = SetupAndLoading> {
    pub(super) state: PhantomData<State>,
    pub(super) data: GameData,
}

impl Default for GameCtx {
    fn default() -> GameCtx<SetupAndLoading> {
        Self {
            state: PhantomData::<SetupAndLoading>,
            data: GameData::default(),
        }
    }
}

/// Common implementation for every state of the `GameContext`
impl<State> GameCtx<State> {
    pub fn transition<T>(&self) -> GameCtx<T> {
        let prev_state = Self::full_type_to_name(&format!("{:?}", self.state));
        let next_state = Self::full_type_to_name(type_name::<T>());
        log::debug!(
            "Game transitions '{}' -> '{}'",
            prev_state.cyan(),
            next_state.green()
        );
        GameCtx {
            state: PhantomData,
            data: self.data.clone(),
        }
    }

    pub fn new_with_game<T>(game: GameData) -> GameCtx<T> {
        GameCtx {
            state: PhantomData,
            data: game,
        }
    }
    pub fn game_mut(&mut self) -> &mut GameData {
        &mut self.data
    }

    pub fn game_ref(&self) -> &GameData {
        &self.data
    }

    fn full_type_to_name(next_state: &str) -> String {
        next_state
            .rsplit("::")
            .next()
            .expect("Expected to have type with :: in path")
            .replace(['"', '>'], "")
    }

    pub(super) fn update_non_active_player_states(&mut self, state_name: &str) {
        let game = &mut self.data;
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
        emit_players_by_players_map(&game.players);
    }
}

/// Player events processor
impl<State> GameCtx<State> {
    pub fn get_fastest_click_player_id(&mut self) -> error_stack::Result<u8, GameplayError> {
        let active_players = self.active_players();

        let id: u8 = match active_players.len() {
            0 => Err(GameplayError::NoActivePlayersLeft)?,
            1 => {
                let keys: Vec<u8> = active_players.keys().cloned().collect();
                *keys.first().expect("Expected to have exactly one element")
            }
            _ => self
                .calc_fastest_click(&active_players)
                .change_context(GameplayError::HubOperationError)?,
        };

        log::info!("Fastest click from user: {}", id);
        Ok(id)
    }

    fn active_players(&mut self) -> HashMap<u8, Player> {
        self.data
            .players
            .iter()
            .filter(|(_, p)| p.allowed_to_click())
            .map(|(id, p)| (*id, p.clone()))
            .collect()
    }

    fn calc_fastest_click(
        &self,
        active_players: &HashMap<u8, Player>,
    ) -> error_stack::Result<u8, GameplayError> {
        let allow_answer_timestamp = self.data.allow_answer_timestamp;
        loop {
            let events = self.data.take_events();
            let filtered =
                Self::filter_irrelevant_events(allow_answer_timestamp, events, active_players);
            if filtered.is_empty() {
                log::debug!("No events after filtering. Waiting for the next iteration");
                sleep(FASTEST_CLICK_ITERATION_DUR);
                continue;
            }

            let sorted = Self::sort_by_timestamp(filtered);
            if let Some(value) = sorted.first() {
                return Ok(value.term_id);
            }
        }
    }

    fn filter_irrelevant_events(
        allow_answer_timestamp: u32,
        events: Vec<PlayerEvent>,
        players: &HashMap<u8, Player>,
    ) -> Vec<PlayerEvent> {
        events
            .iter()
            .filter(|&e| {
                if e.timestamp < allow_answer_timestamp {
                    log::debug!("Answer too early. Skipping: {:?}", e);
                    return false;
                }
                log::debug!("After answer allowed. Event: {:?}", e);
                true
            })
            .filter(|&e| {
                if e.state != TermButtonState::Pressed {
                    log::debug!("Release event. Skipping: {:?}", e);
                    return false;
                }
                log::debug!("Press event - relevant. Event: {:?}", e);
                true
            })
            .filter(|&e| {
                let Some(player) = players.get(&e.term_id) else {
                    log::debug!("Unknown terminal id {} event. Skipping: {:?}", e.term_id, e);
                    return false;
                };
                log::debug!("Player: {} has pressed the button", e.term_id);

                if !player.allowed_to_click() {
                    log::debug!(
                        "Player {} is not allowed to click. Skipping event: {:?}",
                        e.term_id,
                        e
                    );
                    return false;
                }
                true
            })
            .cloned()
            .collect()
    }

    fn sort_by_timestamp(filtered: Vec<PlayerEvent>) -> Vec<PlayerEvent> {
        let mut sorted = filtered;
        sorted.sort_by(|e1, e2| e1.timestamp.cmp(&e2.timestamp));
        sorted
    }
}

///// LEGACY
#[derive(Default, Debug, Clone)]
pub struct RoundStats {
    pub questions_played: i32,
    pub normal_questions_played: i32,
    pub pip_questions_played: i32,
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
    pub round_time: String,
}
