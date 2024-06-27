use crate::core::game_controller::game_mut;
use crate::core::game_entities::{GameplayError, DEFAULT_ICON};
use crate::host_api::events::emit_message;
use crate::hub::hub_api::HubManager;
use crate::player_server::entities::PsPlayer;
use crate::types::Swap;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use crate::to_factored_ms;

pub fn run_player_discovery_loop(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    players_arc: Arc<RwLock<Box<Vec<PsPlayer>>>>,
) {
    let mut players = vec![];
    loop {
        players = discover_and_save_players(players, &hub);

        players_arc.swap(Box::new(players.clone()));
        sleep(Duration::from_millis(to_factored_ms!(1000)));
    }
}

pub fn discover_and_save_players(
    old_players: Vec<PsPlayer>,
    hub: &Arc<RwLock<Box<dyn HubManager>>>,
) -> Vec<PsPlayer> {
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

fn compare_and_merge_players(
    old_players: Vec<PsPlayer>,
    detected_players: &[PsPlayer],
) -> Vec<PsPlayer> {
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

fn is_change_in_players_detected(players: &[PsPlayer], detected_players: &[PsPlayer]) -> bool {
    if detected_players.len() > players.len() {
        return true;
    }

    let current_players_ids: Vec<i32> = players.iter().map(|p| p.id).collect();
    // emit_message(format!("Current player ids: {current_players_ids:?}"));
    // let vec: Vec<u8> = detected_players.iter().map(|p| p.term_id).collect();
    // emit_message(format!("Detected player ids: {:?}", vec));

    for detected_player in detected_players {
        if !current_players_ids.contains(&detected_player.id) {
            return true;
        }
    }

    false
}

fn merge_players(detected_players: &[PsPlayer]) -> Vec<PsPlayer> {
    // TODO: make actual merge instead of simple re-assign
    let players: Vec<PsPlayer> = detected_players
        .iter()
        .map(|p| PsPlayer {
            id: p.id,
            name: p.name.clone(),
            icon: DEFAULT_ICON.to_string(),
        })
        .collect();

    let result = game_mut().push_new_players(players.clone());
    if let Err(err) = result {
        match err.current_context() {
            GameplayError::OperationForbidden => {
                log::debug!("Inappropriate time to push new players")
            }
            _ => log::error!("Can't push new players: {:?}", err),
        }
    }
    players
}
