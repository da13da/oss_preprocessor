use structopt::StructOpt;
use toml;
use regex::Regex;
use std::fs::{File, read_to_string};
use std::path::{Path, PathBuf};

use crate::parsers::Parser;
use crate::parsers::error::ParseError;
use crate::entities::poetry::PoetryLockFile;
use crate::entities::package::Package;

pub struct PoetryParser {}

impl PoetryParser {
    pub fn new() -> Self {
        PoetryParser {}
    }
}

impl Parser for PoetryParser {
    fn parse(&self, path: &PathBuf) -> Result<Vec<Package>, ParseError> {
        let data = read_to_string(path)
            .expect("lock file read failed");
        let lock_file = toml::from_str::<PoetryLockFile>(data.as_str())?;

        Ok(lock_file.to_common_packages())
    }
}
