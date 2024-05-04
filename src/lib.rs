use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

// const BASE_URL: &str = "https://www.space-track.org";
const LOGIN_URL: &str = "https://www.space-track.org/ajaxauth/login";

struct Cookie {
    value: String,
    created_at: Instant,
}

pub struct SpaceTrack {
    credentials: Credentials,
    cookie: Option<Cookie>,
    client: Client,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub identity: String,
    pub password: String,
}

impl SpaceTrack {
    pub fn new(credentials: Credentials) -> SpaceTrack {
        SpaceTrack {
            credentials,
            cookie: None,
            client: Client::new(),
        }
    }

    pub async fn login(&mut self) -> &Cookie {
        let response = self
            .client
            .post(LOGIN_URL)
            .json(&self.credentials)
            .send()
            .await
            .unwrap();

        let cookie = response
            .headers()
            .get("Set-Cookie")
            .unwrap()
            .to_str()
            .unwrap();

        let cookie = Cookie {
            value: cookie.to_string(),
            created_at: Instant::now(),
        };

        self.cookie = Some(cookie);

        self.cookie.as_ref().unwrap()
    }

    // creates a new cookie if the current one is older than 30 minutes
    // or if it doesn't exist at all
    async fn get_cookie(&mut self) -> &String {
        if self.cookie.is_none() {
            return &self.login().await.value;
        }

        let cookie = self.cookie.as_ref().unwrap();

        if cookie.created_at.elapsed().as_secs() > 1800 {
            return &self.login().await.value;
        }

        &self.cookie.as_ref().unwrap().value
    }

    async fn get(&mut self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let cookie = self.get_cookie().await.clone();

        self.client.get(url).header("Cookie", cookie).send().await
    }

    pub async fn boxscore(&mut self) -> Result<(), reqwest::Error> {
        let url = "https://www.space-track.org/basicspacedata/query/class/boxscore";
        let body = self.get(url).await?;

        println!("{}", body.text().await?);

        Ok(())
    }
}
