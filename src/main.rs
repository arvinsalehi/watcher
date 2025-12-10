mod config;
mod server;
mod state;
mod worker;

use config::Config;
use reqwest::Client as ReqwestClient;
use state::AppState;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging.
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let config = Arc::new(Config::from_env().expect("Failed to load configuration"));

    // Initialize shared state.
    let app_state = AppState {
        redis_client: redis::Client::open(config.redis_url.clone())
            .expect("Failed to create Redis client"),
        http_client: ReqwestClient::new(),
        config: config.clone(),
    };

    // Spawn the background worker task.
    worker::spawn_worker_task(app_state.clone());

    // Run the web server.
    server::run_server(app_state).await;
}

