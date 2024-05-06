use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::BOXSCORE_URL,
        utils::{deserialize_optional_string_to_u64, deserialize_string_to_u64},
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum BoxscoreField {
    Country,
    SpadocCd,
    OrbitalTba,
    OrbitalPayloadCount,
    OrbitalRocketBodyCount,
    OrbitalDebrisCount,
    OrbitalTotalCount,
    DecayedPayloadCount,
    DecayedRocketBodyCount,
    DecayedDebrisCount,
    DecayedTotalCount,
    CountryTotal,
}

impl OrderByField for BoxscoreField {
    fn field(&self) -> &str {
        match self {
            BoxscoreField::Country => "COUNTRY",
            BoxscoreField::SpadocCd => "SPADOC_CD",
            BoxscoreField::OrbitalTba => "ORBITAL_TBA",
            BoxscoreField::OrbitalPayloadCount => "ORBITAL_PAYLOAD_COUNT",
            BoxscoreField::OrbitalRocketBodyCount => "ORBITAL_ROCKET_BODY_COUNT",
            BoxscoreField::OrbitalDebrisCount => "ORBITAL_DEBRIS_COUNT",
            BoxscoreField::OrbitalTotalCount => "ORBITAL_TOTAL_COUNT",
            BoxscoreField::DecayedPayloadCount => "DECAYED_PAYLOAD_COUNT",
            BoxscoreField::DecayedRocketBodyCount => "DECAYED_ROCKET_BODY_COUNT",
            BoxscoreField::DecayedDebrisCount => "DECAYED_DEBRIS_COUNT",
            BoxscoreField::DecayedTotalCount => "DECAYED_TOTAL_COUNT",
            BoxscoreField::CountryTotal => "COUNTRY_TOTAL",
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Debug)]
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
    pub async fn boxscore(
        &mut self,
        config: Config<BoxscoreField>,
    ) -> Result<Vec<Boxscore>, Error> {
        Ok(self.get(BOXSCORE_URL, config).await?.json().await?)
    }

    pub async fn all_boxscore(&mut self) -> Result<Vec<Boxscore>, Error> {
        self.boxscore(Config::empty()).await
    }
}
