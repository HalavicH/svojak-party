use serde::*;

// Game entities
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub(super) enum AtomTypeDto {
    say,
    voice,
    video,
    marker,
    image,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct AtomDto {
    #[serde(default = "default_atom_type")]
    pub r#type: AtomTypeDto,
    #[serde(default = "default_atom_content")]
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ScenarioDto {
    #[serde(rename = "$value")]
    pub atoms_list: Vec<AtomDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RightDto {
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct QuestionDto {
    pub scenario: ScenarioDto,
    pub right: RightDto,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct QuestionsDto {
    #[serde(rename = "$value")]
    pub questions_list: Vec<QuestionDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ThemeDto {
    pub name: String,
    pub questions: QuestionsDto,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct ThemesDto {
    #[serde(rename = "$value")]
    pub themes_list: Vec<ThemeDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RoundDto {
    pub name: String,
    #[serde(default = "String::default")]
    pub r#type: String,
    #[serde(rename = "$value")]
    pub themes: ThemesDto,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RoundsDto {
    #[serde(rename = "$value")]
    pub rounds_list: Vec<RoundDto>,
}

// Pack information
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct AuthorDto {
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct InfoDto {
    #[serde(rename = "$value")]
    pub authors: Vec<AuthorDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct PackageDto {
    pub name: String,
    pub version: String,
    pub id: String,
    #[serde(default = "String::default")]
    pub restriction: String,
    pub date: String,
    pub difficulty: u8,
    pub info: InfoDto,
    pub rounds: RoundsDto,
}

fn default_atom_type() -> AtomTypeDto {
    AtomTypeDto::say
}

fn default_atom_content() -> String {
    "".to_string()
}
