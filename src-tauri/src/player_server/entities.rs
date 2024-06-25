use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PsPlayer {
    pub id: i32,
    pub name: Option<String>,
    pub icon: String, // base64
    // pub is_active: bool,
}



