use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use crate::models::{TelemetryPacket, StatusResponse, IngestionResponse};
use crate::telemetry::{state::AppState, validator::TelemetryValidator, errors::TelemetryError};
use tracing::info;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/telemetry", post(ingest_telemetry))
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> Json<StatusResponse> {
    let count = state.buffer.lock().unwrap().len();
    Json(StatusResponse {
        status: format!("operational: {} packets in memory", count),
        uptime_seconds: 0,
    })
}

async fn ingest_telemetry(
    State(state): State<AppState>,
    Json(payload): Json<TelemetryPacket>
) -> Result<Json<IngestionResponse>, TelemetryError> {
    // Advanced: Professional Validation Layer
    TelemetryValidator::validate(&payload)?;

    info!("Ingesting validated telemetry from source: {}", payload.source_id);

    // Advanced: In-memory store persistence
    let response_id = payload.id;
    state.push(payload);

    Ok(Json(IngestionResponse {
        id: response_id,
        status: "received_and_validated".to_string(),
    }))
}
