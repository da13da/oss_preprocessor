use structopt::StructOpt;
use toml;
use regex::Regex;

use crate::parsers::{Parser, ParseError};
use crate::entities::package::Package;

pub struct PipfileParser {}

impl PipfileParser {
    pub fn new() -> Self {
        PipfileParser {}
    }
}

impl Parser for PipfileParser {
    fn parse(&self, data: &String) -> Result<Vec<Package>, ParseError> {
        println!("ToDo");

        let packages: Vec<Package> = Vec::new();
        Ok(packages)
    }
}
