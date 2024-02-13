pub mod song_details_json {
    use rust_decimal::Decimal;
    use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
    use crate::api::beat_leader::models::models::DifficultyDescription;
    use crate::api::beat_saver::models::models::{MapDetail, MapDiff};
    use crate::api::score_saber::models::models::LeaderboardInfo;
    use crate::utils::parse_date_rfc3339_to_timestamp;

    include!(concat!(env!("OUT_DIR"), "/song_details_cache_v1.rs"));


    impl SongDetails {
        pub fn new(bsv_map: &MapDetail, bl_map_difficulties: Option<&Vec<DifficultyDescription>>, ss_leaderboards: Option<&Vec<LeaderboardInfo>>) -> SongDetails {
            let mut difficulties: Vec<Difficulty> = Vec::new();

            for bsv_map_diff in &bsv_map.versions[0].diffs {

                let bl_map_diff = match bl_map_difficulties {
                    Some(diffs) => diffs.iter().find(|&diff| {
                        return diff.difficultyName == Some(bsv_map_diff.difficulty.to_string()) && diff.modeName == Some(bsv_map_diff.characteristic.to_string());
                    }),
                    None => None
                };

                let ss_leaderboard = match ss_leaderboards {
                    Some(leaderboards) => leaderboards.iter().find(|&leaderboard| {
                        return leaderboard.difficulty.get_difficulty_string() == bsv_map_diff.difficulty.to_string() && leaderboard.difficulty.get_characteristic_string() == bsv_map_diff.characteristic.to_string();
                    }),
                    None => None
                };

                difficulties.push(SongDetails::build_difficulty(bsv_map_diff, bl_map_diff, ss_leaderboard));
            }

            SongDetails {
                id: bsv_map.id.to_string(),
                hash: bsv_map.versions.first().unwrap().hash.to_string(),
                name: bsv_map.name.to_string(),
                metadata: Some(MapDetailMetadata {
                    bpm: bsv_map.metadata.bpm,
                    duration: bsv_map.metadata.duration,
                    level_author_name: bsv_map.metadata.levelAuthorName.to_string(),
                    song_author_name: bsv_map.metadata.songAuthorName.to_string(),
                    song_name: bsv_map.metadata.songName.to_string(),
                    song_sub_name: bsv_map.metadata.songSubName.to_string(),
                }),
                uploader: Some(Uploader {
                    name: bsv_map.uploader.name.clone(),
                    id: bsv_map.uploader.id,
                    verified: bsv_map.uploader.verifiedMapper.unwrap_or(false)
                }),
                uploaded_at: parse_date_rfc3339_to_timestamp(&bsv_map.uploaded) as i32,
                tags: bsv_map.tags.clone().unwrap_or(Vec::new()),
                bpm: bsv_map.metadata.bpm,
                ranked: ss_leaderboards.map_or(bsv_map.ranked, |leaderboards| leaderboards.iter().any(|leaderboard| leaderboard.ranked)),
                qualified: ss_leaderboards.map_or(bsv_map.qualified, |leaderboards| leaderboards.iter().any(|leaderboard| leaderboard.qualified)),
                curated: bsv_map.curatedAt.clone().map_or(false, |_| true),
                ranked_bl: bl_map_difficulties.map_or(false, |diffs| diffs.iter().any(|diff| diff.rankedTime.map_or(false, |time| time > 0))),
                nominated_bl: bl_map_difficulties.map_or(false, |diffs| diffs.iter().any(|diff| diff.nominatedTime.map_or(false, |time| time > 0))),
                qualified_bl: bl_map_difficulties.map_or(false, |diffs| diffs.iter().any(|diff| diff.qualifiedTime.map_or(false, |time| time > 0))),
                up_votes: bsv_map.stats.upvotes,
                down_votes: bsv_map.stats.downvotes,
                downloads: bsv_map.stats.downloads,
                duration: bsv_map.metadata.duration,
                automapper: bsv_map.automapper,
                difficulties,
            }
        }

        fn build_difficulty(bsv_map_diff: &MapDiff, bl_map_diff: Option<&DifficultyDescription>, ss_leaderboard: Option<&LeaderboardInfo>) -> Difficulty {

            let starts = ss_leaderboard.map_or(bsv_map_diff.stars.unwrap_or(0.0), |ss| ss.stars);
            let bl_starts = bl_map_diff.map_or(0.0, |bl| bl.stars.unwrap_or(0.0));

            Difficulty {
                difficulty: bsv_map_diff.difficulty.to_string(),
                characteristic: bsv_map_diff.characteristic.to_string(),
                label: bsv_map_diff.label.clone().unwrap_or(String::new()),
                stars: Decimal::from_f32(starts).unwrap().round_dp(2).to_f32().unwrap_or(0.0),
                stars_bl: Decimal::from_f32(bl_starts).unwrap().round_dp(2).to_f32().unwrap_or(0.0),
                njs: bsv_map_diff.njs,
                nps: bsv_map_diff.nps,
                offset: bsv_map_diff.offset,
                me: bsv_map_diff.me.unwrap_or(false),
                ne: bsv_map_diff.ne.unwrap_or(false),
                cinema: bsv_map_diff.cinema.unwrap_or(false),
                chroma: bsv_map_diff.chroma.unwrap_or(false),
                bombs: bsv_map_diff.bombs,
                notes: bsv_map_diff.notes,
                obstacles: bsv_map_diff.obstacles,
            }
        }
    }
}