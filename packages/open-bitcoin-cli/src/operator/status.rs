// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator status contract surface.

use std::{path::PathBuf, time::Duration};

use super::{NetworkSelection, config::OperatorConfigResolution, detect::DetectedInstallation};

/// Operator status request supplied by CLI flags and config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRequest {
    /// Requested render mode.
    pub render_mode: StatusRenderMode,
    /// Optional Open Bitcoin JSONC config path.
    pub maybe_config_path: Option<PathBuf>,
    /// Optional datadir path.
    pub maybe_data_dir: Option<PathBuf>,
    /// Optional Bitcoin network selection.
    pub maybe_network: Option<NetworkSelection>,
    /// Whether collectors may attempt live RPC.
    pub include_live_rpc: bool,
    /// Whether human output should disable color.
    pub no_color: bool,
}

/// Status rendering mode requested by the operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusRenderMode {
    /// Quiet human-readable status.
    Human,
    /// Stable serde-backed JSON rendering.
    Json,
}

/// Inputs needed by a status collector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCollectorInput {
    /// Operator status request.
    pub request: StatusRequest,
    /// Config resolution evidence.
    pub config_resolution: OperatorConfigResolution,
    /// Read-only detection evidence.
    pub detection_evidence: StatusDetectionEvidence,
    /// Optional live RPC adapter input.
    pub maybe_live_rpc: Option<StatusLiveRpcAdapterInput>,
}

/// Detection evidence available to status collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusDetectionEvidence {
    /// Detected installations from the read-only detector.
    pub detected_installations: Vec<DetectedInstallation>,
}

/// Live RPC adapter input without credential values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLiveRpcAdapterInput {
    /// RPC endpoint.
    pub endpoint: String,
    /// Credential source only; no credential contents.
    pub auth_source: StatusRpcAuthSource,
    /// Request timeout.
    pub timeout: Duration,
}

/// Live RPC auth source without sensitive values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusRpcAuthSource {
    /// Use a cookie file path.
    CookieFile {
        /// Cookie path only; never cookie contents.
        path: PathBuf,
    },
    /// User credentials were configured elsewhere, but values are not carried.
    UserCredentialsConfigured,
    /// No auth source is configured.
    None,
}

#[cfg(test)]
mod tests;
