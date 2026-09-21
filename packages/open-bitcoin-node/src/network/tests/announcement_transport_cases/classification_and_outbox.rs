// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn achieved_effect_projection_keeps_peer_and_block_provenance_internal() {
    // Arrange
    let peer_id = 128_216;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x28; 32]);
    let (_, _, capability) =
        prepared_emission(&network, peer_id, compact_message(), block_hash).into_parts();
    network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("complete compact write");

    // Act
    let encoded = announcement_counts(&network).to_string();

    // Assert
    assert!(!encoded.contains(&peer_id.to_string()));
    assert!(!encoded.contains("[40,40,40"));
}

#[test]
fn stale_reconnect_completion_is_classified_without_relay_credit() {
    // Arrange
    let peer_id = 128_217;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x29; 32]);
    let (_, _, capability) =
        prepared_emission(&network, peer_id, compact_message(), block_hash).into_parts();
    let before = announcement_counts(&network);
    network.disconnect_peer(peer_id).expect("disconnect peer");
    network
        .connect_outbound_peer(peer_id, 2)
        .expect("reconnect peer");

    // Act
    let completion = network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("classify stale completion");

    // Assert
    assert_eq!(completion, EffectCompletion::AchievedButStale);
    assert_eq!(announcement_counts(&network), before);
}

#[test]
fn duplicate_completion_is_classified_and_credits_evidence_once() {
    // Arrange
    let peer_id = 128_218;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x2a; 32]);
    let (_, _, capability) =
        prepared_emission(&network, peer_id, compact_message(), block_hash).into_parts();
    let receipt = capability.acknowledge_write();
    let duplicate = receipt.duplicate_for_test();

    // Act
    let first = network
        .complete_peer_emission(receipt)
        .expect("complete emission");
    let state_after_first = announcement_counts(&network);
    let peer_after_first = format!(
        "{:?}",
        network.peer_manager_snapshot().expect("peer manager")
    );
    let replay = network
        .complete_peer_emission(duplicate)
        .expect("classify duplicate");

    // Assert
    assert_eq!(first, EffectCompletion::Applied);
    assert_eq!(replay, EffectCompletion::AlreadyApplied);
    assert_eq!(announcement_counts(&network), state_after_first);
    assert_eq!(
        format!(
            "{:?}",
            network.peer_manager_snapshot().expect("peer manager")
        ),
        peer_after_first
    );
    assert_eq!(
        announcement_counts(&network)["announcement"]["value"]["compact_announced_count"],
        1
    );
}

#[test]
fn announcement_transport_emission_binds_peer_message_block_and_evidence() {
    // Arrange
    let peer_id: open_bitcoin_network::PeerId = 128_201;
    let block_hash = BlockHash::from_byte_array([0x21; 32]);
    let emission = PeerEmission::new(
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
        block_hash,
        sample_effect_capability(peer_id),
    )
    .expect("inventory emission");

    // Act
    let (actual_peer_id, message, capability) = emission.into_parts();
    let receipt = capability.acknowledge_write();

    // Assert
    assert_eq!(actual_peer_id, peer_id);
    assert!(matches!(message, WireNetworkMessage::Inv(_)));
    assert_eq!(receipt.maybe_block_hash(), Some(block_hash));
    assert_eq!(receipt.maybe_member(), None);
    assert_eq!(
        receipt.maybe_evidence_reason(),
        Some(CompactAnnouncementReason::CompactInventoryFallback)
    );
    assert!(!receipt.is_transaction_inventory());
    assert!(!receipt.is_transaction_response());
}

#[test]
fn announcement_transport_outbox_snapshot_fails_closed_at_the_cap() {
    // Arrange
    let snapshot = PeerOutboxSnapshot::new(
        128_202,
        PHASE94_MAX_PEER_QUEUED_MESSAGES,
        PHASE94_MAX_PEER_QUEUED_MESSAGES,
    );

    // Act
    let is_full = snapshot.is_full();

    // Assert
    assert!(is_full);
}
