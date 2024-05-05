use crate::api::dto::QuestionType;
use crate::core::game_entities::{GameState, Player};
use crate::game_pack::pack_content_entities::{PackContent, Round};

#[derive(Default, Debug)]
pub struct GameStats {
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
}

#[derive(Default, Debug)]
pub struct GameContext {
    /// Entities
    pub pack_content: PackContent,
    pub players: Vec<Player>,
    /// Game State
    pub game_state: GameState,
    pub round_index: usize,
    pub active_player_id: u8,
    pub click_for_answer_allowed: bool,
    pub answer_allowed: bool,
    /// Current question
    pub question_theme: String,
    pub question_price: i32,
    pub question_type: QuestionType,
    /// Stats
    pub round_stats: GameStats,
}

// trait Game {
//     fn start();
//     /// 
//     fn select_question(topic: &String, price: &i32) -> Result<(), >;
// }

impl GameContext {
    pub fn new(pack_content: PackContent, players: Vec<Player>) -> Self {
        Self {
            pack_content,
            players,
            ..GameContext::default()
        }
    }

    pub fn start(&mut self) {
        self.game_state = GameState::ChooseQuestion;
    }
}

/// Deprecated API

/// Getters / Setters
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

/// Game API
impl GameContext {
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

        self.round_stats.total_tries = 0;
        self.round_stats.total_wrong_answers = 0;
        self.round_stats.total_correct_answers = 0;

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
