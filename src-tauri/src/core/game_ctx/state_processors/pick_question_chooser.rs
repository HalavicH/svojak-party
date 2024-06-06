use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, mpsc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use error_stack::{Report, ResultExt};
use crate::api::events::emit_message;
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{ChooseQuestion, PickFirstQuestionChooser};
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use crate::hub::hub_api::{calc_current_epoch_ms, PlayerEvent, TermButtonState};

const FASTEST_CLICK_ITERATION_DUR: Duration = Duration::from_secs(1);

struct FastestClickRequest {
    start_time: Instant,
    timeout: Duration,
}

impl FastestClickRequest {
    pub fn new(start_time: Instant, timeout: Duration) -> Self {
        Self {
            start_time,
            timeout,
        }
    }
    pub(crate) fn is_timed_out(&self) -> bool {
        self.start_time.elapsed() >= self.timeout
    }
}

impl GameCtx<PickFirstQuestionChooser> {
    pub fn pick_first_question_chooser(mut self) -> Result<GameCtx<ChooseQuestion>, GameplayError> {
        self.data.allow_answer_timestamp = calc_current_epoch_ms().expect("No epoch today");

        let term_id = match self.get_fastest_click_player_id() {
            Ok(id) => id,
            Err(err) => Err(err.current_context().clone())?,
        };

        emit_message(format!("Fastest player with id: {}", term_id));
        self.data.set_active_player_by_id(term_id);
        self.data
            .set_active_player_state(PlayerState::QuestionChooser);
        Ok(self.transition())
    }

    fn get_fastest_click_player_id(&mut self) -> error_stack::Result<u8, GameplayError> {
        let active_players = self.active_players();

        let id: u8 = match active_players.len() {
            0 => Err(GameplayError::NoActivePlayersLeft)?,
            1 => {
                let keys: Vec<u8> = active_players.keys().cloned()
                    .collect();
                *keys.first()
                    .expect("Expected to have exactly one element")
            }
            _ => {
                let fastest_player_id = self.calc_fastest_click(&active_players)
                    .change_context(GameplayError::HubOperationError)?;
                fastest_player_id
            }
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

    fn calc_fastest_click(&self, active_players: &HashMap<u8, Player>) -> error_stack::Result<u8, GameplayError> {
        let allow_answer_timestamp = self.data.allow_answer_timestamp;
        loop {
            let events = self.data.take_events();
            let filtered = Self::filter_irrelevant_events(allow_answer_timestamp, events, active_players);
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
                    false;
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
