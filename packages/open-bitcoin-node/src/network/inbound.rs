// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_network::{
    AddressDecisionLabel, InactivePermissionEffectLabel, InboundAdmissionDecision,
    InboundAdmissionPolicy, InboundAdmissionRejection, InboundAdmissionRejectionReason,
    InboundAdmissionRequest, InboundAdmissionSlotClass, InboundHandshakeState, InboundPeerRecord,
    InboundPermissionDecision, LocalAdvertisementDecision, PeerAddressBoundaryDecision,
    PeerAddressBoundaryEvidence, PeerConnectionClass, PeerId, PeerState, PermissionEffectLabel,
};

use crate::{
    ChainstateStore,
    status::{InboundAddressDecisionEvent, InboundAddressEvidenceEntry},
};

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
pub struct ManagedAddressBoundaryInfo {
    pub local_advertisement_candidates: Vec<InboundAddressEvidenceEntry>,
    pub suppressed_advertisements: Vec<InboundAddressDecisionEvent>,
    pub getaddr_responses_served: u32,
    pub getaddr_requests_suppressed: u32,
    pub learned_address_entries: u32,
    pub learned_address_rejections: u32,
    pub maybe_latest_address_decision: Option<InboundAddressDecisionEvent>,
}

impl ManagedAddressBoundaryInfo {
    pub fn is_empty(&self) -> bool {
        self.local_advertisement_candidates.is_empty()
            && self.suppressed_advertisements.is_empty()
            && self.getaddr_responses_served == 0
            && self.getaddr_requests_suppressed == 0
            && self.learned_address_entries == 0
            && self.learned_address_rejections == 0
            && self.maybe_latest_address_decision.is_none()
    }
}

impl From<PeerAddressBoundaryEvidence> for ManagedAddressBoundaryInfo {
    fn from(evidence: PeerAddressBoundaryEvidence) -> Self {
        Self {
            local_advertisement_candidates: evidence
                .local_advertisement_candidates
                .iter()
                .map(project_local_advertisement_candidate)
                .collect(),
            suppressed_advertisements: evidence
                .suppressed_advertisements
                .iter()
                .map(project_suppressed_advertisement)
                .collect(),
            getaddr_responses_served: usize_to_u32(evidence.getaddr_responses_served.len()),
            getaddr_requests_suppressed: usize_to_u32(evidence.getaddr_requests_suppressed.len()),
            learned_address_entries: usize_to_u32(evidence.learned_address_entries.len()),
            learned_address_rejections: usize_to_u32(evidence.learned_address_rejection_count),
            maybe_latest_address_decision: evidence
                .maybe_latest_address_decision
                .map(project_address_decision),
        }
    }
}

fn project_local_advertisement_candidate(
    decision: &LocalAdvertisementDecision,
) -> InboundAddressEvidenceEntry {
    InboundAddressEvidenceEntry {
        source: decision.source.as_str().to_string(),
        network_kind: decision.network_kind.as_str().to_string(),
        routability: decision.routability.as_str().to_string(),
        freshness: "fresh".to_string(),
        services_bits: decision.services_bits,
        port: decision.port,
        persistence_eligible: false,
    }
}

fn project_suppressed_advertisement(
    decision: &LocalAdvertisementDecision,
) -> InboundAddressDecisionEvent {
    let label = decision.label.as_str();
    let reason = decision.reason.as_str();
    InboundAddressDecisionEvent {
        outcome: address_decision_outcome(decision.label).to_string(),
        reason: reason.to_string(),
        label: label.to_string(),
        source: decision.source.as_str().to_string(),
        message: format!("address boundary decision {label}: {reason}"),
    }
}

fn project_address_decision(decision: PeerAddressBoundaryDecision) -> InboundAddressDecisionEvent {
    let label = decision.label.as_str();
    let reason = decision.reason.as_str();
    InboundAddressDecisionEvent {
        outcome: address_decision_outcome(decision.label).to_string(),
        reason: reason.to_string(),
        label: label.to_string(),
        source: address_decision_source(decision.label).to_string(),
        message: format!("address boundary decision {label}: {reason}"),
    }
}

fn address_decision_outcome(label: AddressDecisionLabel) -> &'static str {
    match label {
        AddressDecisionLabel::AdvertiseCandidate | AddressDecisionLabel::LearnedAccepted => {
            "accepted"
        }
        AddressDecisionLabel::AdvertiseSuppressed | AddressDecisionLabel::GetAddrSuppressed => {
            "suppressed"
        }
        AddressDecisionLabel::GetAddrServed => "served",
        AddressDecisionLabel::LearnedRejected => "rejected",
        AddressDecisionLabel::FullRelayDeferred => "deferred",
    }
}

fn address_decision_source(label: AddressDecisionLabel) -> &'static str {
    match label {
        AddressDecisionLabel::AdvertiseCandidate | AddressDecisionLabel::AdvertiseSuppressed => {
            "source_local_listener"
        }
        AddressDecisionLabel::GetAddrServed
        | AddressDecisionLabel::GetAddrSuppressed
        | AddressDecisionLabel::LearnedAccepted
        | AddressDecisionLabel::LearnedRejected
        | AddressDecisionLabel::FullRelayDeferred => "source_inbound_addr",
    }
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
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
    pub duplicate_identity_rejections: usize,
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
        self.maybe_latest_permission_decision = None;
        match rejection.reason {
            InboundAdmissionRejectionReason::CapReached => self.cap_rejections += 1,
            InboundAdmissionRejectionReason::ReservedSlotUnavailable => {
                self.reserved_slot_rejections += 1;
            }
            InboundAdmissionRejectionReason::DuplicateEndpoint => {
                self.duplicate_endpoint_rejections += 1;
            }
            InboundAdmissionRejectionReason::DuplicatePeerId => {
                self.duplicate_identity_rejections += 1;
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
        self.observed_active_permission_effects.sort();
        self.observed_inactive_permission_effects.sort();
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    #[rustfmt::skip]
    pub fn inbound_admission_info(&self) -> &ManagedInboundAdmissionInfo { &self.inbound_admission_info }

    #[rustfmt::skip]
    pub fn set_inbound_admission_policy(&mut self, policy: InboundAdmissionPolicy) { self.inbound_admission_policy = policy; }

    pub fn add_inbound_peer(&mut self, identity: PeerId) -> Result<(), ManagedNetworkError> {
        self.peer_manager.add_inbound_peer(identity)?;
        self.known_peers.insert(identity);
        let maybe_record = self
            .peer_manager
            .peer_state(identity)
            .and_then(|peer| peer.maybe_inbound_record.as_ref())
            .cloned();
        if let Some(record) = maybe_record.as_ref() {
            self.inbound_admission_info.record_admit(record);
        }
        Ok(())
    }

    pub(super) fn record_runtime_self_connection_rejection(&mut self, identity: PeerId) {
        let Some(record) = self
            .peer_manager
            .peer_state(identity)
            .and_then(|peer| peer.maybe_inbound_record.as_ref())
        else {
            return;
        };

        let rejection = InboundAdmissionRejection::runtime_self_connection(record);
        self.inbound_admission_info.record_rejection(&rejection);
    }

    pub fn admit_inbound_peer(
        &mut self,
        mut request: InboundAdmissionRequest,
    ) -> InboundAdmissionDecision {
        request.counters = self.peer_manager.inbound_admission_counters();
        request.existing_endpoint_keys = self.peer_manager.inbound_endpoint_keys();
        request.set_existing_identities(self.peer_manager.identities());
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
            let rejection = InboundAdmissionRejection::duplicate_identity(&record);
            self.inbound_admission_info.record_rejection(&rejection);
            return InboundAdmissionDecision::Reject(rejection);
        }

        self.known_peers.insert(record.identity());
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
