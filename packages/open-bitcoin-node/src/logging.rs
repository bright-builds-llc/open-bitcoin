// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Serializable structured log retention and status contracts.

use serde::{Deserialize, Serialize};

use crate::status::{HealthSignal, HealthSignalLevel};

#[cfg(test)]
mod tests;

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

/// Log file rotation cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogRotation {
    Hourly,
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

/// Recent log-derived health signal placeholder.
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
