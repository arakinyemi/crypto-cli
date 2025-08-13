use thiserror::Error;
use std::env::var;

#[derive(Debug)]
pub struct Config{
    pub api_key: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing API key")]
    MissingApiKey,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        var("API_KEY") 
            .map(|k| Self { api_key: k })
            .map_err(|_| ConfigError::MissingApiKey)
    }
}
