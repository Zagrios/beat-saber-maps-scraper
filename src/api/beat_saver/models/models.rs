use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapDetail {
    pub automapper: bool,
    pub bl_qualified: bool,
    pub bl_ranked: bool,
    pub bookmarked: Option<bool>,
    #[serde(skip_serializing, skip_deserializing)]
    pub collaborators: Option<Vec<UserDetail>>,
    pub created_at: Option<String>,
    pub curated_at: Option<String>,
    pub curator: Option<UserDetail>,
    pub declared_ai: String,
    pub deleted_at: Option<String>,
    pub description: String,
    pub id: String,
    pub last_published_at: String,
    pub metadata: MapDetailMetadata,
    pub name: String,
    pub nsfw: Option<bool>,
    pub qualified: bool,
    pub ranked: bool,
    pub stats: MapStats,
    pub tags: Option<Vec<String>>,
    pub updated_at: String,
    pub uploaded: String,
    pub uploader: UserDetail,
    pub versions: Vec<MapVersion>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapVersion {
    #[serde(rename = "coverURL")] // rename from coverURL
    pub cover_url: String,
    pub created_at: String,
    pub diffs: Vec<MapDiff>,
    #[serde(rename = "downloadURL")] // rename from downloadURL
    pub download_url: String,
    pub feedback: Option<String>,
    pub hash: String,
    pub key: Option<String>,
    #[serde(rename = "previewURL")] // rename from previewURL
    pub preview_url: String,
    pub sage_score: Option<f32>,
    pub schelduled_at: Option<String>,
    pub state: String,
    pub test_play_at: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub test_plays: Option<Vec<MapTestplay>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapDiff {
    pub bl_stars: Option<f32>,
    pub bombs: i32,
    pub characteristic: String,
    pub chroma: Option<bool>,
    pub cinema: Option<bool>,
    pub difficulty: String,
    pub events: i32,
    pub label: Option<String>,
    pub length: f32,
    pub max_score: i32,
    pub me: Option<bool>,
    pub ne: Option<bool>,
    pub njs: f32,
    pub notes: i32,
    pub nps: f32,
    pub obstacles: i32,
    pub offset: f32,
    #[serde(skip_serializing, skip_deserializing)]
    pub parity_summary: MapParitySummary,
    pub seconds: f32,
    pub stars: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct MapParitySummary {
    pub errors: i32,
    pub resets: i32,
    pub warns: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapTestplay {
    pub created_at: String,
    pub feedback: Option<String>,
    pub feedback_at: Option<String>,
    pub user: UserDetail,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct MapDetailMetadata {
    pub bpm: f32,
    pub duration: i32,
    pub level_author_name: String,
    pub song_author_name: String,
    pub song_name: String,
    pub song_sub_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapStats {
    pub downloads: i32,
    pub downvotes: i32,
    pub plays: i32,
    pub reviews: Option<i32>,
    pub score: f32,
    pub score_one_dp: Option<f32>,
    pub sentiment: Option<String>,
    pub upvotes: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub admin: bool,
    pub avatar: String,
    pub curator: bool,
    pub curator_tab: Option<bool>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub follow_data: Option<UserFollowData>,
    pub hash: Option<String>,
    pub id: i32,
    pub name: String,
    pub patreon: Option<String>,
    pub playlist_url: String,
    pub stats: Option<UserStats>,
    pub suspended_at: Option<String>,
    pub testplay: Option<bool>,
    pub r#type: String,
    pub unique_set: Option<bool>,
    pub upload_limit: Option<i32>,
    pub verified_mapper: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFollowData {
    pub curation: bool,
    pub followers: i32,
    pub follows: i32,
    pub upload: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub avg_bpm: f32,
    pub avg_duration: f32,
    pub avg_score: f32,
    pub diff_stats: UserDiffStats,
    pub first_upload: String,
    pub last_upload: String,
    pub ranked_maps: i32,
    pub total_downvotes: i32,
    pub total_maps: i32,
    pub total_upvotes: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDiffStats {
    pub easy: i32,
    pub expert: i32,
    pub expert_plus: i32,
    pub hard: i32,
    pub normal: i32,
    pub total: i32
}