use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
pub struct GameIdList {
    pub ids: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "lastChange")]
    pub last_change: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    pub seed: String,
    pub size: u64,
}
