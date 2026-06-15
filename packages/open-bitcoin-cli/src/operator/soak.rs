// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Multi-day soak run contracts.

use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use super::{
    OperatorCli, SoakArgs, SoakCommand,
    config::OperatorConfigResolution,
    detect::DetectionScan,
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
};
use ledger::SoakLedgerLayout;

pub(crate) mod ledger;
pub(crate) mod outcome;
pub(crate) mod report;
pub(crate) mod runtime;

#[cfg(test)]
pub(crate) use runtime::{
    SoakLoopMode, SoakStatusCollector, SoakTestClock, run_bounded_soak_loop, validate_resume_plan,
    write_operator_stop, write_report_projection,
};

#[cfg(test)]
mod tests;

pub(crate) fn execute_soak_command(
    args: &SoakArgs,
    cli: &OperatorCli,
    config_resolution: OperatorConfigResolution,
    detections: DetectionScan,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let datadir = runtime::require_soak_datadir(&config_resolution)?;
    let layout = SoakLedgerLayout::for_datadir(&datadir);
    let maybe_network = config_resolution.maybe_network;
    match &args.command {
        SoakCommand::Start(start) => {
            let mut status_parts =
                super::runtime::status_runtime_parts(cli, config_resolution, detections);
            let mut collector = runtime::RuntimeSoakStatusCollector::new(&mut status_parts);
            let mut clock = runtime::SystemSoakClock;
            runtime::execute_soak_start(
                start,
                cli.format,
                &layout,
                maybe_network,
                &mut collector,
                &mut clock,
            )
        }
        SoakCommand::Resume(resume) => {
            let mut status_parts =
                super::runtime::status_runtime_parts(cli, config_resolution, detections);
            let mut collector = runtime::RuntimeSoakStatusCollector::new(&mut status_parts);
            let mut clock = runtime::SystemSoakClock;
            runtime::execute_soak_resume(resume, cli.format, &layout, &mut collector, &mut clock)
        }
        SoakCommand::Stop(stop) => runtime::execute_soak_stop(stop, cli.format, &layout),
        SoakCommand::Report(report) => runtime::execute_soak_report(report, cli.format, &layout),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct SoakRunId(String);

impl SoakRunId {
    pub(crate) fn try_new(value: impl Into<String>) -> Result<Self, SoakContractError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SoakContractError::EmptyRunId);
        }
        if !trimmed
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
        {
            return Err(SoakContractError::InvalidRunId {
                value,
                reason: "run id may only contain ASCII letters, digits, '-' and '_'".to_string(),
            });
        }

        Ok(Self(trimmed.to_string()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SoakRunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SoakPeerPolicy {
    #[serde(rename = "daemon_configured")]
    DaemonConfigured,
    #[serde(rename = "manual_peers_only")]
    ManualPeersOnly,
    #[serde(rename = "no_dns_seeds")]
    NoDnsSeeds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SoakStopCondition {
    #[serde(rename = "elapsed_time")]
    ElapsedTime,
    #[serde(rename = "target_height")]
    TargetHeight,
    #[serde(rename = "status_verdict")]
    StatusVerdict,
    #[serde(rename = "operator_stop")]
    OperatorStop,
    #[serde(rename = "resource_stop")]
    ResourceStop,
    #[serde(rename = "recovery_stop")]
    RecoveryStop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakBounds {
    pub(crate) elapsed_time_seconds: u64,
    pub(crate) checkpoint_interval_seconds: u64,
    pub(crate) maybe_target_height: Option<u64>,
    pub(crate) datadir: PathBuf,
    pub(crate) network: String,
    pub(crate) peer_policy: SoakPeerPolicy,
    pub(crate) disk_budget_bytes: u64,
    pub(crate) stop_conditions: Vec<SoakStopCondition>,
}

impl SoakBounds {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn try_new(
        elapsed_time_seconds: u64,
        checkpoint_interval_seconds: u64,
        maybe_target_height: Option<u64>,
        datadir: PathBuf,
        network: impl Into<String>,
        peer_policy: SoakPeerPolicy,
        disk_budget_bytes: u64,
        stop_conditions: Vec<SoakStopCondition>,
    ) -> Result<Self, SoakContractError> {
        if elapsed_time_seconds == 0 {
            return Err(SoakContractError::ElapsedTimeRequired);
        }
        if checkpoint_interval_seconds == 0 {
            return Err(SoakContractError::CheckpointIntervalRequired);
        }
        if path_is_empty(&datadir) {
            return Err(SoakContractError::DatadirRequired);
        }

        let network = network.into();
        if network.trim().is_empty() {
            return Err(SoakContractError::NetworkRequired);
        }
        if disk_budget_bytes == 0 {
            return Err(SoakContractError::DiskBudgetRequired);
        }
        if stop_conditions.is_empty() {
            return Err(SoakContractError::StopConditionsRequired);
        }

        Ok(Self {
            elapsed_time_seconds,
            checkpoint_interval_seconds,
            maybe_target_height,
            datadir,
            network: network.trim().to_string(),
            peer_policy,
            disk_budget_bytes,
            stop_conditions,
        })
    }
}

fn path_is_empty(path: &Path) -> bool {
    path.as_os_str().is_empty()
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum SoakContractError {
    #[error("soak run id must not be empty")]
    EmptyRunId,

    #[error("invalid soak run id {value}: {reason}")]
    InvalidRunId { value: String, reason: String },

    #[error("elapsed time seconds must be greater than zero")]
    ElapsedTimeRequired,

    #[error("checkpoint interval seconds must be greater than zero")]
    CheckpointIntervalRequired,

    #[error("soak datadir must not be empty")]
    DatadirRequired,

    #[error("soak network must not be empty")]
    NetworkRequired,

    #[error("disk budget bytes must be greater than zero")]
    DiskBudgetRequired,

    #[error("at least one soak stop condition is required")]
    StopConditionsRequired,
}

#[cfg(test)]
#[path = "soak/tests/runtime.rs"]
mod runtime_tests;
