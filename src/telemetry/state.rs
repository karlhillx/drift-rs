use std::sync::{Arc, Mutex};
use crate::models::TelemetryPacket;
use std::collections::VecDeque;
use tracing::info;

pub struct SharedState {
    // Limited memory buffer for telemetry in a ring-buffer style for performance
    pub buffer: Mutex<VecDeque<TelemetryPacket>>,
    pub capacity: usize,
}

impl SharedState {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    pub fn push(&self, packet: TelemetryPacket) {
        let mut buffer = self.buffer.lock().unwrap();
        if buffer.len() >= self.capacity {
            buffer.pop_front();
        }
        buffer.push_back(packet);
        info!("Telemetry stored, current buffer size: {}", buffer.len());
    }
}

pub type AppState = Arc<SharedState>;
