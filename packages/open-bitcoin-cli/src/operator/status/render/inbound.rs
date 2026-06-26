// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Human rendering for shared inbound peer serving status.

use open_bitcoin_node::status::{
    FieldAvailability, InboundAddressDecisionEvent, InboundAddressEvidenceEntry,
    InboundAdmissionEvent, InboundPeerServingStatus, InboundPermissionDecisionEvent,
};

pub(super) fn inbound_status_text(status: &FieldAvailability<InboundPeerServingStatus>) -> String {
    match status {
        FieldAvailability::Available(status) => available_inbound_status_text(status),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn available_inbound_status_text(status: &InboundPeerServingStatus) -> String {
    format!(
        "listener_state={} bound_endpoints={} preflight_reason={} admitted_inbound_peers={} rejected_inbound_peers={} handshake={} duplicate_rejects={} self_connection_rejects={} cap_rejects={} reserved_slot_rejects={} permission_class={} permissioned_inbound_peers={} protected_inbound_peers={} active_permission_effects={} inactive_permission_effects={} latest_permission_decision={} latest_admission_event={} {}",
        status.listener_state,
        bound_endpoints_text(&status.bound_endpoints),
        status.preflight_reason,
        status.admitted_inbound_peers,
        status.rejected_inbound_peers,
        handshake_text(status),
        status.duplicate_rejects,
        status.self_connection_rejects,
        status.cap_rejects,
        status.reserved_slot_rejects,
        status.permission_class,
        status.permissioned_inbound_peers,
        status.protected_inbound_peers,
        label_list_text(&status.active_permission_effects),
        label_list_text(&status.inactive_permission_effects),
        latest_permission_decision_text(&status.latest_permission_decision),
        latest_event_text(&status.latest_admission_event),
        address_boundary_text(status),
    )
}

fn bound_endpoints_text(endpoints: &[String]) -> String {
    if endpoints.is_empty() {
        return "none".to_string();
    }

    endpoints.join(",")
}

fn handshake_text(status: &InboundPeerServingStatus) -> String {
    format!(
        "awaiting_version={} awaiting_verack={} established={} disconnected={}",
        status.handshake.awaiting_version,
        status.handshake.awaiting_verack,
        status.handshake.established,
        status.handshake.disconnected,
    )
}

fn latest_event_text(event: &FieldAvailability<InboundAdmissionEvent>) -> String {
    match event {
        FieldAvailability::Available(event) => format!(
            "outcome={} reason={} slot_class={} message={}",
            event.outcome, event.reason, event.slot_class, event.message
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn latest_permission_decision_text(
    event: &FieldAvailability<InboundPermissionDecisionEvent>,
) -> String {
    match event {
        FieldAvailability::Available(event) => format!(
            "outcome={} reason={} permission_class={} active_permission_effects={} inactive_permission_effects={} message={}",
            event.outcome,
            event.reason,
            event.permission_class,
            label_list_text(&event.active_permission_effects),
            label_list_text(&event.inactive_permission_effects),
            event.message
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn label_list_text(labels: &[String]) -> String {
    if labels.is_empty() {
        return "none".to_string();
    }

    labels.join(",")
}

fn address_boundary_text(status: &InboundPeerServingStatus) -> String {
    format!(
        "local advertisement candidates: {} local advertisement candidate evidence={} suppressed advertisements: {} suppressed advertisement evidence={} bounded getaddr responses served: {} bounded getaddr requests suppressed: {} learned address entries: {} learned address rejections: {} latest address decision={}",
        status.local_advertisement_candidates.len(),
        address_entries_text(&status.local_advertisement_candidates),
        status.suppressed_advertisements.len(),
        address_decisions_text(&status.suppressed_advertisements),
        status.getaddr_responses_served,
        status.getaddr_requests_suppressed,
        status.learned_address_entries,
        status.learned_address_rejections,
        latest_address_decision_text(&status.latest_address_decision),
    )
}

fn address_entries_text(entries: &[InboundAddressEvidenceEntry]) -> String {
    if entries.is_empty() {
        return "none".to_string();
    }

    entries
        .iter()
        .map(address_entry_text)
        .collect::<Vec<_>>()
        .join(" | ")
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
        entry.persistence_eligible,
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
        .join(" | ")
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
