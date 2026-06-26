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

use core::net::SocketAddr;

use open_bitcoin_primitives::NetworkAddress;

use crate::{InboundListenerEndpoint, ServiceFlags};

use super::{
    AddressClassification, AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind,
    AddressSourceKind, RoutabilityClass, classify_network_address,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAdvertisementInput {
    pub listener_endpoint: InboundListenerEndpoint,
    pub maybe_bound_addr: Option<SocketAddr>,
    pub services: ServiceFlags,
    pub allow_public: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAdvertisementDecision {
    pub label: AddressDecisionLabel,
    pub reason: AddressDecisionReason,
    pub source: AddressSourceKind,
    pub network_kind: AddressNetworkKind,
    pub routability: RoutabilityClass,
    pub services_bits: u64,
    pub port: u16,
    pub maybe_wire_address: Option<NetworkAddress>,
}

pub fn select_local_advertisement_candidates(
    inputs: &[LocalAdvertisementInput],
) -> Vec<LocalAdvertisementDecision> {
    inputs.iter().map(select_local_advertisement).collect()
}

pub fn maybe_version_sender_address(
    decisions: &[LocalAdvertisementDecision],
) -> Option<NetworkAddress> {
    decisions
        .iter()
        .find(|decision| decision.label == AddressDecisionLabel::AdvertiseCandidate)
        .and_then(|decision| decision.maybe_wire_address.clone())
}

fn select_local_advertisement(input: &LocalAdvertisementInput) -> LocalAdvertisementDecision {
    let endpoint = input
        .maybe_bound_addr
        .unwrap_or(input.listener_endpoint.address);
    let classification = classify_network_address(endpoint.ip(), endpoint.port(), input.services);

    if input.allow_public && can_advertise(&classification) {
        return LocalAdvertisementDecision {
            label: AddressDecisionLabel::AdvertiseCandidate,
            reason: AddressDecisionReason::PolicyAccepted,
            source: AddressSourceKind::LocalListener,
            network_kind: classification.network_kind,
            routability: classification.routability,
            services_bits: classification.services_bits,
            port: classification.port,
            maybe_wire_address: classification.maybe_wire_address,
        };
    }

    let reason = suppressed_reason(input, &classification);
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseSuppressed,
        reason,
        source: AddressSourceKind::LocalListener,
        network_kind: classification.network_kind,
        routability: classification.routability,
        services_bits: classification.services_bits,
        port: classification.port,
        maybe_wire_address: None,
    }
}

fn can_advertise(classification: &AddressClassification) -> bool {
    classification.routability == RoutabilityClass::PubliclyRoutable
        && classification.maybe_wire_address.is_some()
}

fn suppressed_reason(
    input: &LocalAdvertisementInput,
    classification: &AddressClassification,
) -> AddressDecisionReason {
    if !input.allow_public && classification.routability == RoutabilityClass::PubliclyRoutable {
        return AddressDecisionReason::PermissionPolicyDenied;
    }

    classification.reason
}
