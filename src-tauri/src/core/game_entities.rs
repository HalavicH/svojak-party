use crate::core::game_entities::GameplayError::PackElementNotPresent;
use crate::core::game_entities::HubStatus::Detected;
use crate::core::game_pack::pack_content_entities::Question;
use crate::player_server::entities::PsPlayer;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const DEFAULT_ICON: &str = "default";

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Idle,
    QuestionChooser,
    Target, // for pig in poke mode
    Answering,
    Inactive,
    Dead,
    AnsweredCorrectly,
    AnsweredWrong,
}

#[derive(Default, Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub score: i32,
    pub answered_correctly: i32,
    pub answered_wrong: i32,
    pub total_tries: i32,
}

#[derive(Debug, Default, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub icon: String,
    pub term_id: u8,
    pub is_used: bool, // todo: remove
    pub state: PlayerState,
    pub stats: PlayerStats,
}

impl From<PsPlayer> for Player {
    fn from(player: PsPlayer) -> Self {
        Self {
            name: player
                .name
                .unwrap_or_else(|| format!("Player {}", player.id)),
            icon: player.icon,
            term_id: player.id as u8,
            is_used: true,
            state: PlayerState::Idle,
            stats: Default::default(),
        }
    }
}

impl Player {
    pub(crate) fn can_answer(&self) -> bool {
        match self.state {
            PlayerState::Idle
            | PlayerState::QuestionChooser
            | PlayerState::Target
            | PlayerState::Answering => true,
            PlayerState::Inactive
            | PlayerState::Dead
            | PlayerState::AnsweredCorrectly
            | PlayerState::AnsweredWrong => false,
        }
    }

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
        self.stats.answered_correctly += 1;
        self.stats.total_tries += 1;
        self.stats.score += question.price;
    }

    pub fn answered_wrong(&mut self, question: &Question) {
        self.state = PlayerState::AnsweredWrong;
        self.stats.answered_wrong += 1;
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
    TopicNotPresent,
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

    #[error("{0}")]
    PackElementNotPresent(GamePackError),
    #[error("Player {0} not present")]
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
    #[error("Player not found")]
    PlayerNotFound,
}

impl From<GamePackError> for GameplayError {
    fn from(value: GamePackError) -> Self {
        PackElementNotPresent(value)
    }
}
