use crate::core::game_ctx::game::GameCtx;
use crate::core::game_ctx::state_structs::{
    AnswerAttemptReceived, ShowRoundStats, CheckEndOfRound, ChooseQuestion, DisplayQuestion,
    EndQuestion, EndTheGame, PickFirstQuestionChooser, SetupAndLoading, StartNextRound,
    WaitingForAnswerRequests,
};
use crate::core::game_ctx::GameData;

#[derive(Debug)]
pub enum GameState {
    /// Configuring players and game pack.
    /// Next state: `PickFirstQuestionChooser` (when game started)
    SetupAndLoading(GameCtx<SetupAndLoading>),

    /// When the game is instantiated & started, this is the first state.
    /// Next state: `ChooseQuestion` (when the first question chooser is selected)
    PickFirstQuestionChooser(GameCtx<PickFirstQuestionChooser>),

    /// The state where the question chooser selects a question.
    /// Next state: `DisplayQuestion` (when question is selected)
    ChooseQuestion(GameCtx<ChooseQuestion>),

    /// When the question is selected, everyone reads it but can't answer until the host allows.
    /// Next state: `WaitingForAnswerRequests` (when host presses 'Allow answer' button)
    DisplayQuestion(GameCtx<DisplayQuestion>),

    /// The host allowed answering the question, and now players can send answer requests.
    /// Next state: `AnswerAttemptReceived` (when the first answer request is received)
    WaitingForAnswerRequests(GameCtx<WaitingForAnswerRequests>),

    /// The quickest player pressed the 'Answer' button first, and now they have the right to try answering the question.
    /// Next state: `EndQuestion` (when the verbal answer from the player is correct or no players are left after wrong answers)
    ///         or: `DisplayQuestion` (when the verbal answer from the player is wrong and remaining players are available)
    AnswerAttemptReceived(GameCtx<AnswerAttemptReceived>),

    /// Any player answered the question correctly or all players answered the question wrong.
    /// In this case, the correct answer is displayed on the screen.
    /// At this point, intermediate player stats can be displayed.
    /// Next state: `CheckEndOfRound` (when the host presses "Next Question")
    EndQuestion(GameCtx<EndQuestion>),

    /// Check if the round is over. If all questions in the round are answered, proceed to round-end actions.
    /// Next state: `CalcRoundStats` (when the round is over)
    ///         or: `ChooseQuestion` (when the round is continuing)
    CheckEndOfRound(GameCtx<CheckEndOfRound>),

    /// Display round statistics, eliminate players with negative scores, etc.
    /// Next state: `StartNextRound` (when a new round is available)
    ///         or: `EndTheGame` (when all rounds are played)
    ShowRoundStats(GameCtx<ShowRoundStats>),

    /// Start the next round by resetting game state and proceeding to question selection.
    /// Next state: `ChooseQuestion` (when the first question of the new round is picked)
    StartNextRound(GameCtx<StartNextRound>),

    /// The game is over, and the final results are displayed.
    EndTheGame(GameCtx<EndTheGame>),
}

impl GameState {
    pub fn show_state_mismatch(&mut self, expected: &str) -> String {
        format!(
            "Expected game state of '{}', found: {}",
            expected,
            self.name()
        )
    }

    pub fn game_mut(&mut self) -> &mut GameData {
        match self {
            GameState::SetupAndLoading(game_ctx) => game_ctx.game_mut(),
            GameState::PickFirstQuestionChooser(game_ctx) => game_ctx.game_mut(),
            GameState::ChooseQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::DisplayQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::WaitingForAnswerRequests(game_ctx) => game_ctx.game_mut(),
            GameState::AnswerAttemptReceived(game_ctx) => game_ctx.game_mut(),
            GameState::EndQuestion(game_ctx) => game_ctx.game_mut(),
            GameState::CheckEndOfRound(game_ctx) => game_ctx.game_mut(),
            GameState::ShowRoundStats(game) => game.game_mut(),
            GameState::StartNextRound(game) => game.game_mut(),
            GameState::EndTheGame(game) => game.game_mut(),
        }
    }

    pub fn game_ctx_ref(&self) -> &GameData {
        match self {
            GameState::SetupAndLoading(game_ctx) => game_ctx.game_ref(),
            GameState::PickFirstQuestionChooser(game_ctx) => game_ctx.game_ref(),
            GameState::ChooseQuestion(game_ctx) => game_ctx.game_ref(),
            GameState::DisplayQuestion(game_ctx) => game_ctx.game_ref(),
            GameState::WaitingForAnswerRequests(game_ctx) => game_ctx.game_ref(),
            GameState::AnswerAttemptReceived(game_ctx) => game_ctx.game_ref(),
            GameState::EndQuestion(game_ctx) => game_ctx.game_ref(),
            GameState::CheckEndOfRound(game_ctx) => game_ctx.game_ref(),
            GameState::ShowRoundStats(game) => game.game_ref(),
            GameState::StartNextRound(game) => game.game_ref(),
            GameState::EndTheGame(game) => game.game_ref(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            GameState::SetupAndLoading(_) => "SetupAndLoading",
            GameState::PickFirstQuestionChooser(_) => "PickFirstQuestionChooser",
            GameState::ChooseQuestion(_) => "ChooseQuestion",
            GameState::DisplayQuestion(_) => "DisplayQuestion",
            GameState::WaitingForAnswerRequests(_) => "WaitingForAnswerRequests",
            GameState::AnswerAttemptReceived(_) => "AnswerAttemptReceived",
            GameState::EndQuestion(_) => "EndQuestion",
            GameState::CheckEndOfRound(_) => "CheckEndOfRound",
            GameState::ShowRoundStats(_) => "ShowRoundStats",
            GameState::StartNextRound(_) => "StartNextRound",
            GameState::EndTheGame(_) => "EndTheGame",
        }
    }

    pub fn from_name_and_game(name: &str, game: GameData) -> GameState {
        let context: GameCtx<SetupAndLoading> = GameCtx::<SetupAndLoading>::new_with_game(game);
        match name {
            "SetupAndLoading" => GameState::SetupAndLoading(context.transition()),
            "PickFirstQuestionChooser" => GameState::PickFirstQuestionChooser(context.transition()),
            "ChooseQuestion" => GameState::ChooseQuestion(context.transition()),
            "DisplayQuestion" => GameState::DisplayQuestion(context.transition()),
            "WaitingForAnswerRequests" => GameState::WaitingForAnswerRequests(context.transition()),
            "AnswerAttemptReceived" => GameState::AnswerAttemptReceived(context.transition()),
            "EndQuestion" => GameState::EndQuestion(context.transition()),
            "CheckEndOfRound" => GameState::CheckEndOfRound(context.transition()),
            "CalcRoundStats" => GameState::ShowRoundStats(context.transition()),
            "StartNextRound" => GameState::StartNextRound(context.transition()),
            "EndTheGame" => GameState::EndTheGame(context.transition()),
            &_ => panic!("Invalid state name {}", name),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::SetupAndLoading(GameCtx::default())
    }
}
