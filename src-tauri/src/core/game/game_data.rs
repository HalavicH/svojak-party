use crate::api::dto::{PlayerEndRoundStatsDto, RoundStatsDto};
use crate::api::events::{emit_players, emit_players_by_players_map, emit_question, emit_round};
use crate::core::game::game_ctx::RoundStats;
use crate::core::game_entities::{Player, PlayerState};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub::hub_api::PlayerEvent;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::types::LazyExpect;

#[derive(Debug, Default, Clone)]
pub struct GameData {
    /// Entities
    pack_content: PackContent,
    pub players: HashMap<u8, Player>,
    /// Game State
    current_round_index: usize,
    current_round: Round,
    pub active_player_id: u8,
    // active_player: &Player, // TODO add reference from the players: HashMap<u8, Player>. Invokes lifetime usage
    pub answer_allowed: bool,
    /// Current question
    pub current_question: Question,
    /// Stats
    pub round_stats: RoundStats,
    /// Event frame. Flushed every new question
    pub events: Arc<RwLock<Vec<PlayerEvent>>>,
    pub allow_answer_timestamp: u32,
    pub round_duration_min: i32,
}

impl GameData {
    pub fn set_pack_content(&mut self, pack_content: PackContent) {
        self.pack_content = pack_content;
    }

    pub fn remove_current_question(&mut self) {
        let topic = &self.current_question.topic.clone();
        let price = self.current_question.price;
        log::debug!(
            "Removing question from the pack: topic: {}, price: {}",
            topic,
            price
        );
        self.pop_question(topic, price);
        emit_round((&self.current_round).into());
    }

    fn pop_question(&mut self, topic_name: &str, price: i32) -> Option<Question> {
        let Some(topic) = self.current_round.topics.get_mut(topic_name) else {
            log::error!(
                "Topic with name: {} not found in round with name: {}",
                topic_name,
                self.current_round.name
            );
            return None;
        };

        self.current_round.questions_left -= 1;
        log::debug!("Questions left: {}", self.current_round.questions_left);
        topic.questions.remove(&price)
    }

    pub fn get_question(&self, topic_name: &str, price: i32) -> Option<&Question> {
        let topic = self.current_round.topics.get(topic_name)?;
        topic.questions.get(&price)
    }
}

impl GameData {
    pub fn has_next_round(&self) -> bool {
        log::debug!(
            "Current round index: {}, rounds len: {}",
            self.current_round_index,
            self.pack_content.rounds.len()
        );
        self.current_round_index < self.pack_content.rounds.len()
    }

    pub fn events_clone(&self) -> Arc<RwLock<Vec<PlayerEvent>>> {
        self.events.clone()
    }

    pub fn is_active_player(&self, other: &Player) -> bool {
        other.term_id == self.active_player_id
    }

    pub fn set_next_round(&mut self) {
        let index = self.current_round_index;
        log::debug!("Incrementing round index to: {}", index);
        let round: &Round = self
            .pack_content
            .rounds
            .get(index)
            .expect_lazy(|| format!("Expected to have round with index: {index}"));
        self.current_round = round.clone();
        let round_dto = round.into();
        emit_round(round_dto);
        self.current_round_index += 1;
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
        log::info!(
            "Player with id: {} changes state from {:?} to {:?}",
            id,
            player.state,
            player_state
        );
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

    pub fn to_round_stats_dto(&self) -> RoundStatsDto {
        let stats = &self.round_stats;
        RoundStatsDto {
            roundName: stats.round_name.clone(),
            questionsPlayed: self.current_round.question_count,
            normalQuestionsPlayed: stats.normal_questions_played,
            pigInPokeQuestionPlayed: stats.pip_questions_played,
            totalCorrectAnswers: stats.total_correct_answers,
            totalWrongAnswers: stats.total_wrong_answers,
            totalTries: stats.total_tries,
            roundTimeSec: 666,
            players: self
                .players
                .values()
                .map(|p| PlayerEndRoundStatsDto {
                    id: p.term_id as i32,
                    name: p.name.clone(),
                    score: p.stats.score,
                    playerIconPath: p.icon.clone(),
                    totalAnswers: p.stats.total_tries,
                    answeredCorrectly: p.stats.answered_correctly,
                    answeredWrong: p.stats.answered_wrong,
                })
                .collect(),
        }
    }
}

/// Internal API
impl GameData {
    pub(in crate::core) fn set_active_player_by_id(&mut self, term_id: u8) {
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
