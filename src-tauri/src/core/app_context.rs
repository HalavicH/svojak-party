use crate::api::events::{
    emit_error, emit_game_state, emit_hub_config, emit_players, emit_question, emit_round,
};
use crate::core::game_entities::{GamePackError, GameState, GameplayError, Player};
use crate::core::player_listener::discover_and_save_players;
use crate::core::term_event_processing::start_event_listener;
use crate::game_pack::game_pack_entites::GamePack;
use crate::hub::hub_api::{HubManager, HubManagerError, HubType, TermEvent};
use crate::hub::hw::hw_hub_manager::HwHubManager;
use crate::hub::web::web_hub_manager::WebHubManager;
use error_stack::{Report, ResultExt};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::AtomicU32;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use crate::types::ArcRwBox;

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
    hub: ArcRwBox<dyn HubManager>,
    player_poling_thread_handle: Option<JoinHandle<()>>,

    // pub window: Arc<RwLock<Box<Option<Window>>>>,
    // Game entities
    pub game_pack: GamePack,
    pub game_state: GameState,
}

unsafe impl Send for AppContext {}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            hub_type: HubType::default(),
            hub: ArcRwBox::new(RwLock::new(Box::<HwHubManager>::default())),
            game_pack: GamePack::default(),
            game_state: GameState::default(),
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

    pub fn hub_lock(&self) -> Arc<RwLock<Box<dyn HubManager>>> {
        self.hub.clone()
    }

    pub fn hub(&self) -> RwLockReadGuard<Box<dyn HubManager>> {
        self.hub
            .read()
            .map_err(|e| {
                Report::new(GameplayError::InternalError)
                    .attach_printable(format!("Can't get HUB for read. {:?}", e))
            })
            .expect("Poisoned")
    }

    pub fn hub_mut(&self) -> RwLockWriteGuard<Box<dyn HubManager>> {
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
    pub fn save_round_duration(&mut self, round_duration_minutes: i32) {
        if let GameState::SetupAndLoading(game) = &mut self.game_state {
            game.set_round_duration(round_duration_minutes)
        } else {
            let state_mismatch = self
                .game_state
                .show_state_mismatch("GameState::SetupAndLoading");
            emit_error(format!("Can't setup round duration. {}", state_mismatch));
        }
    }

    pub fn set_hub_radio_channel(&self, channel_id: u8) {
        let mut hub_guard = self.hub_mut();

        match hub_guard.set_hw_hub_radio_channel(channel_id) {
            Ok(_) => {
                drop(hub_guard); // To release lock
                self.emit_game_config_locking_hub();
            }
            Err(e) => {
                log::error!("{:#?}", e);
                emit_error(e.to_string())
            }
        };
    }

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
        emit_hub_config(self.hub_mut().deref().into());
    }

    pub fn discover_hub(&mut self, path: String) {
        let game_ctx = match &mut self.game_state {
            GameState::SetupAndLoading(game) => game,
            _ => {
                let state_mismatch = self
                    .game_state
                    .show_state_mismatch("GameState::SetupAndLoading");
                emit_error(format!("Can't setup players: {}", state_mismatch));
                return;
            }
        };

        log::debug!(
            "Requested HUB change. Removing players as outdated: {:#?}",
            game_ctx.game_ref().players_map_ref()
        );

        game_ctx.game_mut().erase_players();

        let result = self.hub_mut().probe(&path);
        match result {
            Ok(_) => self.run_polling_for_players(),
            Err(err) => log::error!("Can't initialize hub on port: {}. Error: {:?}", path, err),
        }
        self.emit_game_config_locking_hub();
    }

    /// Players polling
    fn run_polling_for_players(&mut self) {
        if self.player_poling_thread_handle.is_some() {
            log::info!("Player polling thread already started");
            return;
        }

        log::info!("Initial setup of player polling thread");

        let handle = spawn(move || loop {
            discover_and_save_players();
            sleep(Duration::from_secs(2));
        });

        log::info!("Saving new thread handle");
        self.player_poling_thread_handle = Some(handle)
    }

    pub fn emit_game_config_locking_hub(&self) {
        emit_hub_config(self.hub_mut().deref().into());
        emit_players(
            self.game_state
                .game_ref()
                .players_ref_as_vec()
                .into_iter()
                .map(|p| p.into())
                .collect(),
        );
    }

    pub fn emit_game_context(&self) {
        emit_game_state(&self.game_state);
        emit_round(self.game_state.game_ref().current_round_ref().into());
        emit_question(self.game_state.game_ref().current_question_ref().into());
    }
}

/// Game API
impl AppContext {
    pub fn start_new_game(&mut self) -> error_stack::Result<(), GameplayError> {
        let hub = self.hub_lock();
        let game_ctx = match &mut self.game_state {
            GameState::SetupAndLoading(game) => game,
            _ => {
                let state_mismatch = self
                    .game_state
                    .show_state_mismatch("GameState::SetupAndLoading");
                emit_error(format!("Can't start the game: {}", state_mismatch));
                Err(GameplayError::PlayerNotPresent)?
            }
        };

        let (event_tx, event_rx) = mpsc::channel();
        start_event_listener(hub, event_tx);

        let game_ctx = std::mem::take(game_ctx); // Take ownership of the value inside the mutable reference
        let content = self.game_pack.content.clone();
        let game_ctx = game_ctx.start(content, event_rx)?;
        let game_ctx = game_ctx.pick_first_question_chooser()?;
        self.game_state = GameState::ChooseQuestion(game_ctx);
        self.emit_game_config_locking_hub();
        Ok(())
    }

    /// Takes whole game context and maps to config which contains only required elements
    pub fn update_players(&mut self, players: &[Player]) {
        let players = players.iter().fold(HashMap::new(), |mut map, player| {
            map.insert(player.term_id, player.clone());
            map
        });
        self.game_state.game_mut().set_players(players);
    }
}

/// Old Game API
#[deprecated]
impl AppContext {
    pub fn finish_question_prematurely(&mut self) -> error_stack::Result<(), GameplayError> {
        // self.__old_game.answer_allowed = false;
        // self.allow_answer_timestamp
        //     .swap(u32::MAX, Ordering::Relaxed);
        //
        // self.__old_game.round_stats.total_tries += 1;
        // self.__old_game.round_stats.total_wrong_answers += 1;
        //
        // let theme = self.__old_game.question_theme.clone();
        // let price = self.__old_game.question_price;
        // log::info!(">>> Trying to remove question from category: {theme}, price: {price}");
        //
        // self.update_game_state(OldGameState::ChooseQuestion);
        // self.update_non_target_player_states();
        //
        // self.remove_question(&theme, &price)
        //     .change_context(GameplayError::PackElementNotPresent)?;
        Ok(())
    }

    fn remove_question(
        &mut self,
        theme: &String,
        price: &i32,
    ) -> error_stack::Result<(), GamePackError> {
        // log::info!("Try to remove question from category: {theme}, price: {price}");
        // let round = self.__old_game.get_current_round_mut();
        // let theme = round
        //     .themes
        //     .get_mut(theme)
        //     .ok_or(GamePackError::ThemeNotPresent)
        //     .into_report()
        //     .attach_printable(format!("Can't find theme: {theme:?}"))?;
        //
        // let _ = theme
        //     .pop_question(price)
        //     .ok_or(GamePackError::QuestionNotPresent)
        //     .into_report()
        //     .attach_printable(format!(
        //         "Can't find question with price {price:?} in theme: {theme:?}"
        //     ))?;
        //
        // round.questions_left -= 1;
        // log::info!("Question left: {}", round.questions_left);
        Ok(())
    }

    pub fn allow_answer(&mut self) -> error_stack::Result<(), HubManagerError> {
        // let timestamp = get_epoch_ms()?;
        // self.allow_answer_timestamp
        //     .swap(timestamp, Ordering::Relaxed);
        // log::info!("Current answer base timestamp: {timestamp}");
        //
        // self.__old_game.set_active_player_id(0);
        // self.update_non_target_player_states();
        // self.__old_game.click_for_answer_allowed = true;
        Ok(())
    }

    pub fn answer_question(
        &mut self,
        answered_correctly: bool,
    ) -> error_stack::Result<bool, GameplayError> {
        // if !self.__old_game.answer_allowed {
        //     return Err(Report::new(GameplayError::AnswerForbidden));
        // }
        //
        // self.__old_game.answer_allowed = false;
        // self.allow_answer_timestamp
        //     .swap(u32::MAX, Ordering::Relaxed);
        //
        // let active_player_id = self.get_active_player_id();
        // log::info!(
        //     "Active player id: {}. Player ids: {:?}",
        //     active_player_id,
        //     self.get_player_keys()
        // );
        //
        // let response_player = {
        //     let active_player = self
        //         .__old_game
        //         .players
        //         .get_mut(&active_player_id)
        //         .ok_or(GameplayError::PlayerNotPresent)?;
        //     if answered_correctly {
        //         active_player.stats.correct_num += 1;
        //         self.__old_game.round_stats.total_correct_answers += 1;
        //         active_player.stats.score += self.__old_game.question_price;
        //         active_player.state = PlayerState::AnsweredCorrectly;
        //     } else {
        //         active_player.stats.wrong_num += 1;
        //         active_player.stats.score -= self.__old_game.question_price;
        //         active_player.state = PlayerState::AnsweredWrong;
        //     }
        //     self.__old_game.round_stats.total_tries += 1;
        //     active_player.stats.total_tries += 1;
        //     active_player.clone()
        // };
        //
        // log::info!("Current player stats: {:?}", response_player);
        //
        // if self.no_players_to_answer_left() {
        //     log::info!("Nobody answered question correctly");
        //     self.__old_game.round_stats.total_wrong_answers += 1;
        // }
        //
        // let theme = self.__old_game.question_theme.clone();
        // let price = self.__old_game.question_price;
        //
        // let mut retry = true;
        // if answered_correctly || self.no_players_to_answer_left() {
        //     log::info!("Removing question from the pack");
        //
        //     retry = false;
        //     self.update_game_state(OldGameState::ChooseQuestion);
        //     self.update_non_target_player_states();
        //
        //     self.remove_question(&theme, &price)
        //         .change_context(GameplayError::PackElementNotPresent)?;
        // }

        Ok(true)
    }

    pub fn no_players_to_answer_left(&self) -> bool {
        // let players_left = self
        //     .__old_game
        //     .players
        //     .iter()
        //     .filter(|(_, p)| {
        //         p.state != PlayerState::Inactive
        //             && p.state != PlayerState::Dead
        //             && p.state != PlayerState::AnsweredWrong
        //     })
        //     .count();
        // log::debug!("Players to answer left: {}", players_left);
        // players_left == 0
        true
    }

    // pub fn fetch_round_stats(&self) -> RoundStatsDto {
    //     let round = self.__old_game.get_current_round();
    //     let players = self.__old_game.players.values().cloned().collect();
    //     game_to_round_stats_dto(round, &self.__old_game.round_stats, players)
    // }

    // fn update_game_state(&mut self, new_state: OldGameState) {
    //     log::info!(
    //         "Game state {:?} -> {:?}",
    //         self.__old_game.game_state(),
    //         new_state
    //     );
    //     self.__old_game.set_game_state(new_state);
    //     self.update_non_target_player_states();
    // }

    // fn get_player_keys(&self) -> Vec<u8> {
    //     self.__old_game.players.keys().copied().collect()
    // }

    // fn update_non_target_player_states(&mut self) {
    //     let game_state = self.__old_game.game_state().clone();
    //     let active_id = self.get_active_player_id();
    //
    //     self.__old_game.players.iter_mut().for_each(|(id, p)| {
    //         log::debug!(
    //             "Game state: {:?}. Player: {}:{:?}",
    //             game_state,
    //             p.term_id,
    //             p.state
    //         );
    //
    //         if p.term_id == active_id {
    //             log::debug!("Active player. Skipping");
    //             return;
    //         }
    //
    //         if p.state == PlayerState::AnsweredWrong {
    //             log::trace!("Player with id {} becomes inactive", id);
    //             p.state = PlayerState::Inactive;
    //         }
    //
    //         if game_state == OldGameState::ChooseQuestion
    //             || (p.state != PlayerState::Dead && p.state != PlayerState::Inactive)
    //         {
    //             log::trace!("Player with id {} becomes idle", id);
    //             p.state = PlayerState::Idle;
    //         }
    //     });
    // }
}
