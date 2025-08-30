pub mod models;

use std::sync::LazyLock;
use url::Url;

static BASE_URL: LazyLock<Url> = LazyLock::new(|| Url::parse("https://beatsaver.com/api/").unwrap());

pub mod maps {

    use std::error::Error;
    use crate::api::beat_saver::models::api_models::{SearchRequestParams, SearchResponse};
    use crate::api::get_http_client;

    pub async fn latest(params: &SearchRequestParams) -> Result<SearchResponse, Box<dyn Error>> {
        let endpoint = super::BASE_URL.join("maps/")?;
        let query_string = serde_qs::to_string(params)?;
        let url = endpoint.join(&format!("latest?{}", query_string))?;
        let response = get_http_client().get(url).send().await?.json::<SearchResponse>().await?;
        return Ok(response);
    }

}