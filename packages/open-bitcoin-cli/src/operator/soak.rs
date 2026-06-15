// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Multi-day soak run contracts.

use std::{
    fmt, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{OpenBitcoinStatusSnapshot, status::FieldAvailability};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    NetworkSelection, OperatorCli, OperatorOutputFormat, SoakArgs, SoakCommand, SoakPeerPolicyArg,
    SoakResumeArgs, SoakStartArgs, SoakStopArgs, SoakStopConditionArg, SoakStopReasonArg,
    config::OperatorConfigResolution,
    detect::DetectionScan,
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
    status::collect_status_snapshot,
    support::{EvidenceState, LiveSmokeEvidence, derive_full_sync_evidence},
};
use ledger::{
    SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerEventEnvelope, SoakLedgerLayout,
    SoakRunIndex, SoakRunIndexEntry,
};
use outcome::{SoakOutcomeEvidence, SoakOutcomeLabel, classify_soak_outcome};
use report::{SoakReportPaths, write_soak_reports};

pub(crate) mod ledger;
pub(crate) mod outcome;
pub(crate) mod report;

#[cfg(test)]
mod tests;

pub(crate) fn execute_soak_command(
    args: &SoakArgs,
    cli: &OperatorCli,
    config_resolution: OperatorConfigResolution,
    detections: DetectionScan,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let datadir = require_soak_datadir(&config_resolution)?;
    let layout = SoakLedgerLayout::for_datadir(&datadir);
    let maybe_network = config_resolution.maybe_network;
    match &args.command {
        SoakCommand::Start(start) => {
            let mut status_parts =
                super::runtime::status_runtime_parts(cli, config_resolution, detections);
            let mut collector = RuntimeSoakStatusCollector {
                parts: &mut status_parts,
            };
            let mut clock = SystemSoakClock;
            execute_soak_start(
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
            let mut collector = RuntimeSoakStatusCollector {
                parts: &mut status_parts,
            };
            let mut clock = SystemSoakClock;
            execute_soak_resume(resume, cli.format, &layout, &mut collector, &mut clock)
        }
        SoakCommand::Stop(stop) => execute_soak_stop(stop, cli.format, &layout),
        SoakCommand::Report(report) => {
            let run_id = SoakRunId::try_new(report.run_id.clone()).map_err(runtime_error)?;
            validate_index_entry(&layout, &run_id)?;
            let result = write_report_projection(&layout, &run_id)?;
            Ok(render_soak_command_output(cli.format, &run_id, &result)?)
        }
    }
}

fn execute_soak_start(
    args: &SoakStartArgs,
    format: OperatorOutputFormat,
    layout: &SoakLedgerLayout,
    maybe_network: Option<NetworkSelection>,
    collector: &mut dyn SoakStatusCollector,
    clock: &mut dyn SoakClock,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let run_id = match args.maybe_run_id.as_ref() {
        Some(value) => SoakRunId::try_new(value.clone()).map_err(runtime_error)?,
        None => generate_soak_run_id(layout, current_unix_seconds())?,
    };
    reject_run_collision(layout, &run_id)?;
    let bounds = bounds_from_start_args(args, layout, maybe_network)?;
    let started_hint = current_unix_seconds();
    let paths = layout.paths_for_run(&run_id);
    record_run_index(
        layout,
        &run_id,
        &paths.events_path,
        started_hint,
        started_hint,
        None,
    )?;
    let mut ledger = SoakLedger::create(layout, run_id.clone());
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        layout,
        &mut ledger,
        collector,
        clock,
        SoakLoopMode::Start,
    )?;
    record_run_index(
        layout,
        &run_id,
        &result.report_paths.source_ledger_path,
        result.started_at_unix_seconds,
        result.updated_at_unix_seconds,
        Some(result.final_outcome),
    )?;
    render_soak_command_output(format, &run_id, &result)
}

fn execute_soak_resume(
    args: &SoakResumeArgs,
    format: OperatorOutputFormat,
    layout: &SoakLedgerLayout,
    collector: &mut dyn SoakStatusCollector,
    clock: &mut dyn SoakClock,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let run_id = SoakRunId::try_new(args.run_id.clone()).map_err(runtime_error)?;
    let resume = validate_resume_plan(layout, &run_id, args.checkpoint_interval_seconds)?;
    let mut ledger = SoakLedger::resume(layout, run_id.clone(), resume.next_sequence);
    let result = run_bounded_soak_loop(
        &run_id,
        &resume.bounds,
        layout,
        &mut ledger,
        collector,
        clock,
        SoakLoopMode::Resume {
            interrupted_prior_run: resume.interrupted_prior_run,
        },
    )?;
    record_run_index(
        layout,
        &run_id,
        &result.report_paths.source_ledger_path,
        result.started_at_unix_seconds,
        result.updated_at_unix_seconds,
        Some(result.final_outcome),
    )?;
    render_soak_command_output(format, &run_id, &result)
}

fn execute_soak_stop(
    args: &SoakStopArgs,
    format: OperatorOutputFormat,
    layout: &SoakLedgerLayout,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    match args.reason {
        SoakStopReasonArg::OperatorStop => {}
    }
    let run_id = SoakRunId::try_new(args.run_id.clone()).map_err(runtime_error)?;
    validate_index_entry(layout, &run_id)?;
    let result = write_operator_stop(layout, &run_id, current_unix_seconds())?;
    record_run_index(
        layout,
        &run_id,
        &result.report_paths.source_ledger_path,
        result.started_at_unix_seconds,
        result.updated_at_unix_seconds,
        Some(result.final_outcome),
    )?;
    render_soak_command_output(format, &run_id, &result)
}

pub(crate) trait SoakStatusCollector {
    fn collect(&mut self) -> OpenBitcoinStatusSnapshot;
}

struct RuntimeSoakStatusCollector<'a> {
    parts: &'a mut super::runtime::StatusRuntimeParts,
}

impl SoakStatusCollector for RuntimeSoakStatusCollector<'_> {
    fn collect(&mut self) -> OpenBitcoinStatusSnapshot {
        collect_status_snapshot(&self.parts.input, self.parts.maybe_rpc_client.as_deref())
    }
}

pub(crate) trait SoakClock {
    fn now_unix_seconds(&mut self) -> u64;
    fn sleep_until(&mut self, scheduled_unix_seconds: u64);
}

struct SystemSoakClock;

impl SoakClock for SystemSoakClock {
    fn now_unix_seconds(&mut self) -> u64 {
        current_unix_seconds()
    }

    fn sleep_until(&mut self, _scheduled_unix_seconds: u64) {}
}

#[cfg(test)]
pub(crate) struct SoakTestClock {
    now_unix_seconds: u64,
}

#[cfg(test)]
impl SoakTestClock {
    pub(crate) const fn new(now_unix_seconds: u64) -> Self {
        Self { now_unix_seconds }
    }
}

#[cfg(test)]
impl SoakClock for SoakTestClock {
    fn now_unix_seconds(&mut self) -> u64 {
        self.now_unix_seconds
    }

    fn sleep_until(&mut self, scheduled_unix_seconds: u64) {
        self.now_unix_seconds = scheduled_unix_seconds;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SoakLoopMode {
    Start,
    Resume { interrupted_prior_run: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakLoopResult {
    pub(crate) report_paths: SoakReportPaths,
    pub(crate) final_outcome: SoakOutcomeLabel,
    pub(crate) latest_sequence: u64,
    pub(crate) started_at_unix_seconds: u64,
    pub(crate) updated_at_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakResumePlan {
    pub(crate) bounds: SoakBounds,
    pub(crate) interrupted_prior_run: bool,
    pub(crate) next_sequence: u64,
}

pub(crate) fn run_bounded_soak_loop(
    run_id: &SoakRunId,
    bounds: &SoakBounds,
    layout: &SoakLedgerLayout,
    ledger: &mut SoakLedger,
    collector: &mut dyn SoakStatusCollector,
    clock: &mut dyn SoakClock,
    mode: SoakLoopMode,
) -> Result<SoakLoopResult, OperatorRuntimeError> {
    let started_at = clock.now_unix_seconds();
    match mode {
        SoakLoopMode::Start => {
            ledger
                .append_event(
                    started_at,
                    SoakLedgerEvent::Started {
                        bounds: bounds.clone(),
                    },
                )
                .map_err(runtime_error)?;
        }
        SoakLoopMode::Resume {
            interrupted_prior_run,
        } => {
            ledger
                .append_event(
                    started_at,
                    SoakLedgerEvent::Resume {
                        interrupted_prior_run,
                    },
                )
                .map_err(runtime_error)?;
        }
    }

    let deadline = started_at.saturating_add(bounds.elapsed_time_seconds);
    let mut checkpoint_at = started_at;
    let mut final_outcome = None;
    while final_outcome.is_none() {
        clock.sleep_until(checkpoint_at);
        let snapshot = collector.collect();
        let status = checkpoint_status_from_snapshot(&snapshot);
        ledger
            .append_event(checkpoint_at, SoakLedgerEvent::Checkpoint { status })
            .map_err(runtime_error)?;
        final_outcome = evaluate_stop_outcome(bounds, &snapshot, checkpoint_at, deadline);
        if final_outcome.is_some() {
            break;
        }
        let next_checkpoint = checkpoint_at.saturating_add(bounds.checkpoint_interval_seconds);
        checkpoint_at = next_checkpoint.min(deadline);
    }

    let final_outcome = final_outcome.unwrap_or(SoakOutcomeLabel::UnexpectedTermination);
    ledger
        .append_event(
            checkpoint_at,
            SoakLedgerEvent::Stop {
                outcome: final_outcome,
            },
        )
        .map_err(runtime_error)?;
    ledger
        .append_event(
            checkpoint_at,
            SoakLedgerEvent::Verdict {
                outcome: final_outcome,
            },
        )
        .map_err(runtime_error)?;
    let result = write_report_projection(layout, run_id)?;
    Ok(SoakLoopResult {
        final_outcome,
        started_at_unix_seconds: first_started_at(layout, run_id)?.unwrap_or(started_at),
        updated_at_unix_seconds: checkpoint_at,
        ..result
    })
}

pub(crate) fn validate_resume_plan(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
    checkpoint_interval_seconds: u64,
) -> Result<SoakResumePlan, OperatorRuntimeError> {
    if checkpoint_interval_seconds == 0 {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "checkpoint interval seconds must be greater than zero".to_string(),
        });
    }
    validate_index_entry(layout, run_id)?;
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    let mut bounds = started_bounds(&read.events)?;
    bounds.checkpoint_interval_seconds = checkpoint_interval_seconds;
    // D-11 same-run resume matrix: clean_completion refuses; operator_stop,
    // resource_stop, and recovery_stop resume as same-run; unexpected_termination
    // resumes as interrupted recovery evidence.
    let interrupted_prior_run = match latest_verdict(&read.events) {
        Some(SoakOutcomeLabel::CleanCompletion) => {
            return Err(OperatorRuntimeError::InvalidRequest {
                message: format!(
                    "soak run {run_id} latest verdict clean_completion cannot be resumed"
                ),
            });
        }
        Some(
            SoakOutcomeLabel::OperatorStop
            | SoakOutcomeLabel::ResourceStop
            | SoakOutcomeLabel::RecoveryStop,
        ) => false,
        Some(SoakOutcomeLabel::UnexpectedTermination) | None => true,
        Some(SoakOutcomeLabel::DiagnosedBlocker) => {
            return Err(OperatorRuntimeError::InvalidRequest {
                message: format!(
                    "soak run {run_id} ended with diagnosed_blocker and cannot be resumed as the same run"
                ),
            });
        }
    };
    Ok(SoakResumePlan {
        bounds,
        interrupted_prior_run: interrupted_prior_run
            || !has_terminal_stop_and_verdict(&read.events),
        next_sequence: next_sequence(&read.events),
    })
}

pub(crate) fn write_operator_stop(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
    recorded_at_unix_seconds: u64,
) -> Result<SoakLoopResult, OperatorRuntimeError> {
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    let mut ledger = SoakLedger::resume(layout, run_id.clone(), next_sequence(&read.events));
    ledger
        .append_event(
            recorded_at_unix_seconds,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .map_err(runtime_error)?;
    ledger
        .append_event(
            recorded_at_unix_seconds,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .map_err(runtime_error)?;
    let result = write_report_projection(layout, run_id)?;
    Ok(SoakLoopResult {
        final_outcome: SoakOutcomeLabel::OperatorStop,
        started_at_unix_seconds: first_started_at(layout, run_id)?
            .unwrap_or(recorded_at_unix_seconds),
        updated_at_unix_seconds: recorded_at_unix_seconds,
        ..result
    })
}

pub(crate) fn write_report_projection(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
) -> Result<SoakLoopResult, OperatorRuntimeError> {
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    let latest_sequence = read
        .events
        .iter()
        .map(|event| event.sequence)
        .max()
        .unwrap_or(0);
    let final_outcome =
        latest_outcome(&read.events).unwrap_or(SoakOutcomeLabel::UnexpectedTermination);
    let started_at_unix_seconds = first_started_at_from_events(&read.events).unwrap_or(0);
    let updated_at_unix_seconds = read
        .events
        .iter()
        .map(|event| event.recorded_at_unix_seconds)
        .max()
        .unwrap_or(started_at_unix_seconds);
    let report_paths =
        write_soak_reports(&read, &paths.events_path, layout).map_err(runtime_error)?;
    Ok(SoakLoopResult {
        report_paths,
        final_outcome,
        latest_sequence,
        started_at_unix_seconds,
        updated_at_unix_seconds,
    })
}

fn require_soak_datadir(
    config_resolution: &OperatorConfigResolution,
) -> Result<PathBuf, OperatorRuntimeError> {
    config_resolution
        .maybe_data_dir
        .clone()
        .ok_or_else(|| OperatorRuntimeError::InvalidRequest {
            message: "soak commands require a datadir".to_string(),
        })
}

fn bounds_from_start_args(
    args: &SoakStartArgs,
    layout: &SoakLedgerLayout,
    maybe_network: Option<NetworkSelection>,
) -> Result<SoakBounds, OperatorRuntimeError> {
    let datadir = layout.datadir();
    SoakBounds::try_new(
        args.elapsed_time_seconds,
        args.checkpoint_interval_seconds,
        args.maybe_target_height,
        datadir,
        network_name(maybe_network)?,
        peer_policy_from_arg(args.peer_policy),
        args.disk_budget_bytes,
        vec![stop_condition_from_arg(args.stop_condition)],
    )
    .map_err(runtime_error)
}

fn network_name(
    maybe_network: Option<NetworkSelection>,
) -> Result<&'static str, OperatorRuntimeError> {
    match maybe_network {
        Some(NetworkSelection::Mainnet) => Ok("mainnet"),
        Some(NetworkSelection::Testnet) => Ok("testnet"),
        Some(NetworkSelection::Signet) => Ok("signet"),
        Some(NetworkSelection::Regtest) => Ok("regtest"),
        None => Err(OperatorRuntimeError::InvalidRequest {
            message: "soak commands require a network".to_string(),
        }),
    }
}

fn peer_policy_from_arg(value: SoakPeerPolicyArg) -> SoakPeerPolicy {
    match value {
        SoakPeerPolicyArg::DaemonConfigured => SoakPeerPolicy::DaemonConfigured,
        SoakPeerPolicyArg::ManualPeersOnly => SoakPeerPolicy::ManualPeersOnly,
        SoakPeerPolicyArg::NoDnsSeeds => SoakPeerPolicy::NoDnsSeeds,
    }
}

fn stop_condition_from_arg(value: SoakStopConditionArg) -> SoakStopCondition {
    match value {
        SoakStopConditionArg::ElapsedTime => SoakStopCondition::ElapsedTime,
        SoakStopConditionArg::TargetHeight => SoakStopCondition::TargetHeight,
        SoakStopConditionArg::StatusVerdict => SoakStopCondition::StatusVerdict,
        SoakStopConditionArg::OperatorStop => SoakStopCondition::OperatorStop,
        SoakStopConditionArg::ResourceStop => SoakStopCondition::ResourceStop,
        SoakStopConditionArg::RecoveryStop => SoakStopCondition::RecoveryStop,
    }
}

fn render_soak_command_output(
    format: OperatorOutputFormat,
    run_id: &SoakRunId,
    result: &SoakLoopResult,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let final_outcome = outcome_label(result.final_outcome);
    match format {
        OperatorOutputFormat::Human => Ok(OperatorCommandOutcome::success(format!(
            "Soak ledger: {}\nJSON report: {}\nMarkdown report: {}\nRun id: {}\nLatest sequence: {}\nFinal outcome: {}\n",
            result.report_paths.source_ledger_path.display(),
            result.report_paths.json_path.display(),
            result.report_paths.markdown_path.display(),
            run_id,
            result.latest_sequence,
            final_outcome
        ))),
        OperatorOutputFormat::Json => serde_json::to_string_pretty(&json!({
            "ledger_path": result.report_paths.source_ledger_path.display().to_string(),
            "json_report_path": result.report_paths.json_path.display().to_string(),
            "markdown_report_path": result.report_paths.markdown_path.display().to_string(),
            "run_id": run_id.as_str(),
            "latest_sequence": result.latest_sequence,
            "final_outcome": final_outcome,
        }))
        .map(|value| OperatorCommandOutcome::success(format!("{value}\n")))
        .map_err(runtime_error),
    }
}

fn evaluate_stop_outcome(
    bounds: &SoakBounds,
    snapshot: &OpenBitcoinStatusSnapshot,
    checkpoint_at: u64,
    deadline: u64,
) -> Option<SoakOutcomeLabel> {
    for condition in &bounds.stop_conditions {
        match condition {
            SoakStopCondition::ElapsedTime if checkpoint_at >= deadline => {
                return Some(outcome_for_snapshot(snapshot));
            }
            SoakStopCondition::TargetHeight if target_height_reached(bounds, snapshot) => {
                return Some(SoakOutcomeLabel::CleanCompletion);
            }
            SoakStopCondition::StatusVerdict => {
                let outcome = outcome_for_snapshot(snapshot);
                if !matches!(outcome, SoakOutcomeLabel::UnexpectedTermination) {
                    return Some(outcome);
                }
            }
            SoakStopCondition::ResourceStop => {
                let outcome = outcome_for_snapshot(snapshot);
                if matches!(outcome, SoakOutcomeLabel::ResourceStop) {
                    return Some(outcome);
                }
            }
            SoakStopCondition::RecoveryStop => {
                let outcome = outcome_for_snapshot(snapshot);
                if matches!(outcome, SoakOutcomeLabel::RecoveryStop) {
                    return Some(outcome);
                }
            }
            SoakStopCondition::OperatorStop
            | SoakStopCondition::ElapsedTime
            | SoakStopCondition::TargetHeight => {}
        }
    }

    (checkpoint_at >= deadline).then(|| outcome_for_snapshot(snapshot))
}

fn target_height_reached(bounds: &SoakBounds, snapshot: &OpenBitcoinStatusSnapshot) -> bool {
    let Some(target_height) = bounds.maybe_target_height else {
        return false;
    };
    let FieldAvailability::Available(progress) = &snapshot.sync.sync_progress else {
        return false;
    };
    progress.validated_active_chain_height >= target_height
}

fn outcome_for_snapshot(snapshot: &OpenBitcoinStatusSnapshot) -> SoakOutcomeLabel {
    classify_soak_outcome(&SoakOutcomeEvidence {
        maybe_sync_stop_reason: maybe_available(&snapshot.sync.latest_stop_reason),
        maybe_recovery_category: maybe_available(&snapshot.sync.recovery_category),
        maybe_no_progress_diagnosis: maybe_available(&snapshot.sync.no_progress_diagnosis),
        maybe_full_sync_evidence: Some(derive_full_sync_evidence(
            snapshot,
            &missing_live_smoke_evidence(),
        )),
        maybe_process_exit: None,
    })
}

fn checkpoint_status_from_snapshot(snapshot: &OpenBitcoinStatusSnapshot) -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: maybe_available(&snapshot.sync.network),
        maybe_lifecycle: maybe_available(&snapshot.sync.lifecycle).map(|value| serde_label(&value)),
        maybe_latest_stop_reason_label: maybe_available(&snapshot.sync.latest_stop_reason)
            .map(|value| value.label),
        maybe_recovery_category_label: maybe_available(&snapshot.sync.recovery_category)
            .map(|value| value.as_str().to_string()),
        maybe_no_progress_diagnosis_label: maybe_available(&snapshot.sync.no_progress_diagnosis)
            .map(|value| serde_label(&value)),
        maybe_validated_active_chain_height: maybe_available(&snapshot.sync.sync_progress)
            .map(|value| value.validated_active_chain_height),
        maybe_best_known_tip_height: maybe_available(&snapshot.sync.best_known_tip)
            .map(|value| value.height),
        maybe_source_status_path: snapshot
            .config
            .datadir
            .clone()
            .available()
            .map(|path| PathBuf::from(path).join("status-snapshot.json")),
    }
}

fn maybe_available<T: Clone>(value: &FieldAvailability<T>) -> Option<T> {
    match value {
        FieldAvailability::Available(value) => Some(value.clone()),
        FieldAvailability::Unavailable { .. } => None,
    }
}

trait FieldAvailabilityExt<T> {
    fn available(self) -> Option<T>;
}

impl<T> FieldAvailabilityExt<T> for FieldAvailability<T> {
    fn available(self) -> Option<T> {
        match self {
            FieldAvailability::Available(value) => Some(value),
            FieldAvailability::Unavailable { .. } => None,
        }
    }
}

fn missing_live_smoke_evidence() -> LiveSmokeEvidence {
    LiveSmokeEvidence {
        state: EvidenceState::Unavailable,
        report_path: None,
        summary: None,
        reason: Some("live smoke evidence unavailable for soak checkpoint".to_string()),
    }
}

fn validate_index_entry(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
) -> Result<SoakRunIndexEntry, OperatorRuntimeError> {
    let index = load_run_index(layout)?;
    let paths = layout.paths_for_run(run_id);
    let Some(entry) = index.runs.into_iter().find(|entry| &entry.run_id == run_id) else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!("soak run {run_id} was not found in the selected datadir index"),
        });
    };
    if entry.ledger_path != paths.events_path {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!("soak run {run_id} belongs to a different datadir-owned ledger"),
        });
    }
    Ok(entry)
}

fn load_run_index(layout: &SoakLedgerLayout) -> Result<SoakRunIndex, OperatorRuntimeError> {
    let path = layout.run_index_path();
    let text = fs::read_to_string(&path).map_err(|error| OperatorRuntimeError::InvalidRequest {
        message: format!("could not read soak run index {}: {error}", path.display()),
    })?;
    serde_json::from_str(&text).map_err(runtime_error)
}

fn load_run_index_or_empty(
    layout: &SoakLedgerLayout,
) -> Result<SoakRunIndex, OperatorRuntimeError> {
    let path = layout.run_index_path();
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).map_err(runtime_error),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(SoakRunIndex::empty()),
        Err(error) => Err(OperatorRuntimeError::InvalidRequest {
            message: format!("could not read soak run index {}: {error}", path.display()),
        }),
    }
}

fn reject_run_collision(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
) -> Result<(), OperatorRuntimeError> {
    let index = load_run_index_or_empty(layout)?;
    if index.runs.iter().any(|entry| &entry.run_id == run_id)
        || layout.paths_for_run(run_id).events_path.exists()
    {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!("soak run id {run_id} already exists"),
        });
    }
    Ok(())
}

fn generate_soak_run_id(
    layout: &SoakLedgerLayout,
    unix_seconds: u64,
) -> Result<SoakRunId, OperatorRuntimeError> {
    let index = load_run_index_or_empty(layout)?;
    for sequence in 1..=9_999 {
        let candidate = SoakRunId::try_new(format!("soak-{unix_seconds}-{sequence:04}"))
            .map_err(runtime_error)?;
        if !index.runs.iter().any(|entry| entry.run_id == candidate)
            && !layout.paths_for_run(&candidate).events_path.exists()
        {
            return Ok(candidate);
        }
    }
    Err(OperatorRuntimeError::InvalidRequest {
        message: format!("could not allocate soak run id for unix second {unix_seconds}"),
    })
}

fn record_run_index(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
    ledger_path: &Path,
    started_at_unix_seconds: u64,
    updated_at_unix_seconds: u64,
    maybe_outcome: Option<SoakOutcomeLabel>,
) -> Result<(), OperatorRuntimeError> {
    let mut index = load_run_index_or_empty(layout)?;
    index.record_run(SoakRunIndexEntry {
        run_id: run_id.clone(),
        ledger_path: ledger_path.to_path_buf(),
        started_at_unix_seconds,
        updated_at_unix_seconds,
        maybe_outcome,
    });
    index.write_atomic(layout).map_err(runtime_error)
}

fn started_bounds(events: &[SoakLedgerEventEnvelope]) -> Result<SoakBounds, OperatorRuntimeError> {
    events
        .iter()
        .find_map(|envelope| match &envelope.event {
            SoakLedgerEvent::Started { bounds } => Some(bounds.clone()),
            SoakLedgerEvent::Checkpoint { .. }
            | SoakLedgerEvent::Resume { .. }
            | SoakLedgerEvent::Stop { .. }
            | SoakLedgerEvent::Verdict { .. } => None,
        })
        .ok_or_else(|| OperatorRuntimeError::InvalidRequest {
            message: "soak ledger is missing a started event".to_string(),
        })
}

fn first_started_at(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
) -> Result<Option<u64>, OperatorRuntimeError> {
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    Ok(first_started_at_from_events(&read.events))
}

fn first_started_at_from_events(events: &[SoakLedgerEventEnvelope]) -> Option<u64> {
    events.iter().find_map(|envelope| match &envelope.event {
        SoakLedgerEvent::Started { .. } => Some(envelope.recorded_at_unix_seconds),
        SoakLedgerEvent::Checkpoint { .. }
        | SoakLedgerEvent::Resume { .. }
        | SoakLedgerEvent::Stop { .. }
        | SoakLedgerEvent::Verdict { .. } => None,
    })
}

fn latest_verdict(events: &[SoakLedgerEventEnvelope]) -> Option<SoakOutcomeLabel> {
    events
        .iter()
        .rev()
        .find_map(|envelope| match &envelope.event {
            SoakLedgerEvent::Verdict { outcome } => Some(*outcome),
            SoakLedgerEvent::Started { .. }
            | SoakLedgerEvent::Checkpoint { .. }
            | SoakLedgerEvent::Resume { .. }
            | SoakLedgerEvent::Stop { .. } => None,
        })
}

fn latest_outcome(events: &[SoakLedgerEventEnvelope]) -> Option<SoakOutcomeLabel> {
    events
        .iter()
        .rev()
        .find_map(|envelope| match &envelope.event {
            SoakLedgerEvent::Verdict { outcome } | SoakLedgerEvent::Stop { outcome } => {
                Some(*outcome)
            }
            SoakLedgerEvent::Started { .. }
            | SoakLedgerEvent::Checkpoint { .. }
            | SoakLedgerEvent::Resume { .. } => None,
        })
}

fn has_terminal_stop_and_verdict(events: &[SoakLedgerEventEnvelope]) -> bool {
    let has_stop = events
        .iter()
        .any(|envelope| matches!(&envelope.event, SoakLedgerEvent::Stop { .. }));
    let has_verdict = events
        .iter()
        .any(|envelope| matches!(&envelope.event, SoakLedgerEvent::Verdict { .. }));
    has_stop && has_verdict
}

fn next_sequence(events: &[SoakLedgerEventEnvelope]) -> u64 {
    events
        .iter()
        .map(|event| event.sequence)
        .max()
        .unwrap_or(0)
        .saturating_add(1)
}

fn current_unix_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}

fn serde_label<T>(value: &T) -> String
where
    T: Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

fn outcome_label(outcome: SoakOutcomeLabel) -> String {
    serde_label(&outcome)
}

fn runtime_error(error: impl ToString) -> OperatorRuntimeError {
    OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
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
mod runtime_tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use open_bitcoin_node::{
        LogStatus, MetricRetentionPolicy, MetricsStatus,
        status::{
            BestKnownTipSource, BestKnownTipStatus, BuildProvenance, ConfigStatus,
            FieldAvailability, MempoolStatus, NodeRuntimeState, NodeStatus,
            OpenBitcoinStatusSnapshot, PeerStatus, StayCurrentStatus, SyncProgress,
            SyncRecoveryCategory, SyncReorgEvidence, SyncStatus, TipFreshnessStatus, WalletStatus,
        },
    };

    use super::{
        SoakBounds, SoakLoopMode, SoakPeerPolicy, SoakRunId, SoakStatusCollector,
        SoakStopCondition, SoakTestClock,
        ledger::{
            SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerLayout, SoakRunIndex,
            SoakRunIndexEntry,
        },
        outcome::SoakOutcomeLabel,
        run_bounded_soak_loop, validate_resume_plan, write_operator_stop, write_report_projection,
    };

    #[derive(Debug)]
    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new(label: &str) -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time after epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "open-bitcoin-soak-runtime-{label}-{}-{timestamp}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("test directory");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[derive(Debug)]
    struct ScriptedStatusCollector {
        snapshots: Vec<OpenBitcoinStatusSnapshot>,
        index: usize,
    }

    impl ScriptedStatusCollector {
        fn repeating(snapshot: OpenBitcoinStatusSnapshot) -> Self {
            Self {
                snapshots: vec![snapshot],
                index: 0,
            }
        }

        fn from_snapshots(snapshots: Vec<OpenBitcoinStatusSnapshot>) -> Self {
            Self {
                snapshots,
                index: 0,
            }
        }
    }

    impl SoakStatusCollector for ScriptedStatusCollector {
        fn collect(&mut self) -> OpenBitcoinStatusSnapshot {
            let snapshot = self
                .snapshots
                .get(self.index)
                .or_else(|| self.snapshots.last())
                .expect("scripted status snapshot")
                .clone();
            self.index += 1;
            snapshot
        }
    }

    #[test]
    fn soak_runtime_elapsed_writes_started_checkpoints_stop_and_verdict() {
        // Arrange
        let temp = TestDirectory::new("elapsed");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let run_id = SoakRunId::try_new("soak-1700000000-0001").expect("run id");
        let bounds = soak_bounds(temp.path(), Some(144), vec![SoakStopCondition::ElapsedTime]);
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        let mut collector =
            ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
        let mut clock = SoakTestClock::new(1_700_000_000);

        // Act
        let result = run_bounded_soak_loop(
            &run_id,
            &bounds,
            &layout,
            &mut ledger,
            &mut collector,
            &mut clock,
            SoakLoopMode::Start,
        )
        .expect("bounded soak loop");
        let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
            .expect("read events")
            .events;

        // Assert
        assert_eq!(result.final_outcome, SoakOutcomeLabel::CleanCompletion);
        assert_eq!(result.latest_sequence, 8);
        assert!(result.report_paths.json_path.exists());
        assert!(result.report_paths.markdown_path.exists());
        assert!(matches!(&events[0].event, SoakLedgerEvent::Started { .. }));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(&event.event, SoakLedgerEvent::Checkpoint { .. }))
                .count(),
            5
        );
        assert!(matches!(
            &events[6].event,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::CleanCompletion
            }
        ));
        assert!(matches!(
            &events[7].event,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::CleanCompletion
            }
        ));
    }

    #[test]
    fn soak_runtime_target_height_resource_recovery_and_status_verdict_stop_conditions() {
        // Arrange
        let temp = TestDirectory::new("stop-conditions");
        let cases = [
            (
                "target",
                vec![SoakStopCondition::TargetHeight],
                clean_status_snapshot(temp.path(), 144),
                Some(144),
                SoakOutcomeLabel::CleanCompletion,
            ),
            (
                "resource",
                vec![SoakStopCondition::ResourceStop],
                resource_status_snapshot(temp.path()),
                None,
                SoakOutcomeLabel::ResourceStop,
            ),
            (
                "recovery",
                vec![SoakStopCondition::RecoveryStop],
                recovery_status_snapshot(temp.path()),
                None,
                SoakOutcomeLabel::RecoveryStop,
            ),
            (
                "verdict",
                vec![SoakStopCondition::StatusVerdict],
                diagnosed_status_snapshot(temp.path()),
                None,
                SoakOutcomeLabel::DiagnosedBlocker,
            ),
        ];

        // Act / Assert
        for (label, stop_conditions, snapshot, maybe_target_height, expected) in cases {
            let run_id = SoakRunId::try_new(format!("soak-1700000000-{label}")).expect("run id");
            let layout = SoakLedgerLayout::for_datadir(temp.path());
            let mut ledger = SoakLedger::create(&layout, run_id.clone());
            let bounds = soak_bounds(temp.path(), maybe_target_height, stop_conditions);
            let mut collector = ScriptedStatusCollector::from_snapshots(vec![snapshot]);
            let mut clock = SoakTestClock::new(1_700_000_000);

            let result = run_bounded_soak_loop(
                &run_id,
                &bounds,
                &layout,
                &mut ledger,
                &mut collector,
                &mut clock,
                SoakLoopMode::Start,
            )
            .expect("bounded stop condition loop");

            assert_eq!(result.final_outcome, expected, "case {label}");
        }
    }

    #[test]
    fn soak_runtime_resume_refuses_clean_completion_and_flags_interrupted_runs() {
        // Arrange
        let temp = TestDirectory::new("resume");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let clean_run_id = SoakRunId::try_new("soak-1700000000-clean").expect("run id");
        let interrupted_run_id = SoakRunId::try_new("soak-1700000000-interrupted").expect("run id");
        let clean_paths = layout.paths_for_run(&clean_run_id);
        let interrupted_paths = layout.paths_for_run(&interrupted_run_id);
        let mut clean_ledger = SoakLedger::create(&layout, clean_run_id.clone());
        clean_ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("clean started");
        clean_ledger
            .append_event(
                2,
                SoakLedgerEvent::Stop {
                    outcome: SoakOutcomeLabel::CleanCompletion,
                },
            )
            .expect("clean stop");
        clean_ledger
            .append_event(
                3,
                SoakLedgerEvent::Verdict {
                    outcome: SoakOutcomeLabel::CleanCompletion,
                },
            )
            .expect("clean verdict");
        let mut interrupted_ledger = SoakLedger::create(&layout, interrupted_run_id.clone());
        interrupted_ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("interrupted started");
        let mut index = SoakRunIndex::empty();
        index.record_run(SoakRunIndexEntry {
            run_id: clean_run_id.clone(),
            ledger_path: clean_paths.events_path.clone(),
            started_at_unix_seconds: 1,
            updated_at_unix_seconds: 3,
            maybe_outcome: Some(SoakOutcomeLabel::CleanCompletion),
        });
        index.record_run(SoakRunIndexEntry {
            run_id: interrupted_run_id.clone(),
            ledger_path: interrupted_paths.events_path.clone(),
            started_at_unix_seconds: 1,
            updated_at_unix_seconds: 1,
            maybe_outcome: None,
        });
        index.write_atomic(&layout).expect("write index");

        // Act
        let clean_result = validate_resume_plan(&layout, &clean_run_id, 15);
        let interrupted = validate_resume_plan(&layout, &interrupted_run_id, 15)
            .expect("interrupted resume plan");

        // Assert
        assert!(clean_result.is_err());
        assert!(interrupted.interrupted_prior_run);
        assert_eq!(interrupted.next_sequence, 2);
    }

    #[test]
    fn soak_runtime_stop_records_operator_stop_verdict() {
        // Arrange
        let temp = TestDirectory::new("stop");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let run_id = SoakRunId::try_new("soak-1700000000-stop").expect("run id");
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("started");

        // Act
        let result = write_operator_stop(&layout, &run_id, 2).expect("operator stop");
        let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
            .expect("read events")
            .events;

        // Assert
        assert_eq!(result.final_outcome, SoakOutcomeLabel::OperatorStop);
        assert!(matches!(
            &events[1].event,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop
            }
        ));
        assert!(matches!(
            &events[2].event,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop
            }
        ));
    }

    #[test]
    fn soak_runtime_report_rewrites_projection_without_ledger_append() {
        // Arrange
        let temp = TestDirectory::new("report");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let run_id = SoakRunId::try_new("soak-1700000000-report").expect("run id");
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("started");
        ledger
            .append_event(
                2,
                SoakLedgerEvent::Checkpoint {
                    status: checkpoint_status(144),
                },
            )
            .expect("checkpoint");
        let events_path = layout.paths_for_run(&run_id).events_path;
        let before_lines = fs::read_to_string(&events_path)
            .expect("ledger before")
            .lines()
            .count();

        // Act
        let result = write_report_projection(&layout, &run_id).expect("report projection");
        let after_lines = fs::read_to_string(&events_path)
            .expect("ledger after")
            .lines()
            .count();

        // Assert
        assert_eq!(before_lines, after_lines);
        assert_eq!(result.latest_sequence, 2);
        assert!(result.report_paths.json_path.exists());
    }

    fn soak_bounds(
        datadir: &Path,
        maybe_target_height: Option<u64>,
        stop_conditions: Vec<SoakStopCondition>,
    ) -> SoakBounds {
        SoakBounds::try_new(
            60,
            15,
            maybe_target_height,
            datadir.to_path_buf(),
            "regtest",
            SoakPeerPolicy::DaemonConfigured,
            1_048_576,
            stop_conditions,
        )
        .expect("soak bounds")
    }

    fn checkpoint_status(height: u64) -> SoakCheckpointStatus {
        SoakCheckpointStatus {
            maybe_network: Some("regtest".to_string()),
            maybe_lifecycle: Some("active".to_string()),
            maybe_latest_stop_reason_label: None,
            maybe_recovery_category_label: None,
            maybe_no_progress_diagnosis_label: None,
            maybe_validated_active_chain_height: Some(height),
            maybe_best_known_tip_height: Some(height),
            maybe_source_status_path: None,
        }
    }

    fn clean_status_snapshot(datadir: &Path, height: u64) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.sync_progress = FieldAvailability::available(SyncProgress {
            header_height: height,
            block_height: height,
            downloaded_block_height: height,
            connected_block_height: height,
            validated_active_chain_height: height,
            maybe_downloaded_block_hash: Some("11".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_work: Some("work".to_string()),
            progress_ratio: 1.0,
            messages_processed: 1,
            headers_received: 1,
            blocks_received: 1,
        });
        snapshot.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
            source: BestKnownTipSource::HeaderStore,
            height,
            block_hash: "11".repeat(32),
            work: "work".to_string(),
            block_time_unix_seconds: 1_700_000_000,
            observed_at_unix_seconds: 1_700_000_000,
            freshness: TipFreshnessStatus::Fresh,
            peer_agreement: Vec::new(),
        });
        snapshot.sync.stay_current =
            FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
        snapshot
    }

    fn resource_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.recovery_category =
            FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion);
        snapshot
    }

    fn recovery_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.recovery_category =
            FieldAvailability::available(SyncRecoveryCategory::StoreCorruption);
        snapshot
    }

    fn diagnosed_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
            common_ancestor_height: 1,
            common_ancestor_hash: "00".repeat(32),
            disconnected_count: 1,
            connected_count: 0,
            final_active_height: 1,
            final_active_hash: "11".repeat(32),
            fully_persisted: false,
        });
        snapshot
    }

    fn base_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        OpenBitcoinStatusSnapshot {
            node: NodeStatus {
                state: NodeRuntimeState::Running,
                version: "test".to_string(),
            },
            config: ConfigStatus {
                datadir: FieldAvailability::available(datadir.display().to_string()),
                config_paths: Vec::new(),
            },
            service: open_bitcoin_node::status::ServiceStatus {
                manager: FieldAvailability::unavailable("no service manager"),
                lifecycle: FieldAvailability::unavailable("service lifecycle unavailable"),
                installed: FieldAvailability::unavailable("service installed unavailable"),
                enabled: FieldAvailability::unavailable("service enabled unavailable"),
                running: FieldAvailability::unavailable("service running unavailable"),
                service_file_path: FieldAvailability::unavailable("service file unavailable"),
                log_path: FieldAvailability::unavailable("service log unavailable"),
                diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
                restart_resume: FieldAvailability::unavailable(
                    "service restart/resume unavailable",
                ),
            },
            sync: SyncStatus {
                network: FieldAvailability::available("regtest".to_string()),
                chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
                sync_progress: FieldAvailability::unavailable("sync progress unavailable"),
                lifecycle: FieldAvailability::available(
                    open_bitcoin_node::status::SyncLifecycleState::Active,
                ),
                phase: FieldAvailability::available("test".to_string()),
                configured_targets: FieldAvailability::unavailable("targets unavailable"),
                attempt_counters: FieldAvailability::unavailable("attempts unavailable"),
                progress_signal: FieldAvailability::unavailable("signal unavailable"),
                lag: FieldAvailability::unavailable("lag unavailable"),
                last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                    "progress unavailable",
                ),
                latest_stop_reason: FieldAvailability::unavailable("stop reason unavailable"),
                last_error: FieldAvailability::unavailable("last error unavailable"),
                recovery_category: FieldAvailability::unavailable("recovery unavailable"),
                recovery_action: FieldAvailability::unavailable("recovery action unavailable"),
                resource_pressure: FieldAvailability::unavailable("pressure unavailable"),
                best_known_tip: FieldAvailability::unavailable("best tip unavailable"),
                stay_current: FieldAvailability::unavailable("stay current unavailable"),
                stay_current_next_action: FieldAvailability::unavailable(
                    "stay current action unavailable",
                ),
                no_progress_diagnosis: FieldAvailability::unavailable(
                    "no-progress diagnosis unavailable",
                ),
                no_progress_next_action: FieldAvailability::unavailable(
                    "no-progress action unavailable",
                ),
                latest_reorg: FieldAvailability::unavailable("reorg unavailable"),
                reconcile_progress: FieldAvailability::unavailable("reconcile unavailable"),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::unavailable("peer counts unavailable"),
                recent_peers: FieldAvailability::unavailable("recent peers unavailable"),
            },
            mempool: MempoolStatus {
                transactions: FieldAvailability::unavailable("mempool unavailable"),
            },
            wallet: WalletStatus {
                trusted_balance_sats: FieldAvailability::unavailable("wallet unavailable"),
                freshness: FieldAvailability::unavailable("wallet freshness unavailable"),
                scan_progress: FieldAvailability::unavailable("wallet scan unavailable"),
            },
            logs: LogStatus::default(),
            metrics: MetricsStatus::unavailable(
                MetricRetentionPolicy::default(),
                "metrics unavailable",
            ),
            health_signals: Vec::new(),
            build: BuildProvenance::unavailable(),
        }
    }
}
