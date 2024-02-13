use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub itemsPerPage: i32,
    pub page: i32,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardInfoResponse {
    pub id: Option<String>,
    pub song: Song,
    pub difficulty: DifficultyDescription,
    pub plays: i32,
    pub positiveVotes: i32,
    pub starVotes: i32,
    pub negativeVotes: i32,
    pub voteStars: f32,
    #[serde(skip_serializing, skip_deserializing)]
    pub clan: Option<Clan>,
    pub clanRankingContested: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub myScore: Option<()>,
    #[serde(skip_serializing, skip_deserializing)]
    pub qualification: Option<()>,
    #[serde(skip_serializing, skip_deserializing)]
    pub reweight: Option<()>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    pub id: Option<String>,
    pub hash: Option<String>,
    pub name: Option<String>,
    pub sub_name: Option<String>,
    pub author: Option<String>,
    pub mapper: Option<String>,
    pub mapperId: Option<i32>,
    pub collaboratorIds: Option<String>,
    pub coverImage: Option<String>,
    pub fullCoverImage: Option<String>,
    pub downloadUrl: Option<String>,
    pub bpm: Option<f64>,
    pub duration: Option<f64>,
    pub tags: Option<String>,
    pub uploadTime: Option<i32>,
    pub difficulties: Option<Vec<DifficultyDescription>>,
    pub externalsStatuses: Option<Vec<ExternalStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clan {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct DifficultyDescription {
    pub id: i32,
    pub value: i32,
    pub mode: i32,
    pub difficultyName: Option<String>,
    pub modeName: Option<String>,
    pub status: Option<u8>,
    pub modifierValues: Option<ModifiersMap>,
    pub modifiersRating: Option<ModifiersRating>,
    pub nominatedTime: Option<i32>,
    pub qualifiedTime: Option<i32>,
    pub rankedTime: Option<i32>,
    pub stars: Option<f32>,
    pub predictedAcc: Option<f32>,
    pub passRating: Option<f32>,
    pub accRating: Option<f32>,
    pub techRating: Option<f32>,
    pub r#type: Option<i32>,
    pub njs: Option<f32>,
    pub nps: Option<f32>,
    pub notes: Option<i32>,
    pub bombs: Option<i32>,
    pub walls: Option<i32>,
    pub maxScore: Option<i32>,
    pub duration: Option<f64>,
    pub requirements: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalStatus {
    pub id: i32,
    pub status: i32,
    pub timeset: i32,
    pub link: Option<String>,
    pub responsible: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModifiersMap {
    pub modifierId: i32,
    pub da: f32,
    pub fs: f32,
    pub sf: f32,
    pub ss: f32,
    pub gn: f32,
    pub na: f32,
    pub nb: f32,
    pub nf: f32,
    pub no: f32,
    pub pm: f32,
    pub sc: f32,
    pub sa: f32,
    pub op: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModifiersRating {
    pub id: i32,
    pub fsPredictedAcc: f32,
    pub fsPassRating: f32,
    pub fsAccRating: f32,
    pub fsTechRating: f32,
    pub fsStars: f32,
    pub ssPredictedAcc: f32,
    pub ssPassRating: f32,
    pub ssAccRating: f32,
    pub ssTechRating: f32,
    pub ssStars: f32,
    pub sfPredictedAcc: f32,
    pub sfPassRating: f32,
    pub sfAccRating: f32,
    pub sfTechRating: f32,
    pub sfStars: f32
}