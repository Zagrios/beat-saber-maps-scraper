use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardInfo {
    pub id: i32,
    pub song_hash: String,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub difficulty: Difficulty,
    pub max_score: u64,
    pub created_date: Option<String>,
    pub ranked_date: Option<String>,
    pub qualified_date: Option<String>,
    pub loved_date: Option<String>,
    pub ranked: bool,
    pub qualified: bool,
    pub loved: bool,
    #[serde(rename = "maxPP")]
    pub max_pp: f32,
    pub stars: f32,
    pub positive_modifiers: bool,
    pub plays: u64,
    pub daily_plays: u64,
    pub cover_image: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub player_score: Option<()>,
    pub difficulties: Option<Vec<Difficulty>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub leaderboard_id: i32,
    pub difficulty: i32,
    pub game_mode: String,
    pub difficulty_raw: String,
}

impl Difficulty {
    pub fn get_difficulty_string(&self) -> String {
        return match self.difficulty {
            1 => "Easy".to_string(),
            3 => "Normal".to_string(),
            5 => "Hard".to_string(),
            7 => "Expert".to_string(),
            9 => "ExpertPlus".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn get_characteristic_string(&self) -> String {
        // remove "Solo" from gamemode
        return self.game_mode.replace("Solo", "");
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    total: i32,
    page: i32,
    items_per_page: i32,
}