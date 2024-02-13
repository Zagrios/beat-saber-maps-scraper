pub mod models;

use reqwest::blocking::Client;
use once_cell::sync::Lazy;
use url::Url;
use super::get_http_client;

static BASE_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://scoresaber.com/api/").unwrap());

pub mod leaderboard {

    use std::error::Error;
    use crate::api::score_saber::models::api_models::{LeaderboardInfoCollection, LeaderboardRequestParams};

    pub async fn leaderboards(params: &LeaderboardRequestParams) -> Result<LeaderboardInfoCollection, Box<dyn Error>> {
        let mut endpoint = super::BASE_URL.join("leaderboards")?;
        let query_string = serde_qs::to_string(params)?;
        let http_client = super::get_http_client();
        endpoint = endpoint.join(&format!("?{}", query_string))?;

        let response = http_client.get(endpoint).send().await?.json::<LeaderboardInfoCollection>().await?;

        return Ok(response);

    }

}