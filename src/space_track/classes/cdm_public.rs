use crate::{
    space_track::{
        config::OrderByField,
        error::Error,
        urls::CDM_PUBLIC_URL,
        utils::{deserialize_optional_string_to_f64, deserialize_string_to_u64},
        Config,
    },
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum CdmPublicField {
    CdmId,
    Created,
    EmergencyReportable,
    Tca,
    MinRng,
    Pc,
    Sat1Id,
    Sat1Name,
    Sat1ObjectType,
    Sat1Rcs,
    Sat1ExclVol,
    Sat2Id,
    Sat2Name,
    Sat2ObjectType,
    Sat2Rcs,
    Sat2ExclVol,
}

impl OrderByField for CdmPublicField {
    fn field(&self) -> &str {
        match self {
            CdmPublicField::CdmId => "CDM_ID",
            CdmPublicField::Created => "CREATED",
            CdmPublicField::EmergencyReportable => "EMERGENCY_REPORTABLE",
            CdmPublicField::Tca => "TCA",
            CdmPublicField::MinRng => "MIN_RNG",
            CdmPublicField::Pc => "PC",
            CdmPublicField::Sat1Id => "SAT_1_ID",
            CdmPublicField::Sat1Name => "SAT_1_NAME",
            CdmPublicField::Sat1ObjectType => "SAT1_OBJECT_TYPE",
            CdmPublicField::Sat1Rcs => "SAT1_RCS",
            CdmPublicField::Sat1ExclVol => "SAT_1_EXCL_VOL",
            CdmPublicField::Sat2Id => "SAT_2_ID",
            CdmPublicField::Sat2Name => "SAT_2_NAME",
            CdmPublicField::Sat2ObjectType => "SAT2_OBJECT_TYPE",
            CdmPublicField::Sat2Rcs => "SAT2_RCS",
            CdmPublicField::Sat2ExclVol => "SAT_2_EXCL_VOL",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CdmPublic {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub cdm_id: u64,
    pub created: Option<String>,
    pub emergency_reportable: Option<String>,
    pub tca: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub min_rng: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_string_to_f64")]
    pub pc: Option<f64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub sat_1_id: u64,
    pub sat_1_name: Option<String>,
    pub sat1_object_type: Option<String>,
    pub sat1_rcs: Option<String>,
    pub sat_1_excl_vol: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub sat_2_id: u64,
    pub sat_2_name: Option<String>,
    pub sat2_object_type: Option<String>,
    pub sat2_rcs: Option<String>,
    pub sat_2_excl_vol: Option<String>,
}

impl SpaceTrack {
    pub async fn cdm_public(
        &mut self,
        config: Config<CdmPublicField>,
    ) -> Result<Vec<CdmPublic>, Error> {
        Ok(self.get(CDM_PUBLIC_URL, config).await?.json().await?)
    }

    pub async fn all_cdm_public(&mut self) -> Result<Vec<CdmPublic>, Error> {
        self.cdm_public(Config::empty()).await
    }
}
