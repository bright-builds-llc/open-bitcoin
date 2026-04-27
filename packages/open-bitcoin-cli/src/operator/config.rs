// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/common/args.cpp

//! Operator config contract surface.

use std::{fmt, path::PathBuf};

use super::NetworkSelection;

/// Config source precedence used by operator status and onboarding output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperatorConfigSource {
    /// Explicit operator CLI flags.
    CliFlags,
    /// Open Bitcoin environment variables.
    Environment,
    /// Open Bitcoin-owned JSONC configuration.
    OpenBitcoinJsonc,
    /// Baseline-compatible `bitcoin.conf`.
    BitcoinConf,
    /// Cookie-file auth fallback.
    Cookies,
    /// Built-in defaults.
    Defaults,
}

impl OperatorConfigSource {
    /// Deterministic source order from the Phase 17 config precedence contract.
    pub const fn ordered() -> [Self; 6] {
        [
            Self::CliFlags,
            Self::Environment,
            Self::OpenBitcoinJsonc,
            Self::BitcoinConf,
            Self::Cookies,
            Self::Defaults,
        ]
    }

    /// Stable snake_case source name for user-facing reports and tests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CliFlags => "cli_flags",
            Self::Environment => "environment",
            Self::OpenBitcoinJsonc => "open_bitcoin_jsonc",
            Self::BitcoinConf => "bitcoin_conf",
            Self::Cookies => "cookies",
            Self::Defaults => "defaults",
        }
    }
}

impl fmt::Display for OperatorConfigSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Kind of path represented in an operator config report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperatorConfigPathKind {
    /// Open Bitcoin JSONC config path.
    ConfigFile,
    /// Baseline-compatible `bitcoin.conf` path.
    BitcoinConf,
    /// Operator datadir path.
    DataDir,
    /// Cookie file path without exposing cookie contents.
    CookieFile,
}

/// A single config-related path inspected or selected by a source.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperatorConfigPathReport {
    /// Source that supplied or inspected the path.
    pub source: OperatorConfigSource,
    /// Type of config path.
    pub kind: OperatorConfigPathKind,
    /// Filesystem path only; never a credential value.
    pub path: PathBuf,
    /// Whether the path existed when inspected.
    pub present: bool,
}

/// Resolved operator config evidence for status and onboarding consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorConfigResolution {
    /// Ordered sources used to resolve the final operator view.
    pub ordered_sources: Vec<OperatorConfigSource>,
    /// Paths inspected while resolving config.
    pub path_reports: Vec<OperatorConfigPathReport>,
    /// Selected Open Bitcoin JSONC config path, when known.
    pub maybe_config_path: Option<PathBuf>,
    /// Selected datadir path, when known.
    pub maybe_data_dir: Option<PathBuf>,
    /// Selected network, when known.
    pub maybe_network: Option<NetworkSelection>,
}

#[cfg(test)]
mod tests;
