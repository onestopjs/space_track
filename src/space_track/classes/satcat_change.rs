use crate::{
    space_track::{
        config::OrderByField, error::Error, urls::SATCAT_CHANGE_URL,
        utils::deserialize_string_to_u64, Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum SatCatChangeField {
    NoradCatId,
    ObjectNumber,
    CurrentName,
    PreviousName,
    CurrentIntldes,
    PreviousIntldes,
    CurrentCountry,
    PreviousCountry,
    CurrentLaunch,
    PreviousLaunch,
    CurrentDecay,
    PreviousDecay,
    ChangeMade,
}

impl OrderByField for SatCatChangeField {
    fn field(&self) -> &str {
        match self {
            SatCatChangeField::NoradCatId => "NORAD_CAT_ID",
            SatCatChangeField::ObjectNumber => "OBJECT_NUMBER",
            SatCatChangeField::CurrentName => "CURRENT_NAME",
            SatCatChangeField::PreviousName => "PREVIOUS_NAME",
            SatCatChangeField::CurrentIntldes => "CURRENT_INTLDES",
            SatCatChangeField::PreviousIntldes => "PREVIOUS_INTLDES",
            SatCatChangeField::CurrentCountry => "CURRENT_COUNTRY",
            SatCatChangeField::PreviousCountry => "PREVIOUS_COUNTRY",
            SatCatChangeField::CurrentLaunch => "CURRENT_LAUNCH",
            SatCatChangeField::PreviousLaunch => "PREVIOUS_LAUNCH",
            SatCatChangeField::CurrentDecay => "CURRENT_DECAY",
            SatCatChangeField::PreviousDecay => "PREVIOUS_DECAY",
            SatCatChangeField::ChangeMade => "CHANGE_MADE",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct SatCatChange {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub norad_cat_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub object_number: u64,
    pub current_name: String,
    pub previous_name: Option<String>,
    pub current_intldes: String,
    pub previous_intldes: Option<String>,
    pub current_country: String,
    pub previous_country: Option<String>,
    pub current_launch: Option<String>,
    pub previous_launch: Option<String>,
    pub current_decay: Option<String>,
    pub previous_decay: Option<String>,
    pub change_made: Option<String>,
}

impl SpaceTrack {
    pub async fn satcat_change(
        &mut self,
        config: Config<SatCatChangeField>,
    ) -> Result<Vec<SatCatChange>, Error> {
        Ok(self.get(SATCAT_CHANGE_URL, config).await?.json().await?)
    }

    pub async fn all_satcat_change(&mut self) -> Result<Vec<SatCatChange>, Error> {
        self.satcat_change(Config::empty()).await
    }
}
