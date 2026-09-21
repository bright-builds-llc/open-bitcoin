// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn announcement_transport_emission_tx_inventory_sets_kind_without_compact_reason() {
    // Arrange
    let peer_id: open_bitcoin_network::PeerId = 136_301;
    let member = sample_member();

    // Act
    let emission = PeerEmission::try_new_tx_inventory(
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
        member,
        sample_effect_capability(peer_id),
    )
    .expect("tx inventory emission");
    let (_, message, capability) = emission.into_parts();
    let receipt = capability.acknowledge_write();

    // Assert
    assert!(matches!(message, WireNetworkMessage::Inv(_)));
    assert!(receipt.is_transaction_inventory());
    assert!(!receipt.is_transaction_response());
    assert_eq!(receipt.maybe_block_hash(), None);
    assert_eq!(receipt.maybe_member(), Some(member));
    assert_eq!(receipt.maybe_evidence_reason(), None);
}

#[test]
fn announcement_transport_emission_tx_response_sets_kind_without_compact_reason() {
    // Arrange
    let peer_id: open_bitcoin_network::PeerId = 136_302;
    let member = sample_member();

    // Act
    let emission = PeerEmission::try_new_tx_response(
        peer_id,
        WireNetworkMessage::Tx(Transaction::default()),
        member,
        sample_effect_capability(peer_id),
    )
    .expect("tx response emission");
    let (_, message, capability) = emission.into_parts();
    let receipt = capability.acknowledge_write();

    // Assert
    assert!(matches!(message, WireNetworkMessage::Tx(_)));
    assert!(receipt.is_transaction_response());
    assert!(!receipt.is_transaction_inventory());
    assert_eq!(receipt.maybe_block_hash(), None);
    assert_eq!(receipt.maybe_member(), Some(member));
    assert_eq!(receipt.maybe_evidence_reason(), None);
}

#[test]
fn announcement_transport_emission_tx_constructors_reject_wrong_variant_or_peer() {
    // Arrange
    let peer_id: open_bitcoin_network::PeerId = 136_303;
    let other_peer: open_bitcoin_network::PeerId = 136_304;
    let member = sample_member();
    let inv = WireNetworkMessage::Inv(InventoryList::new(Vec::new()));
    let tx = WireNetworkMessage::Tx(Transaction::default());
    let compact_capability = sample_effect_capability(peer_id);

    // Act
    let inventory_from_tx = PeerEmission::try_new_tx_inventory(
        peer_id,
        tx.clone(),
        member,
        sample_effect_capability(peer_id),
    );
    let response_from_inv = PeerEmission::try_new_tx_response(
        peer_id,
        inv.clone(),
        member,
        sample_effect_capability(peer_id),
    );
    let inventory_peer_mismatch = PeerEmission::try_new_tx_inventory(
        peer_id,
        inv,
        member,
        sample_effect_capability(other_peer),
    );
    let compact_rejects_tx = PeerEmission::new(
        peer_id,
        tx,
        BlockHash::from_byte_array([0x33; 32]),
        compact_capability,
    );

    // Assert
    assert!(inventory_from_tx.is_none());
    assert!(response_from_inv.is_none());
    assert!(inventory_peer_mismatch.is_none());
    assert!(compact_rejects_tx.is_none());
}

#[test]
fn tx_inventory_emission_does_not_increment_compact_inventory_fallback() {
    // An INV is announcement, not acknowledgement (D-01).
    // Arrange
    let peer_id = 136_311;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let before = announcement_counts(&network);
    let capability = network
        .prepare_peer_relay_effect(peer_id)
        .expect("peer effect capability");
    let emission = PeerEmission::try_new_tx_inventory(
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
        sample_member(),
        capability,
    )
    .expect("tx inventory emission");
    let (_, _, write_capability) = emission.into_parts();
    let receipt = write_capability.acknowledge_write();
    assert!(receipt.is_transaction_inventory());
    assert!(!receipt.is_transaction_response());

    // Act
    network
        .complete_peer_emission(receipt)
        .expect("complete tx inventory write");

    // Assert
    assert_eq!(
        announcement_counts(&network)["announcement"]["value"]["compact_inventory_fallback_count"],
        before["announcement"]["value"]["compact_inventory_fallback_count"]
    );
}

#[test]
fn compact_inventory_emission_still_increments_compact_inventory_fallback() {
    // Compact INV fallback remains compact evidence; an INV is still announcement, not acknowledgement (D-01).
    // Arrange
    let peer_id = 136_312;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x35; 32]);
    let (_, _, capability) =
        prepared_emission(&network, peer_id, inventory_message(block_hash), block_hash)
            .into_parts();

    // Act
    network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("complete compact inventory write");

    // Assert
    assert_eq!(
        announcement_counts(&network)["announcement"]["value"]["compact_inventory_fallback_count"],
        1
    );
}

#[test]
fn tx_response_emission_constructs_for_wire_tx_only() {
    // Arrange
    let peer_id: open_bitcoin_network::PeerId = 136_313;
    let member = sample_member();

    // Act
    let receipt = PeerEmission::try_new_tx_response(
        peer_id,
        WireNetworkMessage::Tx(Transaction::default()),
        member,
        sample_effect_capability(peer_id),
    )
    .expect("tx response emission")
    .into_parts()
    .2
    .acknowledge_write();
    let rejected_inv = PeerEmission::try_new_tx_response(
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
        member,
        sample_effect_capability(peer_id),
    );

    // Assert
    assert!(receipt.is_transaction_response());
    assert!(!receipt.is_transaction_inventory());
    assert!(rejected_inv.is_none());
}

#[test]
fn compact_constructor_rejects_wire_tx() {
    // Arrange
    let peer_id: open_bitcoin_network::PeerId = 136_314;

    // Act
    let maybe_emission = PeerEmission::new(
        peer_id,
        WireNetworkMessage::Tx(Transaction::default()),
        BlockHash::from_byte_array([0x34; 32]),
        sample_effect_capability(peer_id),
    );

    // Assert
    assert!(maybe_emission.is_none());
}
