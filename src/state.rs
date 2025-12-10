use crate::config::Config;
use reqwest::Client as ReqwestClient;
use std::sync::Arc;

// A struct to hold the application's shared state.
#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub http_client: ReqwestClient,
    pub config: Arc<Config>,
}


