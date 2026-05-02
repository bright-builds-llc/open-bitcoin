// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

use crate::{
    FieldAvailability, StorageError,
    logging::{StructuredLogLevel, StructuredLogRecord},
    status::{HealthSignal, HealthSignalLevel, PeerTelemetry},
};

use super::{PeerSyncOutcome, PeerSyncState, SyncPeerSource, SyncRunSummary};

pub(super) fn progress_ratio(block_height: u64, header_height: u64) -> f64 {
    if header_height == 0 {
        return 1.0;
    }

    (block_height as f64 / header_height as f64).min(1.0)
}

pub(super) fn peer_outcome_log_records(
    outcome: &PeerSyncOutcome,
    timestamp_unix_seconds: u64,
) -> Vec<StructuredLogRecord> {
    let mut records = Vec::new();
    match outcome.state {
        PeerSyncState::Connected => {}
        PeerSyncState::Stalled => records.push(StructuredLogRecord {
            level: StructuredLogLevel::Warn,
            source: "sync".to_string(),
            message: "peer stalled before sending more sync messages".to_string(),
            timestamp_unix_seconds,
        }),
        PeerSyncState::Failed => records.push(StructuredLogRecord {
            level: StructuredLogLevel::Error,
            source: "sync".to_string(),
            message: format!(
                "peer failed during sync reason={}",
                outcome
                    .maybe_failure_reason
                    .as_ref()
                    .map_or("unknown".to_string(), ToString::to_string)
            ),
            timestamp_unix_seconds,
        }),
    }

    if outcome.attempts > 1 {
        records.push(StructuredLogRecord {
            level: StructuredLogLevel::Warn,
            source: "sync".to_string(),
            message: format!("peer retry attempts={} before outcome", outcome.attempts),
            timestamp_unix_seconds,
        });
    }

    records
}

pub(super) fn health_signal_log_record(
    signal: &HealthSignal,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    StructuredLogRecord {
        level: structured_log_level(signal.level),
        source: signal.source.clone(),
        message: signal.message.clone(),
        timestamp_unix_seconds,
    }
}

pub(super) fn peer_telemetry(outcome: &PeerSyncOutcome) -> PeerTelemetry {
    PeerTelemetry {
        peer: outcome.peer.label(),
        source: match outcome.peer.source {
            SyncPeerSource::Manual => "manual".to_string(),
            SyncPeerSource::DnsSeed => "dns_seed".to_string(),
        },
        state: match outcome.state {
            PeerSyncState::Connected => "connected".to_string(),
            PeerSyncState::Stalled => "stalled".to_string(),
            PeerSyncState::Failed => "failed".to_string(),
        },
        network: outcome.network.as_str().to_string(),
        attempts: outcome.attempts,
        maybe_resolved_endpoint: match &outcome.maybe_resolved_endpoint {
            Some(value) => FieldAvailability::available(value.clone()),
            None => FieldAvailability::unavailable("resolved endpoint unavailable"),
        },
        capabilities: match &outcome.maybe_capabilities {
            Some(value) => FieldAvailability::available(format!(
                "services={} start_height={} wtxidrelay={} prefers_headers={} user_agent={}",
                value.services_bits,
                value.start_height,
                value.wtxidrelay,
                value.prefers_headers,
                value.user_agent
            )),
            None => FieldAvailability::unavailable("peer capabilities unavailable"),
        },
        headers_received: outcome.contribution.headers_received as u64,
        blocks_received: outcome.contribution.blocks_received as u64,
        maybe_last_activity_unix_seconds: match outcome.maybe_last_activity_unix_seconds {
            Some(value) => FieldAvailability::available(value),
            None => FieldAvailability::unavailable("peer last activity unavailable"),
        },
        failure_reason: match &outcome.maybe_failure_reason {
            Some(value) => FieldAvailability::available(value.to_string()),
            None => FieldAvailability::unavailable("peer healthy"),
        },
        error: match &outcome.maybe_error {
            Some(value) => FieldAvailability::available(value.clone()),
            None => FieldAvailability::unavailable("peer healthy"),
        },
    }
}

pub(super) fn sync_phase_name(summary: &SyncRunSummary) -> &'static str {
    if summary.best_block_height < summary.best_header_height {
        return "block_download";
    }
    if summary.headers_received > 0 {
        return "header_sync";
    }
    "steady_state"
}

pub(super) fn storage_health_message(error: &StorageError) -> String {
    match error {
        StorageError::InvalidSchemaVersion { .. } => {
            "storage schema version invalid during sync".to_string()
        }
        StorageError::SchemaMismatch { .. } => "storage schema mismatch during sync".to_string(),
        StorageError::Corruption { namespace, .. } => {
            format!("storage corruption in {} during sync", namespace.as_str())
        }
        StorageError::UnavailableNamespace { namespace } => {
            format!("storage namespace unavailable: {}", namespace.as_str())
        }
        StorageError::InterruptedWrite { namespace, .. } => {
            format!(
                "storage write interrupted in {} during sync",
                namespace.as_str()
            )
        }
        StorageError::BackendFailure { namespace, .. } => {
            format!(
                "storage backend failure in {} during sync",
                namespace.as_str()
            )
        }
    }
}

fn structured_log_level(level: HealthSignalLevel) -> StructuredLogLevel {
    match level {
        HealthSignalLevel::Info => StructuredLogLevel::Info,
        HealthSignalLevel::Warn => StructuredLogLevel::Warn,
        HealthSignalLevel::Error => StructuredLogLevel::Error,
    }
}
