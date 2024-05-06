#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]

use std::fmt::Debug;
use crate::api::dto::{HubConfigDto, PackInfoDto, PlayerDto, PlayersDto, QuestionDto, RoundDto};
use crate::core::app_context::app;
use crate::game_pack::pack_content_entities::Round;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard};
use tauri::Window;
use crate::core::game_entities::{GameState, HubStatus};

#[derive(Debug)]
pub enum Event {
    /// Generic
    Message,
    Error,
    /// Game-specific
    HubConFig,
    Players,
    PackInfo,
    Round,
    Question,
    GameState,
}

/// Impl enum to &str conversion
impl<'a> From<Event> for &'a str {
    fn from(val: Event) -> Self {
        match val {
            // Generic
            Event::Message => "message",
            Event::Error => "error",
            // Game-specific
            Event::HubConFig => "HubConFig",
            Event::Players => "Players",
            Event::PackInfo => "PackInfo",
            Event::Round => "Round",
            Event::Question => "Question",
            Event::GameState => "GameState",
        }
    }
}

pub fn emit<S: Serialize + Clone + Debug>(event: Event, message: S) {
    if let Some(window) = window().as_ref() {
        log::debug!("Emitting event of type: {:?}. Payload: {:#?}", event, message);
        window
            .emit(event.into(), message)
            .map_err(|e| format!("Failed to send message: {}", e))
            .expect("Expected to send message to the front-end")
    }
}

lazy_static::lazy_static! {
    static ref WINDOW: Arc<RwLock<Option<Window>>> = Arc::new(RwLock::new(Option::default()));
}

pub fn window() -> RwLockReadGuard<'static, Option<Window>> {
    WINDOW
        .read()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub fn set_window(window: Window) {
    let mut guard = WINDOW.write().expect("Mutex is poisoned");
    *guard = Some(window);
}

/// Generic API
pub fn emit_message<S: Serialize + Clone + Debug>(message: S) {
    emit(Event::Message, message);
}

pub fn emit_error<S: Serialize + Clone + Debug>(message: S) {
    emit(Event::Error, message);
}

/// Game specific events
// GameState
pub fn emit_hub_config(hub_config: HubConfigDto) {
    emit(Event::HubConFig, hub_config);
}

pub fn emit_players(players: PlayersDto) {
    emit(Event::Players, players);
}

pub fn emit_pack_info(pack_info: PackInfoDto) {
    emit(Event::PackInfo, pack_info);
}

pub fn emit_round(round: RoundDto) {
    emit(Event::Round, round);
}

pub fn emit_question(question: QuestionDto) {
    emit(Event::Round, question);
}

pub fn emit_game_state(game_state: &GameState) {
    emit(Event::Round, game_state.get_state_name());
}
