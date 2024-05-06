use crate::api::events::{emit_message, emit_round};
use crate::core::game_entities::{GameplayError, Player, PlayerState};
use crate::core::term_event_processing::receive_fastest_click_from_hub;
use crate::game_pack::pack_content_entities::{PackContent, Question, Round};
use crate::hub::hub_api::{calc_current_epoch_ms, TermEvent};
use error_stack::ResultExt;
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

#[derive(Debug, Default)]
pub struct Game {
    /// Entities
    pack_content: PackContent,
    players: HashMap<u8, Player>,
    /// Game State
    // game_state: GameState,
    current_round_index: usize,
    current_round: Round,
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

impl Game {
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
impl Game {
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

    /// Mutable player operations (used for player monitoring my hub)
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
impl Game {
    fn set_active_player_by_id(&mut self, term_id: u8) {
        log::debug!("Looking for user with id: {}", term_id);
        let player = self.player_by_id_mut(&term_id);
        self.active_player_id = player.term_id;
    }

    fn active_player_mut(&mut self) -> &mut Player {
        let id = self.active_player_id;
        log::debug!("Looking for user with id: {}", id);
        self.player_by_id_mut(&id)
    }

    fn player_by_id_mut(&mut self, term_id: &u8) -> &mut Player {
        let msg = format!(
            "Expected to have term_id: {} in players map: {:?}",
            term_id, self.players
        );
        self.players.get_mut(term_id).expect(&msg)
    }

    fn set_active_player_state(&mut self, player_state: PlayerState) {
        let id = self.active_player_id;
        let player = self.player_by_id_mut(&id);
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

    pub fn game_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    pub fn game_ref(&self) -> &Game {
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
        emit_message(format!(
            "Selected round duration of: {}",
            self.game.round_duration_min
        ));
    }

    pub fn start(
        self,
        pack_content: PackContent,
        event_rx: Receiver<TermEvent>,
    ) -> Result<GameContext<PickFirstQuestionChooser>, GameplayError> {
        let mut game_ctx = self.transition();
        game_ctx.game.pack_content = pack_content;
        if game_ctx.game.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::NotEnoughPlayers);
        }

        game_ctx.game.current_round_index = 0;
        game_ctx.game.set_current_round_by_id(0);

        game_ctx.game.events = Some(Arc::new(Mutex::new(Box::new(event_rx))));
        Ok(game_ctx)
    }
}

impl GameContext<PickFirstQuestionChooser> {
    pub fn pick_first_question_chooser(
        mut self,
    ) -> Result<GameContext<ChooseQuestion>, GameplayError> {
        self.game.allow_answer_timestamp = calc_current_epoch_ms().expect("No epoch today");

        let term_id = match self.receive_fastest_click_player_id() {
            Ok(id) => id,
            Err(err) => Err(err.current_context().clone())?,
        };
        emit_message(format!("Fastest player with id: {}", term_id));
        self.game.set_active_player_by_id(term_id);
        self.game
            .set_active_player_state(PlayerState::QuestionChooser);
        Ok(self.transition())
    }

    fn receive_fastest_click_player_id(&mut self) -> error_stack::Result<u8, GameplayError> {
        let active_players = self.active_players_cnt();
        let active_players_cnt = active_players.len();

        if active_players_cnt == 0 {
            Err(GameplayError::NoActivePlayersLeft)?;
        } else if active_players_cnt == 1 {
            return Ok(active_players
                .first()
                .expect("Expected to have 1 user in list")
                .term_id);
        }

        let receiver = self
            .game
            .events
            .as_ref()
            .expect("Expected to have player event queue to be present at this point of game");

        let allow_answer_timestamp = self.game.allow_answer_timestamp;
        let fastest_player_id = receive_fastest_click_from_hub(
            receiver,
            allow_answer_timestamp,
            self.game.players_map_ref(),
        )
        .change_context(GameplayError::HubOperationError)?;

        log::info!("Fastest click from user: {}", fastest_player_id);
        // self.click_for_answer_allowed = false; /// ????
        // self.answer_allowed = true;

        Ok(fastest_player_id)
    }

    fn active_players_cnt(&mut self) -> Vec<Player> {
        self.game
            .players
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
