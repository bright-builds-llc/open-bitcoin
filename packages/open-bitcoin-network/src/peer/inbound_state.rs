// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use std::collections::BTreeSet;

use crate::{
    DisconnectReason, NetworkError, PeerId,
    inbound::{
        InboundAdmissionCounters, InboundAdmissionRejectionReason, InboundAdmissionSlotClass,
        InboundHandshakeState, InboundPeerRecord, InboundPermissionDecision, PeerConnectionClass,
    },
};

use super::{ConnectionRole, PeerAction, PeerManager, PeerState};

impl PeerManager {
    pub fn inbound_endpoint_keys(&self) -> BTreeSet<String> {
        self.peers
            .values()
            .filter_map(active_inbound_record)
            .map(|record| record.remote_endpoint.clone())
            .collect()
    }

    pub fn inbound_admission_counters(&self) -> InboundAdmissionCounters {
        let mut counters = InboundAdmissionCounters::default();
        for peer in self.peers.values() {
            match peer.role {
                ConnectionRole::Inbound => {
                    let Some(record) = active_inbound_record(peer) else {
                        continue;
                    };
                    counters.current_inbound_peers += 1;
                    if record.slot_class == InboundAdmissionSlotClass::Reserved {
                        counters.current_reserved_inbound_peers += 1;
                    }
                }
                ConnectionRole::Outbound => counters.current_outbound_peers += 1,
            }
        }
        counters
    }

    pub fn add_inbound_peer(&mut self, peer_id: PeerId) -> Result<(), NetworkError> {
        let counters = self.inbound_admission_counters();
        let record = InboundPeerRecord {
            peer_id,
            remote_endpoint: format!("compat-inbound-peer:{peer_id}"),
            slot_class: InboundAdmissionSlotClass::Ordinary,
            connection_class: PeerConnectionClass::OrdinaryInbound,
            permission_decision: InboundPermissionDecision::ordinary(),
            handshake_state: InboundHandshakeState::Accepted,
            maybe_remote_nonce: None,
            observed_inbound_peers: counters.current_inbound_peers,
            observed_outbound_peers: counters.current_outbound_peers,
        };
        self.add_inbound_peer_record(record)
    }

    pub fn add_inbound_peer_record(
        &mut self,
        record: InboundPeerRecord,
    ) -> Result<(), NetworkError> {
        if self.peers.contains_key(&record.peer_id) {
            return Err(NetworkError::PeerAlreadyExists(record.peer_id));
        }
        self.peers
            .insert(record.peer_id, PeerState::from_inbound_record(record));
        Ok(())
    }
}

fn active_inbound_record(peer: &PeerState) -> Option<&InboundPeerRecord> {
    let maybe_record = peer.maybe_inbound_record.as_ref()?;
    if maybe_record.handshake_state == InboundHandshakeState::Disconnected {
        return None;
    }
    Some(maybe_record)
}

pub(super) fn reject_self_connection(peer: &mut PeerState, remote_nonce: u64) -> PeerAction {
    if let Some(record) = peer.maybe_inbound_record.as_mut() {
        record.handshake_state = InboundHandshakeState::Disconnected;
        record.maybe_remote_nonce = Some(remote_nonce);
    }
    peer.maybe_inbound_rejection_reason = Some(InboundAdmissionRejectionReason::SelfConnection);

    PeerAction::Disconnect(DisconnectReason::SelfConnection)
}
