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

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use open_bitcoin_primitives::NetworkAddress;

use crate::{InboundListenerEndpoint, PermissionEffectLabel, ServiceFlags};

use super::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind,
    AddressResponseCache, AddressResponseEntryEvidence, AddressSourceKind, GetAddrPeerEligibility,
    GetAddrRequestState, GetAddrResponseDecision, LearnedAddressBook, LocalAdvertisementInput,
    PHASE92_GETADDR_RESPONSE_LIMIT, PHASE92_LEARNED_ADDR_BATCH_LIMIT, PHASE92_MAX_ADDR_AGE_SECONDS,
    PHASE92_MAX_FUTURE_SKEW_SECONDS, RoutabilityClass, classify_network_address,
    maybe_version_sender_address, privacy_network_deferred_classification, select_getaddr_response,
    select_local_advertisement_candidates, unsupported_future_network_classification,
};

fn listener_endpoint(raw_endpoint: &str) -> InboundListenerEndpoint {
    let address = socket_addr(raw_endpoint);
    InboundListenerEndpoint {
        raw: raw_endpoint.to_string(),
        normalized: address.to_string(),
        address,
    }
}

fn socket_addr(raw_endpoint: &str) -> SocketAddr {
    raw_endpoint
        .parse()
        .expect("test listener endpoint should parse")
}

fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[10] = 0xff;
    bytes[11] = 0xff;
    bytes[12..].copy_from_slice(&octets);
    bytes
}

fn address_announcement(time_unix_seconds: u64, address: NetworkAddress) -> AddressAnnouncement {
    AddressAnnouncement {
        time_unix_seconds: u32::try_from(time_unix_seconds)
            .expect("test timestamp should fit in legacy addr timestamp"),
        address,
    }
}

fn public_ipv4_network_address(a: u8, b: u8, c: u8, d: u8, port: u16) -> NetworkAddress {
    public_ipv4_network_address_with_services(a, b, c, d, port, ServiceFlags::NETWORK)
}

fn public_ipv4_network_address_with_services(
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    port: u16,
    services: ServiceFlags,
) -> NetworkAddress {
    NetworkAddress {
        services: services.bits(),
        address_bytes: ipv4_mapped_address_bytes([a, b, c, d]),
        port,
    }
}

fn public_ipv6_network_address(raw_address: &str, port: u16) -> NetworkAddress {
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: raw_address
            .parse::<Ipv6Addr>()
            .expect("test IPv6 address should parse")
            .octets(),
        port,
    }
}

fn response_entry(
    address: NetworkAddress,
    source: AddressSourceKind,
    last_seen_unix_seconds: u64,
) -> AddressResponseEntryEvidence {
    AddressResponseEntryEvidence {
        network_kind: AddressNetworkKind::Ipv4,
        source,
        first_seen_unix_seconds: last_seen_unix_seconds,
        last_seen_unix_seconds,
        services_bits: address.services,
        port: address.port,
        routability: RoutabilityClass::PubliclyRoutable,
        persistence_eligible: source == AddressSourceKind::InboundAddr,
        address,
    }
}

fn served_response_entries(decision: &GetAddrResponseDecision) -> &[AddressResponseEntryEvidence] {
    let GetAddrResponseDecision::Served {
        label,
        reason,
        entries,
    } = decision
    else {
        panic!("expected getaddr_served decision, got {decision:?}");
    };
    assert_eq!(*label, AddressDecisionLabel::GetAddrServed);
    assert_eq!(*reason, AddressDecisionReason::PolicyAccepted);
    entries
}

fn assert_suppressed_reason(
    decision: &GetAddrResponseDecision,
    expected_reason: AddressDecisionReason,
) {
    let GetAddrResponseDecision::Suppressed { label, reason } = decision else {
        panic!("expected getaddr_suppressed decision, got {decision:?}");
    };
    assert_eq!(*label, AddressDecisionLabel::GetAddrSuppressed);
    assert_eq!(*reason, expected_reason);
}

mod classification_cases;
mod getaddr_cases;
mod learning_cases;
