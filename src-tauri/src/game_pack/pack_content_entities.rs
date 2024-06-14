use crate::api::dto::QuestionType;
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
    pub correct_answer: String,
    pub question_type: QuestionType,
    pub price: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Topic {
    pub name: String,
    pub questions: HashMap<i32, Question>,
}

impl Topic {
    pub fn pop_question_by_price(&mut self, price: &i32) -> Option<Question> {
        self.questions.remove(price)
    }

    pub fn question_by_price(&self, price: &i32) -> Option<&Question> {
        self.questions.get(price)
    }
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
}

impl Round {
    pub fn pop_question(&mut self, topic_name: &str, price: i32) -> Option<Question> {
        let Some(topic) = self.topics.get_mut(topic_name) else {
            log::error!(
                "Topic with name: {} not found in round with name: {}",
                topic_name,
                self.name
            );
            return None;
        };

        self.questions_left -= 1;
        log::debug!("Questions left: {}", self.questions_left);
        topic.questions.remove(&price)
    }

    pub fn is_over(&self) -> bool {
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
