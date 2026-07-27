// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn address_announcement(
    time_unix_seconds: u64,
    address: NetworkAddress,
) -> AddressAnnouncement {
    AddressAnnouncement {
        time_unix_seconds: time_unix_seconds as u32,
        address,
    }
}

pub(super) fn public_ipv4_network_address(a: u8, b: u8, c: u8, d: u8, port: u16) -> NetworkAddress {
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: ipv4_mapped_address_bytes([a, b, c, d]),
        port,
    }
}

pub(super) fn public_ipv6_network_address(raw_address: &str, port: u16) -> NetworkAddress {
    let address: core::net::Ipv6Addr = raw_address.parse().expect("test IPv6 should parse");
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: address.octets(),
        port,
    }
}

pub(super) fn local_advertisement_candidate(address: NetworkAddress) -> LocalAdvertisementDecision {
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseCandidate,
        reason: AddressDecisionReason::PolicyAccepted,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: address.services,
        port: address.port,
        maybe_wire_address: Some(address),
    }
}

pub(super) fn local_advertisement_suppressed(
    address: NetworkAddress,
    reason: AddressDecisionReason,
) -> LocalAdvertisementDecision {
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseSuppressed,
        reason,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: address.services,
        port: address.port,
        maybe_wire_address: None,
    }
}

pub(super) fn assert_no_addr_actions(actions: &[PeerAction]) {
    assert!(
        actions
            .iter()
            .all(|action| !matches!(action, PeerAction::Send(WireNetworkMessage::Addr(_)))),
    );
}

pub(super) fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[..12].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]);
    bytes[12..].copy_from_slice(&octets);
    bytes
}
