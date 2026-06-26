// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_network::{
    AddressDecisionLabel, BanDecision, EvictionDecision, InactivePermissionEffectLabel,
    InboundAdmissionDecision, InboundAdmissionPolicy, InboundAdmissionRejection,
    InboundAdmissionRejectionReason, InboundAdmissionRequest, InboundAdmissionSlotClass,
    InboundHandshakeState, InboundPeerRecord, InboundPermissionDecision, InboundResourceEvent,
    LocalAdvertisementDecision, MisbehaviorDecision, MisbehaviorResponse,
    PeerAddressBoundaryDecision, PeerAddressBoundaryEvidence, PeerConnectionClass, PeerId,
    PeerState, PermissionEffectLabel, UnbanDecision,
};

use crate::{
    ChainstateStore,
    logging::{StructuredLogRecord, inbound_resource_governance_log_record},
    status::{
        InboundAddressDecisionEvent, InboundAddressEvidenceEntry, InboundPeerPolicyEvent,
        InboundResourceGovernanceEvent,
    },
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManagedPeerPolicyInfo {
    pub eviction_candidates_evaluated: u32,
    pub disconnects_requested: u32,
    pub discouraged_peers: u32,
    pub active_bans: u32,
    pub expired_bans: u32,
    pub manual_unbans: u32,
    pub misbehavior_observations: u32,
    pub protected_no_actions: u32,
    pub maybe_latest_peer_policy_decision: Option<InboundPeerPolicyEvent>,
}

impl ManagedPeerPolicyInfo {
    pub fn is_empty(&self) -> bool {
        self.eviction_candidates_evaluated == 0
            && self.disconnects_requested == 0
            && self.discouraged_peers == 0
            && self.active_bans == 0
            && self.expired_bans == 0
            && self.manual_unbans == 0
            && self.misbehavior_observations == 0
            && self.protected_no_actions == 0
            && self.maybe_latest_peer_policy_decision.is_none()
    }

    pub fn from_policy_decisions(
        eviction_candidate_count: usize,
        maybe_eviction: Option<EvictionDecision>,
        misbehavior_decisions: &[MisbehaviorDecision],
        ban_decisions: &[BanDecision],
        unban_decisions: &[UnbanDecision],
    ) -> Self {
        let mut info = Self {
            eviction_candidates_evaluated: usize_to_u32(eviction_candidate_count),
            misbehavior_observations: usize_to_u32(misbehavior_decisions.len()),
            ..Self::default()
        };

        if let Some(eviction) = maybe_eviction.filter(|_| eviction_candidate_count > 0) {
            info.record_eviction_decision(&eviction);
        }
        for decision in misbehavior_decisions {
            info.record_misbehavior_decision(decision);
        }
        for decision in ban_decisions {
            info.record_ban_decision(decision);
        }
        for decision in unban_decisions {
            info.record_unban_decision(decision);
        }

        info
    }

    fn record_eviction_decision(&mut self, decision: &EvictionDecision) {
        match decision {
            EvictionDecision::Select(candidate) => {
                self.disconnects_requested = self.disconnects_requested.saturating_add(1);
                self.maybe_latest_peer_policy_decision = Some(InboundPeerPolicyEvent {
                    outcome: "selected".to_string(),
                    reason: candidate.reason.as_str().to_string(),
                    label: decision.outcome_label().to_string(),
                    source: "source_eviction_policy".to_string(),
                    message: format!(
                        "peer eviction decision {}: {}",
                        decision.outcome_label(),
                        candidate.reason.as_str()
                    ),
                });
            }
            EvictionDecision::Suppress {
                reason,
                protected_peer_count,
            } => {
                self.protected_no_actions = self
                    .protected_no_actions
                    .saturating_add(usize_to_u32(*protected_peer_count));
                self.maybe_latest_peer_policy_decision = Some(InboundPeerPolicyEvent {
                    outcome: "suppressed".to_string(),
                    reason: reason.as_str().to_string(),
                    label: decision.outcome_label().to_string(),
                    source: "source_eviction_policy".to_string(),
                    message: format!(
                        "peer eviction decision {}: {}",
                        decision.outcome_label(),
                        reason.as_str()
                    ),
                });
            }
        }
    }

    fn record_misbehavior_decision(&mut self, decision: &MisbehaviorDecision) {
        match decision.response {
            MisbehaviorResponse::ObserveOnly => {}
            MisbehaviorResponse::Disconnect => {
                self.disconnects_requested = self.disconnects_requested.saturating_add(1);
            }
            MisbehaviorResponse::Discourage => {
                self.discouraged_peers = self.discouraged_peers.saturating_add(1);
            }
            MisbehaviorResponse::Ban => {
                self.active_bans = self.active_bans.saturating_add(1);
            }
            MisbehaviorResponse::ProtectedNoAction => {
                self.protected_no_actions = self.protected_no_actions.saturating_add(1);
            }
        }
        self.maybe_latest_peer_policy_decision = Some(InboundPeerPolicyEvent {
            outcome: decision.response.as_str().to_string(),
            reason: decision.kind.as_str().to_string(),
            label: "misbehavior_policy_decision".to_string(),
            source: "source_misbehavior_policy".to_string(),
            message: format!(
                "misbehavior policy decision {}: {}",
                decision.response.as_str(),
                decision.kind.as_str()
            ),
        });
    }

    fn record_ban_decision(&mut self, decision: &BanDecision) {
        let (reason, source) = match decision {
            BanDecision::Active(entry) => {
                self.active_bans = self.active_bans.saturating_add(1);
                (entry.reason.as_str(), peer_policy_source(entry.source))
            }
            BanDecision::Expired(entry) => {
                self.expired_bans = self.expired_bans.saturating_add(1);
                (entry.reason.as_str(), peer_policy_source(entry.source))
            }
        };
        self.maybe_latest_peer_policy_decision = Some(InboundPeerPolicyEvent {
            outcome: decision.outcome_label().to_string(),
            reason: reason.to_string(),
            label: decision.outcome_label().to_string(),
            source: source.to_string(),
            message: format!("ban policy decision {}: {reason}", decision.outcome_label()),
        });
    }

    fn record_unban_decision(&mut self, decision: &UnbanDecision) {
        match decision {
            UnbanDecision::Unbanned(_) => {
                self.manual_unbans = self.manual_unbans.saturating_add(1);
            }
            UnbanDecision::NotFound(_) => {}
            UnbanDecision::AlreadyExpired(_) => {
                self.expired_bans = self.expired_bans.saturating_add(1);
            }
        }
        self.maybe_latest_peer_policy_decision = Some(InboundPeerPolicyEvent {
            outcome: decision.outcome_label().to_string(),
            reason: unban_decision_reason(decision).to_string(),
            label: decision.outcome_label().to_string(),
            source: "source_unban_policy".to_string(),
            message: format!(
                "unban policy decision {}: {}",
                decision.outcome_label(),
                unban_decision_reason(decision)
            ),
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ManagedResourceGovernanceInfo {
    pub resource_pressure_events: u32,
    pub read_queue_pressure_events: u32,
    pub write_queue_pressure_events: u32,
    pub request_cap_events: u32,
    pub payload_rejections: u32,
    pub timeout_disconnects: u32,
    pub churn_rejections: u32,
    pub reconnect_suppressions: u32,
    pub maybe_latest_resource_governance_decision: Option<InboundResourceGovernanceEvent>,
}

impl ManagedResourceGovernanceInfo {
    pub fn is_empty(&self) -> bool {
        self.resource_pressure_events == 0
            && self.read_queue_pressure_events == 0
            && self.write_queue_pressure_events == 0
            && self.request_cap_events == 0
            && self.payload_rejections == 0
            && self.timeout_disconnects == 0
            && self.churn_rejections == 0
            && self.reconnect_suppressions == 0
            && self.maybe_latest_resource_governance_decision.is_none()
    }

    pub fn record_event(&mut self, event: InboundResourceEvent) {
        match event.next_action.as_str() {
            "resource_pressure_active" => {
                self.resource_pressure_events = self.resource_pressure_events.saturating_add(1);
            }
            "read_queue_pressure" => {
                self.read_queue_pressure_events = self.read_queue_pressure_events.saturating_add(1);
            }
            "write_queue_pressure" => {
                self.write_queue_pressure_events =
                    self.write_queue_pressure_events.saturating_add(1);
            }
            "request_cap_reached" => {
                self.request_cap_events = self.request_cap_events.saturating_add(1);
            }
            "payload_rejected" => {
                self.payload_rejections = self.payload_rejections.saturating_add(1);
            }
            "timeout_disconnect" => {
                self.timeout_disconnects = self.timeout_disconnects.saturating_add(1);
            }
            "churn_rejected" => {
                self.churn_rejections = self.churn_rejections.saturating_add(1);
            }
            "reconnect_suppressed" => {
                self.reconnect_suppressions = self.reconnect_suppressions.saturating_add(1);
            }
            _ => {}
        }
        self.maybe_latest_resource_governance_decision = Some(event.into());
    }

    pub fn maybe_structured_log_record(
        &self,
        timestamp_unix_seconds: u64,
    ) -> Option<StructuredLogRecord> {
        self.maybe_latest_resource_governance_decision
            .as_ref()
            .map(|event| inbound_resource_governance_log_record(event, timestamp_unix_seconds))
    }
}

impl From<InboundResourceEvent> for InboundResourceGovernanceEvent {
    fn from(event: InboundResourceEvent) -> Self {
        Self {
            outcome: event.outcome,
            reason: event.reason,
            label: event.label,
            source: event.source,
            message: event.message,
            next_action: event.next_action,
        }
    }
}

fn peer_policy_source(source: &str) -> &'static str {
    match source {
        "misbehavior_policy" => "source_misbehavior_policy",
        "manual" | "manual_ban" => "source_manual_ban",
        _ => "source_ban_policy",
    }
}

fn unban_decision_reason(decision: &UnbanDecision) -> &'static str {
    match decision {
        UnbanDecision::Unbanned(_) => "manual_unban",
        UnbanDecision::NotFound(_) => "unban_not_found",
        UnbanDecision::AlreadyExpired(_) => "ban_already_expired",
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
