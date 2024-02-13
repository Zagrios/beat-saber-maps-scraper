pub mod models;

use once_cell::sync::Lazy;
use url::Url;
use super::get_http_client;

static BASE_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://api.beatleader.xyz/").unwrap());

pub mod leaderboard {

    use std::error::Error;
    use crate::api::beat_leader::models::api_models::LeaderboardInfoResponseResponseWithMetadata;

    pub async fn leaderboards(page: u32, count: u32) -> Result<LeaderboardInfoResponseResponseWithMetadata, Box<dyn Error>> {
        let endpoint = super::BASE_URL.join("leaderboards")?.query_pairs_mut().append_pair("page", &page.to_string()).append_pair("count", &count.to_string()).finish().to_owned();
        let http_client = super::get_http_client();

        let response = http_client.get(endpoint).send().await?.json::<LeaderboardInfoResponseResponseWithMetadata>().await?;

        return Ok(response);

    }

}