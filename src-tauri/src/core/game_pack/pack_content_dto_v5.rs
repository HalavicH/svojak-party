use crate::core::game_pack::pack_content_dto::{InfoDto, RightDto};
use crate::core::game_pack::pack_content_entities::{
    Atom, Author, Info, PackContent, Question, QuestionMediaType, Round, Topic,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Game entities
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[allow(non_camel_case_types)]
pub(super) enum ItemTypeDtoV4 {
    #[default]
    say,
    audio,
    video,
    marker,
    image,
}

//  <item type="image" isRef="True">100 - Ponyville.png</item>
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ItemDtoV5 {
    #[serde(default = "String::default")]
    #[serde(rename = "$value")]
    pub content: String,
    #[serde(default = "ItemTypeDtoV4::default")]
    pub r#type: ItemTypeDtoV4,
    #[serde(rename = "isRef")]
    pub is_ref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[allow(non_camel_case_types)]
pub enum ParamTypeV5 {
    #[default]
    content,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ParamDtoV5 {
    pub r#type: Option<ParamTypeV5>,
    // #[serde(default = "String::default")]
    pub name: String,
    #[serde(rename = "$value")]
    pub item: ItemDtoV5,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ParamsDtoV5 {
    #[serde(rename = "$value")]
    pub params_list: Vec<ParamDtoV5>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct QuestionDtoV5 {
    pub params: ParamsDtoV5,
    pub right: RightDto,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct QuestionsDtoV5 {
    #[serde(rename = "$value")]
    pub questions_list: Vec<QuestionDtoV5>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ThemeDtoV5 {
    pub name: String,
    pub questions: QuestionsDtoV5,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ThemesDtoV5 {
    #[serde(rename = "$value")]
    pub themes_list: Vec<ThemeDtoV5>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RoundDtoV5 {
    pub name: String,
    #[serde(default = "String::default")]
    pub r#type: String,
    #[serde(rename = "$value")]
    pub themes: ThemesDtoV5,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RoundsDtoV5 {
    #[serde(rename = "$value")]
    pub rounds_list: Vec<RoundDtoV5>,
}

// TODO: map
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct Tag {
    #[serde(rename = "$value")]
    pub tags: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct Tags {
    // #[serde(rename = "$value")]
    // pub tags_list: Tag,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct PackageDtoV5 {
    pub name: String,
    pub version: String,
    pub id: String,
    #[serde(default = "String::default")]
    pub restriction: String,
    pub date: String,
    pub publisher: String,
    pub difficulty: u8,
    pub tags: Tags,
    pub info: InfoDto,
    pub rounds: RoundsDtoV5,
}

impl From<&ParamDtoV5> for Atom {
    fn from(value: &ParamDtoV5) -> Self {
        Self {
            atom_type: {
                match value.item.r#type {
                    ItemTypeDtoV4::say => QuestionMediaType::Text,
                    ItemTypeDtoV4::audio => QuestionMediaType::Voice,
                    ItemTypeDtoV4::video => QuestionMediaType::Video,
                    ItemTypeDtoV4::marker => QuestionMediaType::Marker,
                    ItemTypeDtoV4::image => QuestionMediaType::Image,
                }
            },
            content: value.item.content.clone(),
        }
    }
}
impl From<(String, &QuestionDtoV5)> for Question {
    fn from(tuple: (String, &QuestionDtoV5)) -> Self {
        let (topic, q) = tuple;
        Question {
            topic,
            price: q.price,
            scenario: {
                q.params
                    .params_list
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

impl From<&ThemeDtoV5> for Topic {
    fn from(value: &ThemeDtoV5) -> Self {
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

impl From<&RoundDtoV5> for Round {
    fn from(value: &RoundDtoV5) -> Self {
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

impl From<&PackageDtoV5> for PackContent {
    fn from(dto: &PackageDtoV5) -> Self {
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
