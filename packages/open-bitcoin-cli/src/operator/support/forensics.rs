// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Typed support-bundle forensic evidence.

use std::path::Path;

use open_bitcoin_node::core::consensus::crypto::Sha256;
use serde::Serialize;

use crate::operator::soak::{
    ledger::{
        SoakCheckpointStatus, SoakLedgerEvent, SoakLedgerEventEnvelope, SoakLedgerReadResult,
    },
    outcome::SoakOutcomeLabel,
    report::SoakReportProjection,
};

use super::{EvidenceState, RedactionSummary, SOAK_LEDGER_UNAVAILABLE_REASON, path_to_string};

const CHECKPOINT_CHAIN_ALGORITHM: &str = "sha256-json-v1";
const CHECKPOINT_CHAIN_SEED: &str = "open-bitcoin-support-forensics-v1";
const COLLECTION_FAILED_NEXT_ACTION: &str =
    "Collect a fresh support bundle after confirming the soak ledger is available.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct SupportForensicsEvidence {
    pub(super) state: EvidenceState,
    pub(super) timeline: Vec<ForensicTimelineEntry>,
    pub(super) checkpoint_chain: CheckpointChainEvidence,
    pub(super) narrative: ForensicNarrative,
    pub(super) source: ForensicSourceEvidence,
    pub(super) redaction: ForensicRedactionEvidence,
    pub(super) maybe_unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ForensicTimelineEntry {
    pub(super) sequence: u64,
    pub(super) recorded_at_unix_seconds: u64,
    pub(super) kind: String,
    pub(super) summary: String,
    pub(super) evidence_basis: Vec<String>,
    pub(super) next_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct CheckpointChainEvidence {
    pub(super) state: EvidenceState,
    pub(super) algorithm: String,
    pub(super) event_count: usize,
    pub(super) first_sequence: Option<u64>,
    pub(super) latest_sequence: Option<u64>,
    pub(super) latest_hash: Option<String>,
    pub(super) ordered: bool,
    pub(super) missing_sequence_count: usize,
    pub(super) truncated: bool,
    pub(super) maybe_unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ForensicNarrative {
    pub(super) verdict: ForensicVerdict,
    pub(super) likely_cause: String,
    pub(super) evidence_basis: Vec<String>,
    pub(super) next_action: String,
    pub(super) confidence: ForensicConfidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ForensicVerdict {
    SoakStable,
    BlockerDiagnosed,
    Inconclusive,
    CollectionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ForensicConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ForensicSourceEvidence {
    pub(super) maybe_run_id: Option<String>,
    pub(super) event_count: usize,
    pub(super) checkpoint_count: usize,
    pub(super) maybe_source_ledger_path: Option<String>,
    pub(super) maybe_json_report_path: Option<String>,
    pub(super) maybe_markdown_report_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ForensicRedactionEvidence {
    pub(super) omitted: Vec<String>,
    pub(super) safeguards: Vec<String>,
    pub(super) source: String,
}

impl SupportForensicsEvidence {
    pub(super) fn available(
        read: &SoakLedgerReadResult,
        projection: &SoakReportProjection,
        source_ledger_path: &Path,
        json_report_path: &Path,
        markdown_report_path: &Path,
        redaction: &RedactionSummary,
    ) -> Self {
        let timeline = forensic_timeline(&read.events, source_ledger_path);
        let checkpoint_chain = checkpoint_chain(read);
        let narrative = forensic_narrative(projection);
        Self {
            state: EvidenceState::Available,
            timeline,
            checkpoint_chain,
            narrative,
            source: ForensicSourceEvidence {
                maybe_run_id: Some(projection.run_id.as_str().to_string()),
                event_count: read.events.len(),
                checkpoint_count: projection.checkpoint_count,
                maybe_source_ledger_path: Some(path_to_string(source_ledger_path)),
                maybe_json_report_path: Some(path_to_string(json_report_path)),
                maybe_markdown_report_path: Some(path_to_string(markdown_report_path)),
            },
            redaction: ForensicRedactionEvidence {
                omitted: redaction.omitted.clone(),
                safeguards: redaction.safeguards.clone(),
                source: "support.redaction".to_string(),
            },
            maybe_unavailable_reason: None,
        }
    }

    pub(super) fn unavailable(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            state: EvidenceState::Unavailable,
            timeline: Vec::new(),
            checkpoint_chain: CheckpointChainEvidence::unavailable(reason.clone()),
            narrative: ForensicNarrative {
                verdict: ForensicVerdict::CollectionFailed,
                likely_cause: reason.clone(),
                evidence_basis: vec!["collection_failed".to_string()],
                next_action: COLLECTION_FAILED_NEXT_ACTION.to_string(),
                confidence: ForensicConfidence::Low,
            },
            source: ForensicSourceEvidence {
                maybe_run_id: None,
                event_count: 0,
                checkpoint_count: 0,
                maybe_source_ledger_path: None,
                maybe_json_report_path: None,
                maybe_markdown_report_path: None,
            },
            redaction: ForensicRedactionEvidence {
                omitted: Vec::new(),
                safeguards: Vec::new(),
                source: "support.redaction.unavailable".to_string(),
            },
            maybe_unavailable_reason: Some(reason),
        }
    }
}

impl CheckpointChainEvidence {
    fn unavailable(reason: String) -> Self {
        Self {
            state: EvidenceState::Unavailable,
            algorithm: CHECKPOINT_CHAIN_ALGORITHM.to_string(),
            event_count: 0,
            first_sequence: None,
            latest_sequence: None,
            latest_hash: None,
            ordered: false,
            missing_sequence_count: 0,
            truncated: false,
            maybe_unavailable_reason: Some(reason),
        }
    }
}

fn checkpoint_chain(read: &SoakLedgerReadResult) -> CheckpointChainEvidence {
    let Some(first) = read.events.first() else {
        return CheckpointChainEvidence::unavailable(SOAK_LEDGER_UNAVAILABLE_REASON.to_string());
    };

    let mut previous_hash = Sha256::digest(CHECKPOINT_CHAIN_SEED.as_bytes());
    let mut ordered = true;
    let mut missing_sequence_count = 0;
    let mut maybe_expected_next = None;

    for event in &read.events {
        if let Some(expected_next) = maybe_expected_next
            && event.sequence != expected_next
        {
            ordered = false;
            if event.sequence > expected_next {
                missing_sequence_count += event.sequence - expected_next;
            }
        }
        maybe_expected_next = Some(event.sequence.saturating_add(1));

        let mut input = Vec::from(previous_hash);
        if let Ok(serialized) = serde_json::to_vec(event) {
            input.extend(serialized);
        }
        previous_hash = Sha256::digest(&input);
    }

    // This digest is deterministic ordering/truncation evidence only. It is not
    // authenticity, signing, or an external trust root.
    CheckpointChainEvidence {
        state: EvidenceState::Available,
        algorithm: CHECKPOINT_CHAIN_ALGORITHM.to_string(),
        event_count: read.events.len(),
        first_sequence: Some(first.sequence),
        latest_sequence: read.events.last().map(|event| event.sequence),
        latest_hash: Some(hex_digest(&previous_hash)),
        ordered,
        missing_sequence_count: missing_sequence_count as usize,
        truncated: read.ignored_trailing_bytes > 0,
        maybe_unavailable_reason: None,
    }
}

fn forensic_timeline(
    events: &[SoakLedgerEventEnvelope],
    source_ledger_path: &Path,
) -> Vec<ForensicTimelineEntry> {
    events
        .iter()
        .map(|envelope| timeline_entry(envelope, source_ledger_path))
        .collect()
}

fn timeline_entry(
    envelope: &SoakLedgerEventEnvelope,
    source_ledger_path: &Path,
) -> ForensicTimelineEntry {
    let mut evidence_basis = vec![format!(
        "source_ledger={}",
        path_to_string(source_ledger_path)
    )];
    let mut next_action = None;
    let (kind, summary) = match &envelope.event {
        SoakLedgerEvent::Started { bounds } => (
            "run_start".to_string(),
            started_summary(bounds, &mut evidence_basis),
        ),
        SoakLedgerEvent::Checkpoint { status } => {
            let summary = checkpoint_summary(status, &mut evidence_basis);
            next_action = first_safe([
                status.maybe_recovery_next_action.as_deref(),
                status.maybe_stall_next_action.as_deref(),
                status.maybe_resource_bound_next_action.as_deref(),
            ]);
            ("checkpoint".to_string(), summary)
        }
        SoakLedgerEvent::Resume {
            interrupted_prior_run,
        } => (
            "resume".to_string(),
            format!("interrupted_prior_run={interrupted_prior_run}"),
        ),
        SoakLedgerEvent::Stop { outcome } => (
            "stop".to_string(),
            format!("outcome={}", outcome_label(*outcome)),
        ),
        SoakLedgerEvent::Verdict { outcome } => (
            "verdict".to_string(),
            format!("outcome={}", outcome_label(*outcome)),
        ),
    };

    ForensicTimelineEntry {
        sequence: envelope.sequence,
        recorded_at_unix_seconds: envelope.recorded_at_unix_seconds,
        kind,
        summary,
        evidence_basis,
        next_action,
    }
}

fn checkpoint_summary(status: &SoakCheckpointStatus, evidence_basis: &mut Vec<String>) -> String {
    let mut parts = Vec::new();
    push_safe_part(
        &mut parts,
        evidence_basis,
        "lifecycle",
        status.maybe_lifecycle.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "latest_stop",
        status.maybe_latest_stop_reason_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "recovery_category",
        status.maybe_recovery_category_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "recovery_cause",
        status.maybe_recovery_cause_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "no_progress",
        status.maybe_no_progress_diagnosis_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "progress_credit",
        status.maybe_progress_credit_kind_label.as_deref(),
    );
    push_optional_number(
        &mut parts,
        "progress_height",
        status.maybe_progress_credit_height,
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "last_peer_contribution",
        status.maybe_last_peer_contribution_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "stall",
        status.maybe_stalled_subsystem_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "stall_confidence",
        status.maybe_stall_confidence_label.as_deref(),
    );
    push_safe_part(
        &mut parts,
        evidence_basis,
        "resource_bound",
        status.maybe_resource_bound_state_label.as_deref(),
    );
    push_optional_number(
        &mut parts,
        "validated_active_chain_height",
        status.maybe_validated_active_chain_height,
    );
    push_optional_number(
        &mut parts,
        "best_known_tip_height",
        status.maybe_best_known_tip_height,
    );

    if parts.is_empty() {
        return "checkpoint=no_safe_diagnostic_fields".to_string();
    }
    parts.join(" ")
}

fn forensic_narrative(projection: &SoakReportProjection) -> ForensicNarrative {
    let maybe_checkpoint = projection.latest_checkpoint.as_ref();
    if let Some(narrative) = maybe_checkpoint.and_then(recovery_narrative) {
        return narrative;
    }
    if let Some(narrative) = maybe_checkpoint.and_then(resource_narrative) {
        return narrative;
    }
    if let Some(narrative) = maybe_checkpoint.and_then(stall_narrative) {
        return narrative;
    }
    if let Some(narrative) = maybe_checkpoint.and_then(no_progress_narrative) {
        return narrative;
    }

    let maybe_outcome = projection
        .verdict
        .as_ref()
        .or(projection.stop.as_ref())
        .map(|event| event.outcome);
    match maybe_outcome {
        Some(SoakOutcomeLabel::CleanCompletion)
            if maybe_checkpoint.is_some_and(has_stability_evidence) =>
        {
            ForensicNarrative {
                verdict: ForensicVerdict::SoakStable,
                likely_cause: "soak completed with validated progress evidence".to_string(),
                evidence_basis: vec!["outcome=clean_completion".to_string()],
                next_action: "Archive the support bundle as soak stability evidence.".to_string(),
                confidence: ForensicConfidence::High,
            }
        }
        Some(
            outcome @ (SoakOutcomeLabel::DiagnosedBlocker
            | SoakOutcomeLabel::ResourceStop
            | SoakOutcomeLabel::RecoveryStop),
        ) => {
            let outcome = outcome_label(outcome);
            ForensicNarrative {
                verdict: ForensicVerdict::BlockerDiagnosed,
                likely_cause: format!("outcome={outcome}"),
                evidence_basis: vec![format!("outcome={outcome}")],
                next_action:
                    "Inspect the typed recovery, resource, and stall evidence before retrying."
                        .to_string(),
                confidence: ForensicConfidence::Medium,
            }
        }
        Some(outcome) => ForensicNarrative {
            verdict: ForensicVerdict::Inconclusive,
            likely_cause: format!(
                "outcome={} without diagnostic proof",
                outcome_label(outcome)
            ),
            evidence_basis: vec![format!("outcome={}", outcome_label(outcome))],
            next_action: "Collect another support bundle after the next checkpoint or failure."
                .to_string(),
            confidence: ForensicConfidence::Low,
        },
        None => ForensicNarrative {
            verdict: ForensicVerdict::Inconclusive,
            likely_cause: "final outcome unavailable".to_string(),
            evidence_basis: vec!["missing_final_outcome".to_string()],
            next_action: "Collect a support bundle after the soak writes a stop or verdict event."
                .to_string(),
            confidence: ForensicConfidence::Low,
        },
    }
}

fn recovery_narrative(status: &SoakCheckpointStatus) -> Option<ForensicNarrative> {
    let maybe_cause = first_safe([
        status.maybe_recovery_cause_label.as_deref(),
        status.maybe_recovery_category_label.as_deref(),
        status.maybe_recovery_action_class_label.as_deref(),
    ]);
    let cause = maybe_cause?;
    Some(ForensicNarrative {
        verdict: ForensicVerdict::BlockerDiagnosed,
        likely_cause: format!("recovery={cause}"),
        evidence_basis: vec![format!("recovery={cause}")],
        next_action: first_safe([status.maybe_recovery_next_action.as_deref()])
            .unwrap_or_else(|| "Inspect recovery evidence before retrying.".to_string()),
        confidence: ForensicConfidence::High,
    })
}

fn resource_narrative(status: &SoakCheckpointStatus) -> Option<ForensicNarrative> {
    let state = first_safe([status.maybe_resource_bound_state_label.as_deref()])?;
    if state == "normal" {
        return None;
    }
    let mut basis = vec![format!("resource_bound={state}")];
    for label in &status.resource_bound_labels {
        if let Some(label) = safe_value(label) {
            basis.push(format!("resource_bound_label={label}"));
        }
    }
    Some(ForensicNarrative {
        verdict: ForensicVerdict::BlockerDiagnosed,
        likely_cause: format!("resource_bound={state}"),
        evidence_basis: basis,
        next_action: first_safe([status.maybe_resource_bound_next_action.as_deref()])
            .unwrap_or_else(|| "Inspect bounded resource evidence before retrying.".to_string()),
        confidence: ForensicConfidence::High,
    })
}

fn stall_narrative(status: &SoakCheckpointStatus) -> Option<ForensicNarrative> {
    let stalled_subsystem = first_safe([status.maybe_stalled_subsystem_label.as_deref()])?;
    if stalled_subsystem == "at_tip_waiting" {
        return None;
    }
    let mut basis = vec![format!("stall={stalled_subsystem}")];
    if let Some(confidence) = first_safe([status.maybe_stall_confidence_label.as_deref()]) {
        basis.push(format!("stall_confidence={confidence}"));
    }
    for item in &status.stall_evidence_basis {
        if let Some(item) = safe_value(item) {
            basis.push(format!("stall_basis={item}"));
        }
    }
    Some(ForensicNarrative {
        verdict: ForensicVerdict::BlockerDiagnosed,
        likely_cause: format!("stall={stalled_subsystem}"),
        evidence_basis: basis,
        next_action: first_safe([status.maybe_stall_next_action.as_deref()])
            .unwrap_or_else(|| "Inspect stall diagnosis evidence before retrying.".to_string()),
        confidence: ForensicConfidence::Medium,
    })
}

fn no_progress_narrative(status: &SoakCheckpointStatus) -> Option<ForensicNarrative> {
    let diagnosis = first_safe([status.maybe_no_progress_diagnosis_label.as_deref()])?;
    if diagnosis == "current_at_best_known_tip" {
        return None;
    }
    Some(ForensicNarrative {
        verdict: ForensicVerdict::BlockerDiagnosed,
        likely_cause: format!("no_progress={diagnosis}"),
        evidence_basis: vec![format!("no_progress={diagnosis}")],
        next_action: "Inspect no-progress diagnosis evidence before retrying.".to_string(),
        confidence: ForensicConfidence::Medium,
    })
}

fn has_stability_evidence(status: &SoakCheckpointStatus) -> bool {
    status.maybe_progress_credit_kind_label.is_some()
        || status.maybe_last_useful_work_kind_label.is_some()
        || status.maybe_validated_active_chain_height == status.maybe_best_known_tip_height
            && status.maybe_validated_active_chain_height.is_some()
}

fn started_summary(
    bounds: &crate::operator::soak::SoakBounds,
    evidence_basis: &mut Vec<String>,
) -> String {
    let mut parts = Vec::new();
    push_safe_part(
        &mut parts,
        evidence_basis,
        "network",
        Some(bounds.network.as_str()),
    );
    parts.push(format!("elapsed_seconds={}", bounds.elapsed_time_seconds));
    parts.push(format!(
        "checkpoint_interval_seconds={}",
        bounds.checkpoint_interval_seconds
    ));
    if let Some(target_height) = bounds.maybe_target_height {
        parts.push(format!("target_height={target_height}"));
    }
    parts.join(" ")
}

fn push_safe_part(
    parts: &mut Vec<String>,
    evidence_basis: &mut Vec<String>,
    key: &str,
    maybe_value: Option<&str>,
) {
    let Some(value) = maybe_value else {
        return;
    };
    if let Some(value) = safe_value(value) {
        parts.push(format!("{key}={value}"));
        evidence_basis.push(format!("{key}={value}"));
        return;
    }
    evidence_basis.push("redacted_sensitive_field".to_string());
}

fn push_optional_number(parts: &mut Vec<String>, key: &str, maybe_value: Option<u64>) {
    if let Some(value) = maybe_value {
        parts.push(format!("{key}={value}"));
    }
}

fn first_safe<'a>(values: impl IntoIterator<Item = Option<&'a str>>) -> Option<String> {
    values.into_iter().flatten().find_map(safe_value)
}

fn safe_value(value: &str) -> Option<String> {
    if contains_sensitive_marker(value) {
        return None;
    }
    Some(value.to_string())
}

fn contains_sensitive_marker(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    [
        "secret",
        concat!("rpc", "password"),
        concat!("rpc", "auth"),
        "cookie contents",
        "wallet",
        "credential",
        "raw daemon",
        "raw live-smoke",
        "raw options",
        "endpoint table",
        "unbounded peer table",
        "peer tables",
    ]
    .iter()
    .any(|marker| value.contains(marker))
}

fn outcome_label(outcome: SoakOutcomeLabel) -> String {
    serde_json::to_value(outcome)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

fn hex_digest(bytes: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
