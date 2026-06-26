// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netaddress.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/addrman.h
// - packages/bitcoin-knots/src/addrman.cpp
// - packages/bitcoin-knots/src/addrdb.h
// - packages/bitcoin-knots/src/addrdb.cpp

use open_bitcoin_primitives::NetworkAddress;

use crate::address::{
    AddressDecisionLabel, AddressDecisionReason, AddressList, AddressSourceKind,
    GetAddrResponseDecision, LearnedAddressDecision, LearnedAddressEntry,
    LocalAdvertisementDecision, PHASE92_LEARNED_ADDR_BATCH_LIMIT,
};
use crate::error::{NetworkError, PeerId};

use super::{PeerAction, PeerManager};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerAddressBoundaryDecision {
    pub label: AddressDecisionLabel,
    pub reason: AddressDecisionReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerAddressBoundaryEvidence {
    pub local_advertisement_candidates: Vec<LocalAdvertisementDecision>,
    pub suppressed_advertisements: Vec<LocalAdvertisementDecision>,
    pub getaddr_responses_served: Vec<GetAddrResponseDecision>,
    pub getaddr_requests_suppressed: Vec<GetAddrResponseDecision>,
    pub learned_address_entries: Vec<LearnedAddressEntry>,
    pub learned_address_rejections: Vec<LearnedAddressDecision>,
    pub maybe_latest_address_decision: Option<PeerAddressBoundaryDecision>,
}

impl PeerManager {
    pub fn address_boundary_evidence(&self) -> PeerAddressBoundaryEvidence {
        let local_advertisement_candidates = self
            .local_address_decisions
            .iter()
            .filter(|decision| decision.label == AddressDecisionLabel::AdvertiseCandidate)
            .cloned()
            .collect();
        let suppressed_advertisements = self
            .local_address_decisions
            .iter()
            .filter(|decision| decision.label == AddressDecisionLabel::AdvertiseSuppressed)
            .cloned()
            .collect();

        PeerAddressBoundaryEvidence {
            local_advertisement_candidates,
            suppressed_advertisements,
            getaddr_responses_served: self.getaddr_responses_served.clone(),
            getaddr_requests_suppressed: self.getaddr_requests_suppressed.clone(),
            learned_address_entries: self.learned_addresses.entries().to_vec(),
            learned_address_rejections: self.learned_address_rejections.clone(),
            maybe_latest_address_decision: self.maybe_latest_address_decision,
        }
    }

    pub(super) fn handle_addr(
        &mut self,
        peer_id: PeerId,
        addresses: AddressList,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if !self.peers.contains_key(&peer_id) {
            return Err(NetworkError::UnknownPeer(peer_id));
        }

        let now_unix_seconds = unix_timestamp_for_address_policy(timestamp);
        if addresses.addresses.len() > PHASE92_LEARNED_ADDR_BATCH_LIMIT {
            let batch = self.learned_addresses.learn_batch(
                &addresses.addresses,
                AddressSourceKind::InboundAddr,
                now_unix_seconds,
            );
            self.maybe_latest_address_decision =
                maybe_peer_address_decision(batch.label, batch.reason, &[]);
            return Ok(Vec::new());
        }

        let mut local_rejections = Vec::new();
        let mut learnable_addresses = Vec::new();
        for announcement in addresses.addresses {
            if let Some(rejection) = self.local_address_rejection(&announcement.address) {
                local_rejections.push(rejection);
                continue;
            }
            learnable_addresses.push(announcement);
        }

        let batch = self.learned_addresses.learn_batch(
            &learnable_addresses,
            AddressSourceKind::InboundAddr,
            now_unix_seconds,
        );
        self.learned_address_rejections.extend(
            batch
                .decisions
                .iter()
                .filter(|decision| decision.label == AddressDecisionLabel::LearnedRejected)
                .cloned(),
        );
        self.learned_address_rejections
            .extend(local_rejections.iter().cloned());

        self.maybe_latest_address_decision =
            maybe_peer_address_decision(batch.label, batch.reason, &local_rejections);

        Ok(Vec::new())
    }

    fn local_address_rejection(&self, address: &NetworkAddress) -> Option<LearnedAddressDecision> {
        let local_decision = self.local_address_decisions.iter().find(|decision| {
            decision
                .maybe_wire_address
                .as_ref()
                .is_some_and(|local_address| same_network_endpoint(local_address, address))
        })?;

        Some(LearnedAddressDecision {
            label: AddressDecisionLabel::LearnedRejected,
            reason: AddressDecisionReason::DuplicateAddress,
            source: AddressSourceKind::InboundAddr,
            network_kind: local_decision.network_kind,
            routability: local_decision.routability,
            services_bits: address.services,
            port: address.port,
            persistence_eligible: false,
            maybe_entry: None,
        })
    }
}

fn maybe_peer_address_decision(
    batch_label: AddressDecisionLabel,
    batch_reason: AddressDecisionReason,
    local_rejections: &[LearnedAddressDecision],
) -> Option<PeerAddressBoundaryDecision> {
    if batch_label == AddressDecisionLabel::LearnedRejected
        && batch_reason == AddressDecisionReason::PolicyAccepted
        && local_rejections.is_empty()
    {
        return None;
    }

    if batch_label == AddressDecisionLabel::LearnedRejected
        || batch_reason != AddressDecisionReason::PolicyAccepted
        || !local_rejections.is_empty()
    {
        let (label, reason) = local_rejections
            .last()
            .map(|decision| (decision.label, decision.reason))
            .unwrap_or((batch_label, batch_reason));
        return Some(PeerAddressBoundaryDecision { label, reason });
    }

    Some(PeerAddressBoundaryDecision {
        label: batch_label,
        reason: batch_reason,
    })
}

fn unix_timestamp_for_address_policy(timestamp: i64) -> u64 {
    if timestamp < 0 {
        return 0;
    }

    timestamp as u64
}

fn same_network_endpoint(left: &NetworkAddress, right: &NetworkAddress) -> bool {
    left.address_bytes == right.address_bytes && left.port == right.port
}
