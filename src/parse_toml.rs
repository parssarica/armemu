use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub config: Config,
    pub debugview: Option<Debugview>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub code: Option<String>,
    pub binary: Option<String>,
    pub memory: String,
}

#[derive(Debug, Deserialize)]
pub struct Debugview {
    pub debugmode: bool,
}

pub enum ErrorType {
    FileNotFound,
    Other(()),
}

pub fn parse_file() -> Result<ConfigToml, ErrorType> {
    let content = fs::read("config.toml").map_err(|_| ErrorType::FileNotFound)?;
    let config: ConfigToml =
        toml::from_str(std::str::from_utf8(&content).map_err(|_| ErrorType::Other(()))?)
            .map_err(|_| ErrorType::Other(()))?;

    Ok(config)
}

pub fn parse_memory(mem_indicator: &str) -> usize {
    let x = mem_indicator.replace(" ", "");

    if x.len() < 1 {
        return 0;
    }

    let mut multiplier = 1;

    if x.len() > 2 {
        multiplier = match &*(x[(x.len() - 2)..].to_uppercase()) {
            "TB" => 1099511627776,
            "GB" => 1073741824,
            "MB" => 1048576,
            "KB" => 1024,
            _ => 1,
        }
    }

    let mut last_index = x.len();
    if multiplier != 1 {
        last_index -= 2;
    }

    let Ok(num_part) = &x[..last_index].parse::<usize>() else {
        return 0;
    };

    num_part * multiplier
}
