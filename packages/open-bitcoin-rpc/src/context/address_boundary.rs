// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use std::net::SocketAddr;

use open_bitcoin_network::{
    AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind, AddressSourceKind,
    InboundListenerConfig, InboundListenerEndpoint, LocalAdvertisementDecision,
    LocalAdvertisementInput, RoutabilityClass, ServiceFlags, select_local_advertisement_candidates,
};

use crate::inbound_listener::InboundListenerEvidence;

pub(super) fn local_advertisement_decisions(
    config: &InboundListenerConfig,
    evidence: &InboundListenerEvidence,
    services: ServiceFlags,
) -> Vec<LocalAdvertisementDecision> {
    let configured_endpoints = configured_listener_endpoints(config);
    let mut inputs = Vec::new();
    let mut invalid_decisions = Vec::new();

    for (index, bound_endpoint) in evidence.bound_endpoints.iter().enumerate() {
        match bound_endpoint.parse::<SocketAddr>() {
            Ok(bound_addr) => inputs.push(LocalAdvertisementInput {
                listener_endpoint: listener_endpoint_for_bound(
                    bound_endpoint,
                    bound_addr,
                    configured_endpoints.get(index),
                ),
                maybe_bound_addr: Some(bound_addr),
                services,
                allow_public: config.allow_public,
            }),
            Err(_error) => invalid_decisions.push(invalid_bound_endpoint_decision(
                configured_endpoints.get(index),
                bound_endpoint,
                services,
            )),
        }
    }

    let mut decisions = select_local_advertisement_candidates(&inputs);
    decisions.extend(invalid_decisions);
    decisions
}

fn configured_listener_endpoints(config: &InboundListenerConfig) -> Vec<InboundListenerEndpoint> {
    config
        .listen_addresses
        .iter()
        .filter_map(|raw_endpoint| {
            raw_endpoint
                .trim()
                .parse::<SocketAddr>()
                .ok()
                .map(|address| InboundListenerEndpoint {
                    raw: raw_endpoint.clone(),
                    normalized: address.to_string(),
                    address,
                })
        })
        .collect()
}

fn listener_endpoint_for_bound(
    bound_endpoint: &str,
    bound_addr: SocketAddr,
    maybe_configured_endpoint: Option<&InboundListenerEndpoint>,
) -> InboundListenerEndpoint {
    maybe_configured_endpoint
        .cloned()
        .unwrap_or_else(|| InboundListenerEndpoint {
            raw: bound_endpoint.to_string(),
            normalized: bound_addr.to_string(),
            address: bound_addr,
        })
}

fn invalid_bound_endpoint_decision(
    maybe_configured_endpoint: Option<&InboundListenerEndpoint>,
    bound_endpoint: &str,
    services: ServiceFlags,
) -> LocalAdvertisementDecision {
    let reason = invalid_bound_endpoint_reason(bound_endpoint);
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseSuppressed,
        reason,
        source: AddressSourceKind::LocalListener,
        network_kind: maybe_configured_endpoint
            .map(|endpoint| {
                if endpoint.address.is_ipv4() {
                    AddressNetworkKind::Ipv4
                } else {
                    AddressNetworkKind::Ipv6
                }
            })
            .unwrap_or(AddressNetworkKind::UnsupportedFutureNetwork),
        routability: RoutabilityClass::Invalid,
        services_bits: services.bits(),
        port: maybe_configured_endpoint
            .map(|endpoint| endpoint.address.port())
            .unwrap_or(0),
        maybe_wire_address: None,
    }
}

fn invalid_bound_endpoint_reason(bound_endpoint: &str) -> AddressDecisionReason {
    let Some((_host, raw_port)) = bound_endpoint.rsplit_once(':') else {
        return AddressDecisionReason::UnsupportedAddressNetwork;
    };

    if raw_port.parse::<u16>().is_err() {
        return AddressDecisionReason::InvalidPort;
    }

    AddressDecisionReason::UnsupportedAddressNetwork
}
