use crate::models::TelemetryPacket;
use crate::telemetry::errors::TelemetryError;
use tracing::warn;

pub struct TelemetryValidator;

impl TelemetryValidator {
    pub fn validate(packet: &TelemetryPacket) -> Result<(), TelemetryError> {
        // Example bounds checking for aerospace readings
        // For satellite attitude, battery, and solar panel voltage
        for (sensor, value) in &packet.readings {
            match sensor.as_str() {
                "battery_voltage" => {
                    if *value < 0.0 || *value > 100.0 {
                        warn!("Battery voltage out of bounds: {}", value);
                        return Err(TelemetryError::Validation("Battery voltage out of bounds".into()));
                    }
                }
                "solar_panel_current" => {
                    if *value < 0.0 {
                        warn!("Negative solar panel current: {}", value);
                        return Err(TelemetryError::Validation("Negative solar panel current".into()));
                    }
                }
                _ => {}
            }
        }

        if packet.source_id.is_empty() {
             return Err(TelemetryError::Validation("Empty source_id".into()));
        }

        Ok(())
    }
}
