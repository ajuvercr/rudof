use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error("Reading JSON from {path_name:?}. Error: {error:?}")]
    JsonError {
        path_name: String,
        error: serde_json::Error,
    },

    #[error("Unknown error")]
    Unknown,
}