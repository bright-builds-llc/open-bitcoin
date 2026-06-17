// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Local support evidence bundle generation.

mod evidence;
mod live_smoke;
mod progress_guarantee;
mod render;
mod resource_bounds;

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    MetricsStatus, OpenBitcoinStatusSnapshot, RuntimeMetadata,
    recovery::RecoveryEvidenceSnapshot,
    status::{FieldAvailability, ServiceRestartResumeStatus},
};
use serde::Serialize;
use serde_json::Value;

pub(crate) use evidence::{
    ActiveChainEvidence, EvidenceAvailability, EvidenceState, FullSyncEvidence, LiveSmokeEvidence,
    SummaryEvidence, SupportEvidenceVerdict, derive_full_sync_evidence,
};
#[cfg(test)]
pub(crate) use evidence::{EvidenceVerdictSummary, TipEvidence};
use render::{render_support_markdown, render_support_outcome};
use resource_bounds::{ResourceBoundSupportEvidence, collect_resource_bound_support_evidence};

use super::{
    OperatorOutputFormat, SupportArgs, SupportBundleArgs, SupportCommand,
    config::{
        OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigResolution,
        OperatorCredentialSource,
    },
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
    soak::{
        ledger::{SoakLedger, SoakLedgerLayout, SoakRunIndex},
        outcome::SoakOutcomeLabel,
        report::SoakReportProjection,
    },
};

const SUPPORT_EVIDENCE_JSON: &str = "support-evidence.json";
const SUPPORT_EVIDENCE_MARKDOWN: &str = "support-evidence.md";
const SOAK_LEDGER_UNAVAILABLE_REASON: &str = "soak ledger unavailable";
const SUPPORT_RECOVERY_EVIDENCE_SOURCE: &str = "status.recovery_evidence";
const SUPPORT_PROBE_ONLY_RUNTIME_METADATA_REASON: &str =
    "runtime metadata unavailable: probe-only support bundle does not open Fjall stores";
const SUPPORT_PROBE_ONLY_METRICS_HISTORY_REASON: &str =
    "metrics history unavailable: probe-only support bundle does not open Fjall stores";

pub(crate) fn execute_support_command(
    args: &SupportArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
    status: OpenBitcoinStatusSnapshot,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    match &args.command {
        SupportCommand::Bundle(bundle) => {
            execute_support_bundle(bundle, format, config_resolution, status)
        }
    }
}

fn execute_support_bundle(
    args: &SupportBundleArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
    status: OpenBitcoinStatusSnapshot,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let output_dir = support_output_dir(args, config_resolution)?;
    fs::create_dir_all(&output_dir).map_err(|error| OperatorRuntimeError::InvalidRequest {
        message: format!(
            "could not create support evidence output directory {}: {error}",
            output_dir.display()
        ),
    })?;

    let json_path = output_dir.join(SUPPORT_EVIDENCE_JSON);
    let markdown_path = output_dir.join(SUPPORT_EVIDENCE_MARKDOWN);
    let generated_at_unix_seconds = current_unix_seconds();
    let live_smoke = collect_live_smoke_evidence(args.maybe_live_smoke_report.as_deref());
    let full_sync_evidence = derive_full_sync_evidence(&status, &live_smoke);
    let soak_evidence = collect_soak_support_evidence(config_resolution);
    let resource_bound_evidence = collect_resource_bound_support_evidence(&status, &output_dir);
    let recovery_evidence = RecoverySupportEvidence::from_status(&status.recovery_evidence);
    let store_health = collect_store_health(&status);
    let bundle = SupportEvidenceBundle {
        generated_at_unix_seconds,
        generated_by: "open-bitcoin support bundle".to_string(),
        output: SupportEvidenceOutput {
            directory: path_to_string(&output_dir),
            json_path: path_to_string(&json_path),
            markdown_path: path_to_string(&markdown_path),
        },
        redaction: redaction_summary(),
        config: ConfigEvidence::from_resolution(config_resolution),
        status,
        recovery_evidence,
        store_health,
        live_smoke,
        full_sync_evidence,
        soak_evidence,
        resource_bound_evidence,
    };

    let json_text = serde_json::to_string_pretty(&bundle).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: format!("could not encode support evidence JSON: {error}"),
        }
    })?;
    fs::write(&json_path, format!("{json_text}\n")).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: format!(
                "could not write support evidence JSON {}: {error}",
                json_path.display()
            ),
        }
    })?;

    fs::write(&markdown_path, render_support_markdown(&bundle)).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: format!(
                "could not write support evidence Markdown {}: {error}",
                markdown_path.display()
            ),
        }
    })?;

    Ok(OperatorCommandOutcome::success(render_support_outcome(
        &bundle, format,
    )?))
}

fn support_output_dir(
    args: &SupportBundleArgs,
    config_resolution: &OperatorConfigResolution,
) -> Result<PathBuf, OperatorRuntimeError> {
    if let Some(output_dir) = args.maybe_output_dir.as_ref() {
        return Ok(output_dir.clone());
    }
    let Some(data_dir) = config_resolution.maybe_data_dir.as_ref() else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "support bundle requires --output-dir when no datadir is available"
                .to_string(),
        });
    };

    Ok(data_dir.join("support"))
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct SupportEvidenceBundle {
    generated_at_unix_seconds: u64,
    generated_by: String,
    output: SupportEvidenceOutput,
    redaction: RedactionSummary,
    config: ConfigEvidence,
    status: OpenBitcoinStatusSnapshot,
    recovery_evidence: RecoverySupportEvidence,
    store_health: StoreHealthEvidence,
    live_smoke: LiveSmokeEvidence,
    full_sync_evidence: FullSyncEvidence,
    soak_evidence: SoakSupportEvidence,
    resource_bound_evidence: ResourceBoundSupportEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RecoverySupportEvidence {
    state: EvidenceState,
    category: Option<String>,
    cause: Option<String>,
    action_class: Option<String>,
    evidence_basis: Vec<String>,
    affected_namespace: Option<String>,
    affected_path: Option<String>,
    next_action: Option<String>,
    compatibility_action: Option<String>,
    maybe_unavailable_reason: Option<String>,
    source: String,
}

impl RecoverySupportEvidence {
    fn from_status(status: &FieldAvailability<RecoveryEvidenceSnapshot>) -> Self {
        match status {
            FieldAvailability::Available(evidence) => Self {
                state: EvidenceState::Available,
                category: Some(evidence.category.as_str().to_string()),
                cause: Some(serialized_label(&evidence.cause)),
                action_class: Some(serialized_label(&evidence.action_class)),
                evidence_basis: evidence
                    .evidence_basis
                    .iter()
                    .map(serialized_label)
                    .collect(),
                affected_namespace: evidence.maybe_affected_namespace.clone(),
                affected_path: evidence.maybe_affected_path.clone(),
                next_action: Some(evidence.next_action.clone()),
                compatibility_action: availability_string(&evidence.compatibility_action),
                maybe_unavailable_reason: None,
                source: SUPPORT_RECOVERY_EVIDENCE_SOURCE.to_string(),
            },
            FieldAvailability::Unavailable { reason } => Self {
                state: EvidenceState::Unavailable,
                category: None,
                cause: None,
                action_class: None,
                evidence_basis: Vec::new(),
                affected_namespace: None,
                affected_path: None,
                next_action: None,
                compatibility_action: None,
                maybe_unavailable_reason: Some(reason.clone()),
                source: SUPPORT_RECOVERY_EVIDENCE_SOURCE.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SupportEvidenceOutput {
    directory: String,
    json_path: String,
    markdown_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RedactionSummary {
    omitted: Vec<String>,
    safeguards: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ConfigEvidence {
    sources_considered: Vec<String>,
    selected_config_path: Option<String>,
    selected_bitcoin_conf_path: Option<String>,
    selected_data_dir: Option<String>,
    selected_log_dir: Option<String>,
    selected_metrics_store_path: Option<String>,
    credential_source: CredentialEvidence,
    path_reports: Vec<ConfigPathEvidence>,
}

impl ConfigEvidence {
    fn from_resolution(resolution: &OperatorConfigResolution) -> Self {
        Self {
            sources_considered: resolution
                .source_names()
                .into_iter()
                .map(str::to_string)
                .collect(),
            selected_config_path: resolution.maybe_config_path.as_deref().map(path_to_string),
            selected_bitcoin_conf_path: resolution
                .maybe_bitcoin_conf_path
                .as_deref()
                .map(path_to_string),
            selected_data_dir: resolution.maybe_data_dir.as_deref().map(path_to_string),
            selected_log_dir: resolution.maybe_log_dir.as_deref().map(path_to_string),
            selected_metrics_store_path: resolution
                .maybe_metrics_store_path
                .as_deref()
                .map(path_to_string),
            credential_source: CredentialEvidence::from_source(&resolution.credential_source),
            path_reports: resolution
                .path_reports
                .iter()
                .map(ConfigPathEvidence::from_report)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
enum CredentialEvidence {
    CookieFile { path: String, present: bool },
    UserPasswordConfigured,
    None,
}

impl CredentialEvidence {
    fn from_source(source: &OperatorCredentialSource) -> Self {
        match source {
            OperatorCredentialSource::CookieFile { path, present } => Self::CookieFile {
                path: path_to_string(path),
                present: *present,
            },
            OperatorCredentialSource::UserPasswordConfigured => Self::UserPasswordConfigured,
            OperatorCredentialSource::None => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ConfigPathEvidence {
    source: String,
    kind: String,
    path: String,
    present: bool,
}

impl ConfigPathEvidence {
    fn from_report(report: &OperatorConfigPathReport) -> Self {
        Self {
            source: report.source.as_str().to_string(),
            kind: config_path_kind_name(report.kind).to_string(),
            path: path_to_string(&report.path),
            present: report.present,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct StoreHealthEvidence {
    state: EvidenceState,
    durable_store: EvidenceAvailability,
    runtime_metadata: RuntimeMetadataEvidence,
    metrics_history: MetricsHistoryEvidence,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct RuntimeMetadataEvidence {
    availability: EvidenceAvailability,
    metadata: Option<RuntimeMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct MetricsHistoryEvidence {
    availability: EvidenceAvailability,
    samples: usize,
    status: Option<MetricsStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SoakSupportEvidence {
    state: EvidenceState,
    maybe_run_id: Option<String>,
    maybe_final_outcome: Option<String>,
    maybe_latest_sequence: Option<u64>,
    maybe_source_ledger_path: Option<String>,
    maybe_json_report_path: Option<String>,
    maybe_markdown_report_path: Option<String>,
    maybe_unavailable_reason: Option<String>,
}

impl SoakSupportEvidence {
    fn available(
        run_id: String,
        maybe_final_outcome: Option<String>,
        latest_sequence: u64,
        source_ledger_path: &Path,
        json_report_path: &Path,
        markdown_report_path: &Path,
    ) -> Self {
        Self {
            state: EvidenceState::Available,
            maybe_run_id: Some(run_id),
            maybe_final_outcome,
            maybe_latest_sequence: Some(latest_sequence),
            maybe_source_ledger_path: Some(path_to_string(source_ledger_path)),
            maybe_json_report_path: Some(path_to_string(json_report_path)),
            maybe_markdown_report_path: Some(path_to_string(markdown_report_path)),
            maybe_unavailable_reason: None,
        }
    }

    fn unavailable() -> Self {
        Self {
            state: EvidenceState::Unavailable,
            maybe_run_id: None,
            maybe_final_outcome: None,
            maybe_latest_sequence: None,
            maybe_source_ledger_path: None,
            maybe_json_report_path: None,
            maybe_markdown_report_path: None,
            maybe_unavailable_reason: Some(SOAK_LEDGER_UNAVAILABLE_REASON.to_string()),
        }
    }
}

fn collect_soak_support_evidence(
    config_resolution: &OperatorConfigResolution,
) -> SoakSupportEvidence {
    let Some(data_dir) = config_resolution.maybe_data_dir.as_ref() else {
        return SoakSupportEvidence::unavailable();
    };

    let layout = SoakLedgerLayout::for_datadir(data_dir);
    let index = match fs::read_to_string(layout.run_index_path())
        .ok()
        .and_then(|text| serde_json::from_str::<SoakRunIndex>(&text).ok())
    {
        Some(index) => index,
        None => return SoakSupportEvidence::unavailable(),
    };
    let Some(latest_run) = index.runs.first() else {
        return SoakSupportEvidence::unavailable();
    };

    let run_paths = layout.paths_for_run(&latest_run.run_id);
    if latest_run.ledger_path != run_paths.events_path {
        return SoakSupportEvidence::unavailable();
    }

    let read = match SoakLedger::read_events(&run_paths.events_path) {
        Ok(read) => read,
        Err(_) => return SoakSupportEvidence::unavailable(),
    };
    let projection =
        match SoakReportProjection::from_ledger_events(read.events, &run_paths.events_path) {
            Ok(projection) => projection,
            Err(_) => return SoakSupportEvidence::unavailable(),
        };
    let maybe_final_outcome = projection
        .verdict
        .as_ref()
        .or(projection.stop.as_ref())
        .map(|event| soak_outcome_label(event.outcome));

    SoakSupportEvidence::available(
        projection.run_id.as_str().to_string(),
        maybe_final_outcome,
        projection.latest_sequence,
        &run_paths.events_path,
        &run_paths.report_json_path,
        &run_paths.report_markdown_path,
    )
}

fn collect_store_health(status: &OpenBitcoinStatusSnapshot) -> StoreHealthEvidence {
    let runtime_metadata = runtime_metadata_evidence(status);
    let metrics_history = metrics_history_evidence(status);
    let durable_store = if runtime_metadata.availability.is_available() {
        EvidenceAvailability::available()
    } else {
        runtime_metadata.availability.clone()
    };
    let state = if durable_store.is_available() || metrics_history.availability.is_available() {
        EvidenceState::Available
    } else {
        EvidenceState::Unavailable
    };

    StoreHealthEvidence {
        state,
        durable_store,
        runtime_metadata,
        metrics_history,
    }
}

fn runtime_metadata_evidence(status: &OpenBitcoinStatusSnapshot) -> RuntimeMetadataEvidence {
    match &status.service.restart_resume {
        FieldAvailability::Available(restart_resume)
            if restart_resume_contains_durable_metadata(restart_resume) =>
        {
            RuntimeMetadataEvidence {
                availability: EvidenceAvailability::available(),
                metadata: None,
            }
        }
        FieldAvailability::Available(_) | FieldAvailability::Unavailable { .. } => {
            RuntimeMetadataEvidence {
                availability: EvidenceAvailability::unavailable(
                    SUPPORT_PROBE_ONLY_RUNTIME_METADATA_REASON,
                ),
                metadata: None,
            }
        }
    }
}

fn restart_resume_contains_durable_metadata(restart_resume: &ServiceRestartResumeStatus) -> bool {
    let ServiceRestartResumeStatus {
        prior_shutdown,
        durable_progress,
        stale_inflight,
        recovery_category,
        next_action,
        ..
    } = restart_resume;
    matches!(prior_shutdown, FieldAvailability::Available(_))
        || matches!(durable_progress, FieldAvailability::Available(_))
        || matches!(stale_inflight, FieldAvailability::Available(_))
        || matches!(recovery_category, FieldAvailability::Available(_))
        || matches!(next_action, FieldAvailability::Available(_))
}

fn metrics_history_evidence(status: &OpenBitcoinStatusSnapshot) -> MetricsHistoryEvidence {
    let availability = match &status.metrics.availability {
        open_bitcoin_node::metrics::MetricsAvailability::Available => {
            EvidenceAvailability::available()
        }
        open_bitcoin_node::metrics::MetricsAvailability::Unavailable { .. } => {
            EvidenceAvailability::unavailable(SUPPORT_PROBE_ONLY_METRICS_HISTORY_REASON)
        }
    };
    MetricsHistoryEvidence {
        availability,
        samples: status.metrics.samples.len(),
        status: Some(status.metrics.clone()),
    }
}

fn collect_live_smoke_evidence(maybe_report_path: Option<&Path>) -> LiveSmokeEvidence {
    let Some(report_path) = maybe_report_path else {
        return LiveSmokeEvidence {
            state: EvidenceState::Unavailable,
            report_path: None,
            summary: None,
            reason: Some("live smoke report not provided".to_string()),
        };
    };
    if !report_path.is_file() {
        return LiveSmokeEvidence {
            state: EvidenceState::Unavailable,
            report_path: Some(path_to_string(report_path)),
            summary: None,
            reason: Some(format!(
                "live smoke report unavailable: {} does not exist",
                report_path.display()
            )),
        };
    }

    let text = match fs::read_to_string(report_path) {
        Ok(text) => text,
        Err(error) => {
            return LiveSmokeEvidence {
                state: EvidenceState::Unavailable,
                report_path: Some(path_to_string(report_path)),
                summary: None,
                reason: Some(format!("live smoke report unreadable: {error}")),
            };
        }
    };
    let value: Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            return LiveSmokeEvidence {
                state: EvidenceState::Unavailable,
                report_path: Some(path_to_string(report_path)),
                summary: None,
                reason: Some(format!("live smoke report is not valid JSON: {error}")),
            };
        }
    };
    LiveSmokeEvidence {
        state: EvidenceState::Available,
        report_path: Some(path_to_string(report_path)),
        summary: live_smoke::summary(&value),
        reason: None,
    }
}

fn redaction_summary() -> RedactionSummary {
    RedactionSummary {
        omitted: vec![
            "RPC cookie contents".to_string(),
            "RPC password and RPC auth values".to_string(),
            "wallet private material and raw wallet files".to_string(),
            "raw unbounded log contents".to_string(),
        ],
        safeguards: vec![
            "credential sources are represented as metadata only".to_string(),
            "live smoke reports are summarized from allowlisted fields only".to_string(),
            "logs are limited to existing structured status signals".to_string(),
            "resource bounds are recorded as compact status summaries only".to_string(),
        ],
    }
}

fn config_path_kind_name(kind: OperatorConfigPathKind) -> &'static str {
    match kind {
        OperatorConfigPathKind::ConfigFile => "config_file",
        OperatorConfigPathKind::BitcoinConf => "bitcoin_conf",
        OperatorConfigPathKind::DataDir => "data_dir",
        OperatorConfigPathKind::CookieFile => "cookie_file",
        OperatorConfigPathKind::LogDirectory => "log_directory",
        OperatorConfigPathKind::MetricsStore => "metrics_store",
    }
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn path_to_string(path: &Path) -> String {
    path.display().to_string()
}

fn soak_outcome_label(outcome: SoakOutcomeLabel) -> String {
    serde_json::to_value(outcome)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

fn availability_string(value: &FieldAvailability<String>) -> Option<String> {
    match value {
        FieldAvailability::Available(value) => Some(value.clone()),
        FieldAvailability::Unavailable { .. } => None,
    }
}

fn serialized_label<T>(value: &T) -> String
where
    T: Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests;
