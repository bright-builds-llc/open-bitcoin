// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn append_structured_log_record_writes_jsonl() {
    // Arrange
    let log_dir = test_log_dir("append");
    let record = StructuredLogRecord::new(
        StructuredLogLevel::Warn,
        "sync",
        "peer stalled",
        1_777_225_022,
    );

    // Act
    let written_path =
        append_structured_log_record(&log_dir, &record, LogRetentionPolicy::default())
            .expect("append record");

    // Assert
    assert_eq!(
        written_path.file_name().and_then(|name| name.to_str()),
        Some("open-bitcoin-runtime-20569.jsonl")
    );
    let encoded = fs::read_to_string(&written_path).expect("read log file");
    let decoded: StructuredLogRecord =
        serde_json::from_str(encoded.trim_end()).expect("decode record");
    assert_eq!(decoded, record);
}

#[test]
fn load_log_status_reads_bounded_recent_signals() {
    // Arrange
    let log_dir = test_log_dir("status");
    let policy = LogRetentionPolicy::default();
    let records = [
        StructuredLogRecord::new(StructuredLogLevel::Info, "sync", "connected", 10),
        StructuredLogRecord::new(StructuredLogLevel::Warn, "sync", "peer stalled", 20),
        StructuredLogRecord::new(StructuredLogLevel::Error, "storage", "write failed", 30),
    ];
    for record in &records {
        append_structured_log_record(&log_dir, record, policy).expect("append record");
    }

    // Act
    let status = load_log_status(&log_dir, policy, 1);

    // Assert
    assert_eq!(
        status.path,
        LogPathStatus::available(log_dir.display().to_string())
    );
    assert_eq!(status.retention, policy);
    assert_eq!(status.recent_signals.len(), 1);
    assert_eq!(status.recent_signals[0].source, "storage");
    assert_eq!(status.recent_signals[0].message, "write failed");
}

#[test]
fn missing_log_directory_reports_unavailable_status() {
    // Arrange
    let log_dir = test_log_dir("missing").join("not-created");
    let policy = LogRetentionPolicy::default();

    // Act
    let status = load_log_status(&log_dir, policy, 10);

    // Assert
    assert_eq!(status.retention, policy);
    assert!(status.recent_signals.is_empty());
    match status.path {
        LogPathStatus::Unavailable { reason } => {
            assert!(reason.contains("log path unavailable:"));
            assert!(reason.contains(&log_dir.display().to_string()));
        }
        LogPathStatus::Available { path } => {
            panic!("expected unavailable log path, got {path}");
        }
    }
}
