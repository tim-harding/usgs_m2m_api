use std::fmt::Display;
use thiserror::Error;

use crate::ErrorCode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApiException {
    pub code: ErrorCode,
    pub message: String,
}

impl Display for ApiException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiException {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Error)]
pub enum UsgsError {
    #[error("{0}")]
    ApiException(#[from] ApiException),
    #[error("{0}")]
    Http(#[from] reqwest::Error),
}
