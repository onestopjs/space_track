use crate::{
    space_track::{error::Error, urls::GP_HISTORY_URL},
    Config, GeneralPerturbation, GeneralPerturbationField, SpaceTrack,
};

impl SpaceTrack {
    pub async fn gp_history(
        &mut self,
        config: Config<GeneralPerturbationField>,
        norad_cat_id: u64,
    ) -> Result<Vec<GeneralPerturbation>, Error> {
        let url = format!("{}/NORAD_CAT_ID/{}", GP_HISTORY_URL, norad_cat_id);

        Ok(self.get(&url, config).await?.json().await?)
    }

    pub async fn all_gp_history(
        &mut self,
        norad_cat_id: u64,
    ) -> Result<Vec<GeneralPerturbation>, Error> {
        self.gp_history(Config::empty(), norad_cat_id).await
    }
}
