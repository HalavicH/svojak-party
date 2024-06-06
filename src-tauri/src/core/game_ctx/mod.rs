pub mod game;
pub mod game_state;
pub mod state_processors;
pub mod state_structs;

use crate::api::events::emit_round;
use crate::core::game_ctx::game::GameStats;
use crate::core::game_entities::{Player, PlayerState};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub::hub_api::TermEvent;
/// Entity which holds the whole game context
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct GameData {
    /// Entities
    pub(super) pack_content: PackContent,
    pub(super) players: HashMap<u8, Player>,
    /// Game State
    pub(super) current_round_index: usize,
    pub(super) current_round: Round,
    pub(super) active_player_id: u8,
    // active_player: &Player, // TODO add reference from the players: HashMap<u8, Player>. Invokes lifetime usage
    pub(super) answer_allowed: bool,
    /// Current question
    pub(super) current_question: Question,
    /// Stats
    pub(super) round_stats: GameStats,
    pub(super) events: Option<Arc<Mutex<Box<Receiver<TermEvent>>>>>,
    pub(super) allow_answer_timestamp: u32,
    pub(super) round_duration_min: i32,
}

impl GameData {
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
}

/// External api-state API
/// Get/Set player operations should be available in any state
/// Round data should be available at any state
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

    pub fn players_map_mut(&mut self) -> &mut HashMap<u8, Player> {
        &mut self.players
    }

    pub fn set_players(&mut self, players: HashMap<u8, Player>) {
        self.players = players;
    }
}

/// Internal API
impl GameData {
    pub(super) fn set_active_player_by_id(&mut self, term_id: u8) {
        log::debug!("Looking for user with id: {}", term_id);
        let player = self.player_by_id_mut(&term_id);
        self.active_player_id = player.term_id;
    }

    pub(super) fn active_player_mut(&mut self) -> &mut Player {
        let id = self.active_player_id;
        log::debug!("Looking for user with id: {}", id);
        self.player_by_id_mut(&id)
    }

    pub(super) fn player_by_id_mut(&mut self, term_id: &u8) -> &mut Player {
        let msg = format!(
            "Expected to have term_id: {} in players map: {:?}",
            term_id, self.players
        );
        self.players.get_mut(term_id).expect(&msg)
    }

    pub(super) fn set_active_player_state(&mut self, player_state: PlayerState) {
        let id = self.active_player_id;
        let player = self.player_by_id_mut(&id);
        player.state = player_state;
    }
}
