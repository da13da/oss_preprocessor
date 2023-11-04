use std::error::Error;
use std::fs::{File, read_to_string};
use std::{fs, io};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use serde_json::Value;
use toml;

use crate::entities::package::Package;

mod pipfile;
mod poetry;
use self::pipfile::PipfileParser;
use self::poetry::PoetryParser;

pub trait Parser {
    fn parse(&self, data: &String) -> Result<Vec<Package>, ParseError>;
}

#[derive(Debug)]
pub enum ParseError {
    TomlError(toml::de::Error),
}

impl From<toml::de::Error> for ParseError {
    fn from(err: toml::de::Error) -> Self {
        ParseError::TomlError(err)
    }
}

pub struct LockFileParseClient {
    pub file_path: PathBuf,
    pub lock_file_str: String,
    pub parser: Box<dyn Parser>,
}

impl LockFileParseClient {
    pub fn new(path_buf: PathBuf) -> Result<Self, io::Error> {
        let file_name = path_buf.file_name().unwrap().to_string_lossy();
        if !Self::is_allowed_file(&file_name) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File not allowed.",
            ));
        }

        let path = path_buf.as_path();
        let lock_file_str = read_to_string(path)
            .expect("lock file read failed");

        let parser = Self::resolve_parser(&file_name);
        if parser.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File not implemented.",
            ));
        }

        Ok(Self {
            file_path: path_buf,
            lock_file_str: lock_file_str,
            parser: parser.unwrap(),
        })
    }

    pub fn parse(&self) -> Result<Vec<Package>, ParseError> {
        self.parser.parse(&self.lock_file_str)
    }

    fn resolve_parser(file_name: &Cow<str>) -> Option<Box<dyn Parser>> {
        match file_name.as_ref() {
            "Pipfile.lock" => Some(Box::new(PipfileParser::new())),
            "poetry.lock" => Some(Box::new(PoetryParser::new())),
            _ => None,
        }
    }

    fn is_allowed_file(file_name: &Cow<str>) -> bool {
        let json_file = read_to_string("src/configs/allowed_files.json")
            .expect("Json read failed");
        let allowed_files: Vec<String> = serde_json::from_str(&json_file).unwrap();

        allowed_files.contains(&file_name.to_string())
    }
}
