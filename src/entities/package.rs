use serde::Deserialize;


#[derive(Deserialize, Debug, PartialEq)]
pub struct Package {
    pub category: Option<String>,
    pub description: String,
    pub name: String,
    pub optional: bool,
    #[serde(rename(deserialize = "python-versions"))]
    pub python_versions: String,
    pub version: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LockFile {
    #[serde(rename(deserialize = "package"))]
    pub packages: Vec<Package>,
}
