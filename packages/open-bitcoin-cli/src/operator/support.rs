// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Local support evidence bundle generation.

mod render;

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    FjallNodeStore, MetricRetentionPolicy, MetricsStatus, OpenBitcoinStatusSnapshot,
    RuntimeMetadata, metrics::MetricsAvailability,
};
use serde::Serialize;
use serde_json::{Map, Value, json};

use render::{render_support_markdown, render_support_outcome};

use super::{
    OperatorOutputFormat, SupportArgs, SupportBundleArgs, SupportCommand,
    config::{
        OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigResolution,
        OperatorCredentialSource,
    },
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
};

const SUPPORT_EVIDENCE_JSON: &str = "support-evidence.json";
const SUPPORT_EVIDENCE_MARKDOWN: &str = "support-evidence.md";
const LIVE_SMOKE_SUMMARY_KEYS: &[&str] = &[
    "status",
    "maybeNoProgressCause",
    "maybe_no_progress_cause",
    "nextAction",
    "next_action",
    "reportPath",
    "report_path",
    "markdownPath",
    "markdown_path",
    "startedAtUnixSeconds",
    "started_at_unix_seconds",
    "finishedAtUnixSeconds",
    "finished_at_unix_seconds",
    "timeoutSeconds",
    "timeout_seconds",
    "pollSeconds",
    "poll_seconds",
    "manualPeers",
    "manual_peers",
    "generatedConfigPath",
    "generated_config_path",
];

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
        store_health: collect_store_health(config_resolution),
        live_smoke: collect_live_smoke_evidence(args.maybe_live_smoke_report.as_deref()),
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
    store_health: StoreHealthEvidence,
    live_smoke: LiveSmokeEvidence,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
struct LiveSmokeEvidence {
    state: EvidenceState,
    report_path: Option<String>,
    summary: Option<Value>,
    reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum EvidenceState {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct EvidenceAvailability {
    state: EvidenceState,
    reason: Option<String>,
}

impl EvidenceAvailability {
    const fn available() -> Self {
        Self {
            state: EvidenceState::Available,
            reason: None,
        }
    }

    fn unavailable(reason: impl Into<String>) -> Self {
        Self {
            state: EvidenceState::Unavailable,
            reason: Some(reason.into()),
        }
    }

    const fn is_available(&self) -> bool {
        matches!(self.state, EvidenceState::Available)
    }
}

fn collect_store_health(resolution: &OperatorConfigResolution) -> StoreHealthEvidence {
    let runtime_metadata = collect_runtime_metadata(resolution.maybe_data_dir.as_deref());
    let metrics_history = collect_metrics_history(resolution.maybe_metrics_store_path.as_deref());
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

fn collect_runtime_metadata(maybe_data_dir: Option<&Path>) -> RuntimeMetadataEvidence {
    let Some(data_dir) = maybe_data_dir else {
        return RuntimeMetadataEvidence {
            availability: EvidenceAvailability::unavailable("datadir unavailable"),
            metadata: None,
        };
    };
    if !data_dir.is_dir() {
        return RuntimeMetadataEvidence {
            availability: EvidenceAvailability::unavailable(format!(
                "durable store unavailable: {} does not exist",
                data_dir.display()
            )),
            metadata: None,
        };
    }

    let store = match FjallNodeStore::open(data_dir) {
        Ok(store) => store,
        Err(error) => {
            return RuntimeMetadataEvidence {
                availability: EvidenceAvailability::unavailable(format!(
                    "durable store unavailable: {error}"
                )),
                metadata: None,
            };
        }
    };
    match store.load_runtime_metadata() {
        Ok(Some(metadata)) => RuntimeMetadataEvidence {
            availability: EvidenceAvailability::available(),
            metadata: Some(metadata),
        },
        Ok(None) => RuntimeMetadataEvidence {
            availability: EvidenceAvailability::unavailable(
                "runtime metadata unavailable: no metadata recorded",
            ),
            metadata: None,
        },
        Err(error) => RuntimeMetadataEvidence {
            availability: EvidenceAvailability::unavailable(format!(
                "runtime metadata unavailable: {error}"
            )),
            metadata: None,
        },
    }
}

fn collect_metrics_history(maybe_metrics_path: Option<&Path>) -> MetricsHistoryEvidence {
    let Some(metrics_path) = maybe_metrics_path else {
        return MetricsHistoryEvidence {
            availability: EvidenceAvailability::unavailable("metrics store path unavailable"),
            samples: 0,
            status: None,
        };
    };
    if !metrics_path.is_dir() {
        return MetricsHistoryEvidence {
            availability: EvidenceAvailability::unavailable(format!(
                "metrics history unavailable: {} does not exist",
                metrics_path.display()
            )),
            samples: 0,
            status: None,
        };
    }

    let store = match FjallNodeStore::open(metrics_path) {
        Ok(store) => store,
        Err(error) => {
            return MetricsHistoryEvidence {
                availability: EvidenceAvailability::unavailable(format!(
                    "metrics history unavailable: {error}"
                )),
                samples: 0,
                status: None,
            };
        }
    };
    match store.load_metrics_status(MetricRetentionPolicy::default()) {
        Ok(status) => {
            let samples = status.samples.len();
            let availability = match &status.availability {
                MetricsAvailability::Available => EvidenceAvailability::available(),
                MetricsAvailability::Unavailable { reason } => {
                    EvidenceAvailability::unavailable(reason.clone())
                }
            };
            MetricsHistoryEvidence {
                availability,
                samples,
                status: Some(status),
            }
        }
        Err(error) => MetricsHistoryEvidence {
            availability: EvidenceAvailability::unavailable(format!(
                "metrics history unavailable: {error}"
            )),
            samples: 0,
            status: None,
        },
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
        summary: live_smoke_summary(&value),
        reason: None,
    }
}

fn live_smoke_summary(value: &Value) -> Option<Value> {
    let object = value.as_object()?;
    let mut summary = Map::new();
    for key in LIVE_SMOKE_SUMMARY_KEYS {
        if let Some(item) = object.get(*key) {
            summary.insert((*key).to_string(), sanitize_json_value(item));
        }
    }
    if summary.is_empty() {
        return Some(json!({
            "status": "summary_fields_unavailable"
        }));
    }

    Some(Value::Object(summary))
}

fn sanitize_json_value(value: &Value) -> Value {
    match value {
        Value::String(text) => Value::String(redact_sensitive_text(text)),
        Value::Array(items) => Value::Array(items.iter().map(sanitize_json_value).collect()),
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), sanitize_json_value(value)))
                .collect(),
        ),
        Value::Null | Value::Bool(_) | Value::Number(_) => value.clone(),
    }
}

fn redact_sensitive_text(text: &str) -> String {
    let lowercase = text.to_ascii_lowercase();
    if lowercase.contains("rpcpassword")
        || lowercase.contains("rpcauth")
        || lowercase.contains("__cookie__")
        || lowercase.contains("private_key")
        || lowercase.contains("xprv")
        || lowercase.contains("seed phrase")
    {
        return "[redacted]".to_string();
    }

    text.to_string()
}

fn redaction_summary() -> RedactionSummary {
    RedactionSummary {
        omitted: vec![
            "RPC cookie contents".to_string(),
            "rpcpassword and rpcauth values".to_string(),
            "wallet private material and raw wallet files".to_string(),
            "raw unbounded log contents".to_string(),
        ],
        safeguards: vec![
            "credential sources are represented as metadata only".to_string(),
            "live smoke reports are summarized from allowlisted fields only".to_string(),
            "logs are limited to existing structured status signals".to_string(),
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
