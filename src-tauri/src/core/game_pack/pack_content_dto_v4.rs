use crate::core::game_pack::pack_content_dto::{InfoDto, RightDto};
use crate::core::game_pack::pack_content_entities::{
    Atom, Author, Info, PackContent, Question, QuestionMediaType, Round, Topic,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Game entities
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[allow(non_camel_case_types)]
pub(super) enum AtomTypeDtoV4 {
    #[default]
    say,
    voice,
    video,
    marker,
    image,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct AtomDtoV4 {
    #[serde(default = "AtomTypeDtoV4::default")]
    pub r#type: AtomTypeDtoV4,
    #[serde(default = "String::default")]
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ScenarioDtoV4 {
    #[serde(rename = "$value")]
    pub atoms_list: Vec<AtomDtoV4>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct QuestionDtoV4 {
    pub scenario: ScenarioDtoV4,
    pub right: RightDto,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct QuestionsDtoV4 {
    #[serde(rename = "$value")]
    pub questions_list: Vec<QuestionDtoV4>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ThemeDtoV4 {
    pub name: String,
    pub questions: QuestionsDtoV4,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ThemesDtoV4 {
    #[serde(rename = "$value")]
    pub themes_list: Vec<ThemeDtoV4>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RoundDtoV4 {
    pub name: String,
    #[serde(default = "String::default")]
    pub r#type: String,
    #[serde(rename = "$value")]
    pub themes: ThemesDtoV4,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RoundsDtoV4 {
    #[serde(rename = "$value")]
    pub rounds_list: Vec<RoundDtoV4>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct PackageDtoV4 {
    pub name: String,
    pub version: String,
    pub id: String,
    #[serde(default = "String::default")]
    pub restriction: String,
    pub date: String,
    pub difficulty: u8,
    pub info: InfoDto,
    pub rounds: RoundsDtoV4,
}

impl From<&AtomDtoV4> for Atom {
    fn from(value: &AtomDtoV4) -> Self {
        let media_type = match value.r#type {
            AtomTypeDtoV4::say => QuestionMediaType::Text,
            AtomTypeDtoV4::voice => QuestionMediaType::Voice,
            AtomTypeDtoV4::video => QuestionMediaType::Video,
            AtomTypeDtoV4::marker => QuestionMediaType::Marker,
            AtomTypeDtoV4::image => QuestionMediaType::Image,
        };
        Self {
            content: {
                match media_type {
                    QuestionMediaType::Voice |
                    QuestionMediaType::Video |
                    QuestionMediaType::Image => {
                        value.content[1..].to_owned()
                    }
                    QuestionMediaType::Text |
                    QuestionMediaType::Marker => { value.content.clone() }
                }
            },
            atom_type: media_type,
        }
    }
}
impl From<(String, &QuestionDtoV4)> for Question {
    fn from(tuple: (String, &QuestionDtoV4)) -> Self {
        let (topic, q) = tuple;
        Question {
            topic,
            price: q.price,
            scenario: {
                q.scenario
                    .atoms_list
                    .iter()
                    .map(Atom::from)
                    .collect::<Vec<Atom>>()
            },
            correct_answer: q.right.answer.clone(),
            question_type: Default::default(),
            is_used: false,
        }
    }
}

impl From<&ThemeDtoV4> for Topic {
    fn from(value: &ThemeDtoV4) -> Self {
        Self {
            name: value.name.clone(),
            questions: {
                value
                    .questions
                    .questions_list
                    .iter()
                    .map(|q| (q.price, Question::from((value.name.clone(), q))))
                    .collect::<HashMap<i32, Question>>()
            },
        }
    }
}

impl From<&RoundDtoV4> for Round {
    fn from(value: &RoundDtoV4) -> Self {
        let topics = value
            .themes
            .themes_list
            .iter()
            .map(|t| (t.name.clone(), Topic::from(t)))
            .collect::<HashMap<String, Topic>>();
        let question_count = Vec::from_iter(topics.values())
            .iter()
            .map(|&theme| theme.questions.len() as i32)
            .sum::<i32>();
        Self {
            name: value.name.clone(),
            round_type: value.r#type.clone(),
            topics,
            question_count,
            questions_left: question_count,
            normal_question_count: question_count,
            pip_question_count: 0,
            round_stats: Default::default(),
        }
    }
}

impl From<&PackageDtoV4> for PackContent {
    fn from(dto: &PackageDtoV4) -> Self {
        PackContent {
            name: dto.name.clone(),
            version: dto.version.clone(),
            id: dto.id.clone(),
            restriction: dto.restriction.clone(),
            date: dto.date.clone(),
            difficulty: dto.difficulty,
            info: Info {
                authors: dto
                    .info
                    .authors
                    .iter()
                    .map(|a| Author {
                        name: a.name.clone(),
                    })
                    .collect::<Vec<Author>>(),
            },
            rounds: dto
                .rounds
                .rounds_list
                .iter()
                .map(Round::from)
                .collect::<Vec<Round>>(),
        }
    }
}
