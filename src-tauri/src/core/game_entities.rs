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
    /// Configuring players and game pack.
    /// Next state: `ChooseQuestion` (when game started)
    SetupAndLoading,

    /// When game instantiated & started this is the first state
    /// Next state: `DisplayQuestion` (when question selected)
    ChooseQuestion,

    /// When question selected everyone reads it, but can't answer until host allows
    /// Next state: `WaitingForAnswerRequests` (when host press 'Allow answer' button
    DisplayQuestion,

    /// Host allowed answering the question, from now players can send answer requests
    /// Next state: `AnswerAttemptReceived` (when first event from active player received)
    WaitingForAnswerRequests,

    /// The quickest player pushed 'Answer' button first, and now he has right to try answer the question
    /// Next state: `WrongAnswer` - if verbal answer from player was wrong (when host press button "Wrong answer")
    ///        or : `CorrectAnswer` - if verbal answer from player was correct (when host press button "Correct answer")
    AnswerAttemptReceived,

    // TBD: Actually I don't think that answering actions require state. We need to debate on that
    /// Player answered question wrong - player's score is reduced by question price, player can't answer
    /// this question anymore, and remaining players will try to compete for the next try to answer it
    /// Next state: `DisplayQuestion` - if 1+ players who not answered left (when host presses "Next try")
    ///        or : `NoPlayersToAnswerLeft` - if all players failed to answer the question correctly (when host presses "Next question")
    WrongAnswer,

    /// Player answered question correctly - player receives +score, question is resolved
    /// Next state: `ChooseQuestion` (when  host presses "Next Question")
    CorrectAnswer,

    /// All players answered question wrong and no players left - correct answer is displayed on the screen
    /// Next state: `ChooseQuestion` (when  host presses "Next Question")
    NoPlayersToAnswerLeft,
}
