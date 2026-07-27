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

pub(super) fn assert_request_cap_resource_governance(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
) {
    let info = network.resource_governance_info();
    assert_eq!(info.request_cap_events, 1);
    let latest = info
        .maybe_latest_resource_governance_decision
        .as_ref()
        .expect("latest resource-governance decision");
    assert_eq!(latest.label, "request_cap_reached");
    assert_eq!(latest.next_action, "request_cap_reached");
}

#[test]
fn cap_rejected_inbound_peer_updates_evidence_without_counts() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(202),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(1, 0));
    network.admit_inbound_peer(inbound_request(
        202,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        203,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(
        decision,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::CapReached
    ));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 1);
    assert_eq!(info.inbound_peers, 1);
    assert_eq!(network.inbound_admission_info().rejected_inbound_peers, 1);
    assert_eq!(network.inbound_admission_info().cap_rejections, 1);
}

#[test]
fn managed_network_records_request_cap_event_for_over_cap_inv() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(240),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(240).expect("inbound peer");

    // Act
    let error = network
        .receive_message(
            240,
            WireNetworkMessage::Inv(transaction_inventory(
                PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER + 1,
            )),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap inv should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(240))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn managed_network_records_request_cap_event_for_over_cap_getdata() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(241),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(241).expect("inbound peer");

    // Act
    let error = network
        .receive_message(
            241,
            WireNetworkMessage::GetData(block_inventory(
                PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
            )),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap getdata should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(241))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn managed_network_records_request_cap_event_for_over_cap_getheaders() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(242),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(242).expect("inbound peer");
    let locator = open_bitcoin_core::primitives::BlockLocator {
        block_hashes: (0..=PHASE94_MAX_HEADER_LOCATOR_HASHES)
            .map(hash_from_index)
            .collect(),
    };

    // Act
    let error = network
        .receive_message(
            242,
            WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            },
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap getheaders should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(242))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}
