use serde::*;

// Game entities
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum AtomTypeDto {
    say,
    voice,
    video,
    marker,
    image,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AtomDto {
    #[serde(default = "default_atom_type")]
    pub r#type: AtomTypeDto,
    #[serde(default = "default_atom_content")]
    #[serde(rename = "$value")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ScenarioDto {
    #[serde(rename = "$value")]
    pub atoms_list: Vec<AtomDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RightDto {
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionDto {
    pub scenario: ScenarioDto,
    pub right: RightDto,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionsDto {
    #[serde(rename = "$value")]
    pub questions_list: Vec<QuestionDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ThemeDto {
    pub name: String,
    pub questions: QuestionsDto,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ThemesDto {
    #[serde(rename = "$value")]
    pub themes_list: Vec<ThemeDto>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RoundDto {
    pub name: String,
    #[serde(default = "String::default")]
    pub r#type: String,
    #[serde(rename = "$value")]
    pub themes: ThemesDto,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RoundsDto {
    #[serde(rename = "$value")]
    pub rounds_list: Vec<RoundDto>,
}

// Pack information
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthorDto {
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct InfoDto {
    #[serde(rename = "$value")]
    pub authors: Vec<AuthorDto>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PackageDto {
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
