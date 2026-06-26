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

use crate::message::ServiceFlags;

pub mod advertisement;

pub use advertisement::{
    LocalAdvertisementDecision, LocalAdvertisementInput, maybe_version_sender_address,
    select_local_advertisement_candidates,
};

const IPV4_IN_IPV6_PREFIX: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressNetworkKind {
    Ipv4,
    Ipv6,
    UnsupportedPrivacyNetwork,
    UnsupportedFutureNetwork,
}

impl AddressNetworkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::UnsupportedPrivacyNetwork => "unsupported_privacy_network",
            Self::UnsupportedFutureNetwork => "unsupported_future_network",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutabilityClass {
    PubliclyRoutable,
    NotPubliclyRoutable,
    Invalid,
    PrivacyNetworkDeferred,
}

impl RoutabilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PubliclyRoutable => "publicly_routable",
            Self::NotPubliclyRoutable => "not_publicly_routable",
            Self::Invalid => "invalid",
            Self::PrivacyNetworkDeferred => "privacy_network_deferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSourceKind {
    LocalListener,
    InboundAddr,
}

impl AddressSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalListener => "source_local_listener",
            Self::InboundAddr => "source_inbound_addr",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressDecisionLabel {
    AdvertiseCandidate,
    AdvertiseSuppressed,
    LearnedAccepted,
    LearnedRejected,
    GetAddrServed,
    GetAddrSuppressed,
    FullRelayDeferred,
}

impl AddressDecisionLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvertiseCandidate => "advertise_candidate",
            Self::AdvertiseSuppressed => "advertise_suppressed",
            Self::LearnedAccepted => "learned_accepted",
            Self::LearnedRejected => "learned_rejected",
            Self::GetAddrServed => "getaddr_served",
            Self::GetAddrSuppressed => "getaddr_suppressed",
            Self::FullRelayDeferred => "full_relay_deferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressDecisionReason {
    PolicyAccepted,
    NotPubliclyRoutable,
    PrivacyNetworkDeferred,
    UnsupportedAddressNetwork,
    InvalidPort,
    StaleOrFuture,
    DuplicateAddress,
    OverCapBatch,
    NotInbound,
    PermissionPolicyDenied,
    AlreadyServed,
    EmptyResponseCache,
}

impl AddressDecisionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyAccepted => "policy_accepted",
            Self::NotPubliclyRoutable => "not_publicly_routable",
            Self::PrivacyNetworkDeferred => "privacy_network_deferred",
            Self::UnsupportedAddressNetwork => "unsupported_address_network",
            Self::InvalidPort => "invalid_port",
            Self::StaleOrFuture => "stale_or_future",
            Self::DuplicateAddress => "duplicate_address",
            Self::OverCapBatch => "over_cap_batch",
            Self::NotInbound => "not_inbound",
            Self::PermissionPolicyDenied => "permission_policy_denied",
            Self::AlreadyServed => "already_served",
            Self::EmptyResponseCache => "empty_response_cache",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressClassification {
    pub network_kind: AddressNetworkKind,
    pub routability: RoutabilityClass,
    pub reason: AddressDecisionReason,
    pub services_bits: u64,
    pub port: u16,
    pub maybe_wire_address: Option<NetworkAddress>,
}

impl AddressClassification {
    const fn suppressed(
        network_kind: AddressNetworkKind,
        routability: RoutabilityClass,
        reason: AddressDecisionReason,
        services_bits: u64,
        port: u16,
    ) -> Self {
        Self {
            network_kind,
            routability,
            reason,
            services_bits,
            port,
            maybe_wire_address: None,
        }
    }
}

pub fn classify_network_address(
    address: IpAddr,
    port: u16,
    services: ServiceFlags,
) -> AddressClassification {
    let network_kind = network_kind_for_ip(address);
    let services_bits = services.bits();

    if port == 0 {
        return AddressClassification::suppressed(
            network_kind,
            RoutabilityClass::Invalid,
            AddressDecisionReason::InvalidPort,
            services_bits,
            port,
        );
    }

    if !is_publicly_routable(address) {
        return AddressClassification::suppressed(
            network_kind,
            RoutabilityClass::NotPubliclyRoutable,
            AddressDecisionReason::NotPubliclyRoutable,
            services_bits,
            port,
        );
    }

    AddressClassification {
        network_kind,
        routability: RoutabilityClass::PubliclyRoutable,
        reason: AddressDecisionReason::PolicyAccepted,
        services_bits,
        port,
        maybe_wire_address: Some(NetworkAddress {
            services: services_bits,
            address_bytes: address_bytes(address),
            port,
        }),
    }
}

pub fn privacy_network_deferred_classification(services: ServiceFlags) -> AddressClassification {
    AddressClassification::suppressed(
        AddressNetworkKind::UnsupportedPrivacyNetwork,
        RoutabilityClass::PrivacyNetworkDeferred,
        AddressDecisionReason::PrivacyNetworkDeferred,
        services.bits(),
        0,
    )
}

pub fn unsupported_future_network_classification(services: ServiceFlags) -> AddressClassification {
    AddressClassification::suppressed(
        AddressNetworkKind::UnsupportedFutureNetwork,
        RoutabilityClass::Invalid,
        AddressDecisionReason::UnsupportedAddressNetwork,
        services.bits(),
        0,
    )
}

fn network_kind_for_ip(address: IpAddr) -> AddressNetworkKind {
    match address {
        IpAddr::V4(_) => AddressNetworkKind::Ipv4,
        IpAddr::V6(_) => AddressNetworkKind::Ipv6,
    }
}

fn is_publicly_routable(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => is_publicly_routable_ipv4(address),
        IpAddr::V6(address) => is_publicly_routable_ipv6(address),
    }
}

fn is_publicly_routable_ipv4(address: Ipv4Addr) -> bool {
    let octets = address.octets();
    !(octets[0] == 0
        || address.is_private()
        || address.is_loopback()
        || address.is_link_local()
        || address.is_multicast()
        || address.is_documentation()
        || address.is_broadcast()
        || is_rfc2544_ipv4(octets)
        || is_rfc6598_ipv4(octets))
}

fn is_publicly_routable_ipv6(address: Ipv6Addr) -> bool {
    !(address.is_unspecified()
        || address.is_loopback()
        || address.is_multicast()
        || is_unique_local_ipv6(address)
        || is_unicast_link_local_ipv6(address)
        || is_documentation_ipv6(address)
        || is_orchid_ipv6(address))
}

fn is_rfc2544_ipv4(octets: [u8; 4]) -> bool {
    octets[0] == 198 && (octets[1] == 18 || octets[1] == 19)
}

fn is_rfc6598_ipv4(octets: [u8; 4]) -> bool {
    octets[0] == 100 && (64..=127).contains(&octets[1])
}

fn is_unique_local_ipv6(address: Ipv6Addr) -> bool {
    (address.octets()[0] & 0xfe) == 0xfc
}

fn is_unicast_link_local_ipv6(address: Ipv6Addr) -> bool {
    let octets = address.octets();
    octets[0] == 0xfe && (octets[1] & 0xc0) == 0x80
}

fn is_documentation_ipv6(address: Ipv6Addr) -> bool {
    let segments = address.segments();
    segments[0] == 0x2001 && segments[1] == 0x0db8
}

fn is_orchid_ipv6(address: Ipv6Addr) -> bool {
    let segments = address.segments();
    segments[0] == 0x2001 && ((segments[1] & 0xfff0) == 0x0010 || (segments[1] & 0xfff0) == 0x0020)
}

fn address_bytes(address: IpAddr) -> [u8; 16] {
    match address {
        IpAddr::V4(address) => ipv4_mapped_address_bytes(address),
        IpAddr::V6(address) => address.octets(),
    }
}

fn ipv4_mapped_address_bytes(address: Ipv4Addr) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[..IPV4_IN_IPV6_PREFIX.len()].copy_from_slice(&IPV4_IN_IPV6_PREFIX);
    bytes[IPV4_IN_IPV6_PREFIX.len()..].copy_from_slice(&address.octets());
    bytes
}

#[cfg(test)]
mod tests;
