use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PsPlayer {
    pub id: i32,
    pub name: String,
    pub icon: String, // base64
    // pub is_active: bool,
}



