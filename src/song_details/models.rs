use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::api::beat_saver::models::models::{MapDetail, MapDetailMetadata, MapDiff};
use crate::api::beat_leader::models::models::DifficultyDescription;

#[derive(Serialize, Deserialize, Debug)]
pub struct SongDetails {
    pub id: String,
    pub hash: String,
    pub name: String,
    pub metadata: MapDetailMetadata,
    pub uploader: Uploader,
    pub uploaded: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub bpm: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rankedBL: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominatedBL: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifiedBL: Option<bool>,
    pub upvotes: u32,
    pub downvotes: u32,
    pub duration: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automapper: Option<bool>,
    pub difficulties: Vec<Difficulty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Difficulty {
    pub difficulty: String,
    pub characteristic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starsBL: Option<f32>,
    pub njs: f32,
    pub nps: f32,
    pub offset: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<String>>,
    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uploader {
    name: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    verified: Option<bool>
}

impl SongDetails {
    pub fn new(bsv_map: &MapDetail, bl_map_difficulties: Option<&Vec<DifficultyDescription>>) -> SongDetails {
        let mut difficulties: Vec<Difficulty> = Vec::new();

            for bsv_map_diff in &bsv_map.versions[0].diffs {

                let bl_map_diff = match bl_map_difficulties {
                    Some(diff) => diff.iter().find(|&x| {
                        return x.difficultyName == Some(bsv_map_diff.difficulty.to_string()) && x.modeName == Some(bsv_map_diff.characteristic.to_string());
                    }),
                    None => None
                };

                difficulties.push(SongDetails::build_difficulty(bsv_map_diff, bl_map_diff));
            }

        SongDetails {
            id: bsv_map.id.to_string(),
            hash: bsv_map.versions.first().unwrap().hash.to_string(),
            name: bsv_map.name.to_string(),
            metadata: bsv_map.metadata.clone(),
            uploader: Uploader {
                name: bsv_map.uploader.name.to_string(),
                id: bsv_map.uploader.id.to_string(),
                verified: bsv_map.uploader.verifiedMapper,
            },
            uploaded: bsv_map.uploaded.to_string(),
            tags: bsv_map.tags.clone().unwrap_or(Vec::new()),
            bpm: bsv_map.metadata.bpm,
            ranked: Some(bsv_map.ranked),
            qualified: Some(bsv_map.qualified),
            curated: bsv_map.curatedAt.as_ref().map_or(None, |_| Some(true)),
            rankedBL: None,
            nominatedBL: None,
            qualifiedBL: None,
            upvotes: bsv_map.stats.upvotes,
            downvotes: bsv_map.stats.downvotes,
            duration: bsv_map.metadata.duration,
            automapper: Some(bsv_map.automapper),
            difficulties,
        }
    }

    fn build_difficulty(bsv_map_diff: &MapDiff, bl_map_diff: Option<&DifficultyDescription>) -> Difficulty {

        let mut requirements: Vec<String> = Vec::new();

        if bsv_map_diff.ne.unwrap_or(false) {
            requirements.push(String::from("ne"));
        }

        if bsv_map_diff.me.unwrap_or(false) {
            requirements.push(String::from("me"));
        }

        if bsv_map_diff.cinema.unwrap_or(false) {
            requirements.push(String::from("cinema"));
        }

        if bsv_map_diff.chroma.unwrap_or(false) {
            requirements.push(String::from("chroma"));
        }

        Difficulty {
            difficulty: bsv_map_diff.difficulty.to_string(),
            characteristic: Some(bsv_map_diff.characteristic.to_string()),
            label: bsv_map_diff.label.as_ref().map_or(None, |x| Some(x.to_string())),
            stars: bsv_map_diff.stars,
            starsBL: bl_map_diff.map_or(None, |x| x.stars),
            njs: bsv_map_diff.njs,
            nps: bsv_map_diff.nps,
            offset: bsv_map_diff.offset,
            requirements: match requirements.len() { 0 => None, _ => Some(requirements) },
            bombs: bsv_map_diff.bombs,
            notes: bsv_map_diff.notes,
            obstacles: bsv_map_diff.obstacles,
        }
    }
}