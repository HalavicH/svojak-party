use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::core::game_context::{AnswerAttemptReceived, CalcStatsAndStartNextRound, CheckEndOfRound, ChooseQuestion, DisplayQuestion, EndQuestion, GameContext, SetupAndLoading, WaitingForAnswerRequests};

use crate::core::game_entities::HubStatus::Detected;

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

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    /// Configuring players and game pack.
    /// Next state: `ChooseQuestion` (when game started)
    SetupAndLoading(GameContext<SetupAndLoading>),

    /// When game instantiated & started this is the first state
    /// Next state: `DisplayQuestion` (when question selected)
    ChooseQuestion(GameContext<ChooseQuestion>),

    /// When question selected everyone reads it, but can't answer until host allows
    /// Next state: `WaitingForAnswerRequests` (when host press 'Allow answer' button)
    DisplayQuestion(GameContext<DisplayQuestion>),

    /// Host allowed answering the question, from now players can send answer requests
    /// Next state: `AnswerAttemptReceived` (when first event from active player received)
    WaitingForAnswerRequests(GameContext<WaitingForAnswerRequests>),

    /// The quickest player pushed 'Answer' button first, and now he has right to try answer the question
    /// Next state: `DisplayQuestion` (when verbal answer from player was wrong and remaining players are available)
    ///        or : `EndQuestion` (when verbal answer from player was correct or no players left after wrong answers)
    AnswerAttemptReceived(GameContext<AnswerAttemptReceived>),

    /// Any player answered the question correctly or all players answered question wrong.
    /// In this case correct answer is displayed on the screen
    /// At this point intermediate player stats can be displayed
    /// Next state: `CheckEndOfRound` (when host presses "Next Question")
    EndQuestion(GameContext<EndQuestion>),

    /// Check if the round is over. If all questions in the round are answered, proceed to round-end actions.
    /// Next state: `CalcStatsAndStartNextRound` (when round is over)
    ///        or : `DisplayQuestion` (when round is not over)
    CheckEndOfRound(GameContext<CheckEndOfRound>),

    /// Display round statistics, eliminate players with negative scores, etc.
    /// Start the next round by resetting game state and proceeding to question selection.
    /// Next state: `ChooseQuestion` (when host presses "Start Next Round")
    CalcStatsAndStartNextRound(GameContext<CalcStatsAndStartNextRound>),
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OldGameState {
    #[default]
    SetupAndLoading,
    ChooseQuestion,
    DisplayQuestion,
}