use serde_json;
use thiserror::Error;
use toml;
use yarn_lock_parser::YarnLockError;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Poetry parsing error: {0}")]
    PoetryParseError(#[from] toml::de::Error),

    #[error("yarn.lock parsing error {0}")]
    YarnLockError(#[from] YarnLockError),

    #[error("Incompatible Pipfile.lock spec: {0}")]
    IncompatiblePipfileLockSpec(i32),

    #[error("Deserialize error: {0}")]
    Deserialize(#[from] serde_json::Error),
}
