use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct ConfigToml {
    config: Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub file: String,
    pub memory: String,
}

pub enum ErrorType {
    FileNotFound,
    Other(Box<dyn std::error::Error>),
}

pub fn parse_file() -> Result<Config, ErrorType> {
    let content = fs::read("config.toml").map_err(|_| ErrorType::FileNotFound)?;
    let config: ConfigToml =
        toml::from_str(std::str::from_utf8(&content).map_err(|e| ErrorType::Other(Box::new(e)))?)
            .map_err(|e| ErrorType::Other(Box::new(e)))?;

    Ok(config.config)
}
