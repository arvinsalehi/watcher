use std::env;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Config {
    pub watcher_host: String,
    pub watcher_port: String,
    pub redis_url: String,
    pub logging_api_url: String,
    pub stale_threshold_seconds: u64,
    pub check_interval_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            watcher_host: env::var("WATCHER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            watcher_port: env::var("WATCHER_PORT").unwrap_or_else(|_| "8080".to_string()),
            redis_url: env::var("REDIS_URL").map_err(|e| format!("REDIS_URL: {}", e))?,
            logging_api_url: env::var("LOGGING_API_URL")
                .map_err(|e| format!("LOGGING_API_URL: {}", e))?,
            stale_threshold_seconds: parse_env_var("STALE_THRESHOLD_SECONDS", "30")?,
            check_interval_seconds: parse_env_var("CHECK_INTERVAL_SECONDS", "10")?,
        })
    }
}

fn parse_env_var(var_name: &str, default: &str) -> Result<u64, String> {
    env::var(var_name)
        .unwrap_or_else(|_| default.to_string())
        .parse()
        .map_err(|e: ParseIntError| format!("{}: {}", var_name, e))
}

