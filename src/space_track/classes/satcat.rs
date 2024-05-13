use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::SATCAT_URL,
        utils::{
            deserialize_optional_string_to_u64, deserialize_optional_string_to_u8,
            deserialize_string_to_f64, deserialize_string_to_i8, deserialize_string_to_u16,
            deserialize_string_to_u64,
        },
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum SatCatField {
    IntlDes,
    NoradCatId,
    ObjectType,
    SatName,
    Country,
    Launch,
    Site,
    Decay,
    Period,
    Inclination,
    Apogee,
    Perigee,
    Comment,
    CommentCode,
    RcsValue,
    RcsSize,
    File,
    LaunchYear,
    LaunchNum,
    LaunchPiece,
    Current,
    ObjectName,
    ObjectId,
    ObjectNumber,
}

impl OrderByField for SatCatField {
    fn field(&self) -> &str {
        match self {
            SatCatField::IntlDes => "INTLDES",
            SatCatField::NoradCatId => "NORAD_CAT_ID",
            SatCatField::ObjectType => "OBJECT_TYPE",
            SatCatField::SatName => "SATNAME",
            SatCatField::Country => "COUNTRY",
            SatCatField::Launch => "LAUNCH",
            SatCatField::Site => "SITE",
            SatCatField::Decay => "DECAY",
            SatCatField::Period => "PERIOD",
            SatCatField::Inclination => "INCLINATION",
            SatCatField::Apogee => "APOGEE",
            SatCatField::Perigee => "PERIGEE",
            SatCatField::Comment => "COMMENT",
            SatCatField::CommentCode => "COMMENTCODE",
            SatCatField::RcsValue => "RCSVALUE",
            SatCatField::RcsSize => "RCS_SIZE",
            SatCatField::File => "FILE",
            SatCatField::LaunchYear => "LAUNCH_YEAR",
            SatCatField::LaunchNum => "LAUNCH_NUM",
            SatCatField::LaunchPiece => "LAUNCH_PIECE",
            SatCatField::Current => "CURRENT",
            SatCatField::ObjectName => "OBJECT_NAME",
            SatCatField::ObjectId => "OBJECT_ID",
            SatCatField::ObjectNumber => "OBJECT_NUMBER",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct SatCat {
    #[serde(rename = "INTLDES")]
    pub intl_des: String,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub norad_cat_id: Option<u64>,
    pub object_type: String,
    #[serde(rename = "SATNAME")]
    pub sat_name: String,
    pub country: String,
    pub launch: String,
    pub site: String,
    pub decay: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub period: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub inclination: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub apogee: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub perigee: u64,
    pub comment: Option<String>,
    #[serde(rename = "COMMENTCODE")]
    #[serde(deserialize_with = "deserialize_optional_string_to_u8")]
    pub comment_code: Option<u8>,
    #[serde(rename = "RCSVALUE")]
    #[serde(deserialize_with = "deserialize_string_to_i8")]
    pub rcs_value: i8,
    pub rcs_size: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u16")]
    pub file: u16,
    #[serde(deserialize_with = "deserialize_string_to_u16")]
    pub launch_year: u16,
    #[serde(deserialize_with = "deserialize_string_to_u16")]
    pub launch_num: u16,
    pub launch_piece: String,
    pub current: String,
    pub object_name: String,
    pub object_id: String,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub object_number: Option<u64>,
}

impl SpaceTrack {
    pub async fn satcat(&mut self, config: Config<SatCatField>) -> Result<Vec<SatCat>, Error> {
        Ok(self.get(SATCAT_URL, config).await?.json().await?)
    }

    pub async fn all_satcat(&mut self) -> Result<Vec<SatCat>, Error> {
        self.satcat(Config::empty()).await
    }
}
