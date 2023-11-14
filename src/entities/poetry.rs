use serde::Deserialize;
use thiserror::Error;
use toml;

use crate::entities::package::{Package, Source};

#[derive(Deserialize, Debug, PartialEq)]
pub struct PoetryPackage {
    pub category: Option<String>,
    pub description: String,
    pub name: String,
    pub optional: bool,
    #[serde(rename(deserialize = "python-versions"))]
    pub python_versions: String,
    pub version: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PoetryLockFile {
    #[serde(rename(deserialize = "package"))]
    pub packages: Vec<PoetryPackage>,
}

impl PoetryLockFile {
    pub fn to_common_packages(&self) -> Vec<Package> {
        self.packages
            .iter()
            .map(|pkg| Package {
                name: pkg.name.clone(),
                current_version: pkg.version.clone(),
                latest_version: None,
                source: Source::pypi,
            })
            .collect()
    }
}
