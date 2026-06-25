// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_network::{
    InboundAdmissionDecision, InboundAdmissionPolicy, InboundAdmissionRejection,
    InboundAdmissionRejectionReason, InboundAdmissionRequest, InboundAdmissionSlotClass,
    InboundHandshakeState, InboundPeerRecord, PeerState,
};

use crate::ChainstateStore;

use super::{ManagedNetworkError, ManagedPeerNetwork};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManagedInboundAdmissionInfo {
    pub admitted_inbound_peers: usize,
    pub rejected_inbound_peers: usize,
    pub reserved_inbound_admits: usize,
    pub cap_rejections: usize,
    pub reserved_slot_rejections: usize,
    pub duplicate_endpoint_rejections: usize,
    pub duplicate_peer_id_rejections: usize,
    pub self_connection_rejections: usize,
    pub shutdown_rejections: usize,
    pub maybe_latest_rejection_reason: Option<InboundAdmissionRejectionReason>,
}

impl ManagedInboundAdmissionInfo {
    pub(super) fn record_admit(&mut self, slot_class: InboundAdmissionSlotClass) {
        self.admitted_inbound_peers += 1;
        if slot_class == InboundAdmissionSlotClass::Reserved {
            self.reserved_inbound_admits += 1;
        }
    }

    pub(super) fn record_rejection(&mut self, rejection: &InboundAdmissionRejection) {
        self.rejected_inbound_peers += 1;
        self.maybe_latest_rejection_reason = Some(rejection.reason);
        match rejection.reason {
            InboundAdmissionRejectionReason::CapReached => self.cap_rejections += 1,
            InboundAdmissionRejectionReason::ReservedSlotUnavailable => {
                self.reserved_slot_rejections += 1;
            }
            InboundAdmissionRejectionReason::DuplicateEndpoint => {
                self.duplicate_endpoint_rejections += 1;
            }
            InboundAdmissionRejectionReason::DuplicatePeerId => {
                self.duplicate_peer_id_rejections += 1;
            }
            InboundAdmissionRejectionReason::SelfConnection => self.self_connection_rejections += 1,
            InboundAdmissionRejectionReason::Shutdown => self.shutdown_rejections += 1,
        }
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    #[rustfmt::skip]
    pub fn inbound_admission_info(&self) -> &ManagedInboundAdmissionInfo { &self.inbound_admission_info }

    #[rustfmt::skip]
    pub fn set_inbound_admission_policy(&mut self, policy: InboundAdmissionPolicy) { self.inbound_admission_policy = policy; }

    pub fn add_inbound_peer(
        &mut self,
        peer_id: open_bitcoin_network::PeerId,
    ) -> Result<(), ManagedNetworkError> {
        self.peer_manager.add_inbound_peer(peer_id)?;
        self.peer_ids.insert(peer_id);
        self.inbound_admission_info
            .record_admit(InboundAdmissionSlotClass::Ordinary);
        Ok(())
    }

    pub fn admit_inbound_peer(
        &mut self,
        mut request: InboundAdmissionRequest,
    ) -> InboundAdmissionDecision {
        request.counters = self.peer_manager.inbound_admission_counters();
        request.existing_endpoint_keys = self.peer_manager.inbound_endpoint_keys();
        request.existing_peer_ids = self.peer_manager.peer_ids();
        request.local_nonce = self.local_config.nonce;

        let decision = self.inbound_admission_policy.decide(request);
        match decision {
            InboundAdmissionDecision::Admit(record) => self.record_inbound_admission(record),
            InboundAdmissionDecision::Reject(rejection) => {
                self.inbound_admission_info.record_rejection(&rejection);
                InboundAdmissionDecision::Reject(rejection)
            }
        }
    }

    fn record_inbound_admission(&mut self, record: InboundPeerRecord) -> InboundAdmissionDecision {
        if self
            .peer_manager
            .add_inbound_peer_record(record.clone())
            .is_err()
        {
            let rejection = duplicate_peer_rejection(&record);
            self.inbound_admission_info.record_rejection(&rejection);
            return InboundAdmissionDecision::Reject(rejection);
        }

        self.peer_ids.insert(record.peer_id);
        self.inbound_admission_info.record_admit(record.slot_class);
        InboundAdmissionDecision::Admit(record)
    }
}

pub(super) const fn default_inbound_admission_policy() -> InboundAdmissionPolicy {
    InboundAdmissionPolicy::new(usize::MAX, 0)
}

pub(super) fn is_active_inbound_peer(peer: &PeerState) -> bool {
    let Some(record) = peer.maybe_inbound_record.as_ref() else {
        return true;
    };
    record.handshake_state != InboundHandshakeState::Disconnected
}

fn duplicate_peer_rejection(record: &InboundPeerRecord) -> InboundAdmissionRejection {
    InboundAdmissionRejection {
        reason: InboundAdmissionRejectionReason::DuplicatePeerId,
        peer_id: record.peer_id,
        maybe_endpoint: Some(record.remote_endpoint.clone()),
        message: "inbound peer id already has an admitted peer record".to_string(),
        next_action: "allocate a fresh peer id before retrying admission".to_string(),
    }
}
