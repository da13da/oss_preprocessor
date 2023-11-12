use regex::Regex;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml;

use crate::entities::package::Package;
use crate::entities::pipfile::PipfileLock;
use crate::parsers::error::ParseError;
use crate::parsers::Parser;

pub struct PipfileParser {}

impl PipfileParser {
    pub fn new() -> Self {
        PipfileParser {}
    }
}

impl Parser for PipfileParser {
    fn parse(&self, path: &PathBuf) -> Result<Vec<Package>, ParseError> {
        let file = std::fs::File::open(&path).unwrap();
        let lockfile = PipfileLock::from_reader(file)?;

        Ok(lockfile.to_common_packages())
    }
}
