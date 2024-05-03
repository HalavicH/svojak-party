use crate::api::dto::QuestionType;
use serde::Serialize;
use std::collections::HashMap;

// Game entities
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum QuestionMediaType {
    Say,
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
    pub scenario: Vec<Atom>,
    pub right_answer: String,
    pub question_type: QuestionType,
    pub price: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub name: String,
    pub questions: HashMap<i32, Question>,
}

impl Theme {
    pub fn pop_question(&mut self, price: &i32) -> Option<Question> {
        self.questions.remove(price)
    }

    pub fn get_question(&self, price: &i32) -> Option<&Question> {
        self.questions.get(price)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Round {
    pub name: String,
    pub round_type: String,
    pub themes: HashMap<String, Theme>,
    pub question_count: i32,
    pub normal_question_count: i32,
    pub pip_question_count: i32,
    pub questions_left: i32,
}

impl Round {
    pub fn decrement_round(&mut self) {
        self.questions_left -= 1;
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
