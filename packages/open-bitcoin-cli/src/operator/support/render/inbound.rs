// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Inbound serving support Markdown rendering.

use open_bitcoin_node::status::{
    FieldAvailability, InboundAdmissionEvent, InboundPeerServingStatus,
};

pub(super) fn push_inbound_serving(
    output: &mut String,
    inbound: &FieldAvailability<InboundPeerServingStatus>,
) {
    output.push_str("\n## Inbound Serving\n\n");
    match inbound {
        FieldAvailability::Available(evidence) => push_available_inbound(output, evidence),
        FieldAvailability::Unavailable { reason } => {
            output.push_str(&format!("- Status: Unavailable: {reason}\n"));
        }
    }
}

fn push_available_inbound(output: &mut String, evidence: &InboundPeerServingStatus) {
    output.push_str(&format!("- listener_state: {}\n", evidence.listener_state));
    output.push_str(&format!(
        "- preflight_reason: {}\n",
        evidence.preflight_reason
    ));
    output.push_str(&format!(
        "- bound_endpoints: {}\n",
        csv_or_unavailable(&evidence.bound_endpoints)
    ));
    output.push_str(&format!(
        "- admitted_inbound_peers: {}\n",
        evidence.admitted_inbound_peers
    ));
    output.push_str(&format!(
        "- rejected_inbound_peers: {}\n",
        evidence.rejected_inbound_peers
    ));
    output.push_str(&format!(
        "- handshake.awaiting_version: {}\n",
        evidence.handshake.awaiting_version
    ));
    output.push_str(&format!(
        "- handshake.awaiting_verack: {}\n",
        evidence.handshake.awaiting_verack
    ));
    output.push_str(&format!(
        "- handshake.established: {}\n",
        evidence.handshake.established
    ));
    output.push_str(&format!(
        "- handshake.disconnected: {}\n",
        evidence.handshake.disconnected
    ));
    output.push_str(&format!(
        "- duplicate_rejects: {}\n",
        evidence.duplicate_rejects
    ));
    output.push_str(&format!(
        "- self_connection_rejects: {}\n",
        evidence.self_connection_rejects
    ));
    output.push_str(&format!("- cap_rejects: {}\n", evidence.cap_rejects));
    output.push_str(&format!(
        "- reserved_slot_rejects: {}\n",
        evidence.reserved_slot_rejects
    ));
    push_latest_admission_event(output, &evidence.latest_admission_event);
    output.push_str(&format!("- Next action: {}\n", next_action(evidence)));
}

fn push_latest_admission_event(
    output: &mut String,
    event: &FieldAvailability<InboundAdmissionEvent>,
) {
    match event {
        FieldAvailability::Available(event) => output.push_str(&format!(
            "- latest_admission_event: outcome={} reason={} slot_class={} message={}\n",
            event.outcome, event.reason, event.slot_class, event.message
        )),
        FieldAvailability::Unavailable { reason } => output.push_str(&format!(
            "- latest_admission_event: Unavailable: {reason}\n"
        )),
    }
}

fn next_action(evidence: &InboundPeerServingStatus) -> &'static str {
    if evidence.cap_rejects > 0 || evidence.reserved_slot_rejects > 0 {
        return "Review configured inbound caps and reserved slots before increasing listener exposure.";
    }
    if evidence.duplicate_rejects > 0 || evidence.self_connection_rejects > 0 {
        return "Review duplicate and self-connection admission evidence before changing listener policy.";
    }
    match evidence.preflight_reason.as_str() {
        "disabled" => {
            "Enable Open Bitcoin inbound settings only after listener exposure is intended."
        }
        "no_listen_addresses" => {
            "Set inbound.listen_addresses or -openbitcoinlisten for an explicit listener endpoint."
        }
        "invalid_endpoint" => "Fix the configured inbound listener endpoint syntax.",
        "unsafe_endpoint" => {
            "Use a loopback listener or explicitly acknowledge public exposure before binding."
        }
        "bind_unavailable" | "already_bound" => {
            "Choose an available inbound listener endpoint or stop the process currently using it."
        }
        _ => "No inbound support action required from this bounded evidence.",
    }
}

fn csv_or_unavailable(values: &[String]) -> String {
    if values.is_empty() {
        return "unavailable".to_string();
    }
    values.join(", ")
}
