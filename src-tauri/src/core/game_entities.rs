use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;
use std::sync::{Arc, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use thiserror::Error;

use std::sync::mpsc::Receiver;

use crate::api::dto::QuestionType;
use crate::game_pack::game_pack_entites::GamePack;
use crate::hub_comm::common::hub_api::{HubManager, HubType};
use crate::hub_comm::hw::hw_hub_manager::HwHubManager;
use crate::hub_comm::hw::internal::api_types::TermEvent;
use error_stack::Report;
use serde::{Deserialize, Serialize};
use crate::hub_comm::web::web_hub_manager::WebHubManager;

lazy_static::lazy_static! {
    static ref CONTEXT: Arc<Mutex<GameContext>> = Arc::new(Mutex::new(GameContext::default()));
}

pub fn game() -> MutexGuard<'static, GameContext> {
    CONTEXT.lock().expect("Mutex is poisoned")
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Idle,
    QuestionChooser,
    Target,
    FirstResponse,
    Inactive,
    Dead,
    AnsweredCorrectly,
    AnsweredWrong,
}

#[derive(Default, Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub score: i32,
    pub correct_num: i32,
    pub wrong_num: i32,
    pub total_tries: i32,
}

#[derive(Debug, Default, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub icon: String,
    // todo: make actual image
    pub term_id: u8,
    pub is_used: bool,
    pub state: PlayerState,
    pub stats: PlayerStats,
}

impl Player {
    pub fn new(term_id: u8) -> Self {
        Self {
            term_id,
            ..Default::default()
        }
    }

    pub fn allowed_to_click(&self) -> bool {
        self.state != PlayerState::Dead && self.state != PlayerState::Inactive
    }
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub enum HubStatus {
    Detected,
    #[default]
    NoDevice,
}

#[derive(Debug, Clone, Serialize, Error)]
pub enum GamePackError {
    #[error("Theme not present")]
    ThemeNotPresent,
    #[error("Question not present")]
    QuestionNotPresent,
}

#[derive(Debug)]
pub struct GameContext {
    pub players: HashMap<u8, Player>,
    pub game_pack: GamePack,
    pub hub_type: HubType,
    hub: Arc<RwLock<Box<dyn HubManager>>>,
    pub current: CurrentContext,
    pub event_queue: Option<Receiver<TermEvent>>,
    pub allow_answer_timestamp: Arc<AtomicU32>,
}

unsafe impl Send for GameContext {}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            hub_type: HubType::default(),
            hub: Arc::new(RwLock::new(Box::new(HwHubManager::default()))),
            players: HashMap::default(),
            game_pack: GamePack::default(),
            current: CurrentContext::default(),
            event_queue: None,
            allow_answer_timestamp: Arc::new(AtomicU32::default()),
        }
    }
}

impl GameContext {
    pub fn select_hub_type(&mut self, hub_type: HubType) {
        if self.hub_type == hub_type {
            log::info!("Hub is already set to: {:?}. Nothing to do", hub_type);
            return;
        }

        self.hub_type = hub_type.clone();
        match hub_type {
            HubType::HwHub => {
                log::info!("||| --> Selecting SERIAL hub <---");
                self.hub = Arc::new(RwLock::new(Box::new(HwHubManager::default())))
            }
            HubType::WebHub => {
                log::info!("||| --> Selecting WEB hub <---");
                self.hub = Arc::new(RwLock::new(Box::new(WebHubManager::default())))
            }
        }
    }
    pub fn drop_hub(&mut self) {
        self.hub = Arc::new(RwLock::new(Box::new(HwHubManager::default())))
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
}

#[derive(Default, Debug)]
pub struct CurrentContext {
    pub round_index: usize,
    active_player_id: u8,
    game_state: GameState,
    pub click_for_answer_allowed: bool,
    pub answer_allowed: bool,
    pub question_theme: String,
    pub question_price: i32,
    pub question_type: QuestionType,
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}

impl CurrentContext {
    pub fn active_player_id(&self) -> u8 {
        self.active_player_id
    }
    pub fn set_active_player_id(&mut self, new_id: u8) {
        self.active_player_id = new_id
    }
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }
    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state;
    }
}

#[derive(Debug, Clone, Serialize, Error)]
pub enum GameplayError {
    #[error("Pack element not present")]
    PackElementNotPresent,
    #[error("Player is not present")]
    PlayerNotPresent,
    #[error("HUB operation failed")]
    HubOperationError,
    #[error("Answer forbidden")]
    AnswerForbidden,
    #[error("Operation forbidden")]
    OperationForbidden,
    #[error("Internal error")]
    InternalError,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    #[default]
    SetupAndLoading,
    QuestionChoosing,
    QuestionSelected,
    AnswerAllowed,
    AnswerRequested,
    AnswerWrong,
    AnswerCorrect,
    NoPlayersToAnswerLeft,
}

#[cfg(test)]
mod game_entities_test {
    use crate::core::game_entities::{GameContext, Player};

    #[test]
    fn test_fastest_click() {
        let mut ctx = GameContext::default();
        ctx.players.insert(1, Player::default());
        ctx.players.insert(2, Player::default());
        ctx.players.insert(3, Player::default());
        ctx.players.insert(4, Player::default());
        let i = ctx.get_fastest_click_player_id().expect("Test");
        log::info!("Fastest click from: {i}");
    }
}
