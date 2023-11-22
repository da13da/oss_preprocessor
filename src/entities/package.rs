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

impl Package {
    pub fn extract_owner_repo(&self) -> Option<(String, String)> {
        if self.homepage.is_none() {
            return None;
        }
        let url = self.homepage.as_ref().unwrap();
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() >= 5 && parts[2] == "github.com" {
            return Some((parts[3].to_string(), parts[4].to_string()));
        }
        None
    }
}
