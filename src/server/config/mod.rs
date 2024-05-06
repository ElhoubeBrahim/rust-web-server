mod types;

use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;

use types::{CorsConfig, FileSystemConfig, HostConfig, LogsConfig};

use self::types::ConnectionConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    host: HostConfig,
    file_system: FileSystemConfig,
    connection: ConnectionConfig,
    logs: LogsConfig,
    cors: CorsConfig,
}

impl Config {
    pub fn new() -> Option<Config> {
        // Read config file to string
        let result = fs::read_to_string("server/config.toml");
        match result {
            Ok(_) => {}
            Err(e) => {
                return None;
            }
        }
        let config = result.unwrap();

        // Parse config file to Config struct
        let result = toml::from_str(config.as_str());
        match result {
            Ok(_) => {}
            Err(e) => {
                return None;
            }
        }
        let config: Config = result.unwrap();

        // Return the Config struct
        Some(config)
    }

    pub fn host(&self) -> &HostConfig {
        &self.host
    }
    pub fn file_system(&self) -> &FileSystemConfig {
        &self.file_system
    }

    pub fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    pub fn logs(&self) -> &LogsConfig {
        &self.logs
    }
    pub fn cors(&self) -> &CorsConfig {
        &self.cors
    }
}

lazy_static! {
    pub static ref CONFIG: Option<Config> = Config::new();
}
