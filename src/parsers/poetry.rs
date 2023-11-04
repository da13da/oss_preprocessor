use structopt::StructOpt;
use toml;
use regex::Regex;

use crate::parsers::{Parser, ParseError};
use crate::entities::package::{Package, LockFile};

pub struct PoetryParser {}

impl PoetryParser {
    pub fn new() -> Self {
        PoetryParser {}
    }
}

impl Parser for PoetryParser {
    fn parse(&self, data: &String) -> Result<Vec<Package>, ParseError> {
        let lock_file = toml::from_str::<LockFile>(data.as_str())?;
        Ok(lock_file.packages)
    }
}
