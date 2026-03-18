use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPacket {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub source_id: String,
    pub timestamp: DateTime<Utc>,
    pub instrument_id: String,
    pub readings: HashMap<String, f64>,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct IngestionResponse {
    pub id: Uuid,
    pub status: String,
}
