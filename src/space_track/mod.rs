use auth::Cookie;
use reqwest::Client;
mod auth;
mod classes;
mod config;
mod error;
mod url;
mod urls;
mod utils;

pub use auth::Credentials;
pub use classes::{
    Boxscore, BoxscoreField, CdmPublic, CdmPublicField, Decay, DecayField, GeneralPerturbation,
    GeneralPerturbationField,
};
pub use config::{Config, Direction};
use error::Error;
use url::construct_url;

use self::config::OrderByField;

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

    async fn get<T: OrderByField>(
        &mut self,
        url: &str,
        config: Config<T>,
    ) -> Result<reqwest::Response, Error> {
        let cookie = self.get_cookie().await?.clone();
        let url = construct_url(url, config);

        Ok(self.client.get(url).header("Cookie", cookie).send().await?)
    }
}
