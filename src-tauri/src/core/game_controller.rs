use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use tempfile::TempDir;

use crate::core::game::ctx::state_processors::answer_attempt_received::AnswerQuestionResult as Aqr;
use crate::core::game::ctx::state_processors::check_end_of_round::CheckEndOfRoundResult;
use crate::core::game::ctx::state_processors::choose_question::ChooseQuestionResult;
use crate::core::game::ctx::state_processors::show_round_stats::RoundStatsResult;
use crate::core::game::game_data::GameMode;
use crate::core::game::game_state::GameState;
use crate::core::game_entities::GameplayError;
use crate::core::game_pack::game_pack_entites::GamePack;
use crate::core::game_pack::pack_content_entities::Round;
use crate::host_api::events::{
    emit_error, emit_game_state, emit_players_by_game_data, emit_question, emit_round,
};
use crate::hub::hub_api::PlayerEvent;
use crate::player_server::entities::PsPlayer;

lazy_static::lazy_static! {
    static ref GAME_CONTROLLER: Arc<RwLock<GameController >> = Arc::new(RwLock::new(GameController::default()));
}

pub fn game_mut() -> RwLockWriteGuard<'static, GameController> {
    GAME_CONTROLLER
        .write()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

pub fn game() -> RwLockReadGuard<'static, GameController> {
    GAME_CONTROLLER
        .read()
        .map_err(|e| format!("Mutex is poisoned: {e:#?}"))
        .expect("Mutex is poisoned")
}

#[derive(Debug, Default)]
pub struct GameController {
    pub game_pack: GamePack,
    pub game_state: GameState,
}

macro_rules! get_ctx_ensuring_state {
    ($self:ident, $state_variant:ident) => {
        match &mut $self.game_state {
            GameState::$state_variant(state) => state,
            _ => Err($self
                .handle_state_mismatch_error(concat!("GameState::", stringify!($state_variant))))?,
        }
    };
}

/// Player server API
impl GameController {
    pub fn get_events_handle(&self) -> Arc<RwLock<Vec<PlayerEvent>>> {
        let data = self.game_state.game_ctx_ref();
        data.events.clone()
    }

    // pub fn push_events(&self, events: Vec<PlayerEvent>) {
    //     let data = self.game_state.game_ctx_ref();
    //     let mut events_guard = data.events.write().expect("Expected to be able acquire write lock on events");
    //     events_guard.extend(events);
    // }

    pub fn push_new_players(
        &mut self,
        players: Vec<PsPlayer>,
    ) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, SetupAndLoading);

        let data = ctx.game_mut();
        for player in players {
            data.players.insert(player.id as u8, player.into());
        }
        emit_players_by_game_data(data);
        Ok(())
    }
}

/// Host API
impl GameController {
    // Setup API
    pub fn set_game_pack(&mut self, pack: GamePack) {
        self.game_pack = pack;
    }

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

    pub fn request_context_update(&self) {
        self.emit_game_context();
    }

    // Gameplay host API
    pub fn start_new_game(
        &mut self,
        game_mode: GameMode,
    ) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, SetupAndLoading);

        let content = self.game_pack.content.clone();
        let ctx = ctx.start(content, game_mode)?;
        self.set_game_state(GameState::StartNextRound(ctx));
        self.init_next_round()?;
        self.pick_first_question_chooser()?;
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

        let path = ctx.choose_question(topic, price)?;
        let state = match path {
            ChooseQuestionResult::DisplayQuestion(ctx) => GameState::DisplayQuestion(ctx),
            ChooseQuestionResult::AnswerAttemptReceived(ctx) => {
                GameState::AnswerAttemptReceived(ctx)
            }
        };
        self.set_game_state(state);
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
            CheckEndOfRoundResult::ShowRoundStats(game) => GameState::ShowRoundStats(game),
        });
        Ok(())
    }

    pub fn process_end_of_round(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, ShowRoundStats);

        let path = ctx.get_end_round_path()?;
        match path {
            RoundStatsResult::StartNextRound(ctx) => {
                self.set_game_state(GameState::StartNextRound(ctx));
                self.init_next_round()?;
                self.pick_first_question_chooser()?;
            }
            RoundStatsResult::EndTheGame(ctx, reason) => {
                ctx.calculate_final_results(reason);
                self.set_game_state(GameState::EndTheGame(ctx))
            }
        }
        Ok(())
    }

    fn init_next_round(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, StartNextRound);

        let ctx = ctx.init_next_round()?;
        self.set_game_state(GameState::PickFirstQuestionChooser(ctx));
        Ok(())
    }

    pub fn finish_game(&mut self) -> error_stack::Result<(), GameplayError> {
        let ctx = get_ctx_ensuring_state!(self, EndTheGame);

        let ctx = ctx.finish_game()?;
        self.set_game_state(GameState::SetupAndLoading(ctx));
        Ok(())
    }

    pub fn edit_player_score(&mut self, player_id: i32, score: i32) -> error_stack::Result<(), GameplayError> {
        log::info!("received: player: {player_id} score {score}");

        let data = self.game_state.game_mut();
        let player = data.players.get_mut(&(player_id as u8)).ok_or(GameplayError::PlayerNotFound)?;
        player.stats.score = score;
        emit_players_by_game_data(data);
        Ok(())
    }

    /// Used to reset everything
    pub fn reset_the_game(&mut self) {
        self.game_pack = GamePack::default();
        self.game_state = GameState::default();
        self.emit_game_context();
    }
}

/// Debug API
impl GameController {
    pub fn _dbg_set_game_state(&mut self, name: String) {
        self.set_game_state(GameState::from_name_and_game(
            &name,
            self.game_state.game_ctx_ref().clone(),
        ));
        self.emit_game_context();
    }
}

#[allow(dead_code)]
fn create_temp_directory() -> error_stack::Result<Arc<TempDir>, io::Error> {
    let tmp_dir = TempDir::new()?;
    let temp_dir = Arc::new(tmp_dir);

    Ok(temp_dir)
}

/// Internal API
impl GameController {
    /// This method should be used for every state change to ensure event emission
    pub fn set_game_state(&mut self, state: GameState) {
        self.game_state = state;
        emit_game_state(&self.game_state);
    }

    fn emit_game_context(&self) {
        let game_ctx = self.game_state.game_ctx_ref();
        emit_players_by_game_data(game_ctx);
        emit_question(game_ctx.current_question_ref().into());
        emit_round(
            game_ctx
                .current_round_opt_ref()
                .unwrap_or(&Round::default())
                .into(),
        );
        emit_game_state(&self.game_state);
    }

    fn handle_state_mismatch_error(&mut self, expected_state: &str) -> GameplayError {
        let state_mismatch = self.game_state.show_state_mismatch(expected_state);
        emit_error(format!("Context retrieval failure: {}", state_mismatch));
        GameplayError::OperationForbidden
    }
}
