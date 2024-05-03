use crate::hub_comm::common::hub_api::HubManager;
use error_stack::{IntoReport, Report, Result, ResultExt};
use std::collections::HashMap;
use std::sync::atomic::{Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, RwLock, RwLockReadGuard};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};

use crate::api::dto::{PlayerStatsDto, RoundStatsDto};
use crate::core::game_entities::{
    GameContext, GamePackError, GameState, GameplayError, Player, PlayerState,
};

use crate::game_pack::pack_content_entities::{Question, Round};
use crate::hub_comm::hw::hw_hub_manager::{get_epoch_ms, HubManagerError};
use crate::hub_comm::hw::internal::api_types::TermButtonState::Pressed;
use crate::hub_comm::hw::internal::api_types::TermEvent;

const EVT_POLLING_INTERVAL_MS: u64 = 1000;

impl GameContext {
    pub fn start_the_game(&mut self) -> Result<(), GameplayError> {
        self.update_game_state(GameState::QuestionChoosing);

        if self.players.len() < 2 {
            log::info!("Not enough players to run the game.");
            return Err(GameplayError::PlayerNotPresent).into_report();
        }

        let (event_tx, event_rx) = mpsc::channel();
        self.event_queue = Some(event_rx);

        start_event_listener(
            self.get_hub_ref().clone(),
            event_tx
        );

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

        self.current.set_active_player_id(q_picker_id);
        let player = self
            .players
            .get_mut(&self.current.active_player_id())
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
    ) -> Result<(Question, i32), GameplayError> {
        log::info!("Get question from category: {theme}, price: {price}");

        self.current.set_active_player_id(0);
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

    pub fn remove_question(&mut self, theme: &String, price: &i32) -> Result<(), GamePackError> {
        log::info!("Try to remove question from category: {theme}, price: {price}");
        let round = self.get_current_round_mut();
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

    pub fn finish_question_prematurely(&mut self) -> Result<(), GameplayError> {
        self.current.answer_allowed = false;
        self.allow_answer_timestamp.swap(u32::MAX, Ordering::Relaxed);

        self.current.total_tries += 1;
        self.current.total_wrong_answers += 1;

        let theme = self.current.question_theme.clone();
        let price = self.current.question_price;
        log::info!(">>> Trying to remove question from category: {theme}, price: {price}");

        self.update_game_state(GameState::QuestionChoosing);
        self.update_non_target_player_states();

        self.remove_question(&theme, &price)
            .change_context(GameplayError::PackElementNotPresent)?;
        Ok(())
    }

    pub fn has_next_question(&self) -> bool {
        // self.current.has_next_question
        let has_new_question = self.get_current_round().questions_left > 0;
        log::info!("Has new question: {}", has_new_question);
        has_new_question
    }

    pub fn allow_answer(&mut self) -> Result<(), HubManagerError> {
        let timestamp = get_epoch_ms()?;
        self.allow_answer_timestamp
            .swap(timestamp, Ordering::Relaxed);
        log::info!("Current answer base timestamp: {timestamp}");

        self.current.set_active_player_id(0);
        self.update_non_target_player_states();
        self.current.click_for_answer_allowed = true;
        Ok(())
    }

    pub fn get_fastest_click_player_id(&mut self) -> Result<u8, GameplayError> {
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
        self.current.click_for_answer_allowed = false;
        self.current.answer_allowed = true;
        self.current.set_active_player_id(fastest_player_id);

        self.players
            .get_mut(&fastest_player_id)
            .ok_or(Report::new(GameplayError::PlayerNotPresent))
            .attach_printable(format!("Can't find player with id {}", fastest_player_id))?
            .state = PlayerState::FirstResponse;

        Ok(fastest_player_id)
    }

    pub fn get_active_player_id(&self) -> u8 {
        self.current.active_player_id()
    }

    pub fn answer_question(&mut self, answered_correctly: bool) -> Result<bool, GameplayError> {
        if !self.current.answer_allowed {
            return Err(Report::new(GameplayError::AnswerForbidden));
        }

        self.current.answer_allowed = false;
        self.allow_answer_timestamp.swap(u32::MAX, Ordering::Relaxed);

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
                self.current.total_correct_answers += 1;
                active_player.stats.score += self.current.question_price;
                active_player.state = PlayerState::AnsweredCorrectly;
            } else {
                active_player.stats.wrong_num += 1;
                active_player.stats.score -= self.current.question_price;
                active_player.state = PlayerState::AnsweredWrong;
            }
            self.current.total_tries += 1;
            active_player.stats.total_tries += 1;
            active_player.clone()
        };

        log::info!("Current player stats: {:?}", response_player);

        if self.no_players_to_answer_left() {
            log::info!("Nobody answered question correctly");
            self.current.total_wrong_answers += 1;
        }

        let theme = self.current.question_theme.clone();
        let price = self.current.question_price;

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

    pub fn get_current_round(&self) -> &Round {
        let index = self.current.round_index;
        let round = self.game_pack.content.rounds.get(index)
            .expect(&format!("Expected to have round #{}", index));
        round
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

    pub fn init_next_round(&mut self) {
        if self.is_already_last_round() {
            log::error!("Already final round");
            return;
        }

        self.current.round_index += 1;
        let index = self.current.round_index;
        let round: &Round = self
            .game_pack
            .content
            .rounds
            .get(index)
            .expect(&format!("Expected to have round #{}", index));
        log::info!("Next round name {}", round.name);

        self.current.total_tries = 0;
        self.current.total_wrong_answers = 0;
        self.current.total_correct_answers = 0;

        if self.is_already_last_round() {
            self.kill_players_with_negative_balance();
        }
    }

    fn is_already_last_round(&mut self) -> bool {
        (self.game_pack.content.rounds.len() - 1) == self.current.round_index
    }

    pub fn fetch_round_stats(&self) -> RoundStatsDto {
        let round = self.get_current_round();
        RoundStatsDto {
            roundName: round.name.to_owned(),
            questionNumber: round.question_count,
            normalQuestionNum: round.normal_question_count,
            pigInPokeQuestionNum: round.pip_question_count,
            totalCorrectAnswers: self.current.total_correct_answers,
            totalWrongAnswers: self.current.total_wrong_answers,
            totalTries: self.current.total_tries,
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
            self.current.game_state(),
            new_state
        );
        self.current.set_game_state(new_state);
        self.update_non_target_player_states();
    }

    fn get_question(
        &mut self,
        theme: &String,
        price: &i32,
    ) -> Result<(Question, i32), GamePackError> {
        let round = self.get_current_round_mut();
        let question_number = round.question_count - round.questions_left;

        let theme = round
            .themes
            .get_mut(theme)
            .ok_or(GamePackError::ThemeNotPresent)
            .into_report()
            .attach_printable(format!("Can't find theme: {theme:?}"))?;

        let question = theme
            .get_question(price)
            .ok_or(GamePackError::QuestionNotPresent)
            .into_report()
            .attach_printable(format!(
                "Can't find question with price {price:?} in theme: {theme:?}"
            ))?
            .clone();

        self.current.question_theme = theme.name.clone();
        self.current.question_type = question.question_type.clone();
        self.current.question_price = question.price;
        Ok((question, question_number))
    }

    fn get_fastest_click_from_hub(&mut self) -> Result<u8, HubManagerError> {
        let Some(receiver) = &self.event_queue else {
            return Err(HubManagerError::NotInitializedError.into());
        };

        let start_time = Instant::now();
        let timeout = Duration::from_secs(10);
        let fastest_click: Option<u8> = None;

        loop {
            if start_time.elapsed() >= timeout {
                return Err(Report::new(HubManagerError::NoResponseFromTerminal));
            }

            let events = match Self::get_events(receiver) {
                Ok(events) => events,
                Err(_) => {
                    sleep(Duration::from_millis(100));
                    continue;
                }
            };

            let base_timestamp = self.allow_answer_timestamp.load(Ordering::Relaxed);
            let mut events: Vec<TermEvent> = events.iter()
                .filter(|&e| {
                    return if e.timestamp >= base_timestamp {
                        log::info!("After answer allowed. Event {:?}", e);
                        true
                    } else {
                        log::info!("Answer too early. Event {:?}", e);
                        false
                    }
                })
                .map(|e| e.clone())
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
    ) -> Option<Result<u8, HubManagerError>> {
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

    fn get_events(receiver: &Receiver<TermEvent>) -> Result<Vec<TermEvent>, HubManagerError> {
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

    fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.current.round_index;
        let round = self.game_pack.content.rounds.get_mut(index)
            .expect(&format!("Expected to have round #{}", index));
        round
    }

    fn update_non_target_player_states(&mut self) {
        let game_state = self.current.game_state();
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

pub fn start_event_listener(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    sender: Sender<TermEvent>
) -> JoinHandle<()> {
    log::info!("Starting event listener");

    thread::spawn(move || {
        listen_hub_events(hub, sender);
    })
}

fn listen_hub_events(
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    sender: Sender<TermEvent>
) {
    loop {
        log::debug!("############# NEW ITERATION ###############");
        sleep(Duration::from_millis(EVT_POLLING_INTERVAL_MS));
        let hub_guard = hub.read().expect("Mutex is poisoned");
        let events = hub_guard.read_event_queue().unwrap_or_else(|error| {
            log::error!("Can't get events. Err {:?}", error);
            vec![]
        });

        if events.is_empty() {
            log::debug!("No player events occurred");
            continue;
        }

        events.iter().for_each(|e| {
            process_term_event(&hub_guard, e, &sender);
        });
    }
}

fn process_term_event(
    hub_guard: &RwLockReadGuard<Box<dyn HubManager>>,
    e: &TermEvent,
    sender: &Sender<TermEvent>
) {
    hub_guard
        .set_term_feedback_led(e.term_id, &e.state)
        .unwrap_or_else(|error| {
            log::error!("Can't set term_feedback let. Err {:?}", error);
        });


    sender.send((*e).clone())
        .map_err(|e| {
            log::error!("Can't send the event: {}", e);
        })
        .unwrap_or_default();
}
