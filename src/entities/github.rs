use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub tag_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompareData {
    status: String,
    ahead_by: i32,
    behind_by: i32,
    total_commits: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    pub commit: Commit,
    #[serde(rename = "node_id")]
    pub node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub sha: String,
    pub url: String,
}
