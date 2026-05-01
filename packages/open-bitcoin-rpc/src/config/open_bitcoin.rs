// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/common/args.cpp

//! Open Bitcoin-owned JSONC config contracts.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ConfigError;

pub const OPEN_BITCOIN_CONFIG_FILE_NAME: &str = "open-bitcoin.jsonc";

/// Open Bitcoin-only settings layered above baseline `bitcoin.conf`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OpenBitcoinConfig {
    pub schema_version: u32,
    pub onboarding: OnboardingConfig,
    pub metrics: MetricsConfig,
    pub logs: LogsConfig,
    pub service: ServiceConfig,
    pub dashboard: DashboardConfig,
    pub migration: MigrationConfig,
    pub storage: StorageConfig,
    pub sync: SyncConfig,
}

impl Default for OpenBitcoinConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            onboarding: OnboardingConfig::default(),
            metrics: MetricsConfig::default(),
            logs: LogsConfig::default(),
            service: ServiceConfig::default(),
            dashboard: DashboardConfig::default(),
            migration: MigrationConfig::default(),
            storage: StorageConfig::default(),
            sync: SyncConfig::default(),
        }
    }
}

/// First-run wizard and rerun policy answers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OnboardingConfig {
    pub wizard_answers: BTreeMap<String, String>,
    pub completed_steps: Vec<String>,
    pub non_interactive: bool,
}

/// Metrics config defaults matching the Phase 13 observability contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub sample_interval_seconds: u64,
    pub max_samples_per_series: usize,
    pub max_age_seconds: u64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sample_interval_seconds: 30,
            max_samples_per_series: 2_880,
            max_age_seconds: 86_400,
        }
    }
}

/// Structured log config defaults matching the Phase 13 observability contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LogsConfig {
    pub enabled: bool,
    pub rotation: String,
    pub max_files: u16,
    pub max_age_days: u16,
    pub max_total_bytes: u64,
}

impl Default for LogsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rotation: "daily".to_string(),
            max_files: 14,
            max_age_days: 14,
            max_total_bytes: 268_435_456,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ServiceConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DashboardConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MigrationConfig {
    pub detect_existing_installations: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StorageConfig {
    pub engine: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SyncConfig {
    pub network_enabled: bool,
    pub mode: String,
    #[serde(rename = "manual_peers")]
    pub maybe_manual_peers: Option<Vec<String>>,
    #[serde(rename = "dns_seeds")]
    pub maybe_dns_seeds: Option<Vec<String>>,
    #[serde(rename = "target_outbound_peers")]
    pub maybe_target_outbound_peers: Option<usize>,
}

impl SyncConfig {
    pub fn disabled() -> Self {
        Self {
            network_enabled: false,
            mode: "disabled".to_string(),
            maybe_manual_peers: None,
            maybe_dns_seeds: None,
            maybe_target_outbound_peers: None,
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self::disabled()
    }
}

/// Config source precedence identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    CliFlags,
    Environment,
    OpenBitcoinJsonc,
    BitcoinConf,
    Cookies,
    Defaults,
}

/// Deterministic precedence model for all config sources.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConfigPrecedence;

impl ConfigPrecedence {
    pub const fn ordered_sources() -> [ConfigSource; 6] {
        [
            ConfigSource::CliFlags,
            ConfigSource::Environment,
            ConfigSource::OpenBitcoinJsonc,
            ConfigSource::BitcoinConf,
            ConfigSource::Cookies,
            ConfigSource::Defaults,
        ]
    }
}

pub fn parse_open_bitcoin_jsonc_config(text: &str) -> Result<OpenBitcoinConfig, ConfigError> {
    jsonc_parser::parse_to_serde_value(text, &Default::default()).map_err(|error| {
        ConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: {error}"
        ))
    })
}
