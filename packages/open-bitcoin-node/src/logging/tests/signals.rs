// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
