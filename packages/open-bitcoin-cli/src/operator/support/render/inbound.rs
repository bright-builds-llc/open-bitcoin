// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Inbound serving support Markdown rendering.

use open_bitcoin_node::status::{
    FieldAvailability, InboundAddressDecisionEvent, InboundAddressEvidenceEntry,
    InboundAdmissionEvent, InboundPeerPolicyEvent, InboundPeerServingStatus,
    InboundPermissionDecisionEvent,
};

const INACTIVE_RELAY_PERMISSION_NEXT_ACTION: &str = "Relay, mempool, bloom, and blockfilter permissions are recorded as inactive Phase 91 evidence; do not treat them as relay support.";
const PHASE92_ADDRESS_BOUNDARY_NEXT_ACTION: &str = "Treat Phase 92 as bounded local advertisement and direct getaddr evidence only; peer discovery, unsolicited address relay, DNS seed discovery, UPnP/NAT-PMP discovery, and public-network readiness remain outside this surface.";
const PHASE93_PEER_POLICY_NEXT_ACTION: &str = "Treat Phase 93 as bounded eviction, ban, unban, and misbehavior policy evidence only; review labels and counters before changing peer policy.";

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
    output.push_str(&format!(
        "- permission_class: {}\n",
        evidence.permission_class
    ));
    output.push_str(&format!(
        "- permissioned_inbound_peers: {}\n",
        evidence.permissioned_inbound_peers
    ));
    output.push_str(&format!(
        "- protected_inbound_peers: {}\n",
        evidence.protected_inbound_peers
    ));
    output.push_str(&format!(
        "- active_permission_effects: {}\n",
        label_list_text(&evidence.active_permission_effects)
    ));
    output.push_str(&format!(
        "- inactive_permission_effects: {}\n",
        label_list_text(&evidence.inactive_permission_effects)
    ));
    push_latest_permission_decision(output, &evidence.latest_permission_decision);
    push_latest_admission_event(output, &evidence.latest_admission_event);
    output.push_str(&format!("- Next action: {}\n", next_action(evidence)));
    push_inbound_address_boundary(output, evidence);
    push_inbound_peer_policy(output, evidence);
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

fn push_latest_permission_decision(
    output: &mut String,
    event: &FieldAvailability<InboundPermissionDecisionEvent>,
) {
    match event {
        FieldAvailability::Available(event) => output.push_str(&format!(
            "- latest_permission_decision: outcome={} reason={} permission_class={} active_permission_effects={} inactive_permission_effects={} message={}\n",
            event.outcome,
            event.reason,
            event.permission_class,
            label_list_text(&event.active_permission_effects),
            label_list_text(&event.inactive_permission_effects),
            event.message
        )),
        FieldAvailability::Unavailable { reason } => output.push_str(&format!(
            "- latest_permission_decision: Unavailable: {reason}\n"
        )),
    }
}

fn push_inbound_address_boundary(output: &mut String, evidence: &InboundPeerServingStatus) {
    output.push_str("\n## Inbound Address Boundary Evidence\n\n");
    output.push_str(&format!(
        "- Local advertisement candidates: {} {}\n",
        evidence.local_advertisement_candidates.len(),
        address_entries_text(&evidence.local_advertisement_candidates)
    ));
    output.push_str(&format!(
        "- Suppressed advertisements: {} {}\n",
        evidence.suppressed_advertisements.len(),
        address_decisions_text(&evidence.suppressed_advertisements)
    ));
    output.push_str(&format!(
        "- Bounded getaddr responses served: {}\n",
        evidence.getaddr_responses_served
    ));
    output.push_str(&format!(
        "- Bounded getaddr requests suppressed: {}\n",
        evidence.getaddr_requests_suppressed
    ));
    output.push_str(&format!(
        "- Learned address entries: {}\n",
        evidence.learned_address_entries
    ));
    output.push_str(&format!(
        "- Learned address rejections: {}\n",
        evidence.learned_address_rejections
    ));
    output.push_str(&format!(
        "- Latest address decision: {}\n",
        latest_address_decision_text(&evidence.latest_address_decision)
    ));
    output.push_str(&format!(
        "- Next action: {PHASE92_ADDRESS_BOUNDARY_NEXT_ACTION}\n"
    ));
}

fn address_entries_text(entries: &[InboundAddressEvidenceEntry]) -> String {
    if entries.is_empty() {
        return "none".to_string();
    }
    entries
        .iter()
        .map(address_entry_text)
        .collect::<Vec<_>>()
        .join("; ")
}

fn address_entry_text(entry: &InboundAddressEvidenceEntry) -> String {
    format!(
        "source={} network_kind={} routability={} freshness={} services_bits={} port={} persistence_eligible={}",
        entry.source,
        entry.network_kind,
        entry.routability,
        entry.freshness,
        entry.services_bits,
        entry.port,
        entry.persistence_eligible
    )
}

fn address_decisions_text(events: &[InboundAddressDecisionEvent]) -> String {
    if events.is_empty() {
        return "none".to_string();
    }
    events
        .iter()
        .map(address_decision_text)
        .collect::<Vec<_>>()
        .join("; ")
}

fn address_decision_text(event: &InboundAddressDecisionEvent) -> String {
    format!(
        "outcome={} reason={} label={} source={} message={}",
        event.outcome, event.reason, event.label, event.source, event.message
    )
}

fn latest_address_decision_text(event: &FieldAvailability<InboundAddressDecisionEvent>) -> String {
    match event {
        FieldAvailability::Available(event) => address_decision_text(event),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn push_inbound_peer_policy(output: &mut String, evidence: &InboundPeerServingStatus) {
    output.push_str("\n## Inbound Peer Policy Evidence\n\n");
    output.push_str(&format!(
        "- Eviction candidates evaluated: {}\n",
        evidence.eviction_candidates_evaluated
    ));
    output.push_str(&format!(
        "- Disconnects requested: {}\n",
        evidence.disconnects_requested
    ));
    output.push_str(&format!(
        "- Discouraged peers: {}\n",
        evidence.discouraged_peers
    ));
    output.push_str(&format!("- Active bans: {}\n", evidence.active_bans));
    output.push_str(&format!("- Expired bans: {}\n", evidence.expired_bans));
    output.push_str(&format!("- Manual unbans: {}\n", evidence.manual_unbans));
    output.push_str(&format!(
        "- Misbehavior observations: {}\n",
        evidence.misbehavior_observations
    ));
    output.push_str(&format!(
        "- Protected no-actions: {}\n",
        evidence.protected_no_actions
    ));
    output.push_str(&format!(
        "- Latest peer policy decision: {}\n",
        latest_peer_policy_decision_text(&evidence.latest_peer_policy_decision)
    ));
    output.push_str(&format!(
        "- Next action: {PHASE93_PEER_POLICY_NEXT_ACTION}\n"
    ));
}

fn latest_peer_policy_decision_text(event: &FieldAvailability<InboundPeerPolicyEvent>) -> String {
    match event {
        FieldAvailability::Available(event) => peer_policy_decision_text(event),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn peer_policy_decision_text(event: &InboundPeerPolicyEvent) -> String {
    format!(
        "outcome={} reason={} label={} source={} message={}",
        event.outcome, event.reason, event.label, event.source, event.message
    )
}

fn next_action(evidence: &InboundPeerServingStatus) -> &'static str {
    if has_inactive_relay_like_effects(&evidence.inactive_permission_effects) {
        return INACTIVE_RELAY_PERMISSION_NEXT_ACTION;
    }
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

fn has_inactive_relay_like_effects(effects: &[String]) -> bool {
    effects.iter().any(|effect| {
        matches!(
            effect.as_str(),
            "inactive_relay"
                | "inactive_forcerelay"
                | "inactive_mempool"
                | "inactive_bloomfilter"
                | "inactive_blockfilters"
        )
    })
}

fn csv_or_unavailable(values: &[String]) -> String {
    if values.is_empty() {
        return "unavailable".to_string();
    }
    values.join(", ")
}

fn label_list_text(values: &[String]) -> String {
    if values.is_empty() {
        return "none".to_string();
    }
    values.join(", ")
}
