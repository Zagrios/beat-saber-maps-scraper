mod api;
mod models;
mod utils;

use std::io::{Read, Write};
use std::path::PathBuf;
use crate::api::beat_saver;
use crate::api::beat_saver::models::api_models::{SearchRequestParams, SearchRequestSort};
use crate::api::beat_saver::models::models::MapDetail;
use crate::models::song_details_json::{SongDetails, SongDetailsCache, UploadersList};

const OUT_DIR: &str = "out";
const FILENAME : &str = "song_details_cache_v1";

#[tokio::main]
async fn main() {

    let bsv_maps= scrap_bsv_maps().await;

    let mut difficulty_labels: Vec<String> = vec![];
    let mut uploader_list: UploadersList = UploadersList {
        names: vec![],
        ids: vec![]
    };


    let mut song_details: Vec<SongDetails> = vec![];

    for bsv_map in bsv_maps {

        if uploader_list.ids.iter().find(|&id| *id == bsv_map.uploader.id as u32).is_none() {
            uploader_list.ids.push(bsv_map.uploader.id as u32);
            uploader_list.names.push(bsv_map.uploader.name.clone());
        }

        let map_diff_labels = SongDetails::extract_difficulty_labels(&bsv_map);
        for label in map_diff_labels {
            if !difficulty_labels.contains(&label) {
                difficulty_labels.push(label);
            }
        }

        song_details.push(SongDetails::new(&bsv_map, &uploader_list, &difficulty_labels));
    }

    println!("Scrapped {} song details", song_details.len());

    let song_details_cache = SongDetailsCache {
        total: song_details.len() as u32,
        songs: song_details,
        last_updated: chrono::Utc::now().timestamp() as u32,
        uploaders: Some(uploader_list),
        difficulty_labels: difficulty_labels.iter().map(|label| label.to_string()).collect()
    };

    write_to_json(&song_details_cache);
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

        let maps = response.docs;

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
