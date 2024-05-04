use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::dto::QuestionType;
use crate::core::game_entities::HubStatus::Detected;

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

#[derive(Debug, Serialize, PartialEq, Default, Copy, Clone)]
pub enum HubStatus {
    Detected,
    #[default]
    NoDevice,
}

impl HubStatus {
    pub fn is_live(&self) -> bool {
        *self == Detected
    }
}

#[derive(Debug, Clone, Serialize, Error)]
pub enum GamePackError {
    #[error("Theme not present")]
    ThemeNotPresent,
    #[error("Question not present")]
    QuestionNotPresent,
}

#[derive(Default, Debug)]
pub struct GameContext {
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

impl GameContext {
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
