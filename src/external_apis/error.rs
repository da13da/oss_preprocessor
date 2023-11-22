use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("network error: {0}")]
    Network(#[from] ReqwestError),
    #[error("parsing error: {0}")]
    Parse(#[from] SerdeJsonError),
    #[error("unexpected status code: {0}")]
    StatusCode(u16),
}
