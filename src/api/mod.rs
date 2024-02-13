pub mod beat_saver;
pub mod beat_leader;
pub mod score_saber;

use reqwest::Client;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn get_http_client() -> ClientWithMiddleware {

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(10);

    let client = ClientBuilder::new(
        Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(60))
            .build().unwrap()
    ).with(RetryTransientMiddleware::new_with_policy(retry_policy)).build();

    return client;
}