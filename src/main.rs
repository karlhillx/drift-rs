use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct TelemetryPacket {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    source_id: String,
    timestamp: DateTime<Utc>,
    instrument_id: String,
    readings: serde_json::Value,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    uptime_seconds: u64,
}

#[tokio::main]
async fn main() {
    // a basic tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/telemetry", post(ingest_telemetry));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    info!("Drift telemetry sink listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "operational".to_string(),
        uptime_seconds: 0, // mock uptime
    })
}

async fn ingest_telemetry(Json(payload): Json<TelemetryPacket>) -> Json<serde_json::Value> {
    info!("Ingested packet from source: {}", payload.source_id);
    Json(serde_json::json!({ "id": payload.id, "status": "received" }))
}
