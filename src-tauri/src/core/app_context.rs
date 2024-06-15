use crate::api::events::{
    emit_error, emit_game_state, emit_hub_config, emit_players_by_game_data, emit_question,
    emit_round,
};
use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::game_state::GameState;
use crate::core::game_ctx::state_processors::answer_attempt_received::AnswerQuestionResult as Aqr;
use crate::core::game_entities::{GamePackError, GameplayError, Player};
use crate::core::player_connection_listener::start_listening_for_players_connection;
use crate::core::player_event_listener::start_event_listener;
use crate::game_pack::game_pack_entites::GamePack;
use crate::hub::hub_api::{HubManager, HubType};
use crate::hub::hw::hw_hub_manager::HwHubManager;
use crate::hub::web::web_hub_manager::WebHubManager;
use crate::types::ArcRwBox;
use error_stack::{FutureExt, Report, ResultExt};
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::JoinHandle;
use crate::core::game_ctx::state_processors::check_end_of_round::CheckEndOfRoundResult;

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

    /// This method should be used for every state change to ensure event emission
    pub fn set_game_state(&mut self, state: GameState) {
        self.game_state = state;
        emit_game_state(&self.game_state);
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
                emit_hub_config(self.hub().deref().into());
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
        emit_hub_config(self.hub().deref().into());
    }

    pub fn discover_hub_and_players(&mut self, path: String) {
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
            Ok(_) => {
                emit_hub_config(self.hub().deref().into());

                self.run_polling_for_players()
            },
            Err(err) => log::error!("Can't initialize hub on port: {}. Error: {:?}", path, err),
        }
    }

    pub fn update_players(&mut self, players: &[Player]) {
        let players = players.iter().map(|p| (p.term_id, p.clone())).collect();
        self.game_state.game_mut().set_players(players);
    }

    /// Players polling
    fn run_polling_for_players(&mut self) {
        if self.player_poling_thread_handle.is_some() {
            log::info!("Player polling thread already started");
            return;
        }

        let handle = start_listening_for_players_connection(self.hub_lock());

        log::info!("Saving new thread handle");
        self.player_poling_thread_handle = Some(handle)
    }

    #[deprecated]
    pub fn emit_game_config_locking_hub(&self) {
        emit_hub_config(self.hub_mut().deref().into());
        let game_ctx = self.game_state.game_ctx_ref();
        emit_players_by_game_data(game_ctx);
    }

    #[deprecated]
    pub fn emit_game_context(&self) {
        emit_game_state(&self.game_state);
        emit_round(self.game_state.game_ctx_ref().current_round_ref().into());
        emit_question(self.game_state.game_ctx_ref().current_question_ref().into());
    }
}

macro_rules! get_ctx_ensuring_state {
    ($self:ident, $state_variant:ident) => {
        match &mut $self.game_state {
            GameState::$state_variant(state) => state,
            _ => Err($self.handle_state_mismatch_error(concat!("GameState::", stringify!($state_variant))))?,
        }
    };
}

/// Game API
impl AppContext {
    pub fn start_new_game(&mut self) -> error_stack::Result<(), GameplayError> {
        let hub = self.hub_lock();
        let ctx = get_ctx_ensuring_state!(self, SetupAndLoading);

        start_event_listener(hub, ctx.game_ref().events_clone());

        let content = self.game_pack.content.clone();
        let ctx = ctx.start(content)?;
        self.set_game_state(GameState::PickFirstQuestionChooser(ctx));
        Ok(())
    }

    pub fn pick_first_question_chooser(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, PickFirstQuestionChooser);

        let ctx = ctx.pick_first_question_chooser()?;
        self.set_game_state(GameState::ChooseQuestion(ctx));
        Ok(())
    }

    pub fn select_question(&mut self, topic: &str, price: i32) -> Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, ChooseQuestion);

        let ctx = ctx.choose_question(topic, price)?;
        self.set_game_state(GameState::DisplayQuestion(ctx));
        Ok(())
    }

    pub fn allow_answer(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, DisplayQuestion);

        let ctx = ctx.allow_answer()?;
        self.set_game_state(GameState::WaitingForAnswerRequests(ctx));
        Ok(())
    }

    pub fn wait_for_quickest_player_to_click(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, WaitingForAnswerRequests);

        let id = ctx.get_fastest_click_player_id()?;
        let ctx = ctx.request_answer_by_player_id(id)?;
        self.set_game_state(GameState::AnswerAttemptReceived(ctx));
        Ok(())
    }

    pub fn answer_question(
        &mut self,
        answered_correctly: bool,
    ) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, AnswerAttemptReceived);

        let path = ctx.answer_question(answered_correctly)?;
        self.set_game_state(match path {
            Aqr::EndQuestion(ctx) => GameState::EndQuestion(ctx),
            Aqr::DisplayQuestion(ctx) => GameState::DisplayQuestion(ctx),
        });
        Ok(())
    }

    pub fn stop_asking_and_show_answer(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, DisplayQuestion);

        let ctx = ctx.finish_question_preemptively()?;
        self.set_game_state(GameState::EndQuestion(ctx));
        Ok(())
    }

    pub fn finish_question(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, EndQuestion);

        let ctx = ctx.finish_question()?;
        self.set_game_state(GameState::CheckEndOfRound(ctx));
        self.check_end_of_round()?;
        Ok(())
    }

    pub fn check_end_of_round(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, CheckEndOfRound);

        let path = ctx.check_end_of_round()?;
        self.set_game_state(match path {
            CheckEndOfRoundResult::ChooseQuestion(game) => GameState::ChooseQuestion(game),
            CheckEndOfRoundResult::CalcRoundStats(game) => GameState::CalcRoundStats(game),
        });
        Ok(())
    }
}

/// Helper methods
impl AppContext {
    fn handle_state_mismatch_error(&mut self, expected_state: &str) -> GameplayError {
        let state_mismatch = self.game_state.show_state_mismatch(expected_state);
        emit_error(format!("Context retrieval failure: {}", state_mismatch));
        GameplayError::OperationForbidden
    }
}

/// Old Game API
#[deprecated]
impl AppContext {

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
}

/// Debug API
impl AppContext {
    pub fn _dbg_reset_game(&mut self) {
        let context = AppContext::default();
        self.hub_type = context.hub_type;
        self.hub = context.hub;
        self.player_poling_thread_handle = context.player_poling_thread_handle;
        self.game_pack = context.game_pack;
        self.set_game_state(GameState::SetupAndLoading(GameCtx::default()));
        self.emit_game_config_locking_hub();
        self.emit_game_context();
    }

    pub fn _dbg_set_game_state(&mut self, name: String) {
        self.set_game_state(GameState::from_name_and_game(
            &name,
            self.game_state.game_ctx_ref().clone(),
        ));
        self.emit_game_config_locking_hub();
        self.emit_game_context();
    }
}
