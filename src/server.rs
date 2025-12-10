use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::post,
    Router,
};
use tracing::{error, info, instrument};

/// Creates the router and runs the web server.
pub async fn run_server(state: AppState) {
    let app = Router::new()
        .route("/healthcheck/:entity_id", post(healthcheck_handler))
        .with_state(state.clone());

    let listener_addr = format!("{}:{}", state.config.watcher_host, state.config.watcher_port);
    info!("Healthcheck listener running on {}", listener_addr);
    let listener = tokio::net::TcpListener::bind(&listener_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// The axum handler for incoming healthcheck pings.
#[instrument(skip(state), fields(entity_id = %entity_id))]
async fn healthcheck_handler(
    Path(entity_id): Path<String>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut con = match state.redis_client.get_async_connection().await {
        Ok(con) => con,
        Err(e) => {
            error!("Failed to get Redis connection: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let timestamp = chrono::Utc::now().timestamp();
    info!("Received healthcheck, updating timestamp to {}", timestamp);

    if let Err(e) = redis::cmd("ZADD")
        .arg("entity_healthchecks")
        .arg(timestamp)
        .arg(&entity_id)
        .query_async::<_, ()>(&mut con)
        .await
    {
        error!("Failed to execute ZADD command: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

