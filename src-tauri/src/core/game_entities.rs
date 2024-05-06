use crate::core::game_context::{
    AnswerAttemptReceived, CalcStatsAndStartNextRound, CheckEndOfRound, ChooseQuestion,
    DisplayQuestion, EndQuestion, Game, GameContext, SetupAndLoading, WaitingForAnswerRequests,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    UnexpectedGameState,
    #[error("Answer request timeout")]
    AnswerRequestTimeout,

    #[error("HUB operation failed")]
    PackElementNotPresent,
    #[error("Player is not present")]
    PlayerNotPresent,
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

#[derive(Debug)]
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

impl GameState {
    pub fn show_state_mismatch(&mut self, expected: &str) -> String {
        format!(
            "Expected game state of '{}', found: {}",
            expected,
            self.state_name()
        )
    }

    pub fn game_mut(&mut self) -> &mut Game {
        match self {
            GameState::SetupAndLoading(game_ctx) => game_ctx.game_mut(),
            GameState::ChooseQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::DisplayQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::WaitingForAnswerRequests(game_ctx) => game_ctx.game_mut(),
            GameState::AnswerAttemptReceived(game_ctx) => game_ctx.game_mut(),
            GameState::EndQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::CheckEndOfRound(game_ctx) => game_ctx.game_mut(),
            GameState::CalcStatsAndStartNextRound(game_ctx) => game_ctx.game_mut(),
        }
    }

    pub fn game_ref(&self) -> &Game {
        match self {
            GameState::SetupAndLoading(game_ctx) => game_ctx.game_ref(),
            GameState::ChooseQuestion(game_ctx) => game_ctx.game_ref(),
            GameState::DisplayQuestion(game_ctx) => game_ctx.game_ref(),
            GameState::WaitingForAnswerRequests(game_ctx) => game_ctx.game_ref(),
            GameState::AnswerAttemptReceived(game_ctx) => game_ctx.game_ref(),
            GameState::EndQuestion(game_ctx) => game_ctx.game_ref(),
            GameState::CheckEndOfRound(game_ctx) => game_ctx.game_ref(),
            GameState::CalcStatsAndStartNextRound(game_ctx) => game_ctx.game_ref(),
        }
    }

    pub fn state_name(&self) -> &str {
        match self {
            GameState::SetupAndLoading(_) => "SetupAndLoading",
            GameState::ChooseQuestion(_) => "ChooseQuestion",
            GameState::DisplayQuestion(_) => "DisplayQuestion",
            GameState::WaitingForAnswerRequests(_) => "WaitingForAnswerRequests",
            GameState::AnswerAttemptReceived(_) => "AnswerAttemptReceived",
            GameState::EndQuestion(_) => "EndQuestion",
            GameState::CheckEndOfRound(_) => "CheckEndOfRound",
            GameState::CalcStatsAndStartNextRound(_) => "CalcStatsAndStartNextRound",
        }
    }

    pub fn from_name_and_game(name: &str, game: Game) -> GameState {
        let context = GameContext::<SetupAndLoading>::new_with_game(game);
        match name {
            "ChooseQuestion" => GameState::ChooseQuestion(context.transition()),
            "DisplayQuestion" => GameState::DisplayQuestion(context.transition()),
            "WaitingForAnswerRequests" => GameState::WaitingForAnswerRequests(context.transition()),
            "AnswerAttemptReceived" => GameState::AnswerAttemptReceived(context.transition()),
            "EndQuestion" => GameState::EndQuestion(context.transition()),
            "CheckEndOfRound" => GameState::CheckEndOfRound(context.transition()),
            "CalcStatsAndStartNextRound" => {
                GameState::CalcStatsAndStartNextRound(context.transition())
            }
            _ => GameState::SetupAndLoading(context),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::SetupAndLoading(GameContext::default())
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OldGameState {
    #[default]
    SetupAndLoading,
    ChooseQuestion,
    DisplayQuestion,
}
