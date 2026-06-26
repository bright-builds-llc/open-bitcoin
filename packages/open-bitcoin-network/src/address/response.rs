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

use crate::inbound::PermissionEffectLabel;

use super::{
    AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind, AddressSourceKind,
    LearnedAddressEntry, LocalAdvertisementDecision, PHASE92_MAX_ADDR_AGE_SECONDS,
    PHASE92_MAX_FUTURE_SKEW_SECONDS, RoutabilityClass,
};

pub const PHASE92_GETADDR_RESPONSE_LIMIT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAddrPeerEligibility {
    pub is_inbound: bool,
    pub has_address_response_policy_input: bool,
}

impl GetAddrPeerEligibility {
    pub const fn new(is_inbound: bool, has_address_response_policy_input: bool) -> Self {
        Self {
            is_inbound,
            has_address_response_policy_input,
        }
    }

    pub fn from_permission_effects(is_inbound: bool, effects: &[PermissionEffectLabel]) -> Self {
        Self::new(
            is_inbound,
            effects.contains(&PermissionEffectLabel::AddressResponsePolicyInput),
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetAddrRequestState {
    pub served: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressResponseEntryEvidence {
    pub address: NetworkAddress,
    pub network_kind: AddressNetworkKind,
    pub source: AddressSourceKind,
    pub first_seen_unix_seconds: u64,
    pub last_seen_unix_seconds: u64,
    pub services_bits: u64,
    pub port: u16,
    pub routability: RoutabilityClass,
    pub persistence_eligible: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AddressResponseCache {
    entries: Vec<AddressResponseEntryEvidence>,
}

impl AddressResponseCache {
    pub fn from_entries(entries: Vec<AddressResponseEntryEvidence>) -> Self {
        Self { entries }
    }

    pub fn from_sources(
        local_decisions: &[LocalAdvertisementDecision],
        learned_entries: &[LearnedAddressEntry],
        local_seen_unix_seconds: u64,
    ) -> Self {
        let entries = local_decisions
            .iter()
            .filter_map(|decision| {
                AddressResponseEntryEvidence::from_local_decision(decision, local_seen_unix_seconds)
            })
            .chain(
                learned_entries
                    .iter()
                    .map(AddressResponseEntryEvidence::from_learned_entry),
            )
            .collect();

        Self { entries }
    }

    pub fn entries(&self) -> &[AddressResponseEntryEvidence] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetAddrResponseDecision {
    Served {
        label: AddressDecisionLabel,
        reason: AddressDecisionReason,
        entries: Vec<AddressResponseEntryEvidence>,
    },
    Suppressed {
        label: AddressDecisionLabel,
        reason: AddressDecisionReason,
    },
}

pub fn select_getaddr_response(
    eligibility: GetAddrPeerEligibility,
    request_state: &mut GetAddrRequestState,
    cache: &AddressResponseCache,
    now_unix_seconds: u64,
) -> GetAddrResponseDecision {
    if !eligibility.is_inbound {
        return suppressed(AddressDecisionReason::NotInbound);
    }

    if !eligibility.has_address_response_policy_input {
        return suppressed(AddressDecisionReason::PermissionPolicyDenied);
    }

    if request_state.served {
        return suppressed(AddressDecisionReason::AlreadyServed);
    }

    let entries: Vec<_> = cache
        .entries()
        .iter()
        .filter(|entry| is_selectable_entry(entry, now_unix_seconds))
        .take(PHASE92_GETADDR_RESPONSE_LIMIT)
        .cloned()
        .collect();

    if entries.is_empty() {
        return suppressed(AddressDecisionReason::EmptyResponseCache);
    }

    request_state.served = true;
    GetAddrResponseDecision::Served {
        label: AddressDecisionLabel::GetAddrServed,
        reason: AddressDecisionReason::PolicyAccepted,
        entries,
    }
}

impl AddressResponseEntryEvidence {
    fn from_local_decision(
        decision: &LocalAdvertisementDecision,
        local_seen_unix_seconds: u64,
    ) -> Option<Self> {
        if decision.label != AddressDecisionLabel::AdvertiseCandidate {
            return None;
        }

        let address = decision.maybe_wire_address.clone()?;
        Some(Self {
            address,
            network_kind: decision.network_kind,
            source: decision.source,
            first_seen_unix_seconds: local_seen_unix_seconds,
            last_seen_unix_seconds: local_seen_unix_seconds,
            services_bits: decision.services_bits,
            port: decision.port,
            routability: decision.routability,
            persistence_eligible: false,
        })
    }

    fn from_learned_entry(entry: &LearnedAddressEntry) -> Self {
        Self {
            address: entry.address.clone(),
            network_kind: entry.network_kind,
            source: entry.source,
            first_seen_unix_seconds: entry.first_seen_unix_seconds,
            last_seen_unix_seconds: entry.last_seen_unix_seconds,
            services_bits: entry.services_bits,
            port: entry.address.port,
            routability: entry.routability,
            persistence_eligible: entry.persistence_eligible,
        }
    }
}

fn suppressed(reason: AddressDecisionReason) -> GetAddrResponseDecision {
    GetAddrResponseDecision::Suppressed {
        label: AddressDecisionLabel::GetAddrSuppressed,
        reason,
    }
}

fn is_selectable_entry(entry: &AddressResponseEntryEvidence, now_unix_seconds: u64) -> bool {
    entry.routability == RoutabilityClass::PubliclyRoutable
        && entry.last_seen_unix_seconds
            <= now_unix_seconds.saturating_add(PHASE92_MAX_FUTURE_SKEW_SECONDS)
        && now_unix_seconds.saturating_sub(entry.last_seen_unix_seconds)
            <= PHASE92_MAX_ADDR_AGE_SECONDS
}
