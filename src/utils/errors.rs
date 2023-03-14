use std::fmt::{Display, Result};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub enum LoadingError {
    Io(std::io::Error),
    Invalid(toml::de::Error),
    Encoding(std::string::FromUtf8Error),
}

impl Display for LoadingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            LoadingError::Io(error) => write!(formatter, "IO error: {}", error),
            LoadingError::Invalid(error) => write!(formatter, "TOML error: {}", error),
            LoadingError::Encoding(error) => write!(formatter, "UTF-8 conversion error: {}", error),
        }
    }
}

impl From<std::io::Error> for LoadingError {
    fn from(error: std::io::Error) -> Self {
        LoadingError::Io(error)
    }
}

impl From<toml::de::Error> for LoadingError {
    fn from(error: toml::de::Error) -> Self {
        LoadingError::Invalid(error)
    }
}

impl From<std::string::FromUtf8Error> for LoadingError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        LoadingError::Encoding(error)
    }
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post title")]
    InvalidTitle,
    #[error("Post not found")]
    PostNotFound,
    #[error("Internal server error")]
    ServerError,
}
