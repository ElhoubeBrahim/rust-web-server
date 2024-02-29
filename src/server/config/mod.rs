mod types;

use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;

use types::{CorsConfig, FileSystemConfig, HostConfig, LogsConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
    host: HostConfig,
    file_system: FileSystemConfig,
    logs: LogsConfig,
    cors: CorsConfig,
}

impl Config {
    pub fn new() -> Config {
        let config = fs::read_to_string("server/config.toml")
            .expect("Failed to read the configuration file");
        let config: Config =
            toml::from_str(config.as_str()).expect("Failed to parse the configuration file");

        config
    }

    pub fn host(&self) -> &HostConfig {
        &self.host
    }
    pub fn file_system(&self) -> &FileSystemConfig {
        &self.file_system
    }
    pub fn logs(&self) -> &LogsConfig {
        &self.logs
    }
    pub fn cors(&self) -> &CorsConfig {
        &self.cors
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
