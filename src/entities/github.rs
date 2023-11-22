use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub zipball_url: String,
    pub tarball_url: String,
    pub commit: Commit,
    pub node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub sha: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompareData {
    status: String,
    ahead_by: i32,
    behind_by: i32,
    total_commits: i32,
    commits: Vec<RootCommit>,
    files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootCommit {
    pub sha: String,
    pub node_id: String,
    pub commit: CommitDetail,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub parents: Vec<Parent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitDetail {
    pub author: Author,
    pub committer: Committer,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub comment_count: i64,
    pub verification: Verification,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: i64,
    pub deletions: i64,
    pub changes: i64,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    #[serde(default = "default_patch_value")]
    pub patch: String,
}

fn default_patch_value() -> String {
    "".to_string()
}
