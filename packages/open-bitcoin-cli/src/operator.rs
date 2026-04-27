// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

//! Clap contracts for the Open Bitcoin operator command path.

use std::{ffi::OsString, path::PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::CliError;

pub mod config;
pub mod detect;
pub mod onboarding;
pub mod runtime;
pub mod service;
pub mod status;

/// First-party Open Bitcoin operator CLI contract.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(name = "open-bitcoin")]
#[command(about = "Manage an Open Bitcoin node")]
pub struct OperatorCli {
    #[arg(long = "config", global = true)]
    pub maybe_config_path: Option<PathBuf>,
    #[arg(long = "datadir", global = true)]
    pub maybe_data_dir: Option<PathBuf>,
    #[arg(long = "network", global = true, value_enum)]
    pub maybe_network: Option<NetworkSelection>,
    #[arg(long = "format", global = true, value_enum, default_value = "human")]
    pub format: OperatorOutputFormat,
    #[arg(long = "no-color", global = true)]
    pub no_color: bool,
    #[command(subcommand)]
    pub command: OperatorCommand,
}

/// Operator-owned subcommands. Phase 13 defines shape only.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum OperatorCommand {
    Status(StatusArgs),
    Config(ConfigArgs),
    Service(ServiceArgs),
    Dashboard(DashboardArgs),
    Onboard(OnboardArgs),
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct StatusArgs {
    #[arg(long = "watch")]
    pub watch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ConfigCommand {
    Paths,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct ServiceArgs {
    #[command(subcommand)]
    pub command: ServiceCommand,
    /// Apply changes (default: dry-run only).
    #[arg(long = "apply", global = true)]
    pub apply: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ServiceCommand {
    Status,
    Install,
    Uninstall,
    Enable,
    Disable,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct DashboardArgs {
    #[arg(long = "tick-ms", default_value_t = 1_000)]
    pub tick_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct OnboardArgs {
    #[arg(long = "non-interactive")]
    pub non_interactive: bool,
    #[arg(long = "approve-write")]
    pub approve_write: bool,
    #[arg(long = "force-overwrite")]
    pub force_overwrite: bool,
    #[arg(long = "disable-metrics")]
    pub disable_metrics: bool,
    #[arg(long = "disable-logs")]
    pub disable_logs: bool,
    #[arg(long = "detect-existing")]
    pub detect_existing: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum OperatorOutputFormat {
    #[default]
    Human,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum NetworkSelection {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

/// Top-level route selected before command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliRoute {
    Operator(OperatorCli),
    BitcoinCliCompat(Vec<OsString>),
}

/// Route a shell invocation to the operator parser or compatibility parser.
pub fn route_cli_invocation(binary_name: &str, args: &[OsString]) -> Result<CliRoute, CliError> {
    if binary_name.ends_with("open-bitcoin-cli") {
        return Ok(CliRoute::BitcoinCliCompat(args.to_vec()));
    }

    let mut argv = Vec::with_capacity(args.len() + 1);
    argv.push(OsString::from(binary_name));
    argv.extend(args.iter().cloned());
    let parsed =
        OperatorCli::try_parse_from(argv).map_err(|error| CliError::new(error.to_string()))?;
    Ok(CliRoute::Operator(parsed))
}

#[cfg(test)]
mod tests;
