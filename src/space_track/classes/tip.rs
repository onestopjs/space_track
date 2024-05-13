use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::TIP_URL,
        utils::{deserialize_string_to_f64, deserialize_string_to_u16, deserialize_string_to_u64},
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum TipField {
    NoradCatId,
    MsgEpoch,
    InsertEpoch,
    DecayEpoch,
    Window,
    Rev,
    Direction,
    Lat,
    Lon,
    Incl,
    NextReport,
    Id,
    HighInterest,
    ObjectNumber,
}

impl OrderByField for TipField {
    fn field(&self) -> &str {
        match self {
            TipField::NoradCatId => "NORAD_CAT_ID",
            TipField::MsgEpoch => "MSG_EPOCH",
            TipField::InsertEpoch => "INSERT_EPOCH",
            TipField::DecayEpoch => "DECAY_EPOCH",
            TipField::Window => "WINDOW",
            TipField::Rev => "REV",
            TipField::Direction => "DIRECTION",
            TipField::Lat => "LAT",
            TipField::Lon => "LON",
            TipField::Incl => "INCL",
            TipField::NextReport => "NEXT_REPORT",
            TipField::Id => "ID",
            TipField::HighInterest => "HIGH_INTEREST",
            TipField::ObjectNumber => "OBJECT_NUMBER",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Tip {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub norad_cat_id: u64,
    pub msg_epoch: String,
    pub insert_epoch: String,
    pub decay_epoch: String,
    #[serde(deserialize_with = "deserialize_string_to_u16")]
    pub window: u16,
    #[serde(deserialize_with = "deserialize_string_to_u16")]
    pub rev: u16,
    pub direction: String,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub lat: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub lon: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub incl: f64,
    #[serde(deserialize_with = "deserialize_string_to_u16")]
    pub next_report: u16,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub high_interest: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub object_number: u64,
}

impl SpaceTrack {
    pub async fn tip(&mut self, config: Config<TipField>) -> Result<Vec<Tip>, Error> {
        Ok(self.get(TIP_URL, config).await?.json().await?)
    }

    pub async fn all_tip(&mut self) -> Result<Vec<Tip>, Error> {
        self.tip(Config::empty()).await
    }
}
