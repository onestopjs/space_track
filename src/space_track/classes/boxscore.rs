use serde::Deserialize;

use crate::{
    space_track::{
        utils::{deserialize_optional_string_to_u64, deserialize_string_to_u64},
        Config,
    },
    SpaceTrack,
};

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Boxscore {
    pub country: String,
    pub spadoc_cd: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub orbital_tba: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub orbital_payload_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub orbital_rocket_body_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub orbital_debris_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub orbital_total_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub decayed_payload_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub decayed_rocket_body_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub decayed_debris_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub decayed_total_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub country_total: u64,
}

impl SpaceTrack {
    pub async fn boxscore(&mut self, config: Config) -> Result<Vec<Boxscore>, reqwest::Error> {
        let url = "https://www.space-track.org/basicspacedata/query/class/boxscore";
        let body = self.get(url, config).await?;

        let boxscores: Vec<Boxscore> = body.json().await?;

        Ok(boxscores)
    }

    pub async fn all_boxscore(&mut self) -> Result<Vec<Boxscore>, reqwest::Error> {
        self.boxscore(Config::empty()).await
    }
}
