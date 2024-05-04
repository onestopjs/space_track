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

impl SpaceTrack {
    pub fn new(credentials: Credentials) -> SpaceTrack {
        SpaceTrack {
            credentials,
            cookie: None,
            client: Client::new(),
        }
    }

    async fn get(&mut self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let cookie = self.get_cookie().await.clone();

        self.client.get(url).header("Cookie", cookie).send().await
    }
}
