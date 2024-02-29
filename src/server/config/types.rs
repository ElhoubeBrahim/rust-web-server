use std::net::Ipv4Addr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HostConfig {
  pub host: Ipv4Addr,
  pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct FileSystemConfig {
  pub root_dir: String,
  pub index_file: String,
  pub not_found: String,
  pub server_error: String,
}

#[derive(Debug, Deserialize)]
pub struct LogsConfig {
  pub access_log: String,
  pub error_log: String,
}

#[derive(Debug, Deserialize)]
pub struct CorsConfig {
  pub allow_methods: Vec<String>,
  pub allow_headers: Vec<String>,
  pub allow_origins: Vec<String>,
}
