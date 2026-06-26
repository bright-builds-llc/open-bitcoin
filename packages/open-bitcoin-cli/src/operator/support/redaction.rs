// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::net::SocketAddr;

use open_bitcoin_node::{
    OpenBitcoinStatusSnapshot,
    status::{
        FieldAvailability, InboundAddressDecisionEvent, InboundPeerPolicyEvent,
        InboundPeerServingStatus,
    },
};
use serde::Serialize;

const INBOUND_ENDPOINT_REDACTION_SAFEGUARD: &str = "inbound peer endpoints bounded/redacted";
const INBOUND_PERMISSION_REDACTION_SAFEGUARD: &str =
    "inbound permission labels bounded to machine classes/effects";
const INBOUND_ADDRESS_REDACTION_SAFEGUARD: &str =
    "inbound address boundary evidence bounded/redacted";
const INBOUND_PEER_POLICY_REDACTION_SAFEGUARD: &str =
    "inbound peer policy evidence bounded/redacted";
const REDACTED_PERMISSION_CLASS_LABEL: &str = "redacted_permission_class";
const REDACTED_PERMISSION_EFFECT_LABEL: &str = "redacted_permission_effect";
const REDACTED_ADDRESS_EVIDENCE_LABEL: &str = "redacted_address_evidence";
const REDACTED_PEER_POLICY_LABEL: &str = "redacted_peer_policy_label";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct RedactionSummary {
    pub(crate) omitted: Vec<String>,
    pub(crate) safeguards: Vec<String>,
}

pub(crate) fn redaction_summary() -> RedactionSummary {
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
            INBOUND_ENDPOINT_REDACTION_SAFEGUARD.to_string(),
            INBOUND_PERMISSION_REDACTION_SAFEGUARD.to_string(),
            INBOUND_ADDRESS_REDACTION_SAFEGUARD.to_string(),
            INBOUND_PEER_POLICY_REDACTION_SAFEGUARD.to_string(),
        ],
    }
}

pub(crate) fn support_status_for_bundle(
    mut status: OpenBitcoinStatusSnapshot,
) -> OpenBitcoinStatusSnapshot {
    redact_inbound_endpoint_evidence(&mut status.peers.inbound);
    redact_inbound_permission_evidence(&mut status.peers.inbound);
    redact_inbound_address_evidence(&mut status.peers.inbound);
    redact_inbound_peer_policy_evidence(&mut status.peers.inbound);
    status
}

fn redact_inbound_endpoint_evidence(inbound: &mut FieldAvailability<InboundPeerServingStatus>) {
    let FieldAvailability::Available(evidence) = inbound else {
        return;
    };
    evidence.bound_endpoints = redacted_inbound_endpoint_summary(&evidence.bound_endpoints);
}

fn redact_inbound_permission_evidence(inbound: &mut FieldAvailability<InboundPeerServingStatus>) {
    let FieldAvailability::Available(evidence) = inbound else {
        return;
    };

    evidence.permission_class = sanitized_permission_class_label(&evidence.permission_class);
    sanitize_permission_effect_labels(
        &mut evidence.active_permission_effects,
        is_safe_active_permission_effect_label,
    );
    sanitize_permission_effect_labels(
        &mut evidence.inactive_permission_effects,
        is_safe_inactive_permission_effect_label,
    );

    let FieldAvailability::Available(event) = &mut evidence.latest_permission_decision else {
        return;
    };
    event.permission_class = sanitized_permission_class_label(&event.permission_class);
    sanitize_permission_effect_labels(
        &mut event.active_permission_effects,
        is_safe_active_permission_effect_label,
    );
    sanitize_permission_effect_labels(
        &mut event.inactive_permission_effects,
        is_safe_inactive_permission_effect_label,
    );
    event.message = format!(
        "inbound permission decision {} as {}",
        event.outcome, event.permission_class
    );
}

fn redact_inbound_address_evidence(inbound: &mut FieldAvailability<InboundPeerServingStatus>) {
    let FieldAvailability::Available(evidence) = inbound else {
        return;
    };

    for entry in &mut evidence.local_advertisement_candidates {
        entry.source = sanitized_address_evidence_text(&entry.source);
        entry.network_kind = sanitized_address_evidence_text(&entry.network_kind);
        entry.routability = sanitized_address_evidence_text(&entry.routability);
        entry.freshness = sanitized_address_evidence_text(&entry.freshness);
    }
    for event in &mut evidence.suppressed_advertisements {
        sanitize_address_decision(event);
    }
    if let FieldAvailability::Available(event) = &mut evidence.latest_address_decision {
        sanitize_address_decision(event);
    }
}

fn sanitize_address_decision(event: &mut InboundAddressDecisionEvent) {
    event.outcome = sanitized_address_evidence_text(&event.outcome);
    event.reason = sanitized_address_evidence_text(&event.reason);
    event.label = sanitized_address_evidence_text(&event.label);
    event.source = sanitized_address_evidence_text(&event.source);
    event.message = sanitized_address_evidence_text(&event.message);
}

fn redact_inbound_peer_policy_evidence(inbound: &mut FieldAvailability<InboundPeerServingStatus>) {
    let FieldAvailability::Available(evidence) = inbound else {
        return;
    };

    if let FieldAvailability::Available(event) = &mut evidence.latest_peer_policy_decision {
        sanitize_peer_policy_decision(event);
    }
}

fn sanitize_peer_policy_decision(event: &mut InboundPeerPolicyEvent) {
    event.outcome = sanitized_peer_policy_text(&event.outcome);
    event.reason = sanitized_peer_policy_text(&event.reason);
    event.label = sanitized_peer_policy_text(&event.label);
    event.source = sanitized_peer_policy_text(&event.source);
    event.message = format!("peer policy decision {}: {}", event.outcome, event.reason);
}

fn redacted_inbound_endpoint_summary(endpoints: &[String]) -> Vec<String> {
    let mut loopback = 0;
    let mut non_loopback = 0;
    let mut wildcard = 0;
    let mut compact_label = 0;
    for endpoint in endpoints {
        match inbound_endpoint_class(endpoint) {
            InboundEndpointClass::Loopback => loopback += 1,
            InboundEndpointClass::NonLoopback => non_loopback += 1,
            InboundEndpointClass::Wildcard => wildcard += 1,
            InboundEndpointClass::CompactLabel => compact_label += 1,
        }
    }

    let mut summary = Vec::new();
    push_endpoint_redaction_summary(&mut summary, loopback, "loopback endpoint");
    push_endpoint_redaction_summary(&mut summary, non_loopback, "non-loopback endpoint");
    push_endpoint_redaction_summary(&mut summary, wildcard, "wildcard endpoint");
    push_endpoint_redaction_summary(&mut summary, compact_label, "compact endpoint label");
    summary
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InboundEndpointClass {
    Loopback,
    NonLoopback,
    Wildcard,
    CompactLabel,
}

fn inbound_endpoint_class(endpoint: &str) -> InboundEndpointClass {
    let Ok(address) = endpoint.parse::<SocketAddr>() else {
        return InboundEndpointClass::CompactLabel;
    };
    if address.ip().is_loopback() {
        return InboundEndpointClass::Loopback;
    }
    if address.ip().is_unspecified() {
        return InboundEndpointClass::Wildcard;
    }
    InboundEndpointClass::NonLoopback
}

fn push_endpoint_redaction_summary(summary: &mut Vec<String>, count: usize, singular: &str) {
    if count == 0 {
        return;
    }
    let label = if count == 1 {
        singular.to_string()
    } else {
        format!("{singular}s")
    };
    summary.push(format!("{count} {label} redacted"));
}

fn sanitized_permission_class_label(label: &str) -> String {
    if is_safe_permission_class_label(label) {
        return label.to_string();
    }
    REDACTED_PERMISSION_CLASS_LABEL.to_string()
}

fn is_safe_permission_class_label(label: &str) -> bool {
    matches!(
        label,
        "ordinary_inbound" | "permissioned_inbound" | "protected_inbound"
    )
}

fn sanitize_permission_effect_labels(labels: &mut Vec<String>, is_safe_label: fn(&str) -> bool) {
    let mut sanitized = Vec::with_capacity(labels.len());
    let mut redacted_unknown_label = false;
    for label in labels.iter().map(String::as_str) {
        if is_safe_label(label) {
            sanitized.push(label.to_string());
            continue;
        }
        redacted_unknown_label = true;
    }

    if redacted_unknown_label {
        sanitized.push(REDACTED_PERMISSION_EFFECT_LABEL.to_string());
    }
    *labels = sanitized;
}

fn is_safe_active_permission_effect_label(label: &str) -> bool {
    matches!(
        label,
        "admission_protected"
            | "eviction_policy_protected"
            | "misbehavior_policy_protected"
            | "address_response_policy_input"
            | "download_serving_policy_input"
    )
}

fn is_safe_inactive_permission_effect_label(label: &str) -> bool {
    matches!(
        label,
        "inactive_relay"
            | "inactive_forcerelay"
            | "inactive_mempool"
            | "inactive_bloomfilter"
            | "inactive_blockfilters"
    )
}

fn sanitized_address_evidence_text(value: &str) -> String {
    if contains_raw_address_evidence(value) {
        return REDACTED_ADDRESS_EVIDENCE_LABEL.to_string();
    }
    value.to_string()
}

fn sanitized_peer_policy_text(value: &str) -> String {
    if contains_raw_address_evidence(value) {
        return REDACTED_PEER_POLICY_LABEL.to_string();
    }
    value.to_string()
}

fn contains_raw_address_evidence(value: &str) -> bool {
    let lower_value = value.to_ascii_lowercase();
    value.contains("127.0.0.1:")
        || value.contains("0.0.0.0:")
        || value.contains("::1")
        || lower_value.contains("peer-")
        || lower_value.contains("address_bytes")
        || lower_value.contains("peer_id=")
        || lower_value.contains("operator_loopback")
        || lower_value.contains("operator-loopback")
        || lower_value.contains("raw_permission")
        || lower_value.contains("in,noban")
        || lower_value.contains("rpc_password")
        || lower_value.contains("cookie=")
        || lower_value.contains("config=")
}
