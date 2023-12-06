use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::date::deserialize_date;

#[derive(Debug, Serialize, Deserialize)]
pub struct PyPIPackageDetail {
    pub info: PackageInfo,
    pub releases: HashMap<String, Vec<ReleaseInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub home_page: Option<String>,
    pub project_urls: HashMap<String, String>,
    pub platform: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub filename: String,
    pub url: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub upload_time: DateTime<Utc>,
}

impl PyPIPackageDetail {
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

    pub fn extract_git_url(&self) -> Option<String> {
        let home_page = if self.info.home_page.is_some() {
            self.info.home_page.clone().unwrap()
        } else {
            "".to_string()
        };
        if home_page.contains("github.com") {
            return Some(home_page);
        }

        for (managed_name, project_url) in &self.info.project_urls {
            if project_url.contains("github.com") || project_url.contains("gitlab.com") {
                if project_url.ends_with(self.info.name.as_str())
                    || project_url.ends_with(self.info.name.replace("-", "_").as_str())
                    || project_url
                        .to_lowercase()
                        .ends_with(self.info.name.to_lowercase().as_str())
                {
                    return Some(project_url.clone());
                }
            }
        }

        None
    }
}
