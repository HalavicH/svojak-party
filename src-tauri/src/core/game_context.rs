use crate::api::dto::{QuestionType};
use crate::api::events::emit_message;
use crate::core::game_entities::{GameplayError, OldGameState, Player, PlayerState};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub_comm::hw::hw_hub_manager::{get_epoch_ms};
use crate::hub_comm::hw::internal::api_types::TermEvent;
use rocket::serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SetupAndLoading {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PickFirstQuestionChooser {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChooseQuestion {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DisplayQuestion {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WaitingForAnswerRequests {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnswerAttemptReceived {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EndQuestion {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CheckEndOfRound {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CalcStatsAndStartNextRound {}

#[derive(Debug)]
pub struct GameContext<State = SetupAndLoading> {
    state: PhantomData<State>,
    /// Entities
    pack_content: PackContent,
    players: HashMap<u8, Player>,
    /// Game State
    // game_state: GameState,
    current_round_index: usize,
    active_player_id: u8,
    // active_player: Player, // TODO
    click_for_answer_allowed: bool,
    answer_allowed: bool,
    /// Current question
    current_question: Question,
    /// Stats
    round_stats: GameStats,
    events: Option<Arc<Mutex<Box<Receiver<TermEvent>>>>>,
    allow_answer_timestamp: u32,
    round_duration_min: i32,
}

impl Default for GameContext {
    fn default() -> GameContext<SetupAndLoading> {
        Self {
            state: PhantomData::<SetupAndLoading>,
            pack_content: PackContent::default(),
            players: HashMap::default(),
            // Default values
            // game_state: Default::default(),
            current_round_index: 0,
            active_player_id: 0,
            click_for_answer_allowed: false,
            answer_allowed: false,
            current_question: Default::default(),
            round_stats: Default::default(),
            events: None,
            allow_answer_timestamp: 0,
            round_duration_min: 0,
        }
    }
}

/// Common implementation for every state of the `GameContext`
impl<State> GameContext<State> {
    pub fn transition<T>(self) -> GameContext<T> {
        let prev_state = Self::full_type_to_name(&format!("{:?}", self.state));
        let next_state = Self::full_type_to_name(type_name::<T>());
        log::debug!("Game transitions '{}' -> '{}'", prev_state, next_state);
        GameContext {
            state: PhantomData,
            pack_content: self.pack_content,
            players: self.players,
            // game_state: self.game_state,
            current_round_index: self.current_round_index,
            active_player_id: self.active_player_id,
            click_for_answer_allowed: self.click_for_answer_allowed,
            answer_allowed: self.answer_allowed,
            current_question: self.current_question,
            round_stats: self.round_stats,
            events: self.events,
            allow_answer_timestamp: self.allow_answer_timestamp,
            round_duration_min: self.round_duration_min,
        }
    }

    fn full_type_to_name(next_state: &str) -> String {
        next_state
            .rsplit("::")
            .next()
            .expect("Expected to have type with :: in path")
            .replace(['"', '>'], "")
    }

    fn set_active_player_by_id(&mut self, term_id: u8) {
        let player = self.get_player_by_id_mut(&term_id);
        self.active_player_id = player.term_id;
    }

    fn get_player_by_id_mut(&mut self, term_id: &u8) -> &mut Player {
        self.players
            .get_mut(term_id)
            .expect("For set_active_player_by_id() it's expected to have valid 'term_id' passed")
    }

    fn set_active_player_state(&mut self, player_state: PlayerState) {
        let id = self.active_player_id;
        let player = self.get_player_by_id_mut(&id);
        player.state = player_state;
    }

    pub fn get_current_round(&self) -> &Round {
        let idx = self.current_round_index;
        &self.pack_content.rounds[idx]
    }
}

impl GameContext<SetupAndLoading> {
    pub fn set_round_duration(&mut self, round_duration_min: i32) {
        self.round_duration_min = round_duration_min;
    }
    pub fn start(
        self,
        pack_content: PackContent,
        event_rx: Receiver<TermEvent>,
    ) -> Result<GameContext<PickFirstQuestionChooser>, GameplayError> {
        let mut game = self.transition();
        game.pack_content = pack_content;
        if game.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::NotEnoughPlayers);
        }

        game.events = Some(Arc::new(Mutex::new(Box::new(event_rx))));
        Ok(game)
    }
}

impl GameContext<PickFirstQuestionChooser> {
    pub fn pick_first_question_chooser(
        mut self,
    ) -> Result<GameContext<ChooseQuestion>, GameplayError> {
        self.allow_answer_timestamp = get_epoch_ms().expect("No epoch today");

        let term_id = self.get_fastest_click_player_id()?;
        emit_message(format!("Fastest player with id: {}", term_id));
        self.set_active_player_by_id(term_id);
        self.set_active_player_state(PlayerState::QuestionChooser);

        Ok(self.transition())
    }

    fn get_fastest_click_player_id(&mut self) -> Result<u8, GameplayError> {
        let active_players = self.get_active_players_cnt();
        let active_players_cnt = active_players.len();

        if active_players_cnt == 0 {
            return Err(GameplayError::NoActivePlayersLeft);
        } else if active_players_cnt == 1 {
            return Ok(active_players
                .first()
                .expect("Expected to have 1 user in list")
                .term_id);
        }

        // let fastest_player_id = self
        //     .get_fastest_click_from_hub()
        //     .change_context(GameplayError::HubOperationError)?;
        //
        // log::info!("Fastest click from user: {}", fastest_player_id);
        // self.game.click_for_answer_allowed = false;
        // self.game.answer_allowed = true;
        // self.game.set_active_player_id(fastest_player_id);
        //
        // self.game
        //     .players
        //     .get_mut(&fastest_player_id)
        //     .ok_or(Report::new(GameplayError::PlayerNotPresent))
        //     .attach_printable(format!("Can't find player with id {}", fastest_player_id))?
        //     .state = PlayerState::FirstResponse;

        Ok(0)
    }

    fn get_active_players_cnt(&mut self) -> Vec<Player> {
        self.players
            .values()
            .filter(|&p| p.allowed_to_click())
            .cloned()
            .collect()
    }
}

impl GameContext<ChooseQuestion> {
    pub fn choose_question(self, topic: String, price: i32) -> GameContext<DisplayQuestion> {
        let context = self.transition();
        // context.set_
        log::info!("Picked question! Topic: {}, price: {}", topic, price);
        context
    }
}

///// LEGACY
#[derive(Default, Debug)]
pub struct GameStats {
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}

#[derive(Default, Debug)]
pub struct OldGameContext {
    /// Entities
    pub pack_content: PackContent,
    pub players: HashMap<u8, Player>,
    /// Game State
    pub game_state: OldGameState,
    pub round_index: usize,
    pub active_player_id: u8,
    pub click_for_answer_allowed: bool,
    pub answer_allowed: bool,
    /// Current question
    pub question_theme: String,
    pub question_price: i32,
    pub question_type: QuestionType,
    /// Stats
    pub round_stats: GameStats,
}

// trait Game {
//     fn start();
//     ///
//     fn select_question(topic: &String, price: &i32) -> Result<(), >;
// }

impl OldGameContext {
    pub fn new(pack_content: PackContent, players: HashMap<u8, Player>) -> Self {
        Self {
            pack_content,
            players,
            ..OldGameContext::default()
        }
    }

    pub fn start(&mut self) {
        // self.game_state = GameState::ChooseQuestion;
    }
}

/// Deprecated API

/// Getters / Setters
impl OldGameContext {
    pub fn active_player_id(&self) -> u8 {
        self.active_player_id
    }
    pub fn set_active_player_id(&mut self, new_id: u8) {
        self.active_player_id = new_id
    }
    pub fn game_state(&self) -> &OldGameState {
        &self.game_state
    }
    pub fn set_game_state(&mut self, game_state: OldGameState) {
        self.game_state = game_state;
    }
}

/// Game API
impl OldGameContext {
    pub fn get_current_round(&self) -> &Round {
        let index = self.round_index;
        let round = self
            .pack_content
            .rounds
            .get(index)
            .expect(&format!("Expected to have round #{}", index));
        round
    }

    pub fn is_already_last_round(&self) -> bool {
        (self.pack_content.rounds.len() - 1) == self.round_index
    }

    pub fn load_next_round(&mut self) {
        if self.is_already_last_round() {
            log::error!("Already final round");
            return;
        }

        self.round_index += 1;
        let index = self.round_index;
        let round: &Round = self
            .pack_content
            .rounds
            .get(index)
            .expect(&format!("Expected to have round #{}", index));
        log::info!("Next round name {}", round.name);

        self.round_stats.total_tries = 0;
        self.round_stats.total_wrong_answers = 0;
        self.round_stats.total_correct_answers = 0;

        if self.is_already_last_round() {
            todo!("Wire kill_players_with_negative_balance");
            // self.kill_players_with_negative_balance();
        }
    }

    pub fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.round_index;
        let round = self
            .pack_content
            .rounds
            .get_mut(index)
            .expect(&format!("Expected to have round #{}", index));
        round
    }
}
