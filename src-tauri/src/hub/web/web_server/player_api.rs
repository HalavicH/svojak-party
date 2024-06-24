#![allow(unused)]

use crate::hub::hub_api::calc_current_epoch_ms;
use crate::hub::web::web_server::server::{
    Persistence, PlayerId, PlayerIdentityDto, PlayerWebEvent,
};
use crate::types::LazyExpect;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{get, post, routes};
use rocket_client_addr::ClientAddr;

#[get("/ip-loopback")]
fn ip_loopback(addr: ClientAddr) -> Value {
    log::info!("Got request from ip: {:?}", addr);

    json!({"ip": addr.ip})
}

#[post("/register", data = "<player>")]
fn register_player(
    player: Json<PlayerIdentityDto>,
    state: Persistence,
) -> Result<Json<PlayerIdentityDto>, Status> {
    log::info!("Got player registration attempt: {:?}", player);

    let mut guard = state.lock().expect("Poisoned");
    let player: PlayerIdentityDto = if guard.is_known_ip(&player.ip) {
        log::info!("Ip collision. Retrieving old");
        guard
            .player_by_ip(&player.ip)
            .expect("Expected to be present")
    } else {
        guard.add_player(player.0)
    };

    Ok(Json::from(player))
}

#[post("/event", format = "application/json", data = "<event>")]
fn process_event(event: Json<PlayerWebEvent>, state: Persistence) -> Result<Value, Status> {
    log::debug!("Received event {:?}", event);

    let mut state_guard = state.lock().expect("Expected to get state lock");

    if !state_guard.is_known_ip(&event.ip) {
        log::warn!("Not known IP: {}", event.ip);
        return Err(Status::Unauthorized);
    }

    if !state_guard.players.contains_key(&event.id) {
        log::warn!("Not known Id: {}", event.id);
        return Err(Status::Unauthorized);
    }

    // TODO: Move to the gameplay
    let color = if event.state { "#00FFFF" } else { "#000000" };

    let mut event = event.0;
    event.timestamp = calc_current_epoch_ms().expect("Failed to get epoch");
    state_guard.push_event(event);

    Ok(json!({"color": color}))
}

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("Player-API", |rocket| async {
        rocket.mount("/", routes![register_player, process_event, ip_loopback])
    })
}
