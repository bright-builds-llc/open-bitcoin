// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_network::InboundAdmissionSlotClass;
use open_bitcoin_node::{
    network::{ManagedAddressBoundaryInfo, ManagedInboundAdmissionInfo, ManagedPeerPolicyInfo},
    status::{
        FieldAvailability, INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON,
        INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON,
        INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON, InboundAddressDecisionEvent,
        InboundAdmissionEvent, InboundHandshakeStatusCounts, InboundPeerPolicyEvent,
        InboundPeerServingStatus, InboundPermissionDecisionEvent, InboundPermissionEvidence,
        inbound_status_unavailable,
    },
};

use crate::inbound_listener::InboundListenerEvidence;

use super::{
    ManagedRpcContext,
    resource_governance::{latest_resource_governance_decision, resource_governance_info},
};

impl ManagedRpcContext {
    pub fn current_inbound_status(&self) -> FieldAvailability<InboundPeerServingStatus> {
        let admission = self.inbound_admission_info();
        let address_info = self.network.address_boundary_info();
        let peer_policy_info = self.network.peer_policy_info();
        let maybe_listener_evidence = self.maybe_inbound_listener_evidence.as_ref();
        let resource_info = resource_governance_info(
            self.network.resource_governance_info(),
            maybe_listener_evidence,
        );
        if admission.admitted_inbound_peers == 0
            && admission.rejected_inbound_peers == 0
            && maybe_listener_evidence.is_none()
            && address_info.is_empty()
            && peer_policy_info.is_empty()
            && resource_info.is_empty()
            && self.inbound_permission_validation_failures == 0
        {
            return inbound_status_unavailable();
        }

        let network_info = self.network_info();
        let permission_evidence = inbound_permission_evidence(&admission);
        let latest_address_decision = latest_inbound_address_decision(&address_info);
        let latest_peer_policy_decision = latest_inbound_peer_policy_decision(&peer_policy_info);
        let latest_resource_governance_decision =
            latest_resource_governance_decision(&resource_info);
        FieldAvailability::available(InboundPeerServingStatus {
            listener_state: listener_state(&admission, maybe_listener_evidence),
            bound_endpoints: bound_endpoints(maybe_listener_evidence),
            preflight_reason: preflight_reason(&admission, maybe_listener_evidence),
            admitted_inbound_peers: usize_to_u32(admission.admitted_inbound_peers),
            rejected_inbound_peers: usize_to_u32(admission.rejected_inbound_peers),
            handshake: InboundHandshakeStatusCounts {
                awaiting_version: 0,
                awaiting_verack: 0,
                established: usize_to_u32(network_info.inbound_peers),
                disconnected: 0,
            },
            duplicate_rejects: usize_to_u32(
                admission.duplicate_endpoint_rejections + admission.duplicate_identity_rejections,
            ),
            self_connection_rejects: usize_to_u32(admission.self_connection_rejections),
            cap_rejects: usize_to_u32(admission.cap_rejections),
            reserved_slot_rejects: usize_to_u32(admission.reserved_slot_rejections),
            latest_admission_event: latest_inbound_admission_event(
                &admission,
                maybe_listener_evidence,
            ),
            permissioned_inbound_peers: usize_to_u32(admission.permissioned_inbound_admits),
            protected_inbound_peers: usize_to_u32(admission.protected_inbound_admits),
            permission_class: permission_evidence.permission_class,
            active_permission_effects: permission_evidence.active_permission_effects,
            inactive_permission_effects: permission_evidence.inactive_permission_effects,
            inactive_permission_effect_observations: usize_to_u32(
                admission.inactive_permission_effect_observations,
            ),
            permission_validation_failures: self.inbound_permission_validation_failures,
            latest_permission_decision: latest_inbound_permission_decision(&admission),
            local_advertisement_candidates: address_info.local_advertisement_candidates,
            suppressed_advertisements: address_info.suppressed_advertisements,
            getaddr_responses_served: address_info.getaddr_responses_served,
            getaddr_requests_suppressed: address_info.getaddr_requests_suppressed,
            learned_address_entries: address_info.learned_address_entries,
            learned_address_rejections: address_info.learned_address_rejections,
            latest_address_decision,
            eviction_candidates_evaluated: peer_policy_info.eviction_candidates_evaluated,
            disconnects_requested: peer_policy_info.disconnects_requested,
            discouraged_peers: peer_policy_info.discouraged_peers,
            active_bans: peer_policy_info.active_bans,
            expired_bans: peer_policy_info.expired_bans,
            manual_unbans: peer_policy_info.manual_unbans,
            misbehavior_observations: peer_policy_info.misbehavior_observations,
            protected_no_actions: peer_policy_info.protected_no_actions,
            latest_peer_policy_decision,
            resource_pressure_events: resource_info.resource_pressure_events,
            read_queue_pressure_events: resource_info.read_queue_pressure_events,
            write_queue_pressure_events: resource_info.write_queue_pressure_events,
            request_cap_events: resource_info.request_cap_events,
            payload_rejections: resource_info.payload_rejections,
            timeout_disconnects: resource_info.timeout_disconnects,
            churn_rejections: resource_info.churn_rejections,
            reconnect_suppressions: resource_info.reconnect_suppressions,
            latest_resource_governance_decision,
        })
    }
}

fn listener_state(
    admission: &ManagedInboundAdmissionInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> String {
    maybe_listener_evidence
        .map(|evidence| evidence.listener_state.clone())
        .unwrap_or_else(|| {
            if admission.admitted_inbound_peers > 0 || admission.rejected_inbound_peers > 0 {
                "listening".to_string()
            } else {
                "unavailable".to_string()
            }
        })
}

fn bound_endpoints(maybe_listener_evidence: Option<&InboundListenerEvidence>) -> Vec<String> {
    maybe_listener_evidence
        .map(|evidence| evidence.bound_endpoints.clone())
        .unwrap_or_default()
}

fn preflight_reason(
    admission: &ManagedInboundAdmissionInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> String {
    maybe_listener_evidence
        .map(|evidence| evidence.preflight_reason.clone())
        .unwrap_or_else(|| {
            if admission.admitted_inbound_peers > 0 || admission.rejected_inbound_peers > 0 {
                "ready".to_string()
            } else {
                "unavailable".to_string()
            }
        })
}

fn latest_inbound_admission_event(
    admission: &ManagedInboundAdmissionInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> FieldAvailability<InboundAdmissionEvent> {
    if let Some(reason) = admission.maybe_latest_rejection_reason {
        let reason = reason.as_str().to_string();
        let slot_class = admission
            .maybe_latest_rejection_slot_class
            .map(InboundAdmissionSlotClass::as_str)
            .unwrap_or("ordinary");
        return FieldAvailability::available(InboundAdmissionEvent {
            outcome: "rejected".to_string(),
            reason: reason.clone(),
            slot_class: slot_class.to_string(),
            message: format!("inbound admission rejected: {reason}"),
        });
    }

    if admission.admitted_inbound_peers > 0 {
        let slot_class = admission
            .maybe_latest_permission_decision
            .as_ref()
            .map(|decision| decision.connection_class.slot_class().as_str())
            .unwrap_or("ordinary");
        return FieldAvailability::available(InboundAdmissionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            slot_class: slot_class.to_string(),
            message: "inbound peer admitted".to_string(),
        });
    }

    if let Some(event) = maybe_listener_evidence
        .and_then(|evidence| evidence.maybe_latest_admission_event.as_deref())
    {
        return FieldAvailability::available(InboundAdmissionEvent {
            outcome: "listener".to_string(),
            reason: event.to_string(),
            slot_class: "ordinary".to_string(),
            message: format!("inbound listener event: {event}"),
        });
    }

    FieldAvailability::unavailable("no inbound admission event recorded")
}

fn inbound_permission_evidence(
    admission: &ManagedInboundAdmissionInfo,
) -> InboundPermissionEvidence {
    let mut evidence = InboundPermissionEvidence::ordinary();
    if let Some(permission_decision) = &admission.maybe_latest_permission_decision {
        evidence.permission_class = permission_decision.connection_class.as_str().to_string();
    }
    evidence.active_permission_effects = admission
        .observed_active_permission_effects
        .iter()
        .map(|effect| effect.as_str().to_string())
        .collect();
    evidence.inactive_permission_effects = admission
        .observed_inactive_permission_effects
        .iter()
        .map(|effect| effect.as_str().to_string())
        .collect();
    evidence
}

fn latest_inbound_permission_decision(
    admission: &ManagedInboundAdmissionInfo,
) -> FieldAvailability<InboundPermissionDecisionEvent> {
    let Some(permission_decision) = &admission.maybe_latest_permission_decision else {
        return FieldAvailability::unavailable(INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON);
    };

    let permission_class = permission_decision.connection_class.as_str().to_string();
    FieldAvailability::available(InboundPermissionDecisionEvent {
        outcome: "admitted".to_string(),
        reason: "admitted".to_string(),
        permission_class: permission_class.clone(),
        active_permission_effects: permission_decision
            .active_effects
            .iter()
            .map(|effect| effect.as_str().to_string())
            .collect(),
        inactive_permission_effects: permission_decision
            .inactive_effects
            .iter()
            .map(|effect| effect.as_str().to_string())
            .collect(),
        message: format!("inbound permission decision admitted as {permission_class}"),
    })
}

fn latest_inbound_address_decision(
    address_info: &ManagedAddressBoundaryInfo,
) -> FieldAvailability<InboundAddressDecisionEvent> {
    address_info
        .maybe_latest_address_decision
        .clone()
        .map(FieldAvailability::available)
        .unwrap_or_else(|| {
            FieldAvailability::unavailable(INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON)
        })
}

fn latest_inbound_peer_policy_decision(
    peer_policy_info: &ManagedPeerPolicyInfo,
) -> FieldAvailability<InboundPeerPolicyEvent> {
    peer_policy_info
        .maybe_latest_peer_policy_decision
        .clone()
        .map(FieldAvailability::available)
        .unwrap_or_else(|| {
            FieldAvailability::unavailable(INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON)
        })
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
