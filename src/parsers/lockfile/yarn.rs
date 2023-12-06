use std::path::{Path, PathBuf};
use std::{error::Error, fs};
use yarn_lock_parser::{parse_str, Entry};

use crate::entities::package::{Package, Source};
use crate::parsers::lockfile::error::ParseError;
use crate::parsers::lockfile::Parser;

pub struct YarnParser {}

impl YarnParser {
    pub fn new() -> Self {
        YarnParser {}
    }
}

impl Parser for YarnParser {
    fn parse(&self, path: &PathBuf) -> Result<Vec<Package>, ParseError> {
        let file = fs::read_to_string(path).expect("failed read file");
        let entries = parse_str(&file)?;

        let packages = entries
            .iter()
            .map(|entry| Package {
                name: entry.name.to_string(),
                current_version: entry.version.to_string(),
                latest_version: None,
                source: Source::npm,
                homepage: None,
            })
            .collect::<Vec<Package>>();

        Ok(packages)
    }
}
