use serde::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(super) struct RightDto {
    pub answer: String,
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
