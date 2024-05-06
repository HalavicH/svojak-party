use std::collections::HashMap;
use crate::api::events::{emit_message, emit_players};
use crate::core::app_context::{app_mut, AppContext};
use crate::core::game_entities::{DEFAULT_ICON, Player};

pub fn discover_and_save_players() {
    log::debug!("||| Player polling: new iteration |||");
    let mut app_guard = app_mut();
    let result = {
        let mut guard = app_guard.get_locked_hub_mut();
        guard.discover_players()
    };
    match result {
        Ok(detected_players) => {
            compare_and_merge(&mut app_guard, &detected_players);
        }
        Err(error) => {
            log::error!("Can't discover players: {:?}", error);
        }
    }
    log::debug!("");
}

fn compare_and_merge(app: &mut AppContext, detected_players: &[Player]) {
    let det_pl_cnt = detected_players.len();
    log::debug!("Detected {} players", det_pl_cnt);
    if is_new_players_found(app, detected_players) {
        log::info!("New players found! Merging");
        emit_message(format!(
            "New players detected! Total number: {}",
            det_pl_cnt
        ));
        merge_players(app, detected_players);
        let vec = app.game_state.get_game_ref().players_as_vec();
        emit_players(vec.into_iter().map(|p| p.into()).collect());
    }
}


fn is_new_players_found(app: &AppContext, detected_players: &[Player]) -> bool {
    let players = app.game_state.get_game_ref().get_players_ref();
    if detected_players.len() > players.len() {
        return true;
    }

    let current_players_ids: Vec<u8> = players.keys().cloned().collect();
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

fn merge_players(app: &mut AppContext, detected_players: &[Player]) {
    // TODO: make actual merge instead of simple re-assign
    let players = detected_players
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
    app.game_state.get_game_mut().set_players(players);
}