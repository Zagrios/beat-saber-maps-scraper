use serde::{Serialize, Deserialize};
use crate::api::beat_saver::models::models::MapDetail;

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub docs: Vec<MapDetail>,
    pub redirect: Option<String>,
}

#[derive(Serialize)]
pub enum SearchRequestSort {
    FIRST_PUBLISHED,
    UPDATED,
    LAST_PUBLISHED,
    CREATED,
    CURATED,
}

impl SearchRequestSort {
    pub fn to_string(&self) -> String {
        match self {
            SearchRequestSort::FIRST_PUBLISHED => "FIRST_PUBLISHED",
            SearchRequestSort::UPDATED => "UPDATED",
            SearchRequestSort::LAST_PUBLISHED => "LAST_PUBLISHED",
            SearchRequestSort::CREATED => "CREATED",
            SearchRequestSort::CURATED =>  "CURATED",
        }.to_string()
    }
}

#[derive(Serialize)]
pub struct SearchRequestParams {
    pub after: Option<String>,
    pub automapper: Option<bool>,
    pub before: Option<String>,
    pub pageSize: Option<u32>,
    pub sort: Option<SearchRequestSort>,
    pub verified: Option<String>,
}