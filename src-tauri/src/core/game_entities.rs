use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::game_entities::HubStatus::Detected;
use crate::game_pack::pack_content_entities::Question;

pub const DEFAULT_ICON: &str = "default";

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Idle,
    QuestionChooser,
    Target,
    Answering,
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
    pub(crate) fn can_answer(&self) -> bool {
        match self.state {
            PlayerState::Idle => true,
            PlayerState::QuestionChooser => true,
            PlayerState::Target => true,
            PlayerState::Answering => true,
            PlayerState::Inactive => false,
            PlayerState::Dead => false,
            PlayerState::AnsweredCorrectly => false,
            PlayerState::AnsweredWrong => false,
        }
    }
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

    pub fn answered_correctly(&mut self, question: &Question) {
        self.state = PlayerState::AnsweredCorrectly;
        self.stats.correct_num += 1;
        self.stats.total_tries += 1;
        self.stats.score += question.price;
    }

    pub fn answered_wrong(&mut self, question: &Question) {
        self.state = PlayerState::AnsweredWrong;
        self.stats.wrong_num += 1;
        self.stats.total_tries += 1;
        self.stats.score -= question.price;
    }
}

#[derive(Debug, Serialize, PartialEq, Default, Copy, Clone)]
pub enum HubStatus {
    #[default]
    NotInitialized,
    Detected,
    NoDevice,
    SerialPortError,
    UnknownError,
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
    #[error("Not enough players for game")]
    NotEnoughPlayers,
    #[error("No active players left")]
    NoActivePlayersLeft,
    #[error("HUB operation failed")]
    UnexpectedGameState,
    #[error("Answer request timeout")]
    AnswerRequestTimeout,

    #[error("HUB operation failed")]
    PackElementNotPresent,
    #[error("Player is not present")]
    PlayerNotPresent(u8),
    #[error("HUB operation failed")]
    HubOperationError,
    #[error("Answer forbidden")]
    AnswerForbidden,
    #[error("Operation forbidden for this game state")]
    OperationForbidden,
    #[error("Internal error")]
    InternalError,
    #[error("Broken Hub Connection")]
    BrokenHubConnection,
}
