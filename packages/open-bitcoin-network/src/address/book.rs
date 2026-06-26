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

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use open_bitcoin_primitives::NetworkAddress;

use crate::ServiceFlags;

use super::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind,
    AddressSourceKind, RoutabilityClass, classify_network_address,
};

const IPV4_IN_IPV6_PREFIX: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
];

pub const PHASE92_LEARNED_ADDR_BATCH_LIMIT: usize = 64;
pub const PHASE92_MAX_ADDR_AGE_SECONDS: u64 = 30 * 24 * 60 * 60;
pub const PHASE92_MAX_FUTURE_SKEW_SECONDS: u64 = 10 * 60;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedAddressEntry {
    pub address: NetworkAddress,
    pub network_kind: AddressNetworkKind,
    pub source: AddressSourceKind,
    pub first_seen_unix_seconds: u64,
    pub last_seen_unix_seconds: u64,
    pub services_bits: u64,
    pub routability: RoutabilityClass,
    pub persistence_eligible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedAddressDecision {
    pub label: AddressDecisionLabel,
    pub reason: AddressDecisionReason,
    pub source: AddressSourceKind,
    pub network_kind: AddressNetworkKind,
    pub routability: RoutabilityClass,
    pub services_bits: u64,
    pub port: u16,
    pub persistence_eligible: bool,
    pub maybe_entry: Option<LearnedAddressEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedAddressBatchDecision {
    pub label: AddressDecisionLabel,
    pub reason: AddressDecisionReason,
    pub accepted_count: usize,
    pub rejected_count: usize,
    pub decisions: Vec<LearnedAddressDecision>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LearnedAddressBook {
    entries: Vec<LearnedAddressEntry>,
}

impl LearnedAddressBook {
    pub fn entries(&self) -> &[LearnedAddressEntry] {
        &self.entries
    }

    pub fn learn_batch(
        &mut self,
        announcements: &[AddressAnnouncement],
        source: AddressSourceKind,
        now_unix_seconds: u64,
    ) -> LearnedAddressBatchDecision {
        if announcements.len() > PHASE92_LEARNED_ADDR_BATCH_LIMIT {
            return LearnedAddressBatchDecision {
                label: AddressDecisionLabel::LearnedRejected,
                reason: AddressDecisionReason::OverCapBatch,
                accepted_count: 0,
                rejected_count: announcements.len(),
                decisions: Vec::new(),
            };
        }

        let decisions: Vec<_> = announcements
            .iter()
            .map(|announcement| self.learn_one(announcement, source, now_unix_seconds))
            .collect();
        let accepted_count = decisions
            .iter()
            .filter(|decision| decision.label == AddressDecisionLabel::LearnedAccepted)
            .count();
        let rejected_count = decisions.len() - accepted_count;

        LearnedAddressBatchDecision {
            label: batch_label(accepted_count),
            reason: batch_reason(&decisions),
            accepted_count,
            rejected_count,
            decisions,
        }
    }

    fn learn_one(
        &mut self,
        announcement: &AddressAnnouncement,
        source: AddressSourceKind,
        now_unix_seconds: u64,
    ) -> LearnedAddressDecision {
        let classification = classify_learned_address(&announcement.address);
        if !is_acceptable_classification(classification.routability) {
            return rejected_decision(source, classification.reason, &classification);
        }

        let timestamp = u64::from(announcement.time_unix_seconds);
        if is_stale_or_future(timestamp, now_unix_seconds) {
            return rejected_decision(
                source,
                AddressDecisionReason::StaleOrFuture,
                &classification,
            );
        }

        if self.has_entry_for(&announcement.address) {
            return rejected_decision(
                source,
                AddressDecisionReason::DuplicateAddress,
                &classification,
            );
        }

        let entry = LearnedAddressEntry {
            address: announcement.address.clone(),
            network_kind: classification.network_kind,
            source,
            first_seen_unix_seconds: timestamp,
            last_seen_unix_seconds: timestamp,
            services_bits: classification.services_bits,
            routability: classification.routability,
            persistence_eligible: persistence_eligible(source, classification.routability),
        };
        self.entries.push(entry.clone());

        LearnedAddressDecision {
            label: AddressDecisionLabel::LearnedAccepted,
            reason: AddressDecisionReason::PolicyAccepted,
            source,
            network_kind: entry.network_kind,
            routability: entry.routability,
            services_bits: entry.services_bits,
            port: entry.address.port,
            persistence_eligible: entry.persistence_eligible,
            maybe_entry: Some(entry),
        }
    }

    fn has_entry_for(&self, address: &NetworkAddress) -> bool {
        self.entries
            .iter()
            .any(|entry| same_network_endpoint(&entry.address, address))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LearnedAddressClassification {
    network_kind: AddressNetworkKind,
    routability: RoutabilityClass,
    reason: AddressDecisionReason,
    services_bits: u64,
    port: u16,
}

fn classify_learned_address(address: &NetworkAddress) -> LearnedAddressClassification {
    let classification = classify_network_address(
        ip_address_from_wire_address(address),
        address.port,
        ServiceFlags::from_bits(address.services),
    );

    LearnedAddressClassification {
        network_kind: classification.network_kind,
        routability: classification.routability,
        reason: classification.reason,
        services_bits: classification.services_bits,
        port: classification.port,
    }
}

fn rejected_decision(
    source: AddressSourceKind,
    reason: AddressDecisionReason,
    classification: &LearnedAddressClassification,
) -> LearnedAddressDecision {
    LearnedAddressDecision {
        label: AddressDecisionLabel::LearnedRejected,
        reason,
        source,
        network_kind: classification.network_kind,
        routability: classification.routability,
        services_bits: classification.services_bits,
        port: classification.port,
        persistence_eligible: false,
        maybe_entry: None,
    }
}

fn is_acceptable_classification(routability: RoutabilityClass) -> bool {
    routability == RoutabilityClass::PubliclyRoutable
}

fn is_stale_or_future(timestamp: u64, now_unix_seconds: u64) -> bool {
    timestamp > now_unix_seconds.saturating_add(PHASE92_MAX_FUTURE_SKEW_SECONDS)
        || now_unix_seconds.saturating_sub(timestamp) > PHASE92_MAX_ADDR_AGE_SECONDS
}

fn persistence_eligible(source: AddressSourceKind, routability: RoutabilityClass) -> bool {
    source == AddressSourceKind::InboundAddr && routability == RoutabilityClass::PubliclyRoutable
}

fn same_network_endpoint(left: &NetworkAddress, right: &NetworkAddress) -> bool {
    left.address_bytes == right.address_bytes && left.port == right.port
}

fn batch_label(accepted_count: usize) -> AddressDecisionLabel {
    if accepted_count == 0 {
        return AddressDecisionLabel::LearnedRejected;
    }

    AddressDecisionLabel::LearnedAccepted
}

fn batch_reason(decisions: &[LearnedAddressDecision]) -> AddressDecisionReason {
    if decisions
        .iter()
        .any(|decision| decision.label == AddressDecisionLabel::LearnedAccepted)
        || decisions.is_empty()
    {
        return AddressDecisionReason::PolicyAccepted;
    }

    decisions[0].reason
}

fn ip_address_from_wire_address(address: &NetworkAddress) -> IpAddr {
    if address.address_bytes[..IPV4_IN_IPV6_PREFIX.len()] == IPV4_IN_IPV6_PREFIX {
        let octets = [
            address.address_bytes[12],
            address.address_bytes[13],
            address.address_bytes[14],
            address.address_bytes[15],
        ];
        return IpAddr::V4(Ipv4Addr::from(octets));
    }

    IpAddr::V6(Ipv6Addr::from(address.address_bytes))
}
