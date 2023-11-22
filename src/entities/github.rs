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
