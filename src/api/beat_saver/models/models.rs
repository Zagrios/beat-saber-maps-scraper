use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapDetail {
    pub automapper: bool,
    pub bookmarked: Option<bool>,
    #[serde(skip_serializing, skip_deserializing)]
    pub collaborators: Option<Vec<UserDetail>>,
    pub createdAt: Option<String>,
    pub curatedAt: Option<String>,
    pub curator: Option<UserDetail>,
    pub declaredAi: String,
    pub deletedAt: Option<String>,
    pub description: String,
    pub id: String,
    pub lastPublishedAt: String,
    pub metadata: MapDetailMetadata,
    pub name: String,
    pub qualified: bool,
    pub ranked: bool,
    pub stats: MapStats,
    pub tags: Option<Vec<String>>,
    pub updatedAt: String,
    pub uploaded: String,
    pub uploader: UserDetail,
    pub versions: Vec<MapVersion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapVersion {
    pub coverURL: String,
    pub createdAt: String,
    pub diffs: Vec<MapDiff>,
    pub downloadURL: String,
    pub feedback: Option<String>,
    pub hash: String,
    pub key: Option<String>,
    pub previewURL: String,
    pub sageScore: Option<f32>,
    pub schelduledAt: Option<String>,
    pub state: String,
    pub testPlayAt: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub testPlays: Option<Vec<MapTestplay>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapDiff {
    pub bombs: i32,
    pub characteristic: String,
    pub chroma: Option<bool>,
    pub cinema: Option<bool>,
    pub difficulty: String,
    pub events: i32,
    pub label: Option<String>,
    pub length: f32,
    pub maxScore: i32,
    pub me: Option<bool>,
    pub ne: Option<bool>,
    pub njs: f32,
    pub notes: i32,
    pub nps: f32,
    pub obstacles: i32,
    pub offset: f32,
    #[serde(skip_serializing, skip_deserializing)]
    pub paritySummary: MapParitySummary,
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
pub struct MapTestplay {
    pub createdAt: String,
    pub feedback: Option<String>,
    pub feedbackAt: Option<String>,
    pub user: UserDetail,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct MapDetailMetadata {
    pub bpm: f32,
    pub duration: i32,
    pub levelAuthorName: String,
    pub songAuthorName: String,
    pub songName: String,
    pub songSubName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapStats {
    pub downloads: i32,
    pub downvotes: i32,
    pub plays: i32,
    pub reviews: Option<i32>,
    pub score: f32,
    pub scoreOneDP: Option<f32>,
    pub sentiment: Option<String>,
    pub upvotes: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetail {
    pub admin: bool,
    pub avatar: String,
    pub curator: bool,
    pub curatorTab: Option<bool>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub followData: Option<UserFollowData>,
    pub hash: Option<String>,
    pub id: i32,
    pub name: String,
    pub patreon: Option<String>,
    pub playlistUrl: String,
    pub stats: Option<UserStats>,
    pub suspendedAt: Option<String>,
    pub testplay: Option<bool>,
    pub r#type: String,
    pub uniqueSet: Option<bool>,
    pub uploadLimit: Option<i32>,
    pub verifiedMapper: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowData {
    pub curation: bool,
    pub followers: i32,
    pub follows: i32,
    pub upload: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStats {
    pub avgBpm: f32,
    pub avgDuration: f32,
    pub avgScore: f32,
    pub diffStats: UserDiffStats,
    pub firstUpload: String,
    pub lastUpload: String,
    pub rankedMaps: i32,
    pub totalDownvotes: i32,
    pub totalMaps: i32,
    pub totalUpvotes: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDiffStats {
    pub easy: i32,
    pub expert: i32,
    pub expertPlus: i32,
    pub hard: i32,
    pub normal: i32,
    pub total: i32
}