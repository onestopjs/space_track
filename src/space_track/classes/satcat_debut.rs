use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::SATCAT_DEBUT_URL,
        utils::{
            deserialize_optional_string_to_f64, deserialize_optional_string_to_u64,
            deserialize_optional_string_to_u8, deserialize_string_to_i8, deserialize_string_to_u16,
            deserialize_string_to_u64,
        },
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum SatCatDebutField {
    IntlDes,
    NoradCatId,
    ObjectType,
    SatName,
    Debut,
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

impl OrderByField for SatCatDebutField {
    fn field(&self) -> &str {
        match self {
            SatCatDebutField::IntlDes => "INTLDES",
            SatCatDebutField::NoradCatId => "NORAD_CAT_ID",
            SatCatDebutField::ObjectType => "OBJECT_TYPE",
            SatCatDebutField::SatName => "SATNAME",
            SatCatDebutField::Debut => "DEBUT",
            SatCatDebutField::Country => "COUNTRY",
            SatCatDebutField::Launch => "LAUNCH",
            SatCatDebutField::Site => "SITE",
            SatCatDebutField::Decay => "DECAY",
            SatCatDebutField::Period => "PERIOD",
            SatCatDebutField::Inclination => "INCLINATION",
            SatCatDebutField::Apogee => "APOGEE",
            SatCatDebutField::Perigee => "PERIGEE",
            SatCatDebutField::Comment => "COMMENT",
            SatCatDebutField::CommentCode => "COMMENTCODE",
            SatCatDebutField::RcsValue => "RCSVALUE",
            SatCatDebutField::RcsSize => "RCS_SIZE",
            SatCatDebutField::File => "FILE",
            SatCatDebutField::LaunchYear => "LAUNCH_YEAR",
            SatCatDebutField::LaunchNum => "LAUNCH_NUM",
            SatCatDebutField::LaunchPiece => "LAUNCH_PIECE",
            SatCatDebutField::Current => "CURRENT",
            SatCatDebutField::ObjectName => "OBJECT_NAME",
            SatCatDebutField::ObjectId => "OBJECT_ID",
            SatCatDebutField::ObjectNumber => "OBJECT_NUMBER",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct SatCatDebut {
    #[serde(rename = "INTLDES")]
    pub intl_des: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub norad_cat_id: u64,
    pub object_type: Option<String>,
    pub satname: String,
    pub debut: Option<String>,
    pub country: String,
    pub launch: Option<String>,
    pub site: Option<String>,
    pub decay: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub period: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub inclination: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub apogee: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub perigee: Option<u64>,
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
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub object_number: u64,
}

impl SpaceTrack {
    pub async fn satcat_debut(
        &mut self,
        config: Config<SatCatDebutField>,
    ) -> Result<Vec<SatCatDebut>, Error> {
        Ok(self.get(SATCAT_DEBUT_URL, config).await?.json().await?)
    }

    pub async fn all_satcat_debug(&mut self) -> Result<Vec<SatCatDebut>, Error> {
        self.satcat_debut(Config::empty()).await
    }
}
