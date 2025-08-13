use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP or network error: {0}")]
    Request(reqwest::Error),

    #[error("API returned unexpected data: {0}")]
    NoData(String),

    #[error("JSON parsing error: {0}")]
    Json(serde_json::Error),
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Request(err)
    }
}