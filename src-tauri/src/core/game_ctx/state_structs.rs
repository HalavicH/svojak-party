use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SetupAndLoading {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PickFirstQuestionChooser {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChooseQuestion {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DisplayQuestion {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WaitingForAnswerRequests {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnswerAttemptReceived {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EndQuestion {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CheckEndOfRound {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CalcStatsAndStartNextRound {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CalcRoundStats {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StartNextRound {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EndTheGame {}

