pub mod song_details_json {
    use crate::api::beat_leader::models::models::DifficultyDescription;
    use crate::api::beat_saver::models::models::{MapDetail, MapDiff};
    use crate::api::score_saber::models::models::LeaderboardInfo;
    use crate::utils::parse_date_rfc3339_to_timestamp;

    include!(concat!(env!("OUT_DIR"), "/song_details_cache_v1.rs"));

    const hashChars: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

    impl SongDetails {
        pub fn new(bsv_map: &MapDetail, bl_map_difficulties: Option<&Vec<DifficultyDescription>>, ss_leaderboards: Option<&Vec<LeaderboardInfo>>, uploader_list: &UploadersList, difficulty_labels: &Vec<String>) -> SongDetails {
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

                difficulties.push(SongDetails::build_difficulty(bsv_map_diff, bl_map_diff, ss_leaderboard, difficulty_labels));
            }

            SongDetails {
                id_int: i32::from_str_radix(&bsv_map.id, 16).unwrap() as u32,
                hash_indices: bsv_map.versions[0].hash.chars().map(|c| hashChars.iter().position(|&x| x == c).unwrap() as u32).collect(),
                name: bsv_map.name.chars().take(50).collect(),
                duration: bsv_map.metadata.duration as u32,
                uploader_ref: Some(UploaderRef {
                    uploader_ref_index: uploader_list.ids.iter().position(|id| *id == bsv_map.uploader.id as u32).unwrap() as u32,
                    verified: bsv_map.uploader.verifiedMapper.unwrap_or(false),
                }),
                uploaded_at: parse_date_rfc3339_to_timestamp(&bsv_map.uploaded) as u32,
                tags: bsv_map.tags.clone().map_or(Vec::new(), |tags| tags.iter().map(|tag| MapTag::from_str(tag) as i32).collect()),
                ranked: ss_leaderboards.map_or(bsv_map.ranked, |leaderboards| leaderboards.iter().any(|leaderboard| leaderboard.ranked)),
                qualified: ss_leaderboards.map_or(bsv_map.qualified, |leaderboards| leaderboards.iter().any(|leaderboard| leaderboard.qualified)),
                curated: bsv_map.curatedAt.clone().map_or(false, |_| true),
                ranked_bl: bl_map_difficulties.map_or(false, |diffs| diffs.iter().any(|diff| diff.rankedTime.map_or(false, |time| time > 0))),
                nominated_bl: bl_map_difficulties.map_or(false, |diffs| diffs.iter().any(|diff| diff.nominatedTime.map_or(false, |time| time > 0))),
                qualified_bl: bl_map_difficulties.map_or(false, |diffs| diffs.iter().any(|diff| diff.qualifiedTime.map_or(false, |time| time > 0))),
                up_votes: bsv_map.stats.upvotes as u32,
                down_votes: bsv_map.stats.downvotes as u32,
                downloads: bsv_map.stats.downloads as u32,
                automapper: bsv_map.automapper,
                difficulties,
            }
        }

        fn build_difficulty(bsv_map_diff: &MapDiff, bl_map_diff: Option<&DifficultyDescription>, ss_leaderboard: Option<&LeaderboardInfo>, difficulty_labels: &Vec<String>) -> Difficulty {

            let stars = ss_leaderboard.map_or(bsv_map_diff.stars.unwrap_or(0.0), |ss| ss.stars);
            let bl_stars = bl_map_diff.map_or(0.0, |bl| bl.stars.unwrap_or(0.0));

            Difficulty {
                difficulty: DifficultyLabel::from_str(&bsv_map_diff.difficulty) as i32,
                characteristic: DifficultyCharacteristic::from_str(&bsv_map_diff.characteristic) as i32,
                label_index: difficulty_labels.iter().position(|label| label == &bsv_map_diff.label.clone().unwrap_or("".to_string())).unwrap() as u32,
                stars_t100: (stars * 100.0).round() as u32,
                stars_bl_t100: (bl_stars * 100.0).round() as u32,
                njs_t100: (bsv_map_diff.njs * 100.0).round() as u32,
                nps_t100: (bsv_map_diff.nps * 100.0).round() as u32,
                offset_t100: (bsv_map_diff.offset * 100.0).round() as i32,
                me: bsv_map_diff.me.unwrap_or(false),
                ne: bsv_map_diff.ne.unwrap_or(false),
                cinema: bsv_map_diff.cinema.unwrap_or(false),
                chroma: bsv_map_diff.chroma.unwrap_or(false),
                bombs: bsv_map_diff.bombs as u32,
                notes: bsv_map_diff.notes as u32,
                obstacles: bsv_map_diff.obstacles as u32,
            }
        }

        pub fn extract_difficulty_labels(map: &MapDetail) -> Vec<String> {
            let mut labels: Vec<String> = Vec::new();

            for version in &map.versions {
                for diff in &version.diffs {

                    if !labels.contains(&diff.label.clone().unwrap_or("".to_string())) {
                        labels.push(diff.label.clone().unwrap_or("".to_string()));
                    }
                }
            }

            labels
        }
    }

    impl DifficultyLabel {
        pub fn from_str(label: &str) -> DifficultyLabel {

            let lower_label = label.to_lowercase();

            match lower_label.as_str() {
                "easy" => DifficultyLabel::Easy,
                "normal" => DifficultyLabel::Normal,
                "hard" => DifficultyLabel::Hard,
                "expert" => DifficultyLabel::Expert,
                "expertplus" => DifficultyLabel::ExpertPlus,
                _ => DifficultyLabel::UnknownLabel
            }
        }
    }

    impl DifficultyCharacteristic {

        pub fn from_str(label: &str) -> DifficultyCharacteristic {

            let lower_label = label.to_lowercase().replace(" ", "");

            match lower_label.as_str() {
                "standard" => DifficultyCharacteristic::Standard,
                "onesaber" => DifficultyCharacteristic::OneSaber,
                "noarrows" => DifficultyCharacteristic::NoArrows,
                "90degree" => DifficultyCharacteristic::NinetyDegree,
                "360degree" => DifficultyCharacteristic::ThreesixtyDegree,
                "lightshow" => DifficultyCharacteristic::Lightshow,
                "lawless" => DifficultyCharacteristic::Lawless,
                "legacy" => DifficultyCharacteristic::Legacy,
                _ => DifficultyCharacteristic::UnknownCharacteristic
            }
        }

    }

    impl MapTag {

        pub fn from_str(tag: &str) -> MapTag {
            let lower_tag = tag.to_lowercase();

            match lower_tag.as_str() {
                "dance" => MapTag::Dance,
                "swing" => MapTag::Swing,
                "nightcore" => MapTag::Nightcore,
                "folk" => MapTag::Folk,
                "family" => MapTag::Family,
                "ambient" => MapTag::Ambient,
                "funk" => MapTag::Funk,
                "jazz" => MapTag::Jazz,
                "soul" => MapTag::Soul,
                "speedcore" => MapTag::Speedcore,
                "punk" => MapTag::Punk,
                "rb" => MapTag::Rb,
                "holiday" => MapTag::Holiday,
                "vocaloid" => MapTag::Vocaloid,
                "j-rock" => MapTag::JRock,
                "trance" => MapTag::Trance,
                "drumbass" => MapTag::Drumbass,
                "comedy" => MapTag::Comedy,
                "instrumental" => MapTag::Instrumental,
                "hardcore" => MapTag::Hardcore,
                "k-pop" => MapTag::KPop,
                "kpop" => MapTag::KPop,
                "indie" => MapTag::Indie,
                "techno" => MapTag::Techno,
                "house" => MapTag::House,
                "game" => MapTag::Game,
                "film" => MapTag::Film,
                "alt" => MapTag::Alt,
                "dubstep" => MapTag::Dubstep,
                "metal" => MapTag::Metal,
                "anime" => MapTag::Anime,
                "hiphop" => MapTag::Hiphop,
                "j-pop" => MapTag::JPop,
                "jpop" => MapTag::JPop,
                "rock" => MapTag::Rock,
                "pop" => MapTag::Pop,
                "electronic" => MapTag::Electronic,
                "classical-orchestral" => MapTag::ClassicalOrchestral,
                "accuracy" => MapTag::Accuracy,
                "balanced" => MapTag::Balanced,
                "challenge" => MapTag::Challenge,
                "dancestyle" => MapTag::Dancestyle,
                "fitness" => MapTag::Fitness,
                "speed" => MapTag::Speed,
                "tech" => MapTag::Tech,
                _ => MapTag::UnknownTag,
            }
        }

    }
}