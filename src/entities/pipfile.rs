use std::{collections::HashMap, fmt::Display, io::Read};
use thiserror::Error;

use serde::Deserialize;
use serde_json::{from_reader, from_slice};

use crate::entities::package::{Package, Source};
use crate::parsers::lockfile::error::ParseError;

#[derive(Debug, Deserialize)]
pub struct Meta {
    #[serde(rename = "pipfile-spec")]
    pub pipfile_spec: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Dependency {
    Git {
        #[serde(rename = "ref")]
        git_ref: String,
    },
    Pip {
        version: String,
    },
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git { git_ref } => write!(f, "{git_ref}"),
            Self::Pip { version } => write!(f, "{}", version.trim_start_matches("==")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PipfileLock {
    #[serde(rename = "_meta")]
    pub meta: Meta,
    pub default: HashMap<String, Dependency>,
    pub develop: HashMap<String, Dependency>,
}

impl PipfileLock {
    fn validate(self) -> Result<Self, ParseError> {
        if self.meta.pipfile_spec != 6 {
            return Err(ParseError::IncompatiblePipfileLockSpec(
                self.meta.pipfile_spec,
            ));
        }

        Ok(self)
    }

    pub fn from_reader<R: Read>(reader: R) -> Result<Self, ParseError> {
        let pipfile: PipfileLock = from_reader(reader)?;

        pipfile.validate()
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, ParseError> {
        let pipfile: PipfileLock = from_slice(slice)?;

        pipfile.validate()
    }

    pub fn to_common_packages(&self) -> Vec<Package> {
        self.default
            .iter()
            .filter_map(|(name, dependency)| {
                if let Dependency::Pip { version } = dependency {
                    Some(Package {
                        name: name.clone(),
                        current_version: version.clone(),
                        latest_version: None,
                        source: Source::pypi,
                        homepage: None,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
