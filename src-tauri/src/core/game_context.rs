use crate::api::dto::{QuestionType};
use crate::api::events::emit_message;
use crate::core::game_entities::{GameplayError, OldGameState, Player, PlayerState};
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub_comm::hw::hw_hub_manager::{get_epoch_ms, HubManagerError};
use crate::hub_comm::hw::internal::api_types::TermEvent;
use rocket::serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::{Duration, Instant};
use error_stack::{Report, ResultExt};
use crate::hub_comm::hw::internal::api_types::TermButtonState::Pressed;

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
pub struct Game {
    /// Entities
    pack_content: PackContent,
    players: HashMap<u8, Player>,
    /// Game State
    // game_state: GameState,
    current_round_index: usize,
    active_player_id: u8,
    // active_player: &Player, // TODO add reference from the players: HashMap<u8, Player>. Invokes lifetime usage
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

impl Default for Game {
    fn default() -> Game {
        Self {
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

/// External common-state API
/// Get/Set player operations should be available in any state
/// Round data should be available at any state
impl Game {
    /// Immutable
    pub fn get_players_ref(&self) -> &HashMap<u8, Player> {
        &self.players
    }

    pub fn get_current_round(&self) -> Option<&Round> {
        let idx = self.current_round_index;
        self.pack_content.rounds.get(idx)
    }

    /// Mutable player operations (used for player monitoring my hub)
    pub fn erase_players(&mut self) {
        self.players = HashMap::default();
    }

    pub fn get_players_mut(&mut self) -> &mut HashMap<u8, Player> {
        &mut self.players
    }

    pub fn set_players(&mut self, players: HashMap<u8, Player>) {
        self.players = players;
    }
}

/// Internal API
impl Game {
    fn set_active_player_by_id(&mut self, term_id: u8) {
        log::debug!("Looking for user with id: {}", term_id);
        let player = self.get_player_by_id_mut(&term_id);
        self.active_player_id = player.term_id;
    }
    
    fn get_active_player_mut(&mut self) -> &mut Player {
        let id = self.active_player_id;
        log::debug!("Looking for user with id: {}", id);
        self.get_player_by_id_mut(&id)
    }

    fn get_player_by_id_mut(&mut self, term_id: &u8) -> &mut Player {
        let msg = format!("Expected to have term_id: {} in players map: {:?}", term_id, self.players);
        self.players
            .get_mut(term_id)
            .expect(&msg)
    }

    fn set_active_player_state(&mut self, player_state: PlayerState) {
        let id = self.active_player_id;
        let player = self.get_player_by_id_mut(&id);
        player.state = player_state;
    }
}

#[derive(Debug)]
pub struct GameContext<State = SetupAndLoading> {
    state: PhantomData<State>,
    game: Game,
}


impl Default for GameContext {
    fn default() -> GameContext<SetupAndLoading> {
        Self {
            state: PhantomData::<SetupAndLoading>,
            game: Game::default(),
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
            game: self.game,
        }
    }

    pub fn get_game_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    pub fn get_game_ref(&self) -> &Game {
        &self.game
    }

    fn full_type_to_name(next_state: &str) -> String {
        next_state
            .rsplit("::")
            .next()
            .expect("Expected to have type with :: in path")
            .replace(['"', '>'], "")
    }
}

impl GameContext<SetupAndLoading> {
    pub fn set_round_duration(&mut self, round_duration_min: i32) {
        self.game.round_duration_min = round_duration_min;
    }
    pub fn start(
        self,
        pack_content: PackContent,
        event_rx: Receiver<TermEvent>,
    ) -> Result<GameContext<PickFirstQuestionChooser>, GameplayError> {
        let mut game = self.transition();
        game.game.pack_content = pack_content;
        if game.game.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::NotEnoughPlayers);
        }

        game.game.events = Some(Arc::new(Mutex::new(Box::new(event_rx))));
        Ok(game)
    }
}

impl GameContext<PickFirstQuestionChooser> {
    pub fn pick_first_question_chooser(
        mut self,
    ) -> Result<GameContext<ChooseQuestion>, GameplayError> {
        self.game.allow_answer_timestamp = get_epoch_ms().expect("No epoch today");

        let term_id = match self.get_fastest_click_player_id() {
            Ok(id) => { id }
            Err(err) => { Err(err.current_context().clone())? }
        };
        emit_message(format!("Fastest player with id: {}", term_id));
        self.game.set_active_player_by_id(term_id);
        self.game.set_active_player_state(PlayerState::QuestionChooser);

        Ok(self.transition())
    }

    fn get_fastest_click_player_id(&mut self) -> error_stack::Result<u8, GameplayError> {
        let active_players = self.get_active_players_cnt();
        let active_players_cnt = active_players.len();

        if active_players_cnt == 0 {
            Err(GameplayError::NoActivePlayersLeft)?;
        } else if active_players_cnt == 1 {
            return Ok(active_players
                .first()
                .expect("Expected to have 1 user in list")
                .term_id);
        }

        let fastest_player_id = self
            .get_fastest_click_from_hub()
            .change_context(GameplayError::HubOperationError)?;

        log::info!("Fastest click from user: {}", fastest_player_id);
        // self.click_for_answer_allowed = false; /// ????
        // self.answer_allowed = true;

        Ok(fastest_player_id)
    }

    fn get_fastest_click_from_hub(&mut self) -> error_stack::Result<u8, HubManagerError> {
        let Some(receiver) = &self.game.events else {
            return Err(HubManagerError::NotInitializedError.into());
        };

        let start_time = Instant::now();
        let timeout = Duration::from_secs(10);
        let fastest_click: Option<u8> = None;

        let receiver_guard = receiver.lock().expect("Mutex poisoned");
        loop {
            if start_time.elapsed() >= timeout {
                Err(HubManagerError::NoResponseFromTerminal)?;
            }

            let events = match Self::get_events(&receiver_guard) {
                Ok(events) => events,
                Err(_) => {
                    sleep(Duration::from_millis(100));
                    continue;
                }
            };

            let base_timestamp = self.game.allow_answer_timestamp;
            let mut events: Vec<TermEvent> = events
                .iter()
                .filter(|&e| {
                    if e.timestamp >= base_timestamp {
                        log::info!("After answer allowed. Event {:?}", e);
                        true
                    } else {
                        log::info!("Answer too early. Event {:?}", e);
                        false
                    }
                })
                .cloned()
                .collect();

            events.sort_by(|e1, e2| e1.timestamp.cmp(&e2.timestamp));

            if let Some(value) = self.find_the_fastest_event(&mut events) {
                return value;
            }

            if let Some(fastest_click_id) = fastest_click {
                return Ok(fastest_click_id);
            }

            sleep(Duration::from_secs(1));
        }
    }


    fn find_the_fastest_event(
        &self,
        events: &mut Vec<TermEvent>,
    ) -> Option<error_stack::Result<u8, HubManagerError>> {
        for e in events {
            if e.state != Pressed {
                log::debug!("Release event. Skipping: {:?}", e);
                continue;
            }

            let Some(player) = self.game.players.get(&e.term_id) else {
                log::debug!("Unknown terminal id {} event. Skipping: {:?}", e.term_id, e);
                continue;
            };

            if !player.allowed_to_click() {
                log::debug!(
                    "Player {} is not allowed to click. Skipping: {:?}",
                    e.term_id,
                    e
                );
                continue;
            }

            log::info!("Found the fastest click: {:?}", e);
            return Some(Ok(e.term_id));
        }
        None
    }

    fn get_events(
        receiver: &Receiver<TermEvent>,
    ) -> error_stack::Result<Vec<TermEvent>, HubManagerError> {
        let mut events: Vec<TermEvent> = Vec::new();
        loop {
            match receiver.try_recv() {
                Ok(received_event) => {
                    events.push(received_event);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    log::debug!("Got {} events for now.", events.len());
                    break;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Channel has been disconnected, so we break the loop
                    let report = Report::new(HubManagerError::InternalError)
                        .attach_printable("Pipe disconnected: mpsc::TryRecvError::Disconnected");
                    return Err(report);
                }
            }
        }

        Ok(events)
    }
    fn get_active_players_cnt(&mut self) -> Vec<Player> {
        self.game.players
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
