// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    OpenBitcoinStatusSnapshot,
    status::{
        FieldAvailability, ResourceBoundKind, ResourceBoundSnapshot, ResourcePressureState,
        classify_snapshot_against_disk_budget,
    },
};
use serde::Serialize;

use super::super::{
    SoakBounds, SoakRunId, SoakStopCondition,
    ledger::{
        SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerEventEnvelope,
        SoakLedgerLayout, SoakRunIndex, SoakRunIndexEntry,
    },
    outcome::{SoakOutcomeEvidence, SoakOutcomeLabel, classify_soak_outcome},
};
use crate::operator::{
    runtime::OperatorRuntimeError,
    support::{EvidenceState, LiveSmokeEvidence, derive_full_sync_evidence},
};

pub(super) fn evaluate_stop_outcome(
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
                if resource_bounds_stop_required(bounds, snapshot) {
                    return Some(SoakOutcomeLabel::ResourceStop);
                }
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
            SoakStopCondition::OperatorStop => {
                let outcome = outcome_for_snapshot(snapshot);
                if matches!(outcome, SoakOutcomeLabel::OperatorStop) {
                    return Some(outcome);
                }
            }
            SoakStopCondition::ElapsedTime | SoakStopCondition::TargetHeight => {}
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
    if resource_bounds_overall_stop_required(snapshot) {
        return SoakOutcomeLabel::ResourceStop;
    }

    let maybe_recovery_category = snapshot_recovery_category(snapshot);
    classify_soak_outcome(&SoakOutcomeEvidence {
        maybe_sync_stop_reason: maybe_available(&snapshot.sync.latest_stop_reason),
        maybe_recovery_category,
        maybe_no_progress_diagnosis: maybe_available(&snapshot.sync.no_progress_diagnosis),
        maybe_full_sync_evidence: Some(derive_full_sync_evidence(
            snapshot,
            &missing_live_smoke_evidence(),
        )),
        maybe_process_exit: None,
    })
}

pub(super) fn checkpoint_status_from_snapshot(
    snapshot: &OpenBitcoinStatusSnapshot,
) -> SoakCheckpointStatus {
    let maybe_recovery_evidence = match &snapshot.recovery_evidence {
        FieldAvailability::Available(evidence) => Some(evidence),
        FieldAvailability::Unavailable { .. } => None,
    };
    SoakCheckpointStatus {
        maybe_network: maybe_available(&snapshot.sync.network),
        maybe_lifecycle: maybe_available(&snapshot.sync.lifecycle).map(|value| serde_label(&value)),
        maybe_latest_stop_reason_label: maybe_available(&snapshot.sync.latest_stop_reason)
            .map(|value| value.label),
        maybe_recovery_category_label: maybe_recovery_evidence
            .map(|evidence| evidence.category.as_str().to_string())
            .or_else(|| {
                maybe_available(&snapshot.sync.recovery_category)
                    .map(|value| value.as_str().to_string())
            }),
        maybe_recovery_action_class_label: maybe_recovery_evidence
            .map(|evidence| serde_label(&evidence.action_class)),
        maybe_recovery_cause_label: maybe_recovery_evidence
            .map(|evidence| serde_label(&evidence.cause)),
        maybe_recovery_next_action: maybe_recovery_evidence
            .map(|evidence| evidence.next_action.clone()),
        maybe_no_progress_diagnosis_label: maybe_available(&snapshot.sync.no_progress_diagnosis)
            .map(|value| serde_label(&value)),
        maybe_resource_bound_state_label: resource_bound_state_label(snapshot),
        resource_bound_labels: resource_bound_labels(snapshot),
        maybe_resource_bound_next_action: resource_bound_next_action(snapshot),
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

fn snapshot_recovery_category(
    snapshot: &OpenBitcoinStatusSnapshot,
) -> Option<open_bitcoin_node::status::SyncRecoveryCategory> {
    match &snapshot.recovery_evidence {
        FieldAvailability::Available(evidence) => Some(evidence.category),
        FieldAvailability::Unavailable { .. } => maybe_available(&snapshot.sync.recovery_category),
    }
}

pub(super) fn validate_resource_bound_preflight(
    bounds: &SoakBounds,
    snapshot: &OpenBitcoinStatusSnapshot,
) -> Result<(), OperatorRuntimeError> {
    if !bounds.datadir.is_dir() {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "soak resource-bound preflight requires an existing datadir: {}",
                bounds.datadir.display()
            ),
        });
    }

    let FieldAvailability::Available(resource_bounds) = &snapshot.resource_bounds else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "soak resource-bound preflight could not collect resource bounds".to_string(),
        });
    };

    if resource_bounds.has_unavailable_required_measurements() {
        let mut unavailable = ResourceBoundKind::ALL
            .iter()
            .filter(|kind| {
                !resource_bounds
                    .entries
                    .iter()
                    .any(|entry| entry.kind == **kind)
            })
            .map(|kind| format!("{}: missing", kind.as_str()))
            .collect::<Vec<_>>();
        unavailable.extend(
            resource_bounds
                .entries
                .iter()
                .filter_map(|entry| match &entry.usage {
                    FieldAvailability::Unavailable { reason } => {
                        Some(format!("{}: {reason}", entry.kind.as_str()))
                    }
                    FieldAvailability::Available(_) => None,
                }),
        );
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "soak resource-bound preflight could not assess required resource bounds: {}",
                unavailable.join(", ")
            ),
        });
    }

    if classify_snapshot_against_disk_budget(resource_bounds, bounds.disk_budget_bytes)
        == ResourcePressureState::StopRequired
    {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "soak resource-bound preflight requires disk usage below 95% of the {} byte disk budget",
                bounds.disk_budget_bytes
            ),
        });
    }

    if resource_bounds.overall_level == ResourcePressureState::StopRequired {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "soak resource-bound preflight found stop-required resource pressure: {}",
                pressured_resource_bound_labels(resource_bounds).join(", ")
            ),
        });
    }

    Ok(())
}

fn resource_bounds_stop_required(
    bounds: &SoakBounds,
    snapshot: &OpenBitcoinStatusSnapshot,
) -> bool {
    let FieldAvailability::Available(resource_bounds) = &snapshot.resource_bounds else {
        return false;
    };
    resource_bounds.overall_level == ResourcePressureState::StopRequired
        || classify_snapshot_against_disk_budget(resource_bounds, bounds.disk_budget_bytes)
            == ResourcePressureState::StopRequired
}

fn resource_bounds_overall_stop_required(snapshot: &OpenBitcoinStatusSnapshot) -> bool {
    matches!(
        &snapshot.resource_bounds,
        FieldAvailability::Available(resource_bounds)
            if resource_bounds.overall_level == ResourcePressureState::StopRequired
    )
}

fn resource_bound_state_label(snapshot: &OpenBitcoinStatusSnapshot) -> Option<String> {
    match &snapshot.resource_bounds {
        FieldAvailability::Available(resource_bounds) => {
            Some(resource_bounds.overall_level.as_str().to_string())
        }
        FieldAvailability::Unavailable { .. } => None,
    }
}

fn resource_bound_labels(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<String> {
    match &snapshot.resource_bounds {
        FieldAvailability::Available(resource_bounds) => {
            let labels = resource_bounds
                .entries
                .iter()
                .filter_map(|entry| match &entry.usage {
                    FieldAvailability::Available(usage)
                        if usage.state != ResourcePressureState::Normal =>
                    {
                        Some(format!("{}={}", entry.kind.as_str(), usage.state.as_str()))
                    }
                    FieldAvailability::Unavailable { reason } => {
                        Some(format!("{}=unavailable({reason})", entry.kind.as_str()))
                    }
                    FieldAvailability::Available(_) => None,
                })
                .collect::<Vec<_>>();
            if labels.is_empty() {
                vec!["all_required_bounds=normal".to_string()]
            } else {
                labels
            }
        }
        FieldAvailability::Unavailable { reason } => {
            vec![format!("resource_bounds=unavailable({reason})")]
        }
    }
}

fn resource_bound_next_action(snapshot: &OpenBitcoinStatusSnapshot) -> Option<String> {
    let FieldAvailability::Available(resource_bounds) = &snapshot.resource_bounds else {
        return None;
    };
    resource_bounds
        .entries
        .iter()
        .filter_map(|entry| match &entry.usage {
            FieldAvailability::Available(usage)
                if usage.state == ResourcePressureState::StopRequired =>
            {
                Some(format!("{}: {}", entry.kind.as_str(), usage.next_action))
            }
            FieldAvailability::Available(usage)
                if usage.state == ResourcePressureState::Warning =>
            {
                Some(format!("{}: {}", entry.kind.as_str(), usage.next_action))
            }
            FieldAvailability::Available(_) | FieldAvailability::Unavailable { .. } => None,
        })
        .next()
}

fn pressured_resource_bound_labels(resource_bounds: &ResourceBoundSnapshot) -> Vec<String> {
    resource_bounds
        .entries
        .iter()
        .filter_map(|entry| match &entry.usage {
            FieldAvailability::Available(usage) if usage.state != ResourcePressureState::Normal => {
                Some(format!("{}={}", entry.kind.as_str(), usage.state.as_str()))
            }
            FieldAvailability::Available(_) | FieldAvailability::Unavailable { .. } => None,
        })
        .collect()
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

pub(super) fn validate_index_entry(
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

pub(super) fn reject_run_collision(
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

pub(super) fn generate_soak_run_id(
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

pub(super) fn record_run_index(
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

pub(super) fn started_bounds(
    events: &[SoakLedgerEventEnvelope],
) -> Result<SoakBounds, OperatorRuntimeError> {
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

pub(super) fn first_started_at(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
) -> Result<Option<u64>, OperatorRuntimeError> {
    let paths = layout.paths_for_run(run_id);
    let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
    Ok(first_started_at_from_events(&read.events))
}

pub(super) fn first_started_at_from_events(events: &[SoakLedgerEventEnvelope]) -> Option<u64> {
    events.iter().find_map(|envelope| match &envelope.event {
        SoakLedgerEvent::Started { .. } => Some(envelope.recorded_at_unix_seconds),
        SoakLedgerEvent::Checkpoint { .. }
        | SoakLedgerEvent::Resume { .. }
        | SoakLedgerEvent::Stop { .. }
        | SoakLedgerEvent::Verdict { .. } => None,
    })
}

pub(super) fn latest_verdict(events: &[SoakLedgerEventEnvelope]) -> Option<SoakOutcomeLabel> {
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

pub(super) fn latest_outcome(events: &[SoakLedgerEventEnvelope]) -> Option<SoakOutcomeLabel> {
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

pub(super) fn has_terminal_stop_and_verdict(events: &[SoakLedgerEventEnvelope]) -> bool {
    let has_stop = events
        .iter()
        .any(|envelope| matches!(&envelope.event, SoakLedgerEvent::Stop { .. }));
    let has_verdict = events
        .iter()
        .any(|envelope| matches!(&envelope.event, SoakLedgerEvent::Verdict { .. }));
    has_stop && has_verdict
}

pub(super) fn next_sequence(events: &[SoakLedgerEventEnvelope]) -> u64 {
    events
        .iter()
        .map(|event| event.sequence)
        .max()
        .unwrap_or(0)
        .saturating_add(1)
}

pub(super) fn current_unix_seconds() -> u64 {
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

pub(super) fn outcome_label(outcome: SoakOutcomeLabel) -> String {
    serde_label(&outcome)
}

pub(super) fn runtime_error(error: impl ToString) -> OperatorRuntimeError {
    OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
    }
}
