// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use core::net::IpAddr;
use std::collections::BTreeSet;

use crate::PeerId;

use super::{
    classify_inbound_preflight, InactivePermissionEffectLabel, InboundAdmissionCounters,
    InboundAdmissionDecision, InboundAdmissionPolicy, InboundAdmissionRejectionReason,
    InboundAdmissionRequest, InboundAdmissionSlotClass, InboundHandshakeState,
    InboundListenerActivationDiagnostic, InboundListenerConfig, InboundPermissionDecision,
    InboundPreflightReason, ParsedPeerPermissionClass, PeerConnectionClass,
    PeerPermissionClassRegistry, PeerPermissionDirection, PeerPermissionSet, PeerPermissionToken,
};

fn enabled_config(addresses: Vec<&str>) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: addresses.into_iter().map(str::to_string).collect(),
        max_peers: 8,
        reserved_slots: 2,
        allow_public: false,
        permission_classes: PeerPermissionClassRegistry::default(),
    }
}

fn admission_request(
    peer_id: PeerId,
    remote_endpoint: &str,
    slot_class: InboundAdmissionSlotClass,
    counters: InboundAdmissionCounters,
) -> InboundAdmissionRequest {
    let permission_decision = match slot_class {
        InboundAdmissionSlotClass::Ordinary => InboundPermissionDecision::ordinary(),
        InboundAdmissionSlotClass::Reserved => protected_permission_decision(),
    };
    let mut request = InboundAdmissionRequest::from_permission_decision(
        peer_id,
        remote_endpoint,
        permission_decision,
    );
    request.counters = counters;
    request.existing_endpoint_keys = BTreeSet::new();
    request.existing_peer_ids = BTreeSet::new();
    request.local_nonce = 99;
    request.maybe_remote_nonce = Some(101);
    request
}

fn test_ip(raw: &str) -> IpAddr {
    match raw.parse() {
        Ok(address) => address,
        Err(error) => panic!("test IP address should parse: {error}"),
    }
}

fn permission_decision(permissions: &[&str]) -> InboundPermissionDecision {
    let class = match ParsedPeerPermissionClass::parse("test-class", ["203.0.113.7"], permissions) {
        Ok(class) => class,
        Err(error) => panic!("expected test permission class to parse: {error:?}"),
    };
    PeerPermissionClassRegistry::new([class]).resolve_inbound(test_ip("203.0.113.7"))
}

fn protected_permission_decision() -> InboundPermissionDecision {
    permission_decision(&["in", "noban", "forceinbound"])
}

fn relay_permission_labels(decision: &InboundPermissionDecision) -> Vec<&'static str> {
    decision
        .relay_permission_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
}

mod admission_policy_cases;
mod permission_policy_cases;
mod permission_validation_cases;
mod preflight_cases;
mod shutdown_admission_cases;
