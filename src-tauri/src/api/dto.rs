use crate::core::game_entities::PlayerState;
use crate::game_pack::pack_content_entities::QuestionMediaType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PackErrorData {
    pub path: String,
    pub cause: String,
    pub details: String,
}

////////// Config ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ConfigDto {
    pub hub_port: String,
    pub available_ports: Vec<String>,
    pub radio_channel: i32,
    pub players: Vec<PlayerSetupDto>,
}

////////// Players ///////////
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PlayerSetupDto {
    pub termId: u8,
    pub icon: String,
    pub name: String,
    pub isUsed: bool,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerGameDto {
    pub id: i32,
    pub playerIconPath: String,
    pub playerName: String,
    pub score: i32,
    pub state: PlayerState,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerScoreDto {
    pub id: i32,
    pub score: i32,
}

////////// Pack info ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PackInfoDto {
    pub packName: String,
    pub packAuthor: String,
    pub packRounds: i32,
    pub packTopics: i32,
    pub packQuestions: i32,
    pub packTopicList: Vec<String>,
}

////////// Round ///////////
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct RoundDto {
    pub roundName: String,
    pub roundType: String,
    pub roundTopics: Vec<TopicDto>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct TopicDto {
    pub topicName: String,
    pub questions: Vec<QuestionDto>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct QuestionDto {
    pub index: usize,
    pub price: i32,
}

////////// Round ///////////
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub enum QuestionType {
    #[default]
    Normal,
    PigInPoke,
    Auction,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct QuestionDataDto {
    pub number: i32,
    pub category: String,
    pub price: i32,
    pub questionType: QuestionType,
    pub scenario: Vec<QuestionSceneDto>,
    pub answer: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct QuestionSceneDto {
    pub mediaType: QuestionMediaType,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct RoundStatsDto {
    pub roundName: String,
    pub questionNumber: i32,
    pub normalQuestionNum: i32,
    pub pigInPokeQuestionNum: i32,
    pub totalCorrectAnswers: i32,
    pub totalWrongAnswers: i32,
    pub totalTries: i32,
    pub roundTime: String,
    pub players: Vec<PlayerStatsDto>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerStatsDto {
    pub id: i32,
    pub name: String,
    pub score: i32,
    pub playerIconPath: String,
    pub totalAnswers: i32,
    pub answeredCorrectly: i32,
    pub answeredWrong: i32,
}

////////// HUB DEBUG ///////////
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct HubRequestDto {
    pub cmd: String,
    pub param1: u32,
    pub param2: u32,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct HubResponseDto {
    pub request_frame: String,
    pub response_frame: String,
    pub generic_response_obj: String,
    pub response_obj: String,
}
