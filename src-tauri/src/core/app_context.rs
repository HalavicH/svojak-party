use crate::api::dto::{PlayerEndRoundStatsDto, RoundStatsDto};
use crate::api::events::{emit_app_context, emit_message};
use crate::api::mapper::map_app_context;
use crate::core::game_context::GameContext;
use crate::core::game_entities::{
    GamePackError, GameState, GameplayError, Player, PlayerState, DEFAULT_ICON,
};
use crate::core::game_logic::start_event_listener;
use crate::game_pack::game_pack_entites::GamePack;
use crate::game_pack::pack_content_entities::Question;
use crate::hub_comm::common::hub_api::{HubManager, HubType};
use crate::hub_comm::hw::hw_hub_manager::{get_epoch_ms, HubManagerError, HwHubManager};
use crate::hub_comm::hw::internal::api_types::TermButtonState::Pressed;
use crate::hub_comm::hw::internal::api_types::TermEvent;
use crate::hub_comm::web::web_hub_manager::WebHubManager;
use error_stack::{IntoReport, Report, ResultExt};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

lazy_static::lazy_static! {
    static ref GAME_CONTEXT: Arc<RwLock<AppContext>> = Arc::new(RwLock::new(AppContext::default()));

}

pub fn app_mut() -> RwLockWriteGuard<'static, AppContext> {
    GAME_CONTEXT
        .write()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub fn app() -> RwLockReadGuard<'static, AppContext> {
    GAME_CONTEXT
        .read()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

#[derive(Debug)]
pub struct AppContext {
    // Comm entities
    pub hub_type: HubType,
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    player_poling_thread_handle: Option<JoinHandle<()>>,

    // pub window: Arc<RwLock<Box<Option<Window>>>>,
    // Game entities
    pub game_pack: GamePack,
    pub game: GameContext,

    // TODO: move to game
    pub player_event_listener: Option<Arc<Mutex<Receiver<TermEvent>>>>,
    pub allow_answer_timestamp: Arc<AtomicU32>,
}

unsafe impl Send for AppContext {}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            hub_type: HubType::default(),
            hub: Arc::new(RwLock::new(Box::<HwHubManager>::default())),
            game_pack: GamePack::default(),
            game: GameContext::default(),
            player_event_listener: None,
            allow_answer_timestamp: Arc::new(AtomicU32::default()),
            // window: Arc::new(RwLock::new(Box::<Option<Window>>::default())),
            player_poling_thread_handle: None,
        }
    }
}

/// Field Access API
impl AppContext {
    pub fn drop_hub(&mut self) {
        self.hub = Arc::new(RwLock::new(Box::<HwHubManager>::default()))
    }
    pub fn get_hub_ref(&self) -> &Arc<RwLock<Box<dyn HubManager>>> {
        &self.hub
    }

    pub fn get_unlocked_hub(&self) -> RwLockReadGuard<Box<dyn HubManager>> {
        self.hub
            .read()
            .map_err(|e| {
                Report::new(GameplayError::InternalError)
                    .attach_printable(format!("Can't get HUB for read. {:?}", e))
            })
            .expect("Poisoned")
    }

    pub fn get_locked_hub_mut(&self) -> RwLockWriteGuard<Box<dyn HubManager>> {
        self.hub
            .write()
            .map_err(|e| {
                Report::new(GameplayError::InternalError)
                    .attach_printable(format!("Can't get HUB for write. {:?}", e))
            })
            .expect("Poisoned")
    }

    pub fn set_game_pack(&mut self, pack: GamePack) {
        self.game_pack = pack;
    }
}

/// Setup API
impl AppContext {
    pub fn select_hub_type(&mut self, hub_type: HubType) {
        if self.hub_type == hub_type {
            log::info!("Hub is already set to: {:?}. Nothing to do", hub_type);
            return;
        }

        self.hub_type = hub_type.clone();
        match hub_type {
            HubType::HwHub => {
                log::info!("||| --> Selecting SERIAL hub <---");
                self.hub = Arc::new(RwLock::new(Box::<HwHubManager>::default()))
            }
            HubType::WebHub => {
                log::info!("||| --> Selecting WEB hub <---");
                self.hub = Arc::new(RwLock::new(Box::<WebHubManager>::default()))
            }
        }
        emit_app_context(map_app_context(self, &self.get_locked_hub_mut()));
    }

    pub fn discover_hub(&mut self, path: String) {
        log::debug!(
            "Requested HUB change. Removing players as outdated: {:#?}",
            self.game.players
        );
        self.game.players = HashMap::new();
        let result = self.get_locked_hub_mut().probe(&path);
        match result {
            Ok(_) => self.run_polling_for_players(),
            Err(err) => log::error!("Can't initialize hub on port: {}. Error: {:?}", path, err),
        }
        emit_app_context(map_app_context(self, &self.get_locked_hub_mut()));
    }

    /// Players polling
    fn run_polling_for_players(&mut self) {
        if self.player_poling_thread_handle.is_some() {
            log::info!("Player polling thread already started");
            return;
        }

        log::info!("Initial setup of player polling thread");

        let handle = spawn(move || loop {
            Self::discover_and_save_players();
            sleep(Duration::from_secs(2));
        });

        log::info!("Saving new thread handle");
        self.player_poling_thread_handle = Some(handle)
    }
    fn discover_and_save_players() {
        log::debug!("############# NEW PLAYER POLLING ITERATION ###############");
        let mut app_guard = app_mut();
        let result = {
            let mut guard = app_guard.get_locked_hub_mut();
            guard.discover_players()
        };
        match result {
            Ok(detected_players) => {
                Self::compare_and_merge(&mut app_guard, &detected_players);
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
        if app.is_new_players_found(detected_players) {
            log::info!("New players found! Merging");
            emit_message(format!(
                "New players detected! Total number: {}",
                det_pl_cnt
            ));
            app.merge_players(detected_players);
            emit_app_context(map_app_context(app, &app.get_locked_hub_mut()));
        }
    }

    fn is_new_players_found(&self, detected_players: &[Player]) -> bool {
        if detected_players.len() > self.game.players.len() {
            return true;
        }

        let current_players_ids: Vec<u8> = self.game.players.keys().cloned().collect();
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

    fn merge_players(&mut self, detected_players: &[Player]) {
        // TODO: make actual merge instead of simple re-assign
        self.game.players = detected_players
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
    }
}

/// Game API
#[deprecated]
impl AppContext {
    pub fn finish_game(&mut self) {
        self.game = GameContext::default();
    }
    pub fn start_the_game(&mut self) -> error_stack::Result<(), GameplayError> {
        // Prepare game context
        self.game.pack_content = self.game_pack.content.clone();
        self.update_game_state(GameState::ChooseQuestion);

        if self.game.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::PlayerNotPresent).into_report();
        }

        let (event_tx, event_rx) = mpsc::channel();
        self.player_event_listener = Some(Arc::new(Mutex::new(event_rx)));

        start_event_listener(self.get_hub_ref().clone(), event_tx);

        let ts = get_epoch_ms().expect("No epoch today");
        self.allow_answer_timestamp.swap(ts, Ordering::Relaxed);

        let q_picker_id = match self.get_fastest_click_player_id() {
            Ok(id) => id,
            Err(err) => {
                log::error!("{:#?}", err);

                self.game
                    .players
                    .values()
                    .next()
                    .ok_or(GameplayError::PlayerNotPresent)
                    .into_report()
                    .attach_printable("Can't find any player to play with :D")?
                    .term_id
            }
        };

        self.game.set_active_player_id(q_picker_id);
        let player = self
            .game
            .players
            .get_mut(&self.game.active_player_id())
            .ok_or(GameplayError::PlayerNotPresent)
            .into_report()?;
        player.state = PlayerState::QuestionChooser;
        Ok(())
    }

    #[deprecated]
    pub fn fetch_players(&mut self) -> &HashMap<u8, Player> {
        self.update_non_target_player_states();
        &self.game.players
    }

    #[deprecated]
    pub fn get_pack_question(
        &mut self,
        theme: &String,
        price: &i32,
    ) -> error_stack::Result<(Question, i32), GameplayError> {
        log::info!("Get question from category: {theme}, price: {price}");

        self.game.set_active_player_id(0);
        self.update_non_target_player_states();

        // if *self.current.game_state() != GameState::QuestionChoosing {
        //     return Err(Report::new(GamePackError::QuestionNotPresent)
        //         .change_context(GameplayError::PackElementNotPresent));
        // }

        let (question, question_number) = self
            .get_question(theme, price)
            .change_context(GameplayError::PackElementNotPresent)?;

        self.update_game_state(GameState::DisplayQuestion);

        Ok((question, question_number))
    }

    pub fn remove_question(
        &mut self,
        theme: &String,
        price: &i32,
    ) -> error_stack::Result<(), GamePackError> {
        log::info!("Try to remove question from category: {theme}, price: {price}");
        let round = self.game.get_current_round_mut();
        let theme = round
            .themes
            .get_mut(theme)
            .ok_or(GamePackError::ThemeNotPresent)
            .into_report()
            .attach_printable(format!("Can't find theme: {theme:?}"))?;

        let _ = theme
            .pop_question(price)
            .ok_or(GamePackError::QuestionNotPresent)
            .into_report()
            .attach_printable(format!(
                "Can't find question with price {price:?} in theme: {theme:?}"
            ))?;

        round.questions_left -= 1;
        log::info!("Question left: {}", round.questions_left);
        Ok(())
    }

    pub fn finish_question_prematurely(&mut self) -> error_stack::Result<(), GameplayError> {
        self.game.answer_allowed = false;
        self.allow_answer_timestamp
            .swap(u32::MAX, Ordering::Relaxed);

        self.game.round_stats.total_tries += 1;
        self.game.round_stats.total_wrong_answers += 1;

        let theme = self.game.question_theme.clone();
        let price = self.game.question_price;
        log::info!(">>> Trying to remove question from category: {theme}, price: {price}");

        self.update_game_state(GameState::ChooseQuestion);
        self.update_non_target_player_states();

        self.remove_question(&theme, &price)
            .change_context(GameplayError::PackElementNotPresent)?;
        Ok(())
    }

    pub fn has_next_question(&self) -> bool {
        // self.current.has_next_question
        let has_new_question = self.game.get_current_round().questions_left > 0;
        log::info!("Has new question: {}", has_new_question);
        has_new_question
    }

    pub fn allow_answer(&mut self) -> error_stack::Result<(), HubManagerError> {
        let timestamp = get_epoch_ms()?;
        self.allow_answer_timestamp
            .swap(timestamp, Ordering::Relaxed);
        log::info!("Current answer base timestamp: {timestamp}");

        self.game.set_active_player_id(0);
        self.update_non_target_player_states();
        self.game.click_for_answer_allowed = true;
        Ok(())
    }

    pub fn get_fastest_click_player_id(&mut self) -> error_stack::Result<u8, GameplayError> {
        let players_allowed_to_click_num = self
            .game
            .players
            .values()
            .filter(|&p| p.allowed_to_click())
            .count();
        if players_allowed_to_click_num == 0 {
            let report = Report::new(GameplayError::OperationForbidden)
                .attach_printable("Can't get first click: No players allowed to click left.");
            return Err(report);
        }

        let fastest_player_id = self
            .get_fastest_click_from_hub()
            .change_context(GameplayError::HubOperationError)?;

        log::info!("Fastest click from user: {}", fastest_player_id);
        self.game.click_for_answer_allowed = false;
        self.game.answer_allowed = true;
        self.game.set_active_player_id(fastest_player_id);

        self.game
            .players
            .get_mut(&fastest_player_id)
            .ok_or(Report::new(GameplayError::PlayerNotPresent))
            .attach_printable(format!("Can't find player with id {}", fastest_player_id))?
            .state = PlayerState::FirstResponse;

        Ok(fastest_player_id)
    }

    pub fn get_active_player_id(&self) -> u8 {
        self.game.active_player_id()
    }

    pub fn answer_question(
        &mut self,
        answered_correctly: bool,
    ) -> error_stack::Result<bool, GameplayError> {
        if !self.game.answer_allowed {
            return Err(Report::new(GameplayError::AnswerForbidden));
        }

        self.game.answer_allowed = false;
        self.allow_answer_timestamp
            .swap(u32::MAX, Ordering::Relaxed);

        let active_player_id = self.get_active_player_id();
        log::info!(
            "Active player id: {}. Player ids: {:?}",
            active_player_id,
            self.get_player_keys()
        );

        let response_player = {
            let active_player = self
                .game
                .players
                .get_mut(&active_player_id)
                .ok_or(GameplayError::PlayerNotPresent)?;
            if answered_correctly {
                active_player.stats.correct_num += 1;
                self.game.round_stats.total_correct_answers += 1;
                active_player.stats.score += self.game.question_price;
                active_player.state = PlayerState::AnsweredCorrectly;
            } else {
                active_player.stats.wrong_num += 1;
                active_player.stats.score -= self.game.question_price;
                active_player.state = PlayerState::AnsweredWrong;
            }
            self.game.round_stats.total_tries += 1;
            active_player.stats.total_tries += 1;
            active_player.clone()
        };

        log::info!("Current player stats: {:?}", response_player);

        if self.no_players_to_answer_left() {
            log::info!("Nobody answered question correctly");
            self.game.round_stats.total_wrong_answers += 1;
        }

        let theme = self.game.question_theme.clone();
        let price = self.game.question_price;

        let mut retry = true;
        if answered_correctly || self.no_players_to_answer_left() {
            log::info!("Removing question from the pack");

            retry = false;
            self.update_game_state(GameState::ChooseQuestion);
            self.update_non_target_player_states();

            self.remove_question(&theme, &price)
                .change_context(GameplayError::PackElementNotPresent)?;
        }

        Ok(retry)
    }

    pub fn no_players_to_answer_left(&self) -> bool {
        let players_left = self
            .game
            .players
            .iter()
            .filter(|(_, p)| {
                p.state != PlayerState::Inactive
                    && p.state != PlayerState::Dead
                    && p.state != PlayerState::AnsweredWrong
            })
            .count();
        log::debug!("Players to answer left: {}", players_left);
        players_left == 0
    }

    pub fn fetch_round_stats(&self) -> RoundStatsDto {
        let round = self.game.get_current_round();
        RoundStatsDto {
            roundName: round.name.to_owned(),
            questionNumber: round.question_count,
            normalQuestionNum: round.normal_question_count,
            pigInPokeQuestionNum: round.pip_question_count,
            totalCorrectAnswers: self.game.round_stats.total_correct_answers,
            totalWrongAnswers: self.game.round_stats.total_wrong_answers,
            totalTries: self.game.round_stats.total_tries,
            roundTime: "Not tracked".to_owned(),
            players: self
                .game
                .players
                .values()
                .map(|p| PlayerEndRoundStatsDto {
                    id: p.term_id as i32,
                    name: p.name.to_owned(),
                    score: p.stats.score,
                    playerIconPath: p.icon.to_owned(),
                    totalAnswers: p.stats.total_tries,
                    answeredCorrectly: p.stats.correct_num,
                    answeredWrong: p.stats.wrong_num,
                })
                .collect(),
        }
    }

    fn update_game_state(&mut self, new_state: GameState) {
        log::info!("Game state {:?} -> {:?}", self.game.game_state(), new_state);
        self.game.set_game_state(new_state);
        self.update_non_target_player_states();
    }

    fn get_question(
        &mut self,
        theme: &String,
        price: &i32,
    ) -> error_stack::Result<(Question, i32), GamePackError> {
        let round = self.game.get_current_round_mut();
        let question_number = round.question_count - round.questions_left;

        let theme = round
            .themes
            .get_mut(theme)
            .ok_or(GamePackError::ThemeNotPresent)
            .into_report()
            .attach_printable(format!("Can't find theme: {theme:?}"))?;
        let theme_name = theme.name.clone();

        let question = theme
            .get_question(price)
            .ok_or(GamePackError::QuestionNotPresent)
            .into_report()
            .attach_printable(format!(
                "Can't find question with price {price:?} in theme: {theme:?}"
            ))?
            .clone();

        self.game.question_theme = theme_name;
        self.game.question_type = question.question_type.clone();
        self.game.question_price = question.price;
        Ok((question, question_number))
    }

    fn get_fastest_click_from_hub(&mut self) -> error_stack::Result<u8, HubManagerError> {
        let Some(receiver) = &self.player_event_listener else {
            return Err(HubManagerError::NotInitializedError.into());
        };

        let start_time = Instant::now();
        let timeout = Duration::from_secs(10);
        let fastest_click: Option<u8> = None;

        let receiver_guard = receiver.lock().expect("Mutex poisoned");
        loop {
            if start_time.elapsed() >= timeout {
                return Err(Report::new(HubManagerError::NoResponseFromTerminal));
            }

            let events = match Self::get_events(&receiver_guard) {
                Ok(events) => events,
                Err(_) => {
                    sleep(Duration::from_millis(100));
                    continue;
                }
            };

            let base_timestamp = self.allow_answer_timestamp.load(Ordering::Relaxed);
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

    fn get_player_keys(&self) -> Vec<u8> {
        self.game.players.keys().copied().collect()
    }

    fn update_non_target_player_states(&mut self) {
        let game_state = self.game.game_state().clone();
        let active_id = self.get_active_player_id();

        self.game.players.iter_mut().for_each(|(id, p)| {
            log::debug!(
                "Game state: {:?}. Player: {}:{:?}",
                game_state,
                p.term_id,
                p.state
            );

            if p.term_id == active_id {
                log::debug!("Active player. Skipping");
                return;
            }

            if p.state == PlayerState::AnsweredWrong {
                log::trace!("Player with id {} becomes inactive", id);
                p.state = PlayerState::Inactive;
            }

            if game_state == GameState::ChooseQuestion
                || (p.state != PlayerState::Dead && p.state != PlayerState::Inactive)
            {
                log::trace!("Player with id {} becomes idle", id);
                p.state = PlayerState::Idle;
            }
        });
    }

    #[allow(dead_code)]
    fn kill_players_with_negative_balance(&mut self) {
        self.game.players.iter_mut().for_each(|(_, player)| {
            if player.stats.score < 0 {
                log::info!(
                    "Killing player {:?} because of the negative balance",
                    player
                );
                player.state = PlayerState::Dead;
            }
        });
    }
}

#[cfg(test)]
mod game_entities_test {
    use crate::core::app_context::AppContext;
    use crate::core::game_entities::Player;

    #[test]
    fn test_fastest_click() {
        let mut ctx = AppContext::default();
        ctx.game.players.insert(1, Player::default());
        ctx.game.players.insert(2, Player::default());
        ctx.game.players.insert(3, Player::default());
        ctx.game.players.insert(4, Player::default());
        let i = ctx.get_fastest_click_player_id().expect("Test");
        log::info!("Fastest click from: {i}");
    }
}
