use super::{urls::LOGIN_URL, SpaceTrack};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub identity: String,
    pub password: String,
}

pub struct Cookie {
    value: String,
    created_at: Instant,
}

impl SpaceTrack {
    async fn login(&mut self) -> &Cookie {
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
    pub async fn get_cookie(&mut self) -> &String {
        if self.cookie.is_none() {
            return &self.login().await.value;
        }

        let cookie = self.cookie.as_ref().unwrap();

        if cookie.created_at.elapsed().as_secs() > 1800 {
            return &self.login().await.value;
        }

        &self.cookie.as_ref().unwrap().value
    }
}
