#![allow(unused)]

use rgb::{RGB, RGB8};
use rocket::{routes, get, post, Shutdown, Config, State};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::hub_comm::web::web_server::server::{Persistence, PlayerIdentityDto, PlayerId, PlayerEvent};
use rocket::serde::{Deserialize, Serialize};
use rocket::time::macros::time;
use crate::api::dto::PlayerSetupDto;
use crate::hub_comm::hw::internal::api_types::{TermButtonState, TermEvent};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rgb8Dto {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RGB8> for Rgb8Dto {
    fn from(value: RGB8) -> Self {
        Rgb8Dto {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl Rgb8Dto {
    pub fn into_rgb8(&self) -> RGB8 {
        RGB8 {
            r: self.r,
            g: self.g,
            b: self.g,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TimestampDto {
    pub timestamp: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermLightColorDto {
    pub id: PlayerId,
    pub color: Rgb8Dto,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TermFeedbackState {
    pub id: PlayerId,
    pub state: bool,
}

/// INTERNAL API ENDPOINTS
#[allow(non_snake_case)]
pub mod INTERNAL_API {
    pub const GET_PLAYERS: &str = "/players";
    pub const GET_TIMESTAMP: &str = "/timestamp";
    pub const SET_TIMESTAMP: &str = "/timestamp";
    pub const SET_TERM_COLOR: &str = "/term-color";
    pub const SET_FEEDBACK_STATE: &str = "/feedback-state";
    pub const GET_EVENT_QUEUE: &str = "/get-event-queue";
    pub const TAKE_EVENT_QUEUE: &str = "/take-event-queue";
    pub const SHUTDOWN: &str = "/shutdown";
}

#[get("/players", format = "application/json")]
fn get_players(state: Persistence) -> Json<Vec<PlayerIdentityDto>> {
    let guard = state.lock().expect("Poisoned");
    let players = guard.players.clone();
    let players_dto: Vec<PlayerIdentityDto> = players.values().cloned().collect();
    log::info!("Players are: {:?}", players_dto);
    Json::from(players_dto)
}

#[get("/timestamp", format = "application/json")]
fn get_hub_timestamp(state: Persistence) -> Json<TimestampDto> {
    let guard = state.lock().expect("Poisoned");
    let timestamp = TimestampDto {
        timestamp: guard.base_timestamp
    };
    log::info!("Timestamp to retrieve: {:?}", timestamp);
    Json::from(timestamp)
}

#[post("/timestamp", format = "application/json", data = "<timestamp>")]
fn set_hub_timestamp(timestamp: Json<TimestampDto>, state: Persistence) {
    log::info!("Received Timestamp: {:?}", timestamp);
    let mut guard = state.lock().expect("Poisoned");
    guard.base_timestamp = timestamp.timestamp;
}

#[post("/term-color", format = "application/json", data = "<term_color_dto>")]
fn set_term_light_color(term_color_dto: Json<TermLightColorDto>, state: Persistence) {
    log::info!("Received: {:?}", term_color_dto);
    let mut guard = state.lock().expect("Poisoned");

    // TODO: Set player's color
}

#[post("/feedback-state", format = "application/json", data = "<term_feedback>")]
fn set_term_feedback_led(term_feedback: Json<TermFeedbackState>, state: Persistence) {
    log::info!("Received: {:?}", term_feedback);
    let mut guard = state.lock().expect("Poisoned");

    // TODO: Set player's feedback
}

#[get("/get-event-queue", format = "application/json")]
fn get_event_queue(state: Persistence) -> Json<Vec<TermEvent>> {
    let guard = state.lock().expect("Poisoned");
    let events = guard.events.clone();
    let term_events = map_player_events_to_term_events(events);

    log::info!("Events are: {:?}", term_events);
    Json::from(term_events)
}

#[get("/take-event-queue", format = "application/json")]
fn take_event_queue(state: Persistence) -> Json<Vec<TermEvent>> {
    let mut guard = state.lock().expect("Poisoned");
    let events = guard.events.clone();
    let term_events = map_player_events_to_term_events(events);
    guard.events = vec![];

    log::info!("Events are: {:?}", term_events);
    Json::from(term_events)
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}

#[get("/config")]
fn read_config(rocket_config: &Config) -> String {
    format!("{:#?}", rocket_config)
}

fn map_player_events_to_term_events(events: Vec<PlayerEvent>) -> Vec<TermEvent> {
    events.iter()
        .map(|e| TermEvent {
            term_id: e.id,
            timestamp: e.timestamp,
            state: TermButtonState::from(e.state),
        })
        .collect()
}

pub fn setup() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Internal-API", |rocket| async {
        rocket
            .mount("/", routes![
                get_players,
                get_hub_timestamp,
                set_hub_timestamp,
                set_term_light_color,
                set_term_feedback_led,
                get_event_queue,
                take_event_queue,
                shutdown
            ])
    })
}
