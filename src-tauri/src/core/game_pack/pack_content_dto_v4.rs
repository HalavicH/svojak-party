use serde::{Deserialize, Serialize};
use crate::core::game_pack::pack_content_dto::{InfoDto, RightDto, };

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
