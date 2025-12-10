use crate::state::AppState;
use serde::Serialize;
use std::time::Duration;
use tracing::{error, info, instrument};

// Struct for the JSON payload sent to the logging API.
#[derive(Serialize)]
struct StaleEntitiesPayload<'a> {
    stale_entities: &'a [String],
}

/// Spawns the background task that periodically checks for stale entities.
pub fn spawn_worker_task(state: AppState) {
    tokio::spawn(async move {
        info!("Stale entity checker started.");
        let mut interval = tokio::time::interval(Duration::from_secs(state.config.check_interval_seconds));
        loop {
            interval.tick().await;
            if let Err(e) = check_for_stale_entities(&state).await {
                error!("Error in stale entity checker: {}", e);
            }
        }
    });
}

/// The core logic for the worker: finds stale entities and reports them.
#[instrument(skip_all)]
async fn check_for_stale_entities(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let mut con = state.redis_client.get_async_connection().await?;
    let stale_timestamp = chrono::Utc::now().timestamp() - state.config.stale_threshold_seconds as i64;

    let stale_entities: Vec<String> = redis::cmd("ZRANGEBYSCORE")
        .arg("entity_healthchecks")
        .arg("-inf")
        .arg(stale_timestamp)
        .query_async(&mut con)
        .await?;

    if stale_entities.is_empty() {
        return Ok(());
    }

    info!("Found {} stale entities. Reporting...", stale_entities.len());

    let payload = StaleEntitiesPayload {
        stale_entities: &stale_entities,
    };

    let response = state
        .http_client
        .post(&state.config.logging_api_url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Successfully reported stale entities. Removing from Redis.");
        let _: () = redis::cmd("ZREMRANGEBYSCORE")
            .arg("entity_healthchecks")
            .arg("-inf")
            .arg(stale_timestamp)
            .query_async(&mut con)
            .await?;
    } else {
        error!(
            "Failed to report stale entities. API responded with status: {}. Will retry.",
            response.status()
        );
    }

    Ok(())
}

