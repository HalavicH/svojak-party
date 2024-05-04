use crate::api::dto::{PlayerStatsDto, RoundStatsDto};
use crate::core::game_entities::{
    GameContext, GamePackError, GameState, GameplayError, Player, PlayerState,
};
use crate::core::game_logic::start_event_listener;
use crate::game_pack::pack_content_entities::{Question};
use crate::hub_comm::common::hub_api::{HubManager, HubType};
use crate::hub_comm::hw::hw_hub_manager::{get_epoch_ms, HubManagerError, HwHubManager};
use crate::hub_comm::hw::internal::api_types::TermButtonState::Pressed;
use crate::hub_comm::hw::internal::api_types::TermEvent;
use crate::hub_comm::web::web_hub_manager::WebHubManager;
use error_stack::{IntoReport, Report, ResultExt};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tauri::Window;
use crate::api::events::emit_app_context;
use crate::api::mapper::map_app_context;
use crate::game_pack::game_pack_entites::GamePack;

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
    // pub window: Arc<RwLock<Box<Option<Window>>>>,
    // Game entities
    pub game_pack: GamePack,
    pub game: GameContext,

    pub players: HashMap<u8, Player>,
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
            players: HashMap::default(),
            game_pack: GamePack::default(),
            game: GameContext::default(),
            player_event_listener: None,
            allow_answer_timestamp: Arc::new(AtomicU32::default()),
            // window: Arc::new(RwLock::new(Box::<Option<Window>>::default())),
        }
    }
}

impl AppContext {
    /// Setup API
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
        emit_app_context(map_app_context(self));
    }
    pub fn discover_hub(&mut self, path: String) -> String{
        let result = self.get_locked_hub_mut().probe(&path);
        emit_app_context(map_app_context(self));
        match result {
            Ok(()) => "Detected".to_string(),
            Err(error_stack) => {
                log::error!("Can't open port: {:?}", error_stack);
                let error_case = error_stack.current_context().clone();
                match error_case {
                    HubManagerError::NoResponseFromHub => "No Device".to_string(),
                    HubManagerError::SerialPortError => "Serial Port Error".to_string(),
                    _ => error_case.to_string(),
                }
            }
        }
    }

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

    /// Game API
    pub fn finish_game(&mut self) {
        self.game = GameContext::default();
    }
    pub fn start_the_game(&mut self) -> error_stack::Result<(), GameplayError> {
        // Prepare game context
        self.game.pack_content = self.game_pack.content.clone();
        self.update_game_state(GameState::QuestionChoosing);

        if self.players.len() < 2 {
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

                self.players
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
            .players
            .get_mut(&self.game.active_player_id())
            .ok_or(GameplayError::PlayerNotPresent)
            .into_report()?;
        player.state = PlayerState::QuestionChooser;
        Ok(())
    }

    pub fn fetch_players(&mut self) -> &HashMap<u8, Player> {
        self.update_non_target_player_states();
        &self.players
    }

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

        self.update_game_state(GameState::QuestionSelected);

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

        self.game.total_tries += 1;
        self.game.total_wrong_answers += 1;

        let theme = self.game.question_theme.clone();
        let price = self.game.question_price;
        log::info!(">>> Trying to remove question from category: {theme}, price: {price}");

        self.update_game_state(GameState::QuestionChoosing);
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

        self.players
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
                .players
                .get_mut(&active_player_id)
                .ok_or(GameplayError::PlayerNotPresent)?;
            if answered_correctly {
                active_player.stats.correct_num += 1;
                self.game.total_correct_answers += 1;
                active_player.stats.score += self.game.question_price;
                active_player.state = PlayerState::AnsweredCorrectly;
            } else {
                active_player.stats.wrong_num += 1;
                active_player.stats.score -= self.game.question_price;
                active_player.state = PlayerState::AnsweredWrong;
            }
            self.game.total_tries += 1;
            active_player.stats.total_tries += 1;
            active_player.clone()
        };

        log::info!("Current player stats: {:?}", response_player);

        if self.no_players_to_answer_left() {
            log::info!("Nobody answered question correctly");
            self.game.total_wrong_answers += 1;
        }

        let theme = self.game.question_theme.clone();
        let price = self.game.question_price;

        let mut retry = true;
        if answered_correctly || self.no_players_to_answer_left() {
            log::info!("Removing question from the pack");

            retry = false;
            self.update_game_state(GameState::QuestionChoosing);
            self.update_non_target_player_states();

            self.remove_question(&theme, &price)
                .change_context(GameplayError::PackElementNotPresent)?;
        }

        Ok(retry)
    }

    pub fn no_players_to_answer_left(&self) -> bool {
        let players_left = self
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
            totalCorrectAnswers: self.game.total_correct_answers,
            totalWrongAnswers: self.game.total_wrong_answers,
            totalTries: self.game.total_tries,
            roundTime: "Not tracked".to_owned(),
            players: self
                .players
                .values()
                .map(|p| PlayerStatsDto {
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
        log::info!(
            "Game state {:?} -> {:?}",
            self.game.game_state(),
            new_state
        );
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

            let Some(player) = self.players.get(&e.term_id) else {
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
        self.players.keys().copied().collect()
    }

    fn update_non_target_player_states(&mut self) {
        let game_state = self.game.game_state();
        let active_id = self.get_active_player_id();

        self.players.iter_mut().for_each(|(id, p)| {
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

            if *game_state == GameState::QuestionChoosing
                || (p.state != PlayerState::Dead && p.state != PlayerState::Inactive)
            {
                log::trace!("Player with id {} becomes idle", id);
                p.state = PlayerState::Idle;
            }
        });
    }

    fn kill_players_with_negative_balance(&mut self) {
        self.players.iter_mut().for_each(|(_, player)| {
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
    use crate::core::game_entities::{Player};

    #[test]
    fn test_fastest_click() {
        let mut ctx = AppContext::default();
        ctx.players.insert(1, Player::default());
        ctx.players.insert(2, Player::default());
        ctx.players.insert(3, Player::default());
        ctx.players.insert(4, Player::default());
        let i = ctx.get_fastest_click_player_id().expect("Test");
        log::info!("Fastest click from: {i}");
    }
}
