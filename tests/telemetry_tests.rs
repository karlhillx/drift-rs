use drift_rs::models::TelemetryPacket;
use drift_rs::telemetry::validator::TelemetryValidator;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_validator_valid_packet() {
    let mut readings = HashMap::new();
    readings.insert("battery_voltage".into(), 24.5);
    readings.insert("solar_panel_current".into(), 1.2);

    let packet = TelemetryPacket {
        id: Uuid::new_v4(),
        source_id: "SAT-01".into(),
        timestamp: Utc::now(),
        instrument_id: "POWER-MGMT".into(),
        readings,
    };

    assert!(TelemetryValidator::validate(&packet).is_ok());
}

#[test]
fn test_validator_invalid_battery() {
    let mut readings = HashMap::new();
    readings.insert("battery_voltage".into(), 105.0);

    let packet = TelemetryPacket {
        id: Uuid::new_v4(),
        source_id: "SAT-01".into(),
        timestamp: Utc::now(),
        instrument_id: "POWER-MGMT".into(),
        readings,
    };

    let result = TelemetryValidator::validate(&packet);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Battery voltage out of bounds"));
}

#[test]
fn test_validator_negative_current() {
    let mut readings = HashMap::new();
    readings.insert("solar_panel_current".into(), -0.1);

    let packet = TelemetryPacket {
        id: Uuid::new_v4(),
        source_id: "SAT-01".into(),
        timestamp: Utc::now(),
        instrument_id: "POWER-MGMT".into(),
        readings,
    };

    let result = TelemetryValidator::validate(&packet);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Negative solar panel current"));
}

#[test]
fn test_validator_missing_source() {
    let packet = TelemetryPacket {
        id: Uuid::new_v4(),
        source_id: "".into(),
        timestamp: Utc::now(),
        instrument_id: "NONE".into(),
        readings: HashMap::new(),
    };

    let result = TelemetryValidator::validate(&packet);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Empty source_id"));
}
