mod api;
mod models;
mod utils;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::api::beat_leader::leaderboard::leaderboards;
use crate::api::score_saber::leaderboard::leaderboards as score_saber_leaderboards;
use crate::api::beat_leader::models::models::DifficultyDescription;
use crate::api::beat_saver;
use crate::api::beat_saver::models::api_models::{SearchRequestParams, SearchRequestSort};
use crate::api::beat_saver::models::models::MapDetail;
use crate::api::score_saber::models::api_models::LeaderboardRequestParams;
use crate::api::score_saber::models::models::LeaderboardInfo;
use crate::models::song_details_json::{SongDetails, SongDetailsCache};

const OUT_DIR: &str = "out";
const FILENAME : &str = "song_details_cache_v1";

#[tokio::main]
async fn main() {

    let (bsv_maps, bl_maps_diffs, score_saber_leaderboards) = tokio::join!(scrap_bsv_maps(), scrap_bl_maps_diffs(), scrap_score_saber_leaderboards());

    let mut song_details: Vec<SongDetails> = Vec::new();

    for bsv_map in bsv_maps {
        let hash = &bsv_map.versions.first().unwrap().hash;
        let bl_map_difficulties = bl_maps_diffs.get(hash);
        let ss_leaderboards = score_saber_leaderboards.get(hash);
        song_details.push(SongDetails::new(&bsv_map, bl_map_difficulties, ss_leaderboards));
    }

    println!("Scrapped {} song details", song_details.len());

    write_to_json(&song_details);

    let song_details_cache = SongDetailsCache {
        total: song_details.len() as u32,
        songs: song_details,
        last_updated: chrono::Utc::now().timestamp() as u32
    };

    write_proto_message(&song_details_cache);

    gzip_generated_files();

}

fn write_to_json<T>(data: &T) where T: serde::Serialize {
    let json_filename = format!("{}.json", FILENAME);
    let json = serde_json::to_string(data).unwrap();
    std::fs::create_dir_all(OUT_DIR).unwrap();
    let path = PathBuf::from(OUT_DIR).join(json_filename);
    std::fs::write(path, json).unwrap();
}

fn write_proto_message<T>(data: &T) where T: prost::Message {
    let proto_filename = format!("{}", FILENAME);
    let mut buf = Vec::new();
    data.encode(&mut buf).unwrap();
    std::fs::create_dir_all(OUT_DIR).unwrap();
    let path = PathBuf::from(OUT_DIR).join(proto_filename);
    std::fs::write(path, buf).unwrap();
}

fn read_from_json<T>(path: &str) -> T where T: serde::de::DeserializeOwned {
    let json_filename = format!("{}.json", path);
    let json = std::fs::read_to_string(json_filename).unwrap();
    return serde_json::from_str(&json).unwrap();
}

fn gzip_generated_files() {
    let files = vec![format!("{}.json", FILENAME), FILENAME.to_string()];
    for file in files {

        println!("Gzipping {}", file);

        let path = PathBuf::from(OUT_DIR).join(file);
        let mut file = std::fs::File::open(&path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(&buffer).unwrap();
        let compressed = encoder.finish().unwrap();
        // .json.gz / .gz
        let compressed_path = format!("{}.gz", path.to_str().unwrap());
        std::fs::write(compressed_path, compressed).unwrap();
    }
}

async fn scrap_bsv_maps() -> Vec<MapDetail> {

    println!("Scraping BeatSaver maps...");

    let mut res: Vec<MapDetail> = Vec::new();
    let mut last_updated: Option<String> = None;

    loop {
        let params = SearchRequestParams {
            after: None,
            automapper: Some(true),
            before: last_updated,
            pageSize: Some(100),
            sort: Some(SearchRequestSort::FIRST_PUBLISHED),
            verified: None,
        };

        let response = beat_saver::maps::latest(&params).await.unwrap();

        let mut maps = response.docs;

        if maps.len() == 0 {
            break;
        }

        last_updated = Some(maps.last().unwrap().uploaded.clone());
        res.extend(maps);

        if res.len() % 10 == 0 {
            println!("Scraped {} maps from BeatSaver", res.len());
        }
    }

    return res;
}

async fn scrap_bl_maps_diffs() -> HashMap<String, Vec<DifficultyDescription>> {
    println!("Scraping BeatLeader maps...");

    let mut res: HashMap<String, Vec<DifficultyDescription>> = HashMap::new();

    let mut page = 0;

    loop {
        let mut leaderboards = leaderboards(page, 500).await.unwrap().data;

        if leaderboards.len() == 0 {
            break;
        }

        for leaderboard in leaderboards {
            let mut diffs = res.entry(leaderboard.song.hash.unwrap()).or_insert(Vec::new());
            diffs.push(leaderboard.difficulty);
        }
        page += 1;

        if page % 10 == 0 {
            println!("Scraped {} maps from {} pages of BeatLeader", res.len(), page);
        }

    }

    println!("Scraped {} maps from BeatLeader", res.len());

    return res;
}

async fn scrap_score_saber_leaderboards() -> HashMap<String, Vec<LeaderboardInfo>>{

    let mut res: HashMap<String, Vec<LeaderboardInfo>> = HashMap::new();
    let mut request_params = LeaderboardRequestParams::default();

    // scrap qualified leaderboards

    request_params.qualified = Some(true);

    loop {
        let leaderboards_res = score_saber_leaderboards(&request_params).await.unwrap();

        if leaderboards_res.leaderboards.len() == 0 {
            break;
        }

        for leaderboard in leaderboards_res.leaderboards {
            let mut diffs = res.entry(leaderboard.song_hash.clone()).or_insert(Vec::new());
            diffs.push(leaderboard);
        }

        if request_params.page % 10 == 0 {
            println!("Scraped {} qualified leaderboards from ScoreSaber from {} pages", res.len(), request_params.page);
        }

        request_params.page += 1;

    }

    // scrap ranked leaderboards

    request_params.qualified = None;
    request_params.ranked = Some(true);
    request_params.page = 0;

    loop {
        let leaderboards_res = score_saber_leaderboards(&request_params).await.unwrap();

        if leaderboards_res.leaderboards.len() == 0 {
            break;
        }

        for leaderboard in leaderboards_res.leaderboards {
            let mut diffs = res.entry(leaderboard.song_hash.clone()).or_insert(Vec::new());
            diffs.push(leaderboard);
        }

        if request_params.page % 10 == 0 {
            println!("Scraped {} ranked leaderboards from ScoreSaber from {} pages", res.len(), request_params.page);
        }

        request_params.page += 1;

    }

    println!("Scraped {} leaderboards from ScoreSaber", res.len());

    return res;

}
