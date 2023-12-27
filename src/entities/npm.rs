use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::date::deserialize_date;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpmPackageDetail {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_rev")]
    pub revision: String,
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: Vec<HashMap<String, Version>>,
    pub time: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistTags {
    pub latest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub version: String,
    pub description: String,
    pub main: String,
    pub scripts: Scripts,
    pub author: String,
    pub license: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_npmVersion")]
    pub npm_version: String,
    #[serde(rename = "_nodeVersion")]
    pub node_version: String,
    #[serde(rename = "_npmUser")]
    pub npm_user: User,
    pub dist: Dist,
    pub maintainers: Vec<User>,
    pub directories: Directories,
    #[serde(rename = "_npmOperationalInternal")]
    pub npm_operational_internal: NpmOperationalInternal,
    #[serde(rename = "_hasShrinkwrap")]
    pub has_shrinkwrap: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scripts {
    pub test: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
    pub file_count: i64,
    pub unpacked_size: i64,
    #[serde(rename = "npm-signature")]
    pub npm_signature: String,
    pub signatures: Vec<Signature>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    pub keyid: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directories {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpmOperationalInternal {
    pub host: String,
    pub tmp: String,
}
