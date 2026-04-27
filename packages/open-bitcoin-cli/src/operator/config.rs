// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/common/args.cpp

//! Operator config contract surface.

use std::{
    collections::BTreeMap,
    ffi::OsString,
    fmt, fs,
    path::{Path, PathBuf},
};

use open_bitcoin_rpc::config::{
    DEFAULT_COOKIE_FILE_NAME, OPEN_BITCOIN_CONFIG_FILE_NAME, OpenBitcoinConfig, RpcAuthConfig,
    load_runtime_config_for_args, parse_open_bitcoin_jsonc_config,
};

use super::{NetworkSelection, OperatorCli};

pub const OPEN_BITCOIN_CONFIG_ENV: &str = "OPEN_BITCOIN_CONFIG";
pub const OPEN_BITCOIN_DATADIR_ENV: &str = "OPEN_BITCOIN_DATADIR";
pub const OPEN_BITCOIN_NETWORK_ENV: &str = "OPEN_BITCOIN_NETWORK";
const BITCOIN_CONF_FILE_NAME: &str = "bitcoin.conf";
const LOG_DIRECTORY_NAME: &str = "logs";
const METRICS_STORE_DIRECTORY_NAME: &str = "metrics";

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
    /// Structured log directory.
    LogDirectory,
    /// Metrics store directory.
    MetricsStore,
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

/// Global operator flags that can affect config resolution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OperatorConfigRequest {
    /// Explicit Open Bitcoin JSONC config path from CLI flags.
    pub maybe_config_path: Option<PathBuf>,
    /// Explicit Open Bitcoin datadir from CLI flags.
    pub maybe_data_dir: Option<PathBuf>,
    /// Explicit network from CLI flags.
    pub maybe_network: Option<NetworkSelection>,
}

impl From<&OperatorCli> for OperatorConfigRequest {
    fn from(cli: &OperatorCli) -> Self {
        Self {
            maybe_config_path: cli.maybe_config_path.clone(),
            maybe_data_dir: cli.maybe_data_dir.clone(),
            maybe_network: cli.maybe_network,
        }
    }
}

/// Injected path roots for hermetic operator config resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorConfigRoots {
    /// Default Open Bitcoin datadir used when no higher-precedence source wins.
    pub default_data_dir: PathBuf,
}

impl OperatorConfigRoots {
    /// Create roots from the default Open Bitcoin datadir.
    pub fn new(default_data_dir: impl Into<PathBuf>) -> Self {
        Self {
            default_data_dir: default_data_dir.into(),
        }
    }

    fn default_config_path(&self) -> PathBuf {
        self.default_data_dir.join(OPEN_BITCOIN_CONFIG_FILE_NAME)
    }
}

/// Credential source metadata safe for status/config rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorCredentialSource {
    /// Cookie auth fallback by path only; the cookie value is never read.
    CookieFile {
        /// Cookie path.
        path: PathBuf,
        /// Whether the path existed when inspected.
        present: bool,
    },
    /// User/password auth was configured, but secret values are intentionally omitted.
    UserPasswordConfigured,
    /// No auth source could be inferred.
    None,
}

/// Resolved operator config evidence for status and onboarding consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorConfigResolution {
    /// Ordered sources used to resolve the final operator view.
    pub ordered_sources: Vec<OperatorConfigSource>,
    /// Sources considered for operator reporting in precedence order.
    pub sources_considered: Vec<OperatorConfigSource>,
    /// Paths inspected while resolving config.
    pub path_reports: Vec<OperatorConfigPathReport>,
    /// Selected Open Bitcoin JSONC config path, when known.
    pub maybe_config_path: Option<PathBuf>,
    /// Selected baseline-compatible `bitcoin.conf` path, when known.
    pub maybe_bitcoin_conf_path: Option<PathBuf>,
    /// Selected datadir path, when known.
    pub maybe_data_dir: Option<PathBuf>,
    /// Selected network, when known.
    pub maybe_network: Option<NetworkSelection>,
    /// Selected structured log directory, when known.
    pub maybe_log_dir: Option<PathBuf>,
    /// Selected metrics store directory, when known.
    pub maybe_metrics_store_path: Option<PathBuf>,
    /// Credential source metadata safe to render.
    pub credential_source: OperatorCredentialSource,
    /// Parsed Open Bitcoin-owned JSONC config, when present.
    pub maybe_open_bitcoin_config: Option<OpenBitcoinConfig>,
}

impl Default for OperatorConfigResolution {
    fn default() -> Self {
        let sources = OperatorConfigSource::ordered().to_vec();
        Self {
            ordered_sources: sources.clone(),
            sources_considered: sources,
            path_reports: Vec::new(),
            maybe_config_path: None,
            maybe_bitcoin_conf_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            maybe_log_dir: None,
            maybe_metrics_store_path: None,
            credential_source: OperatorCredentialSource::None,
            maybe_open_bitcoin_config: None,
        }
    }
}

impl OperatorConfigResolution {
    /// Stable source names in report order.
    pub fn source_names(&self) -> Vec<&'static str> {
        self.sources_considered
            .iter()
            .map(|source| source.as_str())
            .collect()
    }
}

/// Operator config resolution failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorConfigError {
    message: String,
}

impl OperatorConfigError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for OperatorConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for OperatorConfigError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Selected<T> {
    value: T,
    source: OperatorConfigSource,
}

/// Resolve operator config paths and source evidence without reading host-global state.
pub fn resolve_operator_config(
    request: &OperatorConfigRequest,
    environment: &BTreeMap<String, String>,
    roots: &OperatorConfigRoots,
) -> Result<OperatorConfigResolution, OperatorConfigError> {
    let selected_config_path = select_path(
        request.maybe_config_path.clone(),
        environment.get(OPEN_BITCOIN_CONFIG_ENV),
        roots.default_config_path(),
    );
    let maybe_open_bitcoin_config = load_open_bitcoin_jsonc(&selected_config_path.value)?;
    let maybe_jsonc_data_dir = maybe_open_bitcoin_config
        .as_ref()
        .and_then(|config| config.onboarding.wizard_answers.get("datadir"))
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let maybe_jsonc_network = maybe_open_bitcoin_config
        .as_ref()
        .and_then(|config| config.onboarding.wizard_answers.get("network"))
        .map(|value| parse_network(value))
        .transpose()?;

    let selected_data_dir = select_path_optional(
        request.maybe_data_dir.clone(),
        environment.get(OPEN_BITCOIN_DATADIR_ENV),
        maybe_jsonc_data_dir,
        roots.default_data_dir.clone(),
    );
    let selected_network = select_network(
        request.maybe_network,
        environment.get(OPEN_BITCOIN_NETWORK_ENV),
        maybe_jsonc_network,
    )?;
    let bitcoin_conf_path = selected_data_dir.value.join(BITCOIN_CONF_FILE_NAME);
    let runtime = load_runtime_from_evidence(
        &selected_data_dir.value,
        &bitcoin_conf_path,
        selected_network.as_ref().map(|selected| selected.value),
        roots,
    )?;
    let credential_source = credential_source_from_auth(
        &runtime.rpc_client.auth,
        selected_data_dir.value.join(DEFAULT_COOKIE_FILE_NAME),
    );
    let maybe_cookie_path = match &credential_source {
        OperatorCredentialSource::CookieFile { path, .. } => Some(path.clone()),
        OperatorCredentialSource::UserPasswordConfigured | OperatorCredentialSource::None => None,
    };
    let log_dir = selected_data_dir.value.join(LOG_DIRECTORY_NAME);
    let metrics_store_path = selected_data_dir.value.join(METRICS_STORE_DIRECTORY_NAME);

    let mut path_reports = vec![
        path_report(
            selected_config_path.source,
            OperatorConfigPathKind::ConfigFile,
            &selected_config_path.value,
        ),
        path_report(
            selected_data_dir.source,
            OperatorConfigPathKind::DataDir,
            &selected_data_dir.value,
        ),
        path_report(
            OperatorConfigSource::BitcoinConf,
            OperatorConfigPathKind::BitcoinConf,
            &bitcoin_conf_path,
        ),
        path_report(
            OperatorConfigSource::Defaults,
            OperatorConfigPathKind::LogDirectory,
            &log_dir,
        ),
        path_report(
            OperatorConfigSource::Defaults,
            OperatorConfigPathKind::MetricsStore,
            &metrics_store_path,
        ),
    ];
    if let Some(cookie_path) = maybe_cookie_path.as_ref() {
        path_reports.push(path_report(
            OperatorConfigSource::Cookies,
            OperatorConfigPathKind::CookieFile,
            cookie_path,
        ));
    }

    let sources = OperatorConfigSource::ordered().to_vec();
    Ok(OperatorConfigResolution {
        ordered_sources: sources.clone(),
        sources_considered: sources,
        path_reports,
        maybe_config_path: Some(selected_config_path.value),
        maybe_bitcoin_conf_path: Some(bitcoin_conf_path),
        maybe_data_dir: Some(selected_data_dir.value),
        maybe_network: selected_network.map(|selected| selected.value),
        maybe_log_dir: Some(log_dir),
        maybe_metrics_store_path: Some(metrics_store_path),
        credential_source,
        maybe_open_bitcoin_config,
    })
}

fn select_path(
    maybe_cli: Option<PathBuf>,
    maybe_env: Option<&String>,
    default_path: PathBuf,
) -> Selected<PathBuf> {
    if let Some(value) = maybe_cli.filter(|path| !path.as_os_str().is_empty()) {
        return Selected {
            value,
            source: OperatorConfigSource::CliFlags,
        };
    }
    if let Some(value) = maybe_env.filter(|value| !value.is_empty()) {
        return Selected {
            value: PathBuf::from(value),
            source: OperatorConfigSource::Environment,
        };
    }
    Selected {
        value: default_path,
        source: OperatorConfigSource::Defaults,
    }
}

fn select_path_optional(
    maybe_cli: Option<PathBuf>,
    maybe_env: Option<&String>,
    maybe_jsonc: Option<PathBuf>,
    default_path: PathBuf,
) -> Selected<PathBuf> {
    if let Some(value) = maybe_cli.filter(|path| !path.as_os_str().is_empty()) {
        return Selected {
            value,
            source: OperatorConfigSource::CliFlags,
        };
    }
    if let Some(value) = maybe_env.filter(|value| !value.is_empty()) {
        return Selected {
            value: PathBuf::from(value),
            source: OperatorConfigSource::Environment,
        };
    }
    if let Some(value) = maybe_jsonc.filter(|path| !path.as_os_str().is_empty()) {
        return Selected {
            value,
            source: OperatorConfigSource::OpenBitcoinJsonc,
        };
    }
    Selected {
        value: default_path,
        source: OperatorConfigSource::Defaults,
    }
}

fn select_network(
    maybe_cli: Option<NetworkSelection>,
    maybe_env: Option<&String>,
    maybe_jsonc: Option<NetworkSelection>,
) -> Result<Option<Selected<NetworkSelection>>, OperatorConfigError> {
    if let Some(value) = maybe_cli {
        return Ok(Some(Selected {
            value,
            source: OperatorConfigSource::CliFlags,
        }));
    }
    if let Some(value) = maybe_env.filter(|value| !value.is_empty()) {
        return Ok(Some(Selected {
            value: parse_network(value)?,
            source: OperatorConfigSource::Environment,
        }));
    }
    if let Some(value) = maybe_jsonc {
        return Ok(Some(Selected {
            value,
            source: OperatorConfigSource::OpenBitcoinJsonc,
        }));
    }
    Ok(Some(Selected {
        value: NetworkSelection::Mainnet,
        source: OperatorConfigSource::Defaults,
    }))
}

fn load_open_bitcoin_jsonc(
    config_path: &Path,
) -> Result<Option<OpenBitcoinConfig>, OperatorConfigError> {
    if !config_path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(config_path).map_err(|error| {
        OperatorConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: {error}"
        ))
    })?;
    parse_open_bitcoin_jsonc_config(&text)
        .map(Some)
        .map_err(|error| {
            OperatorConfigError::new(format!(
                "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: {error}"
            ))
        })
}

fn load_runtime_from_evidence(
    data_dir: &Path,
    bitcoin_conf_path: &Path,
    maybe_network: Option<NetworkSelection>,
    roots: &OperatorConfigRoots,
) -> Result<open_bitcoin_rpc::config::RuntimeConfig, OperatorConfigError> {
    let mut args = Vec::new();
    if data_dir.exists() {
        args.push(OsString::from(format!("-datadir={}", data_dir.display())));
    }
    if bitcoin_conf_path.exists() {
        args.push(OsString::from(format!(
            "-conf={}",
            bitcoin_conf_path.display()
        )));
    }
    if let Some(network) = maybe_network {
        args.push(OsString::from(format!(
            "-chain={}",
            network_chain_name(network)
        )));
    }
    load_runtime_config_for_args(&args, &roots.default_data_dir)
        .map_err(|error| OperatorConfigError::new(error.to_string()))
}

fn credential_source_from_auth(
    auth: &RpcAuthConfig,
    default_cookie_path: PathBuf,
) -> OperatorCredentialSource {
    match auth {
        RpcAuthConfig::Cookie { maybe_cookie_file } => {
            let path = maybe_cookie_file.clone().unwrap_or(default_cookie_path);
            OperatorCredentialSource::CookieFile {
                present: path.exists(),
                path,
            }
        }
        RpcAuthConfig::UserPassword { .. } => OperatorCredentialSource::UserPasswordConfigured,
    }
}

fn path_report(
    source: OperatorConfigSource,
    kind: OperatorConfigPathKind,
    path: &Path,
) -> OperatorConfigPathReport {
    OperatorConfigPathReport {
        source,
        kind,
        path: path.to_path_buf(),
        present: path.exists(),
    }
}

fn parse_network(value: &str) -> Result<NetworkSelection, OperatorConfigError> {
    match value {
        "main" | "mainnet" => Ok(NetworkSelection::Mainnet),
        "test" | "testnet" | "testnet3" => Ok(NetworkSelection::Testnet),
        "signet" => Ok(NetworkSelection::Signet),
        "regtest" => Ok(NetworkSelection::Regtest),
        _ => Err(OperatorConfigError::new(format!(
            "invalid OPEN_BITCOIN_NETWORK value: {value}"
        ))),
    }
}

fn network_chain_name(network: NetworkSelection) -> &'static str {
    match network {
        NetworkSelection::Mainnet => "main",
        NetworkSelection::Testnet => "testnet",
        NetworkSelection::Signet => "signet",
        NetworkSelection::Regtest => "regtest",
    }
}

#[cfg(test)]
mod tests;
