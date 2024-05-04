use super::{error::Error, urls::LOGIN_URL, SpaceTrack};
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
    async fn login(&mut self) -> Result<&Cookie, Error> {
        let response = self
            .client
            .post(LOGIN_URL)
            .json(&self.credentials)
            .send()
            .await?;

        let cookie = match response.headers().get("Set-Cookie") {
            Some(cookie) => cookie.to_str()?,
            None => return Err(Error::CookieError),
        };

        let cookie = Cookie {
            value: cookie.to_string(),
            created_at: Instant::now(),
        };

        self.cookie = Some(cookie);

        self.cookie.as_ref().ok_or(Error::CookieError)
    }

    // creates a new cookie if the current one is older than 30 minutes
    // or if it doesn't exist at all
    pub async fn get_cookie(&mut self) -> Result<&String, Error> {
        if self.cookie.is_none() {
            return Ok(&self.login().await?.value);
        }

        let cookie = match &self.cookie {
            Some(cookie) => cookie,
            None => return Err(Error::CookieError),
        };

        if cookie.created_at.elapsed().as_secs() > 1800 {
            return Ok(&self.login().await?.value);
        }

        Ok(&self.cookie.as_ref().ok_or(Error::CookieError)?.value)
    }
}
