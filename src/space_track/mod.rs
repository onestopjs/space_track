use auth::Cookie;
use reqwest::Client;
mod auth;
mod classes;
mod urls;
mod utils;

pub use auth::Credentials;
pub use classes::{Boxscore, Decay};

pub struct SpaceTrack {
    credentials: Credentials,
    cookie: Option<Cookie>,
    client: Client,
}

pub struct Config {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            limit: Some(100),
            offset: None,
        }
    }

    pub fn empty() -> Config {
        Config {
            limit: None,
            offset: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Config {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Config {
        self.offset = Some(offset);
        self
    }
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

    async fn get(
        &mut self,
        url: &str,
        config: Config,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let cookie = self.get_cookie().await.clone();

        let url = construct_url(url, config);
        println!("{:?}", url);

        self.client.get(url).header("Cookie", cookie).send().await
    }
}
