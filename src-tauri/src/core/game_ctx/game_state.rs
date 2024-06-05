use crate::core::game_ctx::game::Game;
use crate::core::game_ctx::state_structs::{
    AnswerAttemptReceived, CalcStatsAndStartNextRound, CheckEndOfRound, ChooseQuestion,
    DisplayQuestion, EndQuestion, SetupAndLoading, WaitingForAnswerRequests,
};
use crate::core::game_ctx::GameCtx;

#[derive(Debug)]
pub enum GameState {
    /// Configuring players and game pack.
    /// Next state: `ChooseQuestion` (when game started)
    SetupAndLoading(Game<SetupAndLoading>),

    /// When game instantiated & started this is the first state
    /// Next state: `DisplayQuestion` (when question selected)
    PickFirstQuestionChooser(Game<SetupAndLoading>),

    /// When game instantiated & started this is the first state
    /// Next state: `DisplayQuestion` (when question selected)
    ChooseQuestion(Game<ChooseQuestion>),

    /// When question selected everyone reads it, but can't answer until host allows
    /// Next state: `WaitingForAnswerRequests` (when host press 'Allow answer' button)
    DisplayQuestion(Game<DisplayQuestion>),

    /// Host allowed answering the question, from now players can send answer requests
    /// Next state: `AnswerAttemptReceived` (when first event from active player received)
    WaitingForAnswerRequests(Game<WaitingForAnswerRequests>),

    /// The quickest player pushed 'Answer' button first, and now he has right to try answer the question
    /// Next state: `DisplayQuestion` (when verbal answer from player was wrong and remaining players are available)
    ///        or : `EndQuestion` (when verbal answer from player was correct or no players left after wrong answers)
    AnswerAttemptReceived(Game<AnswerAttemptReceived>),

    /// Any player answered the question correctly or all players answered question wrong.
    /// In this case correct answer is displayed on the screen
    /// At this point intermediate player stats can be displayed
    /// Next state: `CheckEndOfRound` (when host presses "Next Question")
    EndQuestion(Game<EndQuestion>),

    /// Check if the round is over. If all questions in the round are answered, proceed to round-end actions.
    /// Next state: `CalcStatsAndStartNextRound` (when round is over)
    ///        or : `DisplayQuestion` (when round is not over)
    CheckEndOfRound(Game<CheckEndOfRound>),

    /// Display round statistics, eliminate players with negative scores, etc.
    /// Start the next round by resetting game state and proceeding to question selection.
    /// Next state: `ChooseQuestion` (when host presses "Start Next Round")
    CalcStatsAndStartNextRound(Game<CalcStatsAndStartNextRound>),
}

impl GameState {
    pub fn show_state_mismatch(&mut self, expected: &str) -> String {
        format!(
            "Expected game state of '{}', found: {}",
            expected,
            self.state_name()
        )
    }

    pub fn game_mut(&mut self) -> &mut GameCtx {
        match self {
            GameState::SetupAndLoading(game_ctx) => game_ctx.game_mut(),
            GameState::PickFirstQuestionChooser(game_ctx) => game_ctx.game_mut(),
            GameState::ChooseQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::DisplayQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::WaitingForAnswerRequests(game_ctx) => game_ctx.game_mut(),
            GameState::AnswerAttemptReceived(game_ctx) => game_ctx.game_mut(),
            GameState::EndQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::CheckEndOfRound(game_ctx) => game_ctx.game_mut(),
            GameState::CalcStatsAndStartNextRound(game_ctx) => game_ctx.game_mut(),
        }
    }

    pub fn game_ctx_ref(&self) -> &GameCtx {
        match self {
            GameState::SetupAndLoading(game_ctx) => game_ctx.game_ref(),
            GameState::PickFirstQuestionChooser(game_ctx) => game_ctx.game_ref(),
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
            GameState::PickFirstQuestionChooser(_) => "PickFirstQuestionChooser",
            GameState::ChooseQuestion(_) => "ChooseQuestion",
            GameState::DisplayQuestion(_) => "DisplayQuestion",
            GameState::WaitingForAnswerRequests(_) => "WaitingForAnswerRequests",
            GameState::AnswerAttemptReceived(_) => "AnswerAttemptReceived",
            GameState::EndQuestion(_) => "EndQuestion",
            GameState::CheckEndOfRound(_) => "CheckEndOfRound",
            GameState::CalcStatsAndStartNextRound(_) => "CalcStatsAndStartNextRound",
        }
    }

    pub fn from_name_and_game(name: &str, game: GameCtx) -> GameState {
        let context = Game::<SetupAndLoading>::new_with_game(game);
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
        GameState::SetupAndLoading(Game::default())
    }
}
