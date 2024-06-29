use crate::host_api::dto::QuestionType;
use serde::Serialize;
use std::collections::HashMap;

// Game entities
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum QuestionMediaType {
    Text,
    Voice,
    Video,
    Marker,
    Image,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub atom_type: QuestionMediaType,
    pub content: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Question {
    pub topic: String,
    pub scenario: Vec<Atom>,
    pub correct_answer: Vec<Atom>,
    pub question_type: QuestionType,
    pub price: i32,
    pub is_used: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Topic {
    pub name: String,
    pub questions: HashMap<i32, Question>,
}

///// LEGACY
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RoundStats {
    pub questions_played: i32,
    pub normal_questions_played: i32,
    pub pip_questions_played: i32,
    pub total_correct_answers: i32,
    pub total_wrong_answers: i32,
    pub total_tries: i32,
    pub round_time: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Round {
    pub name: String,
    pub round_type: String,
    pub topics: HashMap<String, Topic>,
    pub question_count: i32,
    pub normal_question_count: i32,
    pub pip_question_count: i32,
    pub questions_left: i32,
    pub round_stats: RoundStats,
}

impl Round {
    pub fn is_round_over(&self) -> bool {
        self.questions_left == 0
    }
}

// Pack information
#[derive(Debug, PartialEq, Clone)]
pub struct Author {
    pub name: String,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Info {
    pub authors: Vec<Author>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PackContent {
    pub name: String,
    pub version: String,
    pub id: String,
    pub restriction: String,
    pub date: String,
    pub difficulty: u8,
    pub info: Info,
    pub rounds: Vec<Round>,
}
