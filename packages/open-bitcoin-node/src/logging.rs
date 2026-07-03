// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Serializable structured log retention and status contracts.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{error::Error, fmt, path::PathBuf};

use crate::status::{
    HealthSignal, HealthSignalLevel, InboundPeerPolicyEvent, InboundResourceGovernanceEvent,
    relay_evidence::{
        RelayEvidenceCounters, RelayEvidenceField, RelayEvidenceStatus, RelayRecoveryCounters,
    },
};

pub mod prune;
pub mod writer;

#[cfg(test)]
mod tests;

pub const INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE: &str = "inbound_resource_governance";
pub const INBOUND_PEER_POLICY_LOG_SOURCE: &str = "inbound_peer_policy";
pub const RELAY_MEMPOOL_LOG_SOURCE: &str = "relay_mempool";
const REDACTED_RESOURCE_FIELD: &str = "redacted_resource_field";
const REDACTED_PEER_POLICY_FIELD: &str = "redacted_peer_policy_field";

/// Supported structured log levels for operator-facing summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Structured runtime log record written by Open Bitcoin-owned adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredLogRecord {
    pub level: StructuredLogLevel,
    pub source: String,
    pub message: String,
    pub timestamp_unix_seconds: u64,
}

/// Error returned by structured log filesystem adapters.
#[derive(Debug)]
pub enum StructuredLogError {
    Io {
        action: &'static str,
        path: PathBuf,
        source: std::io::Error,
    },
    Json {
        source: serde_json::Error,
    },
}

impl StructuredLogError {
    pub(crate) fn io(
        action: &'static str,
        path: impl Into<PathBuf>,
        source: std::io::Error,
    ) -> Self {
        Self::Io {
            action,
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for StructuredLogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                action,
                path,
                source,
            } => write!(
                formatter,
                "{action} failed for {}: {source}",
                path.display()
            ),
            Self::Json { source } => {
                write!(formatter, "structured log JSON encoding failed: {source}")
            }
        }
    }
}

impl Error for StructuredLogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source } => Some(source),
        }
    }
}

impl StructuredLogRecord {
    pub fn new(
        level: StructuredLogLevel,
        source: impl Into<String>,
        message: impl Into<String>,
        timestamp_unix_seconds: u64,
    ) -> Self {
        Self {
            level,
            source: source.into(),
            message: message.into(),
            timestamp_unix_seconds,
        }
    }
}

pub fn inbound_resource_governance_log_record(
    event: &InboundResourceGovernanceEvent,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    let message = format!(
        "outcome={} reason={} label={} source={} message={} next_action={}",
        sanitized_resource_log_field(&event.outcome),
        sanitized_resource_log_field(&event.reason),
        sanitized_resource_log_field(&event.label),
        sanitized_resource_log_field(&event.source),
        sanitized_resource_log_field(&event.message),
        sanitized_resource_log_field(&event.next_action)
    );

    StructuredLogRecord::new(
        StructuredLogLevel::Warn,
        INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE,
        message,
        timestamp_unix_seconds,
    )
}

pub fn inbound_peer_policy_log_record(
    event: &InboundPeerPolicyEvent,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    let message = format!(
        "outcome={} reason={} label={} source={} message={}",
        sanitized_peer_policy_log_field(&event.outcome),
        sanitized_peer_policy_log_field(&event.reason),
        sanitized_peer_policy_log_field(&event.label),
        sanitized_peer_policy_log_field(&event.source),
        sanitized_peer_policy_log_field(&event.message)
    );

    StructuredLogRecord::new(
        StructuredLogLevel::Warn,
        INBOUND_PEER_POLICY_LOG_SOURCE,
        message,
        timestamp_unix_seconds,
    )
}

pub fn relay_mempool_log_record(
    relay: &RelayEvidenceStatus,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    let counters = match &relay.outcome_counters {
        RelayEvidenceField::Implemented(counters) => *counters,
        RelayEvidenceField::Unavailable { .. }
        | RelayEvidenceField::Deferred { .. }
        | RelayEvidenceField::IntentionallyDifferent { .. } => RelayEvidenceCounters::default(),
    };
    let recovery = match &relay.recovery_counters {
        RelayEvidenceField::Implemented(counters) => *counters,
        RelayEvidenceField::Unavailable { .. }
        | RelayEvidenceField::Deferred { .. }
        | RelayEvidenceField::IntentionallyDifferent { .. } => RelayRecoveryCounters::default(),
    };
    let message = format!(
        "accepted={} rejected={} orphaned={} requested={} served={} announced={} suppressed={} evicted={} expired={} rebroadcast_deferred={} recovered={} dropped_confirmed={} dropped_duplicate={} dropped_missing_parent={} dropped_policy_incompatible={} dropped_evicted={}",
        counters.accepted_count,
        counters.rejected_count,
        counters.orphaned_count,
        counters.requested_count,
        counters.served_count,
        counters.announced_count,
        counters.suppressed_count,
        counters.evicted_count,
        counters.expired_count,
        counters.rebroadcast_deferred_count,
        recovery.recovered_count,
        recovery.dropped_confirmed_count,
        recovery.dropped_duplicate_count,
        recovery.dropped_missing_parent_count,
        recovery.dropped_policy_incompatible_count,
        recovery.dropped_evicted_count
    );

    StructuredLogRecord::new(
        StructuredLogLevel::Info,
        RELAY_MEMPOOL_LOG_SOURCE,
        message,
        timestamp_unix_seconds,
    )
}

fn sanitized_resource_log_field(value: &str) -> Cow<'_, str> {
    if value.is_empty()
        || value.len() > 128
        || value.contains('=')
        || contains_sensitive_resource_marker(value)
        || looks_like_socket_address(value)
        || looks_like_hex_material(value)
    {
        return Cow::Borrowed(REDACTED_RESOURCE_FIELD);
    }

    Cow::Borrowed(value)
}

fn sanitized_peer_policy_log_field(value: &str) -> Cow<'_, str> {
    if value.is_empty()
        || value.len() > 128
        || value.contains('=')
        || contains_sensitive_peer_policy_marker(value)
        || looks_like_socket_address(value)
        || looks_like_hex_material(value)
    {
        return Cow::Borrowed(REDACTED_PEER_POLICY_FIELD);
    }

    Cow::Borrowed(value)
}

fn contains_sensitive_resource_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains(&["peer", "_id"].concat())
        || lower.contains(&["raw", "_end", "point"].concat())
        || lower.contains(&["payload", "_bytes"].concat())
        || lower.contains(&["permission", "_string"].concat())
        || lower.contains(&["cred", "ential"].concat())
        || lower.contains(&["sec", "ret"].concat())
}

fn contains_sensitive_peer_policy_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains(&["peer", "_id"].concat())
        || lower.contains("peer-")
        || lower.contains(&["raw", "_end", "point"].concat())
        || lower.contains(&["payload", "_bytes"].concat())
        || lower.contains(&["permission", "_string"].concat())
        || lower.contains(&["cred", "ential"].concat())
        || lower.contains(&["sec", "ret"].concat())
        || lower.contains("cookie")
}

fn looks_like_socket_address(value: &str) -> bool {
    if value.contains("://") {
        return true;
    }

    let Some((host, port)) = value.rsplit_once(':') else {
        return false;
    };
    !host.is_empty()
        && !port.is_empty()
        && port.chars().all(|character| character.is_ascii_digit())
        && host
            .chars()
            .any(|character| character == '.' || character == ':')
}

fn looks_like_hex_material(value: &str) -> bool {
    let maybe_hex = value.strip_prefix("0x").unwrap_or(value);
    maybe_hex.len() >= 16
        && maybe_hex
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_hexdigit())
}

/// Log file rotation cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogRotation {
    Daily,
}

/// Bounded retention policy for structured log files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogRetentionPolicy {
    pub rotation: LogRotation,
    pub max_files: u16,
    pub max_age_days: u16,
    pub max_total_bytes: u64,
}

impl Default for LogRetentionPolicy {
    fn default() -> Self {
        Self {
            rotation: LogRotation::Daily,
            max_files: 14,
            max_age_days: 14,
            max_total_bytes: 268_435_456,
        }
    }
}

/// Availability of a structured log path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum LogPathStatus {
    Available { path: String },
    Unavailable { reason: String },
}

impl LogPathStatus {
    pub fn available(path: impl Into<String>) -> Self {
        Self::Available { path: path.into() }
    }

    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }
}

/// Recent log-derived warning or error signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentLogSignal {
    pub level: StructuredLogLevel,
    pub source: String,
    pub message: String,
    pub timestamp_unix_seconds: u64,
}

/// Logging status projection embedded in the shared node status snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogStatus {
    pub path: LogPathStatus,
    pub retention: LogRetentionPolicy,
    pub recent_signals: Vec<RecentLogSignal>,
}

impl Default for LogStatus {
    fn default() -> Self {
        Self {
            path: LogPathStatus::unavailable("log path unavailable until runtime logger starts"),
            retention: LogRetentionPolicy::default(),
            recent_signals: Vec::new(),
        }
    }
}

pub fn recent_log_signals_from_records(
    records: &[StructuredLogRecord],
    limit: usize,
) -> Vec<RecentLogSignal> {
    let mut signals: Vec<(usize, RecentLogSignal)> = records
        .iter()
        .enumerate()
        .filter_map(|(index, record)| {
            recent_log_signal_from_record(record).map(|signal| (index, signal))
        })
        .collect();

    signals.sort_by(|(left_index, left), (right_index, right)| {
        right
            .timestamp_unix_seconds
            .cmp(&left.timestamp_unix_seconds)
            .then_with(|| left_index.cmp(right_index))
    });
    signals.truncate(limit);
    signals.into_iter().map(|(_, signal)| signal).collect()
}

pub fn health_signals_from_recent_logs(signals: &[RecentLogSignal]) -> Vec<HealthSignal> {
    signals
        .iter()
        .filter_map(|signal| {
            let level = match signal.level {
                StructuredLogLevel::Warn => HealthSignalLevel::Warn,
                StructuredLogLevel::Error => HealthSignalLevel::Error,
                StructuredLogLevel::Trace
                | StructuredLogLevel::Debug
                | StructuredLogLevel::Info => {
                    return None;
                }
            };

            Some(HealthSignal {
                level,
                source: signal.source.clone(),
                message: signal.message.clone(),
            })
        })
        .collect()
}

fn recent_log_signal_from_record(record: &StructuredLogRecord) -> Option<RecentLogSignal> {
    if !matches!(
        record.level,
        StructuredLogLevel::Warn | StructuredLogLevel::Error
    ) {
        return None;
    }

    Some(RecentLogSignal {
        level: record.level,
        source: record.source.clone(),
        message: record.message.clone(),
        timestamp_unix_seconds: record.timestamp_unix_seconds,
    })
}
