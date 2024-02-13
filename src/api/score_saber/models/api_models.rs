use serde::{Deserialize, Serialize};
use crate::api::score_saber::models::models::{LeaderboardInfo, Metadata};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardInfoCollection {
    pub leaderboards: Vec<LeaderboardInfo>,
    pub metadata: Option<Metadata>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardRequestParams {
    pub search: Option<String>,
    pub verified: Option<bool>,
    pub ranked: Option<bool>,
    pub qualified: Option<bool>,
    pub loved: Option<bool>,
    pub minStar: Option<f32>,
    pub maxStar: Option<f32>,
    pub category: Option<LeaderboardRequestCategory>,
    pub sort: Option<LeaderboardRequestSort>,
    pub unique: Option<bool>,
    pub page: u32,
    pub withMetadata: Option<bool>,
}

impl LeaderboardRequestParams {
    pub fn default() -> LeaderboardRequestParams {
        LeaderboardRequestParams {
            search: None,
            verified: None,
            ranked: None,
            qualified: None,
            loved: None,
            minStar: None,
            maxStar: None,
            category: None,
            sort: None,
            unique: None,
            page: 0,
            withMetadata: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LeaderboardRequestCategory {
    TRENDING = 0,
    DATE_RANKED = 1,
    SCORES_SET = 2,
    STAR_DIFFICULTY = 3,
    AUTHOR = 4,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LeaderboardRequestSort {
    DESC = 0,
    ASC = 1,
}