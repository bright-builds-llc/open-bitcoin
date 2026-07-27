// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use super::*;

#[test]
fn ordinary_inbound_admission_cannot_consume_reserved_capacity() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(204),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));
    network.admit_inbound_peer(inbound_request(
        204,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        205,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(
        decision,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::ReservedSlotUnavailable
    ));
    assert_eq!(network.network_info().inbound_peers, 1);
    assert_eq!(network.inbound_admission_info().reserved_slot_rejections, 1,);
}

#[test]
fn reserved_inbound_admission_uses_reserved_capacity_then_rejects_when_exhausted() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(206),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));

    // Act
    let admitted = network.admit_inbound_peer(inbound_request(
        206,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Reserved,
    ));
    let rejected = network.admit_inbound_peer(inbound_request(
        207,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Reserved,
    ));

    // Assert
    assert!(matches!(admitted, InboundAdmissionDecision::Admit(_)));
    assert!(matches!(
        rejected,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::ReservedSlotUnavailable
    ));
    let admission = network.inbound_admission_info();
    assert_eq!(admission.reserved_inbound_admits, 1);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 1);
    assert_eq!(admission.active_permission_effect_observations, 4);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
    assert_eq!(admission.reserved_slot_rejections, 1);
    assert_eq!(network.network_info().inbound_peers, 1);
}

#[test]
fn inbound_admission_preserves_outbound_count_and_observed_outbound_evidence() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(208),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 0));
    network
        .connect_outbound_peer(208, 1)
        .expect("outbound peer");

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        209,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 2);
    assert_eq!(info.inbound_peers, 1);
    assert_eq!(info.outbound_peers, 1);
    let inbound_record = network
        .peer_manager()
        .peer_state(209)
        .and_then(|peer| peer.maybe_inbound_record.as_ref())
        .expect("inbound record");
    assert_eq!(inbound_record.observed_outbound_peers, 1);
}

#[test]
fn permissioned_and_protected_inbound_admits_do_not_starve_outbound_accounting() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(212),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(3, 1));
    network
        .connect_outbound_peer(212, 1)
        .expect("outbound peer");

    // Act
    let permissioned = network.admit_inbound_peer(permissioned_inbound_request(
        213,
        "127.0.0.1:18447",
        &["in", "download", "addr"],
    ));
    let protected = network.admit_inbound_peer(inbound_request(
        214,
        "127.0.0.1:18448",
        InboundAdmissionSlotClass::Reserved,
    ));

    // Assert
    assert!(matches!(permissioned, InboundAdmissionDecision::Admit(_)));
    assert!(matches!(protected, InboundAdmissionDecision::Admit(_)));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 3);
    assert_eq!(info.inbound_peers, 2);
    assert_eq!(info.outbound_peers, 1);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.permissioned_inbound_admits, 1);
    assert_eq!(admission.protected_inbound_admits, 1);
    assert_eq!(admission.reserved_inbound_admits, 1);
    assert_eq!(admission.active_permission_effect_observations, 6);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
}

#[test]
fn duplicate_inbound_endpoint_or_peer_id_rejects_before_counts_change() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(210),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(4, 0));
    network.admit_inbound_peer(inbound_request(
        210,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Act
    let duplicate_endpoint = network.admit_inbound_peer(inbound_request(
        211,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));
    let duplicate_peer_id = network.admit_inbound_peer(inbound_request(
        210,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(
        duplicate_endpoint,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::DuplicateEndpoint
    ));
    assert!(matches!(
        duplicate_peer_id,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::DuplicatePeerId
    ));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 1);
    assert_eq!(info.inbound_peers, 1);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.duplicate_endpoint_rejections, 1);
    assert_eq!(admission.duplicate_identity_rejections, 1);
    assert_eq!(admission.rejected_inbound_peers, 2);
}
