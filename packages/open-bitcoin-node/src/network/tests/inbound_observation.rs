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
fn managed_inbound_admission_increments_inbound_counts() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(201),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 0));

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        201,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 1);
    assert_eq!(info.inbound_peers, 1);
    assert_eq!(info.outbound_peers, 0);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.admitted_inbound_peers, 1);
    assert_eq!(admission.ordinary_inbound_admits, 1);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.active_permission_effect_observations, 0);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
    assert_eq!(admission.rejected_inbound_peers, 0);
}

#[test]
fn permissioned_inbound_admission_counts_effects_without_reserved_capacity() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(211),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));

    // Act
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        211,
        "127.0.0.1:18446",
        &["in", "download", "addr", "relay", "mempool"],
    ));

    // Assert
    let InboundAdmissionDecision::Admit(record) = decision else {
        panic!("expected permissioned inbound admission");
    };
    assert_eq!(
        record.connection_class,
        PeerConnectionClass::PermissionedInbound,
    );
    assert_eq!(record.slot_class, InboundAdmissionSlotClass::Ordinary);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.admitted_inbound_peers, 1);
    assert_eq!(admission.ordinary_inbound_admits, 0);
    assert_eq!(admission.permissioned_inbound_admits, 1);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.reserved_inbound_admits, 0);
    assert_eq!(admission.active_permission_effect_observations, 2);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
}
