#![allow(unused)]

use crate::hub::web::web_server::{internal_api, player_api};
use rocket::form::FromForm;
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{Build, Rocket};
use std::collections::HashMap;
use std::sync::Mutex;

pub type PlayerId = i32;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(super) struct PlayerWebEvent {
    pub id: PlayerId,
    pub ip: String,
    pub timestamp: u32,
    // TODO: rename to is_button_pressed
    pub state: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PlayerIdentityDto {
    pub id: PlayerId,
    pub name: String,
    pub ip: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(super) struct ServerState {
    pub base_timestamp: u32,
    pub players: HashMap<PlayerId, PlayerIdentityDto>,
    pub events: Vec<PlayerWebEvent>,
}

impl ServerState {
    pub fn has_player_name(&self, name: &String) -> bool {
        let players_with_such_name = self.players.values().filter(|&p| p.name.eq(name)).count();
        players_with_such_name > 0
    }
    pub fn is_known_ip(&self, ip: &String) -> bool {
        self.players.values().filter(|&p| p.ip == *ip).count() > 0
    }

    pub fn add_player(&mut self, mut player: PlayerIdentityDto) -> PlayerIdentityDto {
        let id = (self.players.len() + 1) as PlayerId;
        player.id = id;
        self.players.insert(id, player.clone());
        player
    }

    pub fn update_player(&mut self, mut player: PlayerIdentityDto) -> PlayerId {
        let id = (self.players.len() + 1) as PlayerId;
        player.id = id;
        self.players.insert(id, player);
        id
    }

    pub fn push_event(&mut self, event: PlayerWebEvent) {
        self.events.push(event);
    }

    pub fn player_by_ip(&self, ip: &String) -> Option<PlayerIdentityDto> {
        let players: Vec<PlayerIdentityDto> = self
            .players
            .values()
            .filter(|&p| p.ip == *ip)
            .cloned()
            .collect();

        if players.is_empty() {
            return None;
        }

        Some(players[0].clone())
    }
}

pub(super) type SharedServerState = Mutex<ServerState>;
pub(super) type Persistence<'a> = &'a State<Mutex<ServerState>>;

pub fn setup() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Persistence-API", |rocket| async {
        rocket
            .manage(SharedServerState::default())
            .mount("/", FileServer::from(relative!("static")))
    })
}

// #[rocket::launch]
pub fn launch() -> Rocket<Build> {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8888)).merge(("address", "0.0.0.0")))
        .attach(setup())
        .attach(player_api::setup())
        .attach(internal_api::setup())
}
