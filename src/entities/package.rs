use clap::builder::Str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub homepage: Option<String>,
    pub source: Source,
}

#[derive(Debug, Deserialize)]
pub enum Source {
    pypi,
    npm,
    gem,
}
