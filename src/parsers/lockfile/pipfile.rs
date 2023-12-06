use std::path::PathBuf;

use crate::entities::package::Package;
use crate::entities::pipfile::PipfileLock;
use crate::parsers::lockfile::error::ParseError;
use crate::parsers::lockfile::Parser;

pub struct PipfileParser {}

impl PipfileParser {
    pub fn new() -> Self {
        PipfileParser {}
    }
}

impl Parser for PipfileParser {
    fn parse(&self, path: &PathBuf) -> Result<Vec<Package>, ParseError> {
        let file = std::fs::File::open(&path).expect("failed open file");
        let lockfile = PipfileLock::from_reader(file)?;

        Ok(lockfile.to_common_packages())
    }
}
