// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Rendering helpers for support bundle command output.

use serde::Serialize;
use serde_json::{Value, json};

use crate::operator::{OperatorOutputFormat, runtime::OperatorRuntimeError};

use super::{EvidenceAvailability, EvidenceState, SupportEvidenceBundle};

pub(super) fn render_support_outcome(
    bundle: &SupportEvidenceBundle,
    format: OperatorOutputFormat,
) -> Result<String, OperatorRuntimeError> {
    match format {
        OperatorOutputFormat::Human => Ok(format!(
            "Support evidence written:\nJSON: {}\nMarkdown: {}\n",
            bundle.output.json_path, bundle.output.markdown_path
        )),
        OperatorOutputFormat::Json => {
            let output = json!({
                "json_path": bundle.output.json_path,
                "markdown_path": bundle.output.markdown_path,
                "generated_at_unix_seconds": bundle.generated_at_unix_seconds,
                "redaction": bundle.redaction,
            });
            serde_json::to_string_pretty(&output)
                .map(|text| format!("{text}\n"))
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: format!("could not encode support command output: {error}"),
                })
        }
    }
}

pub(super) fn render_support_markdown(bundle: &SupportEvidenceBundle) -> String {
    let mut output = String::new();
    output.push_str("# Open Bitcoin Support Evidence\n\n");
    output.push_str(&format!(
        "- Generated: {}\n",
        bundle.generated_at_unix_seconds
    ));
    output.push_str(&format!("- JSON: {}\n", bundle.output.json_path));
    output.push_str(&format!("- Markdown: {}\n\n", bundle.output.markdown_path));

    output.push_str("## Redaction\n\n");
    for item in &bundle.redaction.omitted {
        output.push_str(&format!("- Omitted: {item}\n"));
    }
    for item in &bundle.redaction.safeguards {
        output.push_str(&format!("- Safeguard: {item}\n"));
    }

    output.push_str("\n## Config\n\n");
    push_optional_path(
        &mut output,
        "Datadir",
        bundle.config.selected_data_dir.as_deref(),
    );
    push_optional_path(
        &mut output,
        "Open Bitcoin config",
        bundle.config.selected_config_path.as_deref(),
    );
    push_optional_path(
        &mut output,
        "Bitcoin config",
        bundle.config.selected_bitcoin_conf_path.as_deref(),
    );
    push_optional_path(
        &mut output,
        "Logs",
        bundle.config.selected_log_dir.as_deref(),
    );
    push_optional_path(
        &mut output,
        "Metrics",
        bundle.config.selected_metrics_store_path.as_deref(),
    );

    output.push_str("\n## Status Snapshot\n\n");
    output.push_str(&format!(
        "- Node state: {}\n",
        json_string(&bundle.status.node.state)
    ));
    output.push_str(&format!("- Version: {}\n", bundle.status.node.version));
    output.push_str(&format!(
        "- Health signals: {}\n",
        bundle.status.health_signals.len()
    ));
    output.push_str(&format!(
        "- Metrics samples: {}\n",
        bundle.status.metrics.samples.len()
    ));
    output.push_str(&format!(
        "- Service lifecycle: {}\n",
        json_compact(&bundle.status.service.lifecycle)
    ));
    output.push_str(&format!(
        "- Service restart/resume: {}\n",
        json_compact(&bundle.status.service.restart_resume)
    ));
    output.push_str(&format!(
        "- Logs path: {}\n",
        json_compact(&bundle.status.logs.path)
    ));
    output.push_str(&format!(
        "- Metrics availability: {}\n",
        json_compact(&bundle.status.metrics.availability)
    ));

    output.push_str("\n## Store Health\n\n");
    output.push_str(&format!(
        "- Overall: {}\n",
        evidence_state_name(bundle.store_health.state)
    ));
    output.push_str(&format!(
        "- Runtime metadata: {}\n",
        availability_name(&bundle.store_health.runtime_metadata.availability)
    ));
    output.push_str(&format!(
        "- Metrics history: {}\n",
        availability_name(&bundle.store_health.metrics_history.availability)
    ));

    output.push_str("\n## Full Sync Evidence\n\n");
    push_full_sync_evidence(&mut output, &bundle.full_sync_evidence);

    output.push_str("\n## Live Smoke\n\n");
    output.push_str(&format!(
        "- State: {}\n",
        evidence_state_name(bundle.live_smoke.state)
    ));
    if let Some(report_path) = bundle.live_smoke.report_path.as_ref() {
        output.push_str(&format!("- Report: {report_path}\n"));
    }
    if let Some(reason) = bundle.live_smoke.reason.as_ref() {
        output.push_str(&format!("- Reason: {reason}\n"));
    }
    if let Some(summary) = bundle.live_smoke.summary.as_ref() {
        push_live_smoke_summary(&mut output, summary);
    }

    output
}

fn push_full_sync_evidence(output: &mut String, evidence: &super::FullSyncEvidence) {
    output.push_str(&format!(
        "- Evidence verdict: {}\n",
        json_string(&evidence.verdict.label)
    ));
    output.push_str(&format!(
        "- Connected active chain: {}\n",
        active_chain_summary(&evidence.connected_active_chain)
    ));
    output.push_str(&format!(
        "- Validated active chain: {}\n",
        active_chain_summary(&evidence.validated_active_chain)
    ));
    output.push_str(&format!(
        "- Restart/resume checkpoints: {}\n",
        summary_evidence_text(&evidence.restart_resume_checkpoints)
    ));
    output.push_str(&format!(
        "- Stay-current window: {}\n",
        summary_evidence_text(&evidence.stay_current_window)
    ));
    output.push_str(&format!(
        "- Peer contribution: {}\n",
        summary_evidence_text(&evidence.peer_contribution)
    ));
    output.push_str(&format!(
        "- No-progress or reorg events: {}\n",
        summary_evidence_text(&evidence.no_progress_or_reorg_events)
    ));
    output.push_str(&format!(
        "- Resource pressure: {}\n",
        summary_evidence_text(&evidence.resource_pressure)
    ));
    output.push_str(&format!(
        "- Recovery: {}\n",
        summary_evidence_text(&evidence.recovery)
    ));
    output.push_str(&format!(
        "- Verdict justifications: {}\n",
        evidence.verdict.justifications.join(", ")
    ));
}

fn active_chain_summary(evidence: &super::ActiveChainEvidence) -> String {
    let base = format!(
        "height={} hash={} work={}",
        evidence
            .height
            .map(|value| value.to_string())
            .unwrap_or_else(|| "Unavailable".to_string()),
        evidence.hash.as_deref().unwrap_or("Unavailable"),
        evidence.work.as_deref().unwrap_or("Unavailable")
    );
    match evidence.maybe_unavailable_reason.as_ref() {
        Some(reason) => format!("{base}; Unavailable: {reason}"),
        None => base,
    }
}

fn summary_evidence_text(evidence: &super::SummaryEvidence) -> String {
    if let Some(reason) = evidence.maybe_unavailable_reason.as_ref() {
        return format!("Unavailable: {reason}");
    }

    evidence
        .summary
        .clone()
        .unwrap_or_else(|| "unavailable".to_string())
}

fn push_live_smoke_summary(output: &mut String, summary: &Value) {
    let Some(object) = summary.as_object() else {
        output.push_str(&format!("- Summary: {summary}\n"));
        return;
    };

    push_summary_field(output, "Status", object.get("status"));
    push_summary_field(output, "Progress detected", object.get("progressDetected"));
    push_summary_field(
        output,
        "No-progress cause",
        object
            .get("maybeNoProgressCause")
            .or_else(|| object.get("maybe_no_progress_cause")),
    );
    push_summary_field(
        output,
        "Next action",
        object
            .get("nextAction")
            .or_else(|| object.get("next_action")),
    );
    push_summary_field(output, "Header delta", object.get("headerDelta"));
    push_summary_field(output, "Block delta", object.get("blockDelta"));
    push_summary_field(
        output,
        "First header progress",
        object.get("firstHeaderProgress"),
    );
    push_summary_field(
        output,
        "First block progress",
        object.get("firstBlockProgress"),
    );
    if let Some(restart_resume_evidence) = object.get("restartResumeEvidence") {
        push_summary_field(
            output,
            "Restart/resume evidence",
            Some(restart_resume_evidence),
        );
        push_summary_field(
            output,
            "Recovery diagnosis",
            restart_resume_evidence.get("recoveryDiagnosis"),
        );
    }
    if let Some(final_status) = object.get("finalStatus") {
        push_summary_field(output, "Final status", Some(final_status));
        push_summary_field(
            output,
            "Recovery category",
            final_status.get("recoveryCategory"),
        );
        push_summary_field(
            output,
            "Resource pressure",
            final_status.get("resourcePressure"),
        );
    }
}

fn push_summary_field(output: &mut String, label: &str, maybe_value: Option<&Value>) {
    let Some(value) = maybe_value else {
        return;
    };

    output.push_str(&format!("- {label}: {}\n", summary_value(value)));
}

fn summary_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Object(_) => {
            value.to_string()
        }
    }
}

fn push_optional_path(output: &mut String, label: &str, maybe_path: Option<&str>) {
    let path = maybe_path.unwrap_or("unavailable");
    output.push_str(&format!("- {label}: {path}\n"));
}

fn json_string<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

fn json_compact<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .map(|value| value.to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

fn availability_name(availability: &EvidenceAvailability) -> &'static str {
    evidence_state_name(availability.state)
}

const fn evidence_state_name(state: EvidenceState) -> &'static str {
    match state {
        EvidenceState::Available => "available",
        EvidenceState::Unavailable => "unavailable",
    }
}
