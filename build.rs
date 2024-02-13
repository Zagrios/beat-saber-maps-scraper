

fn main() {
    let mut config = prost_build::Config::new();

    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde_with::skip_serializing_none]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");

    config.compile_protos(&["proto/song_details_cache_v1.proto"], &["proto/"]).unwrap();
}