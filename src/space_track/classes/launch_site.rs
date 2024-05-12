use crate::{
    space_track::{config::OrderByField, error::Error, urls::LAUNCH_SITE_URL, Config},
    SpaceTrack,
};
use serde::{Deserialize, Serialize};

pub enum LaunchSiteField {
    SiteCode,
    LaunchSite,
}

impl OrderByField for LaunchSiteField {
    fn field(&self) -> &str {
        match self {
            LaunchSiteField::SiteCode => "SITE_CODE",
            LaunchSiteField::LaunchSite => "LAUNCH_SITE",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct LaunchSite {
    pub site_code: String,
    pub launch_site: String,
}

impl SpaceTrack {
    pub async fn launch_site(
        &mut self,
        config: Config<LaunchSiteField>,
    ) -> Result<Vec<LaunchSite>, Error> {
        Ok(self.get(LAUNCH_SITE_URL, config).await?.json().await?)
    }

    pub async fn all_launch_site(&mut self) -> Result<Vec<LaunchSite>, Error> {
        self.launch_site(Config::empty()).await
    }
}
