use crate::host_api::events::emit_message;
use crate::core::game_controller::game_mut;
use crate::core::game_entities::{Player, DEFAULT_ICON};
use crate::hub::hub_api::HubManager;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

pub fn start_listening_for_players_connection(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
) -> JoinHandle<()> {
    log::info!("Initial setup of player polling thread");

    spawn(move || discovery_loop(hub))
}

fn discovery_loop(hub: Arc<RwLock<Box<dyn HubManager>>>) {
    let mut players = vec![];
    loop {
        players = discover_and_save_players(players, &hub);
        sleep(Duration::from_secs(2));
    }
}

pub fn discover_and_save_players(
    old_players: Vec<Player>,
    hub: &Arc<RwLock<Box<dyn HubManager>>>,
) -> Vec<Player> {
    log::debug!("||| Player polling: new iteration |||");
    let result = {
        let mut guard = hub.write().expect("Expected to get write handle");
        guard.discover_players()
    };
    match result {
        Ok(detected_players) => {
            return compare_and_merge_players(old_players, &detected_players);
        }
        Err(error) => {
            log::error!("Can't discover players: {:?}", error);
        }
    }
    old_players
}

fn compare_and_merge_players(old_players: Vec<Player>, detected_players: &[Player]) -> Vec<Player> {
    let det_pl_cnt = detected_players.len();
    log::debug!("Detected {} players", det_pl_cnt);
    if is_change_in_players_detected(&old_players, detected_players) {
        log::info!("New players found! Merging");
        emit_message(format!(
            "New players detected! Total number: {}",
            det_pl_cnt
        ));
        return merge_players(detected_players);
    }
    old_players
}

fn is_change_in_players_detected(players: &[Player], detected_players: &[Player]) -> bool {
    if detected_players.len() > players.len() {
        return true;
    }

    let current_players_ids: Vec<u8> = players.iter().map(|p| p.term_id).collect();
    // emit_message(format!("Current player ids: {current_players_ids:?}"));
    // let vec: Vec<u8> = detected_players.iter().map(|p| p.term_id).collect();
    // emit_message(format!("Detected player ids: {:?}", vec));

    for detected_player in detected_players {
        if !current_players_ids.contains(&detected_player.term_id) {
            return true;
        }
    }

    false
}

fn merge_players(detected_players: &[Player]) -> Vec<Player> {
    // TODO: make actual merge instead of simple re-assign
    let players: HashMap<u8, Player> = detected_players
        .iter()
        .map(|p| {
            let player = Player {
                name: format!("Player {}", p.term_id),
                icon: DEFAULT_ICON.to_string(),
                ..p.to_owned()
            };
            (p.term_id, player)
        })
        .collect();

    let players_v = players.values().cloned().collect();
    let mut app = game_mut();
    app.game_state.game_mut().set_players(players);
    players_v
}
