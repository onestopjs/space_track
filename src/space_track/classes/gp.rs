use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::GP_URL,
        utils::{
            deserialize_optional_string_to_f64, deserialize_optional_string_to_u16,
            deserialize_optional_string_to_u32, deserialize_optional_string_to_u64,
            deserialize_optional_string_to_u8, deserialize_string_to_u64,
        },
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum GeneralPerturbationField {
    CcsdsOmmVers,
    Comment,
    CreationDate,
    Originator,
    ObjectName,
    ObjectId,
    CenterName,
    RefFrame,
    TimeSystem,
    MeanElementTheory,
    Epoch,
    MeanMotion,
    Eccentricity,
    Inclination,
    RaOfAscNode,
    ArgOfPericenter,
    MeanAnomaly,
    EphemerisType,
    ClassificationType,
    NoradCatId,
    ElementSetNo,
    RevAtEpoch,
    Bstar,
    MeanMotionDot,
    MeanMotionDdot,
    SemimajorAxis,
    Period,
    Apoapsis,
    Periapsis,
    ObjectType,
    RcsSize,
    CountryCode,
    LaunchDate,
    Site,
    DecayDate,
    File,
    GpId,
    TleLine0,
    TleLine1,
    TleLine2,
}

impl OrderByField for GeneralPerturbationField {
    fn field(&self) -> &str {
        match self {
            GeneralPerturbationField::CcsdsOmmVers => "CCSDS_OMM_VERS",
            GeneralPerturbationField::Comment => "COMMENT",
            GeneralPerturbationField::CreationDate => "CREATION_DATE",
            GeneralPerturbationField::Originator => "ORIGINATOR",
            GeneralPerturbationField::ObjectName => "OBJECT_NAME",
            GeneralPerturbationField::ObjectId => "OBJECT_ID",
            GeneralPerturbationField::CenterName => "CENTER_NAME",
            GeneralPerturbationField::RefFrame => "REF_FRAME",
            GeneralPerturbationField::TimeSystem => "TIME_SYSTEM",
            GeneralPerturbationField::MeanElementTheory => "MEAN_ELEMENT_THEORY",
            GeneralPerturbationField::Epoch => "EPOCH",
            GeneralPerturbationField::MeanMotion => "MEAN_MOTION",
            GeneralPerturbationField::Eccentricity => "ECCENTRICITY",
            GeneralPerturbationField::Inclination => "INCLINATION",
            GeneralPerturbationField::RaOfAscNode => "RA_OF_ASC_NODE",
            GeneralPerturbationField::ArgOfPericenter => "ARG_OF_PERICENTER",
            GeneralPerturbationField::MeanAnomaly => "MEAN_ANOMALY",
            GeneralPerturbationField::EphemerisType => "EPHEMERIS_TYPE",
            GeneralPerturbationField::ClassificationType => "CLASSIFICATION_TYPE",
            GeneralPerturbationField::NoradCatId => "NORAD_CAT_ID",
            GeneralPerturbationField::ElementSetNo => "ELEMENT_SET_NO",
            GeneralPerturbationField::RevAtEpoch => "REV_AT_EPOCH",
            GeneralPerturbationField::Bstar => "BSTAR",
            GeneralPerturbationField::MeanMotionDot => "MEAN_MOTION_DOT",
            GeneralPerturbationField::MeanMotionDdot => "MEAN_MOTION_DDOT",
            GeneralPerturbationField::SemimajorAxis => "SEMIMAJOR_AXIS",
            GeneralPerturbationField::Period => "PERIOD",
            GeneralPerturbationField::Apoapsis => "APOAPSIS",
            GeneralPerturbationField::Periapsis => "PERIAPSIS",
            GeneralPerturbationField::ObjectType => "OBJECT_TYPE",
            GeneralPerturbationField::RcsSize => "RCS_SIZE",
            GeneralPerturbationField::CountryCode => "COUNTRY_CODE",
            GeneralPerturbationField::LaunchDate => "LAUNCH_DATE",
            GeneralPerturbationField::Site => "SITE",
            GeneralPerturbationField::DecayDate => "DECAY_DATE",
            GeneralPerturbationField::File => "FILE",
            GeneralPerturbationField::GpId => "GP_ID",
            GeneralPerturbationField::TleLine0 => "TLE_LINE0",
            GeneralPerturbationField::TleLine1 => "TLE_LINE1",
            GeneralPerturbationField::TleLine2 => "TLE_LINE2",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct GeneralPerturbation {
    pub ccsds_omm_vers: String,
    pub comment: String,
    pub creation_date: Option<String>,
    pub originator: String,
    pub object_name: Option<String>,
    pub object_id: Option<String>,
    pub center_name: String,
    pub ref_frame: String,
    pub time_system: String,
    pub mean_element_theory: String,
    pub epoch: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub mean_motion: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub eccentricity: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub inclination: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub ra_of_asc_node: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub arg_of_pericenter: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub mean_anomaly: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u8")]
    pub ephemeris_type: Option<u8>,
    pub classification_type: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub norad_cat_id: u64,
    #[serde(deserialize_with = "deserialize_optional_string_to_u16")]
    pub element_set_no: Option<u16>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u32")]
    pub rev_at_epoch: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub bstar: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub mean_motion_dot: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub mean_motion_ddot: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub semimajor_axis: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub period: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub apoapsis: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub periapsis: Option<f64>,
    pub object_type: Option<String>,
    pub rcs_size: Option<String>,
    pub country_code: Option<String>,
    pub launch_date: Option<String>,
    pub site: Option<String>,
    pub decay_date: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string_to_u64")]
    pub file: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub gp_id: u64,
    pub tle_line0: Option<String>,
    pub tle_line1: Option<String>,
    pub tle_line2: Option<String>,
}

impl SpaceTrack {
    pub async fn gp(
        &mut self,
        config: Config<GeneralPerturbationField>,
    ) -> Result<Vec<GeneralPerturbation>, Error> {
        Ok(self.get(GP_URL, config).await?.json().await?)
    }

    pub async fn all_gp(&mut self) -> Result<Vec<GeneralPerturbation>, Error> {
        self.gp(Config::empty()).await
    }
}
