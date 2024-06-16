pub mod game;
pub mod game_state;
pub mod state_processors;
pub mod state_structs;

use crate::api::events::{emit_players, emit_players_by_game_data, emit_players_by_players_map, emit_question, emit_round};
use crate::core::game_ctx::game::GameStats;
use crate::core::game_entities::{Player, PlayerState};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub::hub_api::PlayerEvent;
/// Entity which holds the whole game context
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default, Clone)]
pub struct GameData {
    /// Entities
    pack_content: PackContent,
    players: HashMap<u8, Player>,
    /// Game State
    current_round_index: usize,
    current_round: Round,
    active_player_id: u8,
    // active_player: &Player, // TODO add reference from the players: HashMap<u8, Player>. Invokes lifetime usage
    answer_allowed: bool,
    /// Current question
    current_question: Question,
    /// Stats
    round_stats: GameStats,
    /// Event frame. Flushed every new question
    events: Arc<RwLock<Vec<PlayerEvent>>>,
    allow_answer_timestamp: u32,
    round_duration_min: i32,
}

impl GameData {
    pub fn has_next_round(&self) -> bool {
        self.current_round_index + 1 < self.pack_content.rounds.len()
    }

    pub fn events_clone(&self) -> Arc<RwLock<Vec<PlayerEvent>>> {
        self.events.clone()
    }

    pub fn is_active_player(&self, other: &Player) -> bool {
        other.term_id == self.active_player_id
    }

    pub fn set_current_round_by_id(&mut self, index: usize) {
        let round: &Round = self
            .pack_content
            .rounds
            .get(index)
            .expect("Expected to have round with index");
        self.current_round = round.clone();
        let round_dto = round.into();
        emit_round(round_dto);
    }

    pub fn current_player_clone(&self) -> Player {
        let id = self.active_player_id;
        log::debug!("Trying to get player by id: {}", id);
        self.player_by_id(id).clone()
    }
}

/// External api-state API
/// Get/Set player operations should be available in any state
/// Round data should be available at any state
/// Avoid returning mutable references to the internal state to prevent event synchronization issues
impl GameData {
    /// Immutable
    pub fn players_map_ref(&self) -> &HashMap<u8, Player> {
        &self.players
    }

    pub fn players_ref_as_vec(&self) -> Vec<&Player> {
        log::debug!("Players: {:#?}", self.players);
        self.players.values().collect()
    }

    pub fn current_round_ref(&self) -> &Round {
        &self.current_round
    }

    pub fn current_question_ref(&self) -> &Question {
        &self.current_question
    }

    /// Mutable player operations (used for player monitoring by hub)
    pub fn erase_players(&mut self) {
        self.players = HashMap::default();
    }

    pub fn set_players(&mut self, players: HashMap<u8, Player>) {
        emit_players(players.values().map(|p| p.into()).collect());
        self.players = players;
    }

    pub fn set_active_player_state(&mut self, player_state: PlayerState) {
        let id = self.active_player_id;
        let player = self.player_by_id_mut(&id);
        log::info!("Player with id: {} changes state from {:?} to {:?}", id, player.state, player_state);
        player.state = player_state;
        emit_players_by_players_map(&self.players);
    }

    pub fn set_active_player_id(&mut self, term_id: u8) {
        self.active_player_id = term_id;
    }

    pub fn set_current_question(&mut self, question: Question) {
        self.current_question = question;
        emit_question((&self.current_question).into());
    }

    pub fn take_events(&self) -> Vec<PlayerEvent> {
        let mut guard = self.events.write().expect("Expected to get events reader");
        let events_batch = guard.clone();
        guard.clear();
        events_batch
    }
}

/// Internal API
impl GameData {
    pub(super) fn set_active_player_by_id(&mut self, term_id: u8) {
        log::debug!("Looking for user with id: {}", term_id);
        let player = self.player_by_id_mut(&term_id);
        self.active_player_id = player.term_id;
    }

    fn player_by_id_mut(&mut self, term_id: &u8) -> &mut Player {
        let msg = format!(
            "Expected to have term_id: {} in players map: {:?}",
            term_id, self.players
        );
        self.players.get_mut(term_id).expect(&msg)
    }

    fn player_by_id(&self, term_id: u8) -> &Player {
        let msg = format!(
            "Expected to have term_id: {} in players map: {:?}",
            term_id, self.players
        );
        self.players.get(&term_id).expect(&msg)
    }
}
