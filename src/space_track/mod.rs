use auth::Cookie;
use reqwest::Client;
mod auth;
mod classes;
mod config;
mod error;
mod urls;
mod utils;

pub use auth::Credentials;
pub use classes::{Boxscore, Decay};
pub use config::Config;
use error::Error;

pub struct SpaceTrack {
    credentials: Credentials,
    cookie: Option<Cookie>,
    client: Client,
}

fn construct_url(base: &str, config: Config) -> String {
    let mut url = base.to_string();

    url.push_str("/");

    if let Some(limit) = config.limit {
        url.push_str(&format!("limit/{}", limit));

        if let Some(offset) = config.offset {
            url.push_str(&format!(",{}", offset));
        }
    }

    url
}

impl SpaceTrack {
    pub fn new(credentials: Credentials) -> SpaceTrack {
        SpaceTrack {
            credentials,
            cookie: None,
            client: Client::new(),
        }
    }

    async fn get(&mut self, url: &str, config: Config) -> Result<reqwest::Response, Error> {
        let cookie = self.get_cookie().await?.clone();
        let url = construct_url(url, config);

        Ok(self.client.get(url).header("Cookie", cookie).send().await?)
    }
}
