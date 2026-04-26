// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    LogPathStatus, LogRetentionPolicy, LogRotation, LogStatus, RecentLogSignal, StructuredLogLevel,
    StructuredLogRecord, health_signals_from_recent_logs, recent_log_signals_from_records,
};
use crate::status::HealthSignalLevel;

#[test]
fn default_log_retention_matches_operator_contract() {
    // Arrange / Act
    let policy = LogRetentionPolicy::default();

    // Assert
    assert_eq!(policy.rotation, LogRotation::Daily);
    assert_eq!(policy.max_files, 14);
    assert_eq!(policy.max_age_days, 14);
    assert_eq!(policy.max_total_bytes, 268_435_456);
}

#[test]
fn structured_log_levels_use_snake_case_json() {
    // Arrange
    let levels = [
        (StructuredLogLevel::Trace, "\"trace\""),
        (StructuredLogLevel::Debug, "\"debug\""),
        (StructuredLogLevel::Info, "\"info\""),
        (StructuredLogLevel::Warn, "\"warn\""),
        (StructuredLogLevel::Error, "\"error\""),
    ];

    // Act / Assert
    for (level, expected_json) in levels {
        let encoded = serde_json::to_string(&level).expect("level json");
        assert_eq!(encoded, expected_json);
    }
}

#[test]
fn unavailable_log_path_round_trips_through_json() {
    // Arrange
    let path = LogPathStatus::unavailable("node stopped");

    // Act
    let encoded = serde_json::to_string(&path).expect("path json");
    let decoded: LogPathStatus = serde_json::from_str(&encoded).expect("path decode");

    // Assert
    assert_eq!(decoded, path);
    assert!(encoded.contains("node stopped"));
}

#[test]
fn default_log_status_exposes_retention_and_no_signals() {
    // Arrange / Act
    let status = LogStatus::default();

    // Assert
    assert_eq!(status.retention, LogRetentionPolicy::default());
    assert!(status.recent_signals.is_empty());
    assert_eq!(
        serde_json::to_value(&status.path).expect("path json")["state"],
        "unavailable"
    );
}

#[test]
fn structured_log_record_serializes_required_fields() {
    // Arrange
    let record = StructuredLogRecord::new(
        StructuredLogLevel::Warn,
        "sync",
        "peer stalled",
        1_777_225_022,
    );

    // Act
    let encoded = serde_json::to_value(&record).expect("record json");

    // Assert
    assert_eq!(encoded["level"], "warn");
    assert_eq!(encoded["source"], "sync");
    assert_eq!(encoded["message"], "peer stalled");
    assert_eq!(encoded["timestamp_unix_seconds"], 1_777_225_022);
}

#[test]
fn recent_log_signals_filter_bound_and_order_warnings_errors() {
    // Arrange
    let records = vec![
        StructuredLogRecord::new(StructuredLogLevel::Info, "sync", "connected", 10),
        StructuredLogRecord::new(StructuredLogLevel::Warn, "sync", "peer stalled", 20),
        StructuredLogRecord::new(StructuredLogLevel::Error, "storage", "write failed", 30),
        StructuredLogRecord::new(StructuredLogLevel::Warn, "logging", "rotate delayed", 30),
    ];

    // Act
    let signals = recent_log_signals_from_records(&records, 2);

    // Assert
    assert_eq!(signals.len(), 2);
    assert_eq!(signals[0].level, StructuredLogLevel::Error);
    assert_eq!(signals[0].source, "storage");
    assert_eq!(signals[0].message, "write failed");
    assert_eq!(signals[1].level, StructuredLogLevel::Warn);
    assert_eq!(signals[1].source, "logging");
    assert_eq!(signals[1].message, "rotate delayed");
}

#[test]
fn recent_log_signals_map_to_health_sources() {
    // Arrange
    let signals = vec![
        RecentLogSignal {
            level: StructuredLogLevel::Warn,
            source: "sync".to_string(),
            message: "peer stalled".to_string(),
            timestamp_unix_seconds: 30,
        },
        RecentLogSignal {
            level: StructuredLogLevel::Error,
            source: "storage".to_string(),
            message: "write failed".to_string(),
            timestamp_unix_seconds: 20,
        },
    ];

    // Act
    let health_signals = health_signals_from_recent_logs(&signals);

    // Assert
    assert_eq!(health_signals.len(), 2);
    assert_eq!(health_signals[0].level, HealthSignalLevel::Warn);
    assert_eq!(health_signals[0].source, "sync");
    assert_eq!(health_signals[0].message, "peer stalled");
    assert_eq!(health_signals[1].level, HealthSignalLevel::Error);
    assert_eq!(health_signals[1].source, "storage");
    assert_eq!(health_signals[1].message, "write failed");
}
