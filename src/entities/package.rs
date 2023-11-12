use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: Source,
}

#[derive(Debug, Deserialize)]
pub enum Source {
    pypi,
    npm,
    gem,
}
