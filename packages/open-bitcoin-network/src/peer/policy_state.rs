// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.h
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp

use crate::{
    EvictionCandidateInput, InboundHandshakeState, PeerConnectionClass, PeerId,
    PermissionEffectLabel, peer::PeerState,
};

pub(super) fn eviction_candidate_input(
    peer_id: PeerId,
    peer: &PeerState,
) -> EvictionCandidateInput {
    let maybe_record = peer.maybe_inbound_record.as_ref();
    let active_permission_effects = maybe_record
        .map(|record| record.permission_decision.active_effects().to_vec())
        .unwrap_or_default();
    let connection_class = maybe_record
        .map(|record| record.connection_class.as_str())
        .unwrap_or("ordinary_inbound");
    let slot_class = maybe_record
        .map(|record| record.slot_class.as_str())
        .unwrap_or("ordinary");
    let handshake_state = maybe_record
        .map(|record| record.handshake_state)
        .unwrap_or(InboundHandshakeState::Established);
    let diversity_group = maybe_record
        .map(|record| endpoint_diversity_group(&record.remote_endpoint))
        .unwrap_or_else(|| "unknown".to_string());

    EvictionCandidateInput {
        peer_label: peer_policy_label(peer_id),
        handshake_state,
        connection_class,
        slot_class,
        requested_inventory_count: peer.requested_blocks.len()
            + peer.requested_txids.len()
            + peer.requested_wtxids.len(),
        active_permission_effects,
        diversity_group,
    }
}

pub(super) fn peer_policy_protected(peer: &PeerState) -> bool {
    peer.maybe_inbound_record
        .as_ref()
        .map(|record| {
            record
                .permission_decision
                .active_effects()
                .contains(&PermissionEffectLabel::MisbehaviorPolicyProtected)
                || record.connection_class == PeerConnectionClass::ProtectedInbound
        })
        .unwrap_or(false)
}

fn endpoint_diversity_group(endpoint: &str) -> String {
    let maybe_host = endpoint.rsplit_once(':').map(|(host, _port)| host);
    maybe_host.unwrap_or(endpoint).to_string()
}

pub(super) fn peer_policy_label(peer_id: PeerId) -> String {
    format!("peer-{peer_id}")
}

#[cfg(test)]
mod tests {
    use crate::{
        ConnectionRole, InboundAdmissionSlotClass, InboundHandshakeState, InboundPeerRecord,
        InboundPermissionDecision,
    };

    use super::*;

    #[test]
    fn protected_connection_class_sets_peer_policy_protection() {
        // Arrange
        let mut peer = PeerState::new(ConnectionRole::Inbound);
        peer.maybe_inbound_record = Some(InboundPeerRecord {
            peer_id: 44,
            remote_endpoint: "127.0.0.1:18444".to_string(),
            slot_class: InboundAdmissionSlotClass::Reserved,
            connection_class: PeerConnectionClass::ProtectedInbound,
            permission_decision: InboundPermissionDecision::ordinary(),
            handshake_state: InboundHandshakeState::Handshaking,
            maybe_remote_nonce: None,
            observed_inbound_peers: 1,
            observed_outbound_peers: 0,
        });

        // Act
        let protected = peer_policy_protected(&peer);

        // Assert
        assert!(protected);
    }
}
