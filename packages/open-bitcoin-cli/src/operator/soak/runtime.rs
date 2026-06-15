// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::path::PathBuf;

use open_bitcoin_node::OpenBitcoinStatusSnapshot;
use serde_json::json;

use super::{
    SoakBounds, SoakPeerPolicy, SoakRunId, SoakStopCondition,
    ledger::{SoakLedger, SoakLedgerEvent, SoakLedgerLayout},
    outcome::SoakOutcomeLabel,
    report::{SoakReportPaths, write_soak_reports},
};
use crate::operator::{
    NetworkSelection, OperatorOutputFormat, SoakPeerPolicyArg, SoakReportArgs, SoakResumeArgs,
    SoakStartArgs, SoakStopArgs, SoakStopConditionArg, SoakStopReasonArg,
    config::OperatorConfigResolution,
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
    status::collect_status_snapshot,
};
use helpers::{
    checkpoint_status_from_snapshot, current_unix_seconds, evaluate_stop_outcome, first_started_at,
    first_started_at_from_events, generate_soak_run_id, has_terminal_stop_and_verdict,
    latest_outcome, latest_verdict, next_sequence, outcome_label, record_run_index,
    reject_run_collision, runtime_error, started_bounds, validate_index_entry,
};

mod helpers;

pub(crate) fn execute_soak_start(
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

pub(crate) fn execute_soak_resume(
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
            run_started_at_unix_seconds: resume.started_at_unix_seconds,
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

pub(crate) fn execute_soak_stop(
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

pub(crate) fn execute_soak_report(
    args: &SoakReportArgs,
    format: OperatorOutputFormat,
    layout: &SoakLedgerLayout,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let run_id = SoakRunId::try_new(args.run_id.clone()).map_err(runtime_error)?;
    validate_index_entry(layout, &run_id)?;
    let result = write_report_projection(layout, &run_id)?;
    render_soak_command_output(format, &run_id, &result)
}

pub(crate) trait SoakStatusCollector {
    fn collect(&mut self) -> OpenBitcoinStatusSnapshot;
}

pub(crate) struct RuntimeSoakStatusCollector<'a> {
    parts: &'a mut super::super::runtime::StatusRuntimeParts,
}

impl<'a> RuntimeSoakStatusCollector<'a> {
    pub(super) const fn new(parts: &'a mut super::super::runtime::StatusRuntimeParts) -> Self {
        Self { parts }
    }
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

pub(crate) struct SystemSoakClock;

impl SoakClock for SystemSoakClock {
    fn now_unix_seconds(&mut self) -> u64 {
        current_unix_seconds()
    }

    fn sleep_until(&mut self, scheduled_unix_seconds: u64) {
        let now = current_unix_seconds();
        if scheduled_unix_seconds > now {
            std::thread::sleep(std::time::Duration::from_secs(scheduled_unix_seconds - now));
        }
    }
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
    Resume {
        interrupted_prior_run: bool,
        run_started_at_unix_seconds: u64,
    },
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
    pub(crate) started_at_unix_seconds: u64,
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
    let invocation_started_at = clock.now_unix_seconds();
    let run_started_at = match mode {
        SoakLoopMode::Start => invocation_started_at,
        SoakLoopMode::Resume {
            run_started_at_unix_seconds,
            ..
        } => run_started_at_unix_seconds,
    };
    let invocation_marker_sequence = match mode {
        SoakLoopMode::Start => {
            ledger
                .append_event(
                    invocation_started_at,
                    SoakLedgerEvent::Started {
                        bounds: bounds.clone(),
                    },
                )
                .map_err(runtime_error)?
                .sequence
        }
        SoakLoopMode::Resume {
            interrupted_prior_run,
            ..
        } => {
            ledger
                .append_event(
                    invocation_started_at,
                    SoakLedgerEvent::Resume {
                        interrupted_prior_run,
                    },
                )
                .map_err(runtime_error)?
                .sequence
        }
    };

    let deadline = run_started_at.saturating_add(bounds.elapsed_time_seconds);
    let mut checkpoint_at = invocation_started_at;
    let mut final_outcome = None;
    while final_outcome.is_none() {
        clock.sleep_until(checkpoint_at);
        if let Some(result) =
            existing_terminal_result_after_sequence(layout, run_id, invocation_marker_sequence)?
        {
            return Ok(result);
        }
        let snapshot = collector.collect();
        if let Some(result) =
            existing_terminal_result_after_sequence(layout, run_id, invocation_marker_sequence)?
        {
            return Ok(result);
        }
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
    if let Some(result) =
        existing_terminal_result_after_sequence(layout, run_id, invocation_marker_sequence)?
    {
        return Ok(result);
    }
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
        started_at_unix_seconds: first_started_at(layout, run_id)?.unwrap_or(run_started_at),
        updated_at_unix_seconds: checkpoint_at,
        ..result
    })
}

fn existing_terminal_result_after_sequence(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
    invocation_marker_sequence: u64,
) -> Result<Option<SoakLoopResult>, OperatorRuntimeError> {
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    let later_events = read
        .events
        .iter()
        .filter(|event| event.sequence > invocation_marker_sequence)
        .cloned()
        .collect::<Vec<_>>();
    if has_terminal_stop_and_verdict(&later_events) {
        return write_report_projection(layout, run_id).map(Some);
    }
    Ok(None)
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
    let started_at_unix_seconds = first_started_at_from_events(&read.events).ok_or_else(|| {
        OperatorRuntimeError::InvalidRequest {
            message: "soak ledger is missing a started event".to_string(),
        }
    })?;
    bounds.checkpoint_interval_seconds = checkpoint_interval_seconds;
    let invocation_events = latest_invocation_events(&read.events);
    // D-11 same-run resume matrix: clean_completion refuses; operator_stop,
    // resource_stop, and recovery_stop resume as same-run; unexpected_termination
    // resumes as interrupted recovery evidence.
    let interrupted_prior_run = if !has_terminal_stop_and_verdict(invocation_events) {
        true
    } else {
        match latest_verdict(invocation_events) {
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
        }
    };
    Ok(SoakResumePlan {
        bounds,
        interrupted_prior_run,
        next_sequence: next_sequence(&read.events),
        started_at_unix_seconds,
    })
}

fn latest_invocation_events(
    events: &[super::ledger::SoakLedgerEventEnvelope],
) -> &[super::ledger::SoakLedgerEventEnvelope] {
    let latest_invocation_index = events
        .iter()
        .rposition(|envelope| {
            matches!(
                &envelope.event,
                SoakLedgerEvent::Started { .. } | SoakLedgerEvent::Resume { .. }
            )
        })
        .unwrap_or(0);
    &events[latest_invocation_index..]
}

pub(crate) fn write_operator_stop(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
    recorded_at_unix_seconds: u64,
) -> Result<SoakLoopResult, OperatorRuntimeError> {
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    if has_terminal_stop_and_verdict(&read.events) {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!("soak run {run_id} already has a terminal verdict"),
        });
    }
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

pub(crate) fn require_soak_datadir(
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
