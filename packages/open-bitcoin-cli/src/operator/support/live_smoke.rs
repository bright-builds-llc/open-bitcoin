// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Allowlisted live-smoke support summary projection.

use serde_json::{Map, Value};

const RESULT_SUMMARY_KEYS: &[&str] = &[
    "status",
    "progressDetected",
    "maybeNoProgressCause",
    "nextAction",
    "headerDelta",
    "blockDelta",
];
const TOP_LEVEL_SUMMARY_KEYS: &[&str] = &[
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
    "generatedConfigPath",
    "generated_config_path",
    "recoveryActionClass",
    "recovery_action_class",
    "recoveryCause",
    "recovery_cause",
    "recoveryNextAction",
    "recovery_next_action",
    "maybeRecoveryEvidenceUnavailableReason",
    "maybe_recovery_evidence_unavailable_reason",
];
const FIRST_HEADER_DIRECT_KEYS: &[&str] = &[
    "observedAtUnixSeconds",
    "maybePeer",
    "maybeSource",
    "maybeResolvedEndpoint",
];
const HEADER_HEIGHT_KEYS: &[&str] = &["headerHeight"];
const FIRST_BLOCK_DIRECT_KEYS: &[&str] = &[
    "kind",
    "height",
    "blockHash",
    "observedAtUnixSeconds",
    "maybePeer",
    "maybeSource",
    "maybeResolvedEndpoint",
];
const BLOCK_PROGRESS_HEIGHT_KEYS: &[&str] = &["downloadedBlockHeight", "connectedBlockHeight"];
const RESTART_PROGRESS_SUMMARY_KEYS: &[&str] = &[
    "headerHeight",
    "downloadedBlockHeight",
    "connectedBlockHeight",
    "phase",
    "lifecycle",
    "maybeLastError",
    "maybeLastSuccessfulProgressUnixSeconds",
    "maybeDownloadedBlockHash",
    "maybeConnectedBlockHash",
];
const SAME_DATADIR_KEYS: &[&str] = &["requestedPathMatched", "resolvedPathMatched"];
const RESTART_PROGRESS_DELTA_KEYS: &[&str] =
    &["headerDelta", "downloadedBlockDelta", "connectedBlockDelta"];
const RESTART_PEER_OUTCOME_SUMMARY_KEYS: &[&str] = &[
    "connected",
    "failed",
    "failureCauses",
    "handshook",
    "skipped",
];
const RECOVERY_DIAGNOSIS_KEYS: &[&str] = &[
    "category",
    "maybeLastError",
    "maybeNoProgressCause",
    "maybePeerFailureReason",
    "maybeStorageRecoveryAction",
];
const FINAL_STATUS_KEYS: &[&str] = &[
    "headerHeight",
    "downloadedBlockHeight",
    "connectedBlockHeight",
    "validatedActiveChainHeight",
    "maybeValidatedActiveChainHeightUnavailableReason",
    "maybeValidatedActiveChainHash",
    "maybeValidatedActiveChainWork",
    "blockHeight",
    "phase",
    "lifecycle",
    "outboundPeers",
    "messagesProcessed",
    "stayCurrent",
    "stayCurrentNextAction",
    "noProgressDiagnosis",
    "noProgressNextAction",
    "recoveryCategory",
    "recoveryActionClass",
    "recovery_action_class",
    "recoveryCause",
    "recovery_cause",
    "recoveryNextAction",
    "recovery_next_action",
    "maybeRecoveryEvidenceUnavailableReason",
    "maybe_recovery_evidence_unavailable_reason",
    "maybeLastError",
    "maybeLastSuccessfulProgressUnixSeconds",
];
const RECOVERY_EVIDENCE_KEYS: &[&str] = &[
    "state",
    "category",
    "cause",
    "action_class",
    "actionClass",
    "evidence_basis",
    "evidenceBasis",
    "affected_namespace",
    "affectedNamespace",
    "affected_path",
    "affectedPath",
    "next_action",
    "nextAction",
    "compatibility_action",
    "compatibilityAction",
    "maybe_unavailable_reason",
    "maybeUnavailableReason",
    "source",
];
const BEST_KNOWN_TIP_KEYS: &[&str] = &[
    "source",
    "height",
    "blockHash",
    "work",
    "blockTimeUnixSeconds",
    "observedAtUnixSeconds",
    "freshness",
];
const LATEST_REORG_KEYS: &[&str] = &[
    "commonAncestorHeight",
    "commonAncestorHash",
    "disconnectedCount",
    "connectedCount",
    "finalActiveHeight",
    "finalActiveHash",
    "fullyPersisted",
];
const RECONCILE_PROGRESS_KEYS: &[&str] = &[
    "state",
    "connectedCount",
    "finalActiveHeight",
    "finalActiveHash",
    "missingBlockCount",
];
const PEER_CONTRIBUTION_KEYS: &[&str] =
    &["connected", "failed", "attempted", "handshook", "skipped"];
const RESOURCE_PRESSURE_KEYS: &[&str] = &[
    "blocksInFlight",
    "maxHeaderRequestsInFlightPerPeer",
    "maxHeadersPerMessage",
    "maxBlocksInFlightPerPeer",
    "maxBlocksInFlightTotal",
    "maxMessagesPerPeer",
    "maxSyncRounds",
    "outboundPeers",
    "targetOutboundPeers",
];

pub(super) fn summary(value: &Value) -> Option<Value> {
    let object = value.as_object()?;
    if let Some(summary) = summary_from_schema_v2(object) {
        return Some(summary);
    }
    if let Some(summary) = summary_from_top_level(object) {
        return Some(summary);
    }

    Some(summary_fields_unavailable())
}

fn summary_from_schema_v2(object: &Map<String, Value>) -> Option<Value> {
    let mut summary = Map::new();
    if let Some(result) = object.get("result").and_then(Value::as_object) {
        copy_fields(&mut summary, result, RESULT_SUMMARY_KEYS);
        insert_summarized_field(
            &mut summary,
            "firstHeaderProgress",
            result.get("firstHeaderProgress"),
            summarize_first_header_progress,
        );
        insert_summarized_field(
            &mut summary,
            "firstBlockProgress",
            result.get("firstBlockProgress"),
            summarize_first_block_progress,
        );
        insert_summarized_field(
            &mut summary,
            "restartResumeEvidence",
            result.get("restartResumeEvidence"),
            summarize_restart_resume_evidence,
        );
    }
    insert_summarized_field(
        &mut summary,
        "finalStatus",
        object.get("final_status"),
        summarize_final_status,
    );

    value_from_map(summary)
}

fn summary_from_top_level(object: &Map<String, Value>) -> Option<Value> {
    let mut summary = Map::new();
    copy_fields(&mut summary, object, TOP_LEVEL_SUMMARY_KEYS);
    insert_summarized_field(
        &mut summary,
        "recoveryEvidence",
        object
            .get("recoveryEvidence")
            .or_else(|| object.get("recovery_evidence")),
        |value| summarize_object_fields(value, RECOVERY_EVIDENCE_KEYS),
    );
    value_from_map(summary)
}

fn summarize_first_header_progress(value: &Value) -> Option<Value> {
    if value.is_null() {
        return Some(Value::Null);
    }
    let object = value.as_object()?;
    let mut summary = Map::new();
    copy_fields(&mut summary, object, FIRST_HEADER_DIRECT_KEYS);
    insert_summarized_field(&mut summary, "before", object.get("before"), |value| {
        summarize_object_fields(value, HEADER_HEIGHT_KEYS)
    });
    insert_summarized_field(&mut summary, "after", object.get("after"), |value| {
        summarize_object_fields(value, HEADER_HEIGHT_KEYS)
    });

    value_from_map(summary)
}

fn summarize_first_block_progress(value: &Value) -> Option<Value> {
    if value.is_null() {
        return Some(Value::Null);
    }
    let object = value.as_object()?;
    let mut summary = Map::new();
    copy_fields(&mut summary, object, FIRST_BLOCK_DIRECT_KEYS);
    insert_summarized_field(&mut summary, "before", object.get("before"), |value| {
        summarize_object_fields(value, BLOCK_PROGRESS_HEIGHT_KEYS)
    });
    insert_summarized_field(&mut summary, "after", object.get("after"), |value| {
        summarize_object_fields(value, BLOCK_PROGRESS_HEIGHT_KEYS)
    });

    value_from_map(summary)
}

fn summarize_restart_resume_evidence(value: &Value) -> Option<Value> {
    if value.is_null() {
        return Some(Value::Null);
    }
    let object = value.as_object()?;
    let mut summary = Map::new();
    copy_fields(
        &mut summary,
        object,
        &["restartStatus", "duplicateConnectVerdict"],
    );
    insert_summarized_field(
        &mut summary,
        "sameDatadir",
        object.get("sameDatadir"),
        |value| summarize_object_fields(value, SAME_DATADIR_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "maybePostRestartProgressDelta",
        object.get("maybePostRestartProgressDelta"),
        |value| summarize_object_fields(value, RESTART_PROGRESS_DELTA_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "peerOutcomeSummary",
        object.get("peerOutcomeSummary"),
        |value| summarize_object_fields(value, RESTART_PEER_OUTCOME_SUMMARY_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "recoveryDiagnosis",
        object.get("recoveryDiagnosis"),
        |value| summarize_object_fields(value, RECOVERY_DIAGNOSIS_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "beforeRestart",
        object.get("beforeRestart"),
        |value| summarize_object_fields(value, RESTART_PROGRESS_SUMMARY_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "afterRestart",
        object.get("afterRestart"),
        |value| summarize_object_fields(value, RESTART_PROGRESS_SUMMARY_KEYS),
    );

    value_from_map(summary)
}

fn summarize_final_status(value: &Value) -> Option<Value> {
    if value.is_null() {
        return Some(Value::Null);
    }
    let object = value.as_object()?;
    let mut summary = Map::new();
    copy_fields(&mut summary, object, FINAL_STATUS_KEYS);
    insert_summarized_field(
        &mut summary,
        "bestKnownTip",
        object.get("bestKnownTip"),
        |value| summarize_object_fields(value, BEST_KNOWN_TIP_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "latestReorg",
        object.get("latestReorg"),
        |value| summarize_object_fields(value, LATEST_REORG_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "reconcileProgress",
        object.get("reconcileProgress"),
        |value| summarize_object_fields(value, RECONCILE_PROGRESS_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "resourcePressure",
        object.get("resourcePressure"),
        |value| summarize_object_fields(value, RESOURCE_PRESSURE_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "peerContribution",
        object.get("peerContribution"),
        |value| summarize_object_fields(value, PEER_CONTRIBUTION_KEYS),
    );
    insert_summarized_field(
        &mut summary,
        "recoveryEvidence",
        object
            .get("recoveryEvidence")
            .or_else(|| object.get("recovery_evidence")),
        |value| summarize_object_fields(value, RECOVERY_EVIDENCE_KEYS),
    );

    value_from_map(summary)
}

fn summarize_object_fields(value: &Value, keys: &[&str]) -> Option<Value> {
    if value.is_null() {
        return Some(Value::Null);
    }
    let object = value.as_object()?;
    let mut summary = Map::new();
    copy_fields(&mut summary, object, keys);
    value_from_map(summary)
}

fn insert_summarized_field(
    target: &mut Map<String, Value>,
    key: &str,
    maybe_value: Option<&Value>,
    summarize: impl FnOnce(&Value) -> Option<Value>,
) {
    let Some(value) = maybe_value.and_then(summarize) else {
        return;
    };

    target.insert(key.to_string(), value);
}

fn copy_fields(target: &mut Map<String, Value>, source: &Map<String, Value>, keys: &[&str]) {
    for key in keys {
        if let Some(item) = source.get(*key) {
            target.insert((*key).to_string(), sanitize_json_value(item));
        }
    }
}

fn value_from_map(summary: Map<String, Value>) -> Option<Value> {
    if summary.is_empty() {
        return None;
    }

    Some(Value::Object(summary))
}

fn summary_fields_unavailable() -> Value {
    let mut summary = Map::new();
    summary.insert(
        "status".to_string(),
        Value::String("summary_fields_unavailable".to_string()),
    );
    Value::Object(summary)
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
    if lowercase.contains("authorization:")
        || lowercase.contains("authorization=")
        || lowercase.contains("bearer ")
        || lowercase.contains("basic ")
        || lowercase.contains("rpcpassword")
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

#[cfg(test)]
mod tests;
