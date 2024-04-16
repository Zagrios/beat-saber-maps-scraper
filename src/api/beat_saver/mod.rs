pub mod models;

use once_cell::sync::Lazy;
use url::Url;

static BASE_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://beatsaver.com/api/").unwrap());

pub mod maps {

    use std::error::Error;

    use once_cell::sync::Lazy;
    use url::Url;
    use crate::api::beat_saver::models::api_models::{SearchRequestParams, SearchRequestSort, SearchResponse};
    use crate::api::get_http_client;

    static ENDPOINT: Lazy<Url> = Lazy::new(|| super::BASE_URL.join("maps/").unwrap());

    pub async fn latest(params: &SearchRequestParams) -> Result<SearchResponse, Box<dyn Error>> {
        let query_string = serde_qs::to_string(params)?;
        let url = ENDPOINT.join(&format!("latest?{}", query_string))?;
        let response = get_http_client().get(url).send().await?.json::<SearchResponse>().await?;
        return Ok(response);
    }

}