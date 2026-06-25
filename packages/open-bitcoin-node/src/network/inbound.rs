// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_network::{
    InactivePermissionEffectLabel, InboundAdmissionDecision, InboundAdmissionPolicy,
    InboundAdmissionRejection, InboundAdmissionRejectionReason, InboundAdmissionRequest,
    InboundAdmissionSlotClass, InboundHandshakeState, InboundPeerRecord, InboundPermissionDecision,
    PeerConnectionClass, PeerState, PermissionEffectLabel,
};

use crate::ChainstateStore;

use super::{ManagedNetworkError, ManagedPeerNetwork};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedInboundPermissionDecisionInfo {
    pub connection_class: PeerConnectionClass,
    pub active_effects: Vec<PermissionEffectLabel>,
    pub inactive_effects: Vec<InactivePermissionEffectLabel>,
}

impl ManagedInboundPermissionDecisionInfo {
    fn from_decision(decision: &InboundPermissionDecision) -> Self {
        Self {
            connection_class: decision.connection_class(),
            active_effects: decision.active_effects().to_vec(),
            inactive_effects: decision.inactive_effects().to_vec(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManagedInboundAdmissionInfo {
    pub admitted_inbound_peers: usize,
    pub rejected_inbound_peers: usize,
    pub ordinary_inbound_admits: usize,
    pub permissioned_inbound_admits: usize,
    pub protected_inbound_admits: usize,
    pub reserved_inbound_admits: usize,
    pub active_permission_effect_observations: usize,
    pub inactive_permission_effect_observations: usize,
    pub observed_active_permission_effects: Vec<PermissionEffectLabel>,
    pub observed_inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    pub cap_rejections: usize,
    pub reserved_slot_rejections: usize,
    pub duplicate_endpoint_rejections: usize,
    pub duplicate_peer_id_rejections: usize,
    pub self_connection_rejections: usize,
    pub shutdown_rejections: usize,
    pub maybe_latest_rejection_reason: Option<InboundAdmissionRejectionReason>,
    pub maybe_latest_rejection_slot_class: Option<InboundAdmissionSlotClass>,
    pub maybe_latest_permission_decision: Option<ManagedInboundPermissionDecisionInfo>,
}

impl ManagedInboundAdmissionInfo {
    pub(super) fn record_admit(&mut self, record: &InboundPeerRecord) {
        self.admitted_inbound_peers += 1;
        self.maybe_latest_rejection_reason = None;
        self.maybe_latest_rejection_slot_class = None;
        match record.connection_class {
            PeerConnectionClass::OrdinaryInbound => self.ordinary_inbound_admits += 1,
            PeerConnectionClass::PermissionedInbound => self.permissioned_inbound_admits += 1,
            PeerConnectionClass::ProtectedInbound => self.protected_inbound_admits += 1,
            PeerConnectionClass::Outbound | PeerConnectionClass::ManualConfigured => {}
        }
        if record.slot_class == InboundAdmissionSlotClass::Reserved {
            self.reserved_inbound_admits += 1;
        }
        self.active_permission_effect_observations +=
            record.permission_decision.active_effects().len();
        self.inactive_permission_effect_observations +=
            record.permission_decision.inactive_effects().len();
        self.record_permission_effects(&record.permission_decision);
        self.maybe_latest_permission_decision = Some(
            ManagedInboundPermissionDecisionInfo::from_decision(&record.permission_decision),
        );
    }

    pub(super) fn record_rejection(&mut self, rejection: &InboundAdmissionRejection) {
        self.rejected_inbound_peers += 1;
        self.maybe_latest_rejection_reason = Some(rejection.reason);
        self.maybe_latest_rejection_slot_class = Some(rejection.slot_class);
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

    fn record_permission_effects(&mut self, decision: &InboundPermissionDecision) {
        for effect in decision.active_effects() {
            if !self.observed_active_permission_effects.contains(effect) {
                self.observed_active_permission_effects.push(*effect);
            }
        }
        for effect in decision.inactive_effects() {
            if !self.observed_inactive_permission_effects.contains(effect) {
                self.observed_inactive_permission_effects.push(*effect);
            }
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
        let maybe_record = self
            .peer_manager
            .peer_state(peer_id)
            .and_then(|peer| peer.maybe_inbound_record.as_ref())
            .cloned();
        if let Some(record) = maybe_record.as_ref() {
            self.inbound_admission_info.record_admit(record);
        }
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
        self.inbound_admission_info.record_admit(&record);
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
        slot_class: record.slot_class,
        maybe_endpoint: Some(record.remote_endpoint.clone()),
        message: "inbound peer id already has an admitted peer record".to_string(),
        next_action: "allocate a fresh peer id before retrying admission".to_string(),
    }
}
