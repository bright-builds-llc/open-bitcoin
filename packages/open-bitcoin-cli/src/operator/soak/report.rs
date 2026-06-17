// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Soak report projections.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;

use super::{
    SoakBounds, SoakRunId,
    ledger::{
        SOAK_LEDGER_SCHEMA_VERSION, SoakCheckpointStatus, SoakLedgerEvent, SoakLedgerEventEnvelope,
        SoakLedgerLayout, SoakLedgerReadResult,
    },
    outcome::SoakOutcomeLabel,
};

// Redaction guard: report projections must not include raw daemon logs,
// raw live-smoke reports, private wallet data, RPC credentials, or peer-list dumps.

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SoakReportProjection {
    pub(crate) schema_version: u16,
    pub(crate) is_projection: bool,
    pub(crate) source_ledger_path: String,
    pub(crate) latest_sequence: u64,
    pub(crate) run_id: SoakRunId,
    pub(crate) bounds: SoakBounds,
    pub(crate) started_at_unix_seconds: u64,
    pub(crate) checkpoint_count: usize,
    pub(crate) resume_count: usize,
    pub(crate) interrupted_resume_count: usize,
    pub(crate) latest_checkpoint: Option<SoakCheckpointStatus>,
    pub(crate) stop: Option<SoakReportOutcomeEvent>,
    pub(crate) verdict: Option<SoakReportOutcomeEvent>,
}

impl SoakReportProjection {
    pub(crate) fn from_ledger_events(
        events: Vec<SoakLedgerEventEnvelope>,
        source_ledger_path: impl AsRef<Path>,
    ) -> Result<Self, SoakReportError> {
        let Some(first_event) = events.first() else {
            return Err(SoakReportError::EmptyLedger);
        };
        let run_id = first_event.run_id.clone();
        let source_ledger_path = source_ledger_path.as_ref().display().to_string();
        let latest_sequence = events
            .iter()
            .map(|envelope| envelope.sequence)
            .max()
            .unwrap_or(0);
        let mut maybe_bounds = None;
        let mut maybe_started_at_unix_seconds = None;
        let mut checkpoint_count = 0;
        let mut resume_count = 0;
        let mut interrupted_resume_count = 0;
        let mut latest_checkpoint = None;
        let mut stop = None;
        let mut verdict = None;

        for envelope in events {
            if envelope.run_id != run_id {
                return Err(SoakReportError::InconsistentRunId {
                    expected: run_id,
                    actual: envelope.run_id,
                });
            }
            match envelope.event {
                SoakLedgerEvent::Started { bounds } => {
                    maybe_bounds = Some(bounds);
                    maybe_started_at_unix_seconds = Some(envelope.recorded_at_unix_seconds);
                }
                SoakLedgerEvent::Checkpoint { status } => {
                    checkpoint_count += 1;
                    latest_checkpoint = Some(*status);
                }
                SoakLedgerEvent::Resume {
                    interrupted_prior_run,
                } => {
                    resume_count += 1;
                    if interrupted_prior_run {
                        interrupted_resume_count += 1;
                    }
                }
                SoakLedgerEvent::Stop { outcome } => {
                    stop = Some(SoakReportOutcomeEvent::from_envelope(&envelope, outcome));
                }
                SoakLedgerEvent::Verdict { outcome } => {
                    verdict = Some(SoakReportOutcomeEvent::from_envelope(&envelope, outcome));
                }
            }
        }

        let Some(bounds) = maybe_bounds else {
            return Err(SoakReportError::MissingStartedEvent);
        };
        let Some(started_at_unix_seconds) = maybe_started_at_unix_seconds else {
            return Err(SoakReportError::MissingStartedEvent);
        };

        Ok(Self {
            schema_version: SOAK_LEDGER_SCHEMA_VERSION,
            is_projection: true,
            source_ledger_path,
            latest_sequence,
            run_id,
            bounds,
            started_at_unix_seconds,
            checkpoint_count,
            resume_count,
            interrupted_resume_count,
            latest_checkpoint,
            stop,
            verdict,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SoakReportOutcomeEvent {
    pub(crate) sequence: u64,
    pub(crate) recorded_at_unix_seconds: u64,
    pub(crate) outcome: SoakOutcomeLabel,
}

impl SoakReportOutcomeEvent {
    fn from_envelope(envelope: &SoakLedgerEventEnvelope, outcome: SoakOutcomeLabel) -> Self {
        Self {
            sequence: envelope.sequence,
            recorded_at_unix_seconds: envelope.recorded_at_unix_seconds,
            outcome,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakReportPaths {
    pub(crate) json_path: PathBuf,
    pub(crate) markdown_path: PathBuf,
    pub(crate) source_ledger_path: PathBuf,
    pub(crate) latest_sequence: u64,
}

pub(crate) fn render_soak_report_json(
    projection: &SoakReportProjection,
) -> Result<String, SoakReportError> {
    serde_json::to_string_pretty(projection)
        .map(|text| format!("{text}\n"))
        .map_err(SoakReportError::Encode)
}

pub(crate) fn render_soak_report_markdown(projection: &SoakReportProjection) -> String {
    let mut output = String::new();
    output.push_str("# Open Bitcoin Soak Report\n\n");
    output.push_str(&format!(
        "Source ledger: {}\n",
        projection.source_ledger_path
    ));
    output.push_str(&format!(
        "Latest sequence: {}\n",
        projection.latest_sequence
    ));
    output.push_str("Report is a projection: true\n");
    output.push_str(&format!("Run id: {}\n", projection.run_id));
    output.push_str(&format!(
        "Started at: {}\n",
        projection.started_at_unix_seconds
    ));
    output.push_str(&format!("Network: {}\n", projection.bounds.network));
    output.push_str(&format!(
        "Checkpoint count: {}\n",
        projection.checkpoint_count
    ));
    output.push_str(&format!("Resume count: {}\n", projection.resume_count));
    output.push_str(&format!(
        "Interrupted resumes: {}\n",
        projection.interrupted_resume_count
    ));
    output.push_str(&format!(
        "Final outcome: {}\n",
        projection
            .verdict
            .as_ref()
            .or(projection.stop.as_ref())
            .map(|event| outcome_label(event.outcome))
            .unwrap_or_else(|| "unavailable".to_string())
    ));
    if let Some(checkpoint) = projection.latest_checkpoint.as_ref() {
        output.push_str("\n## Latest Checkpoint\n\n");
        push_optional(&mut output, "Network", checkpoint.maybe_network.as_deref());
        push_optional(
            &mut output,
            "Lifecycle",
            checkpoint.maybe_lifecycle.as_deref(),
        );
        push_optional(
            &mut output,
            "Latest stop reason",
            checkpoint.maybe_latest_stop_reason_label.as_deref(),
        );
        push_optional(
            &mut output,
            "Recovery category",
            checkpoint.maybe_recovery_category_label.as_deref(),
        );
        push_optional(
            &mut output,
            "Recovery action class",
            checkpoint.maybe_recovery_action_class_label.as_deref(),
        );
        push_optional(
            &mut output,
            "Recovery cause",
            checkpoint.maybe_recovery_cause_label.as_deref(),
        );
        push_optional(
            &mut output,
            "Recovery next action",
            checkpoint.maybe_recovery_next_action.as_deref(),
        );
        push_optional(
            &mut output,
            "No-progress diagnosis",
            checkpoint.maybe_no_progress_diagnosis_label.as_deref(),
        );
        push_progress_credit(&mut output, checkpoint);
        if !checkpoint
            .progress_credit_rejected_activity_labels
            .is_empty()
        {
            output.push_str(&format!(
                "- Rejected progress activity: {}\n",
                checkpoint
                    .progress_credit_rejected_activity_labels
                    .join(", ")
            ));
        }
        push_optional_u64(
            &mut output,
            "Expected progress window seconds",
            checkpoint.maybe_expected_progress_window_seconds,
        );
        push_no_progress_threshold(&mut output, checkpoint);
        push_last_useful_work(&mut output, checkpoint);
        push_optional(
            &mut output,
            "Last peer contribution",
            checkpoint.maybe_last_peer_contribution_label.as_deref(),
        );
        push_optional(
            &mut output,
            "Stalled subsystem",
            checkpoint.maybe_stalled_subsystem_label.as_deref(),
        );
        push_optional(
            &mut output,
            "Stall confidence",
            checkpoint.maybe_stall_confidence_label.as_deref(),
        );
        if !checkpoint.stall_evidence_basis.is_empty() {
            output.push_str(&format!(
                "- Stall evidence basis: {}\n",
                checkpoint.stall_evidence_basis.join(", ")
            ));
        }
        push_optional(
            &mut output,
            "Stall next action",
            checkpoint.maybe_stall_next_action.as_deref(),
        );
        push_optional(
            &mut output,
            "Resource bound state",
            checkpoint.maybe_resource_bound_state_label.as_deref(),
        );
        if !checkpoint.resource_bound_labels.is_empty() {
            output.push_str(&format!(
                "- Resource bound labels: {}\n",
                checkpoint.resource_bound_labels.join(", ")
            ));
        }
        push_optional(
            &mut output,
            "Resource bound next action",
            checkpoint.maybe_resource_bound_next_action.as_deref(),
        );
        push_optional_u64(
            &mut output,
            "Validated active-chain height",
            checkpoint.maybe_validated_active_chain_height,
        );
        push_optional_u64(
            &mut output,
            "Best-known-tip height",
            checkpoint.maybe_best_known_tip_height,
        );
        if let Some(path) = checkpoint.maybe_source_status_path.as_ref() {
            output.push_str(&format!("- Source status: {}\n", path.display()));
        }
    }
    output
}

pub(crate) fn write_soak_reports(
    read_result: &SoakLedgerReadResult,
    source_ledger_path: impl AsRef<Path>,
    layout: &SoakLedgerLayout,
) -> Result<SoakReportPaths, SoakReportError> {
    let source_ledger_path = source_ledger_path.as_ref().to_path_buf();
    let projection =
        SoakReportProjection::from_ledger_events(read_result.events.clone(), &source_ledger_path)?;
    let run_paths = layout.paths_for_run(&projection.run_id);
    fs::create_dir_all(&run_paths.run_dir).map_err(|source| SoakReportError::Io {
        path: run_paths.run_dir.clone(),
        action: "create soak report directory",
        source,
    })?;
    let json = render_soak_report_json(&projection)?;
    let markdown = render_soak_report_markdown(&projection);
    fs::write(&run_paths.report_json_path, json).map_err(|source| SoakReportError::Io {
        path: run_paths.report_json_path.clone(),
        action: "write soak JSON report",
        source,
    })?;
    fs::write(&run_paths.report_markdown_path, markdown).map_err(|source| SoakReportError::Io {
        path: run_paths.report_markdown_path.clone(),
        action: "write soak Markdown report",
        source,
    })?;

    Ok(SoakReportPaths {
        json_path: run_paths.report_json_path,
        markdown_path: run_paths.report_markdown_path,
        source_ledger_path,
        latest_sequence: projection.latest_sequence,
    })
}

fn push_optional(output: &mut String, label: &str, maybe_value: Option<&str>) {
    if let Some(value) = maybe_value {
        output.push_str(&format!("- {label}: {value}\n"));
    }
}

fn push_optional_u64(output: &mut String, label: &str, maybe_value: Option<u64>) {
    if let Some(value) = maybe_value {
        output.push_str(&format!("- {label}: {value}\n"));
    }
}

fn push_progress_credit(output: &mut String, checkpoint: &SoakCheckpointStatus) {
    let Some(kind) = checkpoint.maybe_progress_credit_kind_label.as_deref() else {
        return;
    };
    let mut parts = vec![format!("kind={kind}")];
    if let Some(height) = checkpoint.maybe_progress_credit_height {
        parts.push(format!("height={height}"));
    }
    if let Some(hash) = checkpoint.maybe_progress_credit_hash.as_deref() {
        parts.push(format!("hash={hash}"));
    }
    if let Some(work) = checkpoint.maybe_progress_credit_work.as_deref() {
        parts.push(format!("work={work}"));
    }
    if let Some(source_unix_seconds) = checkpoint.maybe_progress_credit_source_unix_seconds {
        parts.push(format!("source_unix_seconds={source_unix_seconds}"));
    }
    output.push_str(&format!("- Progress credit: {}\n", parts.join(" ")));
}

fn push_no_progress_threshold(output: &mut String, checkpoint: &SoakCheckpointStatus) {
    let mut parts = Vec::new();
    if let Some(state) = checkpoint
        .maybe_no_progress_threshold_state_label
        .as_deref()
    {
        parts.push(format!("state={state}"));
    }
    if let Some(seconds) = checkpoint.maybe_no_progress_threshold_seconds {
        parts.push(format!("seconds={seconds}"));
    }
    if !parts.is_empty() {
        output.push_str(&format!("- No-progress threshold: {}\n", parts.join(" ")));
    }
}

fn push_last_useful_work(output: &mut String, checkpoint: &SoakCheckpointStatus) {
    let Some(kind) = checkpoint.maybe_last_useful_work_kind_label.as_deref() else {
        return;
    };
    let mut parts = vec![format!("kind={kind}")];
    if let Some(height) = checkpoint.maybe_last_useful_work_height {
        parts.push(format!("height={height}"));
    }
    output.push_str(&format!("- Last useful work: {}\n", parts.join(" ")));
}

fn outcome_label(outcome: SoakOutcomeLabel) -> String {
    serde_json::to_value(outcome)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum SoakReportError {
    #[error("cannot project a soak report from an empty ledger")]
    EmptyLedger,

    #[error("soak ledger is missing a started event")]
    MissingStartedEvent,

    #[error("soak ledger contains inconsistent run ids: expected {expected}, actual {actual}")]
    InconsistentRunId {
        expected: SoakRunId,
        actual: SoakRunId,
    },

    #[error("could not encode soak report JSON: {0}")]
    Encode(#[source] serde_json::Error),

    #[error("I/O error while attempting to {action} at {path:?}: {source}")]
    Io {
        path: PathBuf,
        action: &'static str,
        #[source]
        source: std::io::Error,
    },
}
