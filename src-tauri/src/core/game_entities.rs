use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::dto::QuestionType;
use crate::core::game_entities::HubStatus::Detected;
use crate::game_pack::pack_content_entities::{PackContent, Round};

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
    /// Content
    pub pack_content: PackContent,
    /// Game State
    pub round_index: usize,
    pub active_player_id: u8,
    pub game_state: GameState,
    pub click_for_answer_allowed: bool,
    pub answer_allowed: bool,
    /// Current question
    pub question_theme: String,
    pub question_price: i32,
    pub question_type: QuestionType,
    /// Stats
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}

impl GameContext {
    /// Getters / Setters
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

    /// Game API
    pub fn get_current_round(&self) -> &Round {
        let index = self.round_index;
        let round = self
            .pack_content
            .rounds
            .get(index)
            .expect(&format!("Expected to have round #{}", index));
        round
    }

    pub fn is_already_last_round(&self) -> bool {
        (self.pack_content.rounds.len() - 1) == self.round_index
    }

    pub fn load_next_round(&mut self) {
        if self.is_already_last_round() {
            log::error!("Already final round");
            return;
        }

        self.round_index += 1;
        let index = self.round_index;
        let round: &Round = self
            .pack_content
            .rounds
            .get(index)
            .expect(&format!("Expected to have round #{}", index));
        log::info!("Next round name {}", round.name);

        self.total_tries = 0;
        self.total_wrong_answers = 0;
        self.total_correct_answers = 0;

        if self.is_already_last_round() {
            todo!("Wire kill_players_with_negative_balance");
            // self.kill_players_with_negative_balance();
        }
    }

    pub fn get_current_round_mut(&mut self) -> &mut Round {
        let index = self.round_index;
        let round = self
            .pack_content
            .rounds
            .get_mut(index)
            .expect(&format!("Expected to have round #{}", index));
        round
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
