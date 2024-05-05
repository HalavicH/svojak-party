use std::string::ToString;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::dto::QuestionType;
use crate::core::game_entities::HubStatus::Detected;
use crate::game_pack::pack_content_entities::{PackContent, Round};

pub const DEFAULT_ICON: &str = "default";

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
    #[default]
    NotInitialized,
    Detected,
    NoDevice,
    SerialPortError,
    UnknownError
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
