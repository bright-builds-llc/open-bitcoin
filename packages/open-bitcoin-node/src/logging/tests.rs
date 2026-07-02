// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    INBOUND_PEER_POLICY_LOG_SOURCE, INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE, LogPathStatus,
    LogRetentionPolicy, LogRotation, LogStatus, RELAY_MEMPOOL_LOG_SOURCE, RecentLogSignal,
    StructuredLogLevel, StructuredLogRecord, health_signals_from_recent_logs,
    inbound_peer_policy_log_record, inbound_resource_governance_log_record,
    recent_log_signals_from_records, relay_mempool_log_record,
};
use super::{
    prune::{LogFileMetadata, plan_log_retention},
    writer::{append_structured_log_record, load_log_status},
};
use crate::status::{
    HealthSignalLevel, InboundPeerPolicyEvent, InboundResourceGovernanceEvent,
    relay_evidence::{RelayEvidenceCounters, RelayEvidenceField, RelayEvidenceStatus},
};
use std::{
    fs,
    path::PathBuf,
    process,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

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
fn inbound_resource_governance_log_record_uses_allowlisted_fields() {
    // Arrange
    let event = InboundResourceGovernanceEvent {
        outcome: "rejected".to_string(),
        reason: "invalid_checksum".to_string(),
        label: "payload_rejected".to_string(),
        source: "source_envelope_gate".to_string(),
        message: "inbound_message_resource_governance".to_string(),
        next_action: "payload_rejected".to_string(),
    };

    // Act
    let record = inbound_resource_governance_log_record(&event, 1_777_225_022);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Warn);
    assert_eq!(record.source, INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_022);
    assert_eq!(
        record.message,
        "outcome=rejected reason=invalid_checksum label=payload_rejected source=source_envelope_gate message=inbound_message_resource_governance next_action=payload_rejected"
    );
}

#[test]
fn inbound_resource_governance_log_record_redacts_suspicious_raw_fields() {
    // Arrange
    let event = InboundResourceGovernanceEvent {
        outcome: "rejected".to_string(),
        reason: "peer_id=42".to_string(),
        label: "raw_endpoint=192.0.2.1:8333".to_string(),
        source: "payload_bytes=00112233445566778899aabbccddeeff".to_string(),
        message: "permission_string=admin credential=fixture".to_string(),
        next_action: "secret=fixture".to_string(),
    };

    // Act
    let record = inbound_resource_governance_log_record(&event, 1_777_225_022);

    // Assert
    assert_eq!(record.source, INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE);
    assert_eq!(
        record.message,
        "outcome=rejected reason=redacted_resource_field label=redacted_resource_field source=redacted_resource_field message=redacted_resource_field next_action=redacted_resource_field"
    );
    assert!(!record.message.contains("42"));
    assert!(!record.message.contains("192.0.2.1:8333"));
    assert!(!record.message.contains("00112233445566778899aabbccddeeff"));
    assert!(!record.message.contains("admin"));
}

#[test]
fn inbound_peer_policy_log_record_uses_allowlisted_fields() {
    // Arrange
    let event = InboundPeerPolicyEvent {
        outcome: "ban_active".to_string(),
        reason: "manual_ban".to_string(),
        label: "ban_active".to_string(),
        source: "source_peer_policy_runtime_bridge".to_string(),
        message: "peer_policy_runtime_bridge".to_string(),
    };

    // Act
    let record = inbound_peer_policy_log_record(&event, 1_777_225_023);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Warn);
    assert_eq!(record.source, INBOUND_PEER_POLICY_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_023);
    assert_eq!(
        record.message,
        "outcome=ban_active reason=manual_ban label=ban_active source=source_peer_policy_runtime_bridge message=peer_policy_runtime_bridge"
    );
}

#[test]
fn inbound_peer_policy_log_record_redacts_suspicious_raw_fields() {
    // Arrange
    let event = InboundPeerPolicyEvent {
        outcome: "ban_active".to_string(),
        reason: "127.0.0.1:18444".to_string(),
        label: "peer_id=42".to_string(),
        source: "raw_endpoint=192.0.2.1:8333".to_string(),
        message: "permission_string=admin credential=fixture secret=fixture cookie=value payload_bytes=00112233445566778899aabbccddeeff".to_string(),
    };

    // Act
    let record = inbound_peer_policy_log_record(&event, 1_777_225_023);

    // Assert
    assert_eq!(record.source, INBOUND_PEER_POLICY_LOG_SOURCE);
    assert_eq!(
        record.message,
        "outcome=ban_active reason=redacted_peer_policy_field label=redacted_peer_policy_field source=redacted_peer_policy_field message=redacted_peer_policy_field"
    );
    for raw in [
        "127.0.0.1:18444",
        "peer_id=42",
        "192.0.2.1:8333",
        "admin",
        "credential",
        "secret",
        "cookie=value",
        "00112233445566778899aabbccddeeff",
    ] {
        assert!(!record.message.contains(raw));
    }
}

#[test]
fn relay_mempool_log_record_uses_fixed_outcome_counts() {
    // Arrange
    let relay = RelayEvidenceStatus::with_counters(RelayEvidenceCounters {
        accepted_count: 1,
        rejected_count: 2,
        orphaned_count: 3,
        requested_count: 4,
        served_count: 5,
        announced_count: 6,
        suppressed_count: 7,
        evicted_count: 8,
        expired_count: 9,
        rebroadcast_deferred_count: 10,
    });

    // Act
    let record = relay_mempool_log_record(&relay, 1_777_225_105);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Info);
    assert_eq!(record.source, RELAY_MEMPOOL_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_105);
    assert_eq!(
        record.message,
        "accepted=1 rejected=2 orphaned=3 requested=4 served=5 announced=6 suppressed=7 evicted=8 expired=9 rebroadcast_deferred=10"
    );
}

#[test]
fn relay_mempool_log_record_omits_sensitive_and_dynamic_material() {
    // Arrange
    let mut relay = RelayEvidenceStatus::with_counters(RelayEvidenceCounters {
        accepted_count: 1,
        rejected_count: 0,
        orphaned_count: 0,
        requested_count: 0,
        served_count: 0,
        announced_count: 0,
        suppressed_count: 0,
        evicted_count: 0,
        expired_count: 0,
        rebroadcast_deferred_count: 0,
    });
    relay.mempool_admission = RelayEvidenceField::unavailable(
        "txid=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    );
    relay.local_submission =
        RelayEvidenceField::deferred("wtxid=abcdef0123456789 endpoint=127.0.0.1:18444");
    relay.fanout =
        RelayEvidenceField::deferred("peer_id=42 permission_string=forcerelay dynamic_label=raw");
    relay.public_relay = RelayEvidenceField::intentionally_different(
        "credential=fixture cookie=value secret=fixture reject_reason=freeform",
    );

    // Act
    let record = relay_mempool_log_record(&relay, 1_777_225_106);

    // Assert
    assert_eq!(record.source, RELAY_MEMPOOL_LOG_SOURCE);
    assert_eq!(
        record.message,
        "accepted=1 rejected=0 orphaned=0 requested=0 served=0 announced=0 suppressed=0 evicted=0 expired=0 rebroadcast_deferred=0"
    );
    for raw in [
        "0123456789abcdef",
        "wtxid",
        "127.0.0.1:18444",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
        "reject_reason",
    ] {
        assert!(!record.message.contains(raw), "leaked {raw}");
    }
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

#[test]
fn retention_prunes_by_max_files() {
    // Arrange
    let files = vec![
        log_metadata(10, 100),
        log_metadata(11, 100),
        log_metadata(12, 100),
        LogFileMetadata::new(PathBuf::from("debug.log"), 100),
    ];
    let policy = LogRetentionPolicy {
        max_files: 2,
        max_age_days: 30,
        max_total_bytes: 1_000,
        ..LogRetentionPolicy::default()
    };

    // Act
    let selected = plan_log_retention(&files, policy, 12 * 86_400);

    // Assert
    assert_eq!(selected, vec![managed_path(10)]);
}

#[test]
fn retention_prunes_by_max_age() {
    // Arrange
    let files = vec![
        log_metadata(17, 100),
        log_metadata(18, 100),
        log_metadata(20, 100),
    ];
    let policy = LogRetentionPolicy {
        max_files: 10,
        max_age_days: 2,
        max_total_bytes: 1_000,
        ..LogRetentionPolicy::default()
    };

    // Act
    let selected = plan_log_retention(&files, policy, 20 * 86_400);

    // Assert
    assert_eq!(selected, vec![managed_path(17)]);
}

#[test]
fn retention_prunes_by_total_bytes() {
    // Arrange
    let files = vec![
        log_metadata(10, 75),
        log_metadata(11, 75),
        log_metadata(12, 75),
    ];
    let policy = LogRetentionPolicy {
        max_files: 10,
        max_age_days: 30,
        max_total_bytes: 150,
        ..LogRetentionPolicy::default()
    };

    // Act
    let selected = plan_log_retention(&files, policy, 12 * 86_400);

    // Assert
    assert_eq!(selected, vec![managed_path(10)]);
}

fn test_log_dir(name: &str) -> PathBuf {
    let counter = NEXT_TEMP_DIR.fetch_add(1, Ordering::SeqCst);
    let path = std::env::temp_dir().join(format!(
        "open-bitcoin-logging-{name}-{}-{counter}",
        process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("remove stale test directory");
    }
    path
}

fn log_metadata(unix_day: u64, size_bytes: u64) -> LogFileMetadata {
    LogFileMetadata::new(managed_path(unix_day), size_bytes)
}

fn managed_path(unix_day: u64) -> PathBuf {
    PathBuf::from(format!("open-bitcoin-runtime-{unix_day}.jsonl"))
}
