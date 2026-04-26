// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Serializable structured log retention and status contracts.

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::{LogPathStatus, LogRetentionPolicy, LogRotation, LogStatus, StructuredLogLevel};

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
}
