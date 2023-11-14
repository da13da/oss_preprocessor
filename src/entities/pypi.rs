use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::date::deserialize_date;

#[derive(Debug, Serialize, Deserialize)]
pub struct PyPIPackageResponse {
    pub info: PackageInfo,
    pub releases: HashMap<String, Vec<ReleaseInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub filename: String,
    pub url: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub upload_time: DateTime<Utc>,
}

impl PyPIPackageResponse {
    pub fn latest_version(&self) -> Option<String> {
        let mut latest_release: Option<(&String, &Vec<ReleaseInfo>)> = None;
        for (version, releases) in &self.releases {
            if let Some(release) = releases.iter().max_by_key(|r| &r.upload_time) {
                if latest_release.is_none()
                    || latest_release.unwrap().1[0].upload_time < release.upload_time
                {
                    latest_release = Some((version, releases));
                }
            }
        }

        latest_release.map(|(version, _)| version.clone())
    }
}
