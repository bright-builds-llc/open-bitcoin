// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use serde::{Deserialize, Serialize};

use super::FieldAvailability;

/// Recent operator health signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthSignal {
    pub level: HealthSignalLevel,
    pub source: String,
    pub message: String,
}

/// Severity of a health signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthSignalLevel {
    Info,
    Warn,
    Error,
}

/// Build metadata displayed in status and support output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildProvenance {
    pub version: String,
    pub commit: FieldAvailability<String>,
    pub build_time: FieldAvailability<String>,
    pub target: FieldAvailability<String>,
    pub profile: FieldAvailability<String>,
}

impl BuildProvenance {
    pub fn unavailable() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            commit: FieldAvailability::unavailable("commit unavailable"),
            build_time: FieldAvailability::unavailable("build time unavailable"),
            target: FieldAvailability::unavailable("target unavailable"),
            profile: FieldAvailability::unavailable("profile unavailable"),
        }
    }
}
