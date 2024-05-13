use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::DECAY_URL,
        utils::{
            deserialize_optional_string_to_u64, deserialize_string_to_u64, deserialize_string_to_u8,
        },
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum DecayField {
    NoradCatId,
    ObjectNumber,
    ObjectName,
    IntlDes,
    ObjectId,
    Rcs,
    RcsSize,
    Country,
    MsgEpoch,
    DecayEpoch,
    Source,
    MsgType,
    Precedence,
}

impl OrderByField for DecayField {
    fn field(&self) -> &str {
        match self {
            DecayField::NoradCatId => "NORAD_CAT_ID",
            DecayField::ObjectNumber => "OBJECT_NUMBER",
            DecayField::ObjectName => "OBJECT_NAME",
            DecayField::IntlDes => "INTLDES",
            DecayField::ObjectId => "OBJECT_ID",
            DecayField::Rcs => "RCS",
            DecayField::RcsSize => "RCS_SIZE",
            DecayField::Country => "COUNTRY",
            DecayField::MsgEpoch => "MSG_EPOCH",
            DecayField::DecayEpoch => "DECAY_EPOCH",
            DecayField::Source => "SOURCE",
            DecayField::MsgType => "MSG_TYPE",
            DecayField::Precedence => "PRECEDENCE",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Decay {
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub norad_cat_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub object_number: Option<u64>,
    pub object_name: String,
    #[serde(rename = "INTLDES")]
    pub intl_des: String,
    pub object_id: String,
    #[serde(deserialize_with = "deserialize_string_to_u8")]
    pub rcs: u8,
    pub rcs_size: Option<String>,
    pub country: String,
    pub msg_epoch: Option<String>,
    pub decay_epoch: Option<String>,
    pub source: String,
    pub msg_type: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub precedence: u64,
}

impl SpaceTrack {
    pub async fn decay(&mut self, config: Config<DecayField>) -> Result<Vec<Decay>, Error> {
        Ok(self.get(DECAY_URL, config).await?.json().await?)
    }

    pub async fn all_decay(&mut self) -> Result<Vec<Decay>, Error> {
        self.decay(Config::empty()).await
    }
}
