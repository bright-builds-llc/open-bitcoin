// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Rendering helpers for support bundle command output.

mod block_relay;
mod inbound;
mod relay;

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

    push_config_evidence(&mut output, &bundle.config);

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

    relay::push_relay_mempool_evidence(&mut output, &bundle.status.mempool);
    block_relay::push_block_relay_evidence(&mut output, &bundle.status.block_relay);
    inbound::push_inbound_serving(&mut output, &bundle.status.peers.inbound);
    output.push_str("\n## Recovery Evidence\n\n");
    push_recovery_evidence(&mut output, &bundle.recovery_evidence);

    output.push_str("\n## Resource Bound Evidence\n\n");
    push_resource_bound_evidence(&mut output, &bundle.resource_bound_evidence);

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

    output.push_str("\n## Soak Evidence\n\n");
    push_soak_evidence(&mut output, &bundle.soak_evidence);

    push_support_forensics(&mut output, &bundle.support_forensics);

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

fn push_recovery_evidence(output: &mut String, evidence: &super::RecoverySupportEvidence) {
    match evidence.state {
        EvidenceState::Available => {
            output.push_str(&format!(
                "- Category: {}\n",
                evidence.category.as_deref().unwrap_or("unavailable")
            ));
            output.push_str(&format!(
                "- Cause: {}\n",
                evidence.cause.as_deref().unwrap_or("unavailable")
            ));
            output.push_str(&format!(
                "- Action class: {}\n",
                evidence.action_class.as_deref().unwrap_or("unavailable")
            ));
            output.push_str(&format!(
                "- Evidence basis: {}\n",
                evidence.evidence_basis.join(", ")
            ));
            output.push_str(&format!(
                "- Affected namespace: {}\n",
                evidence
                    .affected_namespace
                    .as_deref()
                    .unwrap_or("unavailable")
            ));
            output.push_str(&format!(
                "- Affected path: {}\n",
                evidence.affected_path.as_deref().unwrap_or("unavailable")
            ));
            output.push_str(&format!(
                "- Next action: {}\n",
                evidence.next_action.as_deref().unwrap_or("unavailable")
            ));
            output.push_str(&format!(
                "- Compatibility action: {}\n",
                evidence
                    .compatibility_action
                    .as_deref()
                    .unwrap_or("unavailable")
            ));
        }
        EvidenceState::Unavailable => {
            let reason = evidence
                .maybe_unavailable_reason
                .as_deref()
                .unwrap_or("recovery evidence unavailable");
            output.push_str(&format!("- Status: Unavailable: {reason}\n"));
        }
    }
    output.push_str(&format!("- Source: {}\n", evidence.source));
}

fn push_soak_evidence(output: &mut String, evidence: &super::SoakSupportEvidence) {
    output.push_str(&format!(
        "- State: {}\n",
        evidence_state_name(evidence.state)
    ));
    output.push_str(&format!(
        "- Run: {}\n",
        evidence.maybe_run_id.as_deref().unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- Final outcome: {}\n",
        evidence
            .maybe_final_outcome
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- Source ledger: {}\n",
        evidence
            .maybe_source_ledger_path
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- JSON report: {}\n",
        evidence
            .maybe_json_report_path
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- Markdown report: {}\n",
        evidence
            .maybe_markdown_report_path
            .as_deref()
            .unwrap_or("unavailable")
    ));
    let maybe_latest_sequence = evidence
        .maybe_latest_sequence
        .map(|sequence| sequence.to_string());
    output.push_str(&format!(
        "- Latest sequence: {}\n",
        maybe_latest_sequence.as_deref().unwrap_or("unavailable")
    ));
    if let Some(reason) = evidence.maybe_unavailable_reason.as_ref() {
        output.push_str(&format!("- Reason: {reason}\n"));
    }
}

fn push_support_forensics(
    output: &mut String,
    evidence: &super::forensics::SupportForensicsEvidence,
) {
    output.push_str("\n## Forensic Timeline\n\n");
    output.push_str(&format!(
        "- State: {}\n",
        evidence_state_name(evidence.state)
    ));
    if let Some(reason) = evidence.maybe_unavailable_reason.as_ref() {
        output.push_str(&format!("- Reason: {reason}\n"));
    }
    for entry in &evidence.timeline {
        let basis = csv_or_unavailable(&entry.evidence_basis);
        let next_action = entry.next_action.as_deref().unwrap_or("unavailable");
        output.push_str(&format!(
            "- Sequence {}: kind={} recorded_at={} summary={} basis={} next_action={}\n",
            entry.sequence,
            entry.kind,
            entry.recorded_at_unix_seconds,
            entry.summary,
            basis,
            next_action
        ));
    }

    let checkpoint = &evidence.checkpoint_chain;
    let first_sequence = checkpoint
        .first_sequence
        .map(|sequence| sequence.to_string())
        .unwrap_or_else(|| "unavailable".to_string());
    let latest_sequence = checkpoint
        .latest_sequence
        .map(|sequence| sequence.to_string())
        .unwrap_or_else(|| "unavailable".to_string());
    output.push_str("\n## Checkpoint Chain\n\n");
    output.push_str(&format!(
        "- State: {}\n",
        evidence_state_name(checkpoint.state)
    ));
    output.push_str(&format!("- Algorithm: {}\n", checkpoint.algorithm));
    output.push_str(&format!("- Event count: {}\n", checkpoint.event_count));
    output.push_str(&format!("- First sequence: {first_sequence}\n"));
    output.push_str(&format!("- Latest sequence: {latest_sequence}\n"));
    output.push_str(&format!(
        "- Latest hash: {}\n",
        checkpoint.latest_hash.as_deref().unwrap_or("unavailable")
    ));
    output.push_str(&format!("- Ordered: {}\n", checkpoint.ordered));
    output.push_str(&format!(
        "- Missing sequence count: {}\n",
        checkpoint.missing_sequence_count
    ));
    output.push_str(&format!("- Truncated: {}\n", checkpoint.truncated));
    if let Some(reason) = checkpoint.maybe_unavailable_reason.as_ref() {
        output.push_str(&format!("- Reason: {reason}\n"));
    }

    let narrative = &evidence.narrative;
    output.push_str("\n## Failure Narrative\n\n");
    output.push_str(&format!("- Verdict: {}\n", json_string(&narrative.verdict)));
    output.push_str(&format!("- Likely cause: {}\n", narrative.likely_cause));
    output.push_str(&format!(
        "- Evidence basis: {}\n",
        csv_or_unavailable(&narrative.evidence_basis)
    ));
    output.push_str(&format!("- Next action: {}\n", narrative.next_action));
    output.push_str(&format!(
        "- Confidence: {}\n",
        json_string(&narrative.confidence)
    ));
    output.push_str(&format!(
        "- Run: {}\n",
        evidence
            .source
            .maybe_run_id
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- Source event count: {}\n",
        evidence.source.event_count
    ));
    output.push_str(&format!(
        "- Source checkpoint count: {}\n",
        evidence.source.checkpoint_count
    ));
    output.push_str(&format!(
        "- Source ledger: {}\n",
        evidence
            .source
            .maybe_source_ledger_path
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- JSON report: {}\n",
        evidence
            .source
            .maybe_json_report_path
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- Markdown report: {}\n",
        evidence
            .source
            .maybe_markdown_report_path
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!(
        "- Redaction omitted: {}\n",
        csv_or_unavailable(&evidence.redaction.omitted)
    ));
    output.push_str(&format!(
        "- Redaction safeguards: {}\n",
        csv_or_unavailable(&evidence.redaction.safeguards)
    ));
    output.push_str(&format!(
        "- Redaction source: {}\n",
        evidence.redaction.source
    ));
}

fn push_resource_bound_evidence(
    output: &mut String,
    evidence: &super::ResourceBoundSupportEvidence,
) {
    output.push_str(&format!(
        "- State: {}\n",
        evidence_state_name(evidence.state)
    ));
    output.push_str(&format!(
        "- Overall: {}\n",
        evidence
            .maybe_overall_level
            .as_deref()
            .unwrap_or("unavailable")
    ));
    output.push_str(&format!("- Source: {}\n", evidence.source));
    let maybe_projected_bundle_size = evidence
        .maybe_projected_bundle_size_bytes
        .map(|size| size.to_string());
    output.push_str(&format!(
        "- Projected support-bundle size: {} bytes\n",
        maybe_projected_bundle_size
            .as_deref()
            .unwrap_or("unavailable")
    ));
    if let Some(reason) = evidence.maybe_unavailable_reason.as_ref() {
        output.push_str(&format!("- Reason: {reason}\n"));
    }
    for entry in &evidence.entries {
        let current = entry
            .current
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unavailable".to_string());
        let limit = entry
            .limit
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unavailable".to_string());
        let unit = entry.unit.as_deref().unwrap_or("unavailable");
        let next_action = entry.next_action.as_deref().unwrap_or("unavailable");
        output.push_str(&format!(
            "- {}: state={} current={} limit={} unit={} next_action={}\n",
            entry.kind, entry.state, current, limit, unit, next_action
        ));
        if let Some(reason) = entry.maybe_unavailable_reason.as_ref() {
            output.push_str(&format!("  unavailable_reason={reason}\n"));
        }
    }
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
        "- Progress guarantee: {}\n",
        summary_evidence_text(&evidence.progress_guarantee)
    ));
    output.push_str(&format!(
        "- Stall diagnosis: {}\n",
        summary_evidence_text(&evidence.stall_diagnosis)
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

fn push_config_evidence(output: &mut String, config: &super::ConfigEvidence) {
    output.push_str("\n## Config\n\n");
    for (label, maybe_path) in [
        ("Datadir", config.selected_data_dir.as_deref()),
        (
            "Open Bitcoin config",
            config.selected_config_path.as_deref(),
        ),
        (
            "Bitcoin config",
            config.selected_bitcoin_conf_path.as_deref(),
        ),
        ("Logs", config.selected_log_dir.as_deref()),
        ("Metrics", config.selected_metrics_store_path.as_deref()),
    ] {
        push_optional_path(output, label, maybe_path);
    }
}

fn push_optional_path(output: &mut String, label: &str, maybe_path: Option<&str>) {
    let path = maybe_path.unwrap_or("unavailable");
    output.push_str(&format!("- {label}: {path}\n"));
}

fn csv_or_unavailable(values: &[String]) -> String {
    if values.is_empty() {
        return "unavailable".to_string();
    }
    values.join(", ")
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
