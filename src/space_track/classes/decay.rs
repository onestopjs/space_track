use serde::Deserialize;

use crate::{
    space_track::{
        error::Error,
        urls::DECAY_URL,
        utils::{
            deserialize_optional_string_to_u64, deserialize_string_to_u64, deserialize_string_to_u8,
        },
        Config,
    },
    SpaceTrack,
};

/*
Field	Type	Null	Key	Default	Extra
NORAD_CAT_ID	int(11) unsigned	YES
OBJECT_NUMBER	int(11) unsigned	YES
OBJECT_NAME	char(25)	NO
INTLDES	char(12)	NO
OBJECT_ID	char(12)	NO
RCS	int(1)	NO		0
RCS_SIZE	varchar(6)	YES
COUNTRY	char(6)	NO
MSG_EPOCH	datetime	YES
DECAY_EPOCH	varchar(24)	YES
SOURCE	varchar(9)	NO
MSG_TYPE	varchar(10)	NO
PRECEDENCE	bigint(20)	NO		0
*/

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Decay {
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub norad_cat_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub object_number: Option<u64>,
    pub object_name: String,
    pub intldes: String,
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
    pub async fn decay(&mut self, config: Config) -> Result<Vec<Decay>, Error> {
        Ok(self.get(DECAY_URL, config).await?.json().await?)
    }

    pub async fn all_decay(&mut self) -> Result<Vec<Decay>, Error> {
        self.decay(Config::empty()).await
    }
}
