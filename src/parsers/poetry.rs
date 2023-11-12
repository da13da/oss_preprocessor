use regex::Regex;
use std::fs::{read_to_string, File};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml;

use crate::entities::package::Package;
use crate::entities::poetry::PoetryLockFile;
use crate::parsers::error::ParseError;
use crate::parsers::Parser;

pub struct PoetryParser {}

impl PoetryParser {
    pub fn new() -> Self {
        PoetryParser {}
    }
}

impl Parser for PoetryParser {
    fn parse(&self, path: &PathBuf) -> Result<Vec<Package>, ParseError> {
        let data = read_to_string(path).expect("lock file read failed");
        let lock_file = toml::from_str::<PoetryLockFile>(data.as_str())?;

        Ok(lock_file.to_common_packages())
    }
}
