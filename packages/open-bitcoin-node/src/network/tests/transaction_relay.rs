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
fn managed_network_transaction_relay_default_constructor_suppresses_getdata() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(501),
        PolicyConfig::default(),
    );
    let transaction = Transaction::default();
    let inventory = transaction_relay_inventory(&transaction);
    network
        .connect_outbound_peer(501, 1)
        .expect("outbound peer");

    // Act
    let network_info = network.network_info();
    let outbound = network
        .receive_message(
            501,
            WireNetworkMessage::Inv(inventory),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("default-off transaction inventory")
        .outbound;

    // Assert
    assert!(!network_info.relay);
    assert!(outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_enabled_outbound_translates_request_action_to_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(502);
    network
        .connect_outbound_peer(502, 1)
        .expect("txid outbound peer");
    network
        .connect_outbound_peer(503, 1)
        .expect("wtxid outbound peer");
    network
        .receive_message(
            503,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");
    let transaction = Transaction::default();
    let txid_inventory = transaction_relay_inventory(&transaction);
    let wtxid_inventory = witness_transaction_relay_inventory(&transaction);

    // Act
    let network_info = network.network_info();
    let txid_outbound = network
        .receive_message(
            502,
            WireNetworkMessage::Inv(txid_inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("txid inventory")
        .outbound;
    let wtxid_outbound = network
        .receive_message(
            503,
            WireNetworkMessage::Inv(wtxid_inventory.clone()),
            3,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxid inventory")
        .outbound;

    // Assert
    assert!(network_info.relay);
    assert_getdata(&txid_outbound, txid_inventory);
    assert_getdata(&wtxid_outbound, wtxid_inventory);
}

#[test]
fn managed_network_transaction_relay_enabled_ordinary_inbound_suppresses_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(504);
    network
        .add_inbound_peer(504)
        .expect("ordinary inbound peer");
    let inventory = transaction_relay_inventory(&Transaction::default());

    // Act
    let outbound = network
        .receive_message(
            504,
            WireNetworkMessage::Inv(inventory),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("ordinary inbound inventory")
        .outbound;

    // Assert
    assert!(outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_enabled_protected_only_inbound_suppresses_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(505);
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));
    let decision = network.admit_inbound_peer(inbound_request(
        505,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Reserved,
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let inventory = transaction_relay_inventory(&Transaction::default());

    // Act
    let outbound = network
        .receive_message(
            505,
            WireNetworkMessage::Inv(inventory),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("protected-only inbound inventory")
        .outbound;

    // Assert
    assert!(outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_duplicate_suppression_emits_no_extra_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(506);
    network.connect_outbound_peer(506, 1).expect("first peer");
    network
        .connect_outbound_peer(507, 1)
        .expect("duplicate peer");
    let inventory = transaction_relay_inventory(&Transaction::default());

    // Act
    let first_outbound = network
        .receive_message(
            506,
            WireNetworkMessage::Inv(inventory.clone()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory")
        .outbound;
    let duplicate_outbound = network
        .receive_message(
            507,
            WireNetworkMessage::Inv(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("duplicate inventory")
        .outbound;

    // Assert
    assert_getdata(&first_outbound, inventory);
    assert!(duplicate_outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_timeout_fallback_returns_getdata_for_alternate_peer() {
    // Arrange
    let mut network = relay_enabled_managed_network(508);
    network.connect_outbound_peer(508, 1).expect("first peer");
    network
        .connect_outbound_peer(509, 1)
        .expect("fallback peer");
    let inventory = transaction_relay_inventory(&Transaction::default());
    network
        .receive_message(
            508,
            WireNetworkMessage::Inv(inventory.clone()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory");
    network
        .receive_message(
            509,
            WireNetworkMessage::Inv(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("fallback inventory");

    // Act
    let fallback_messages = network
        .expire_transaction_requests(1 + PHASE101_GETDATA_TX_INTERVAL_SECONDS)
        .expect("expire requests");

    // Assert
    assert_targeted_getdata(&fallback_messages, 509, inventory);
}

#[test]
fn managed_network_transaction_relay_notfound_fallback_returns_getdata_for_alternate_peer() {
    // Arrange
    let mut network = relay_enabled_managed_network(510);
    network.connect_outbound_peer(510, 1).expect("first peer");
    network
        .connect_outbound_peer(511, 1)
        .expect("fallback peer");
    let inventory = transaction_relay_inventory(&Transaction::default());
    network
        .receive_message(
            510,
            WireNetworkMessage::Inv(inventory.clone()),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory");
    network
        .receive_message(
            511,
            WireNetworkMessage::Inv(inventory.clone()),
            11,
            verify_flags(),
            consensus_params(),
        )
        .expect("fallback inventory");

    // Act
    let result = network
        .receive_sync_message(
            510,
            WireNetworkMessage::NotFound(inventory.clone()),
            12,
            verify_flags(),
            consensus_params(),
        )
        .expect("notfound");

    // Assert
    assert!(result.outbound.is_empty());
    assert_targeted_getdata(&result.targeted_outbound, 511, inventory);
}

#[test]
fn managed_network_transaction_relay_disconnect_fallback_returns_getdata_for_alternate_peer() {
    // Arrange
    let mut network = relay_enabled_managed_network(512);
    network.connect_outbound_peer(512, 1).expect("first peer");
    network
        .connect_outbound_peer(513, 1)
        .expect("fallback peer");
    let inventory = transaction_relay_inventory(&Transaction::default());
    network
        .receive_message(
            512,
            WireNetworkMessage::Inv(inventory.clone()),
            20,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory");
    network
        .receive_message(
            513,
            WireNetworkMessage::Inv(inventory.clone()),
            21,
            verify_flags(),
            consensus_params(),
        )
        .expect("fallback inventory");

    // Act
    let fallback_messages = network
        .disconnect_peer_with_transaction_cleanup(512, 22)
        .expect("disconnect cleanup");

    // Assert
    assert_targeted_getdata(&fallback_messages, 513, inventory);
}
