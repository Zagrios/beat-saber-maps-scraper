use serde::{Serialize, Deserialize};

use super::models::Metadata;
use super::models::LeaderboardInfoResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardInfoResponseResponseWithMetadata {
    pub metadata: Metadata,
    pub data: Vec<LeaderboardInfoResponse>
}