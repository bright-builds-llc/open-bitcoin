// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use crate::network::{
    AnnouncementPreparationOutcome, EffectCompletion, ManagedNetworkHandle, PeerEmission,
    PeerOutboxSnapshot,
};
use open_bitcoin_network::{HeadersMessage, InventoryList};

use super::*;

fn prepared_emission(
    network: &ManagedNetworkHandle,
    peer_id: open_bitcoin_network::PeerId,
    message: WireNetworkMessage,
    block_hash: BlockHash,
) -> PeerEmission {
    let capability = network
        .prepare_peer_relay_effect(peer_id)
        .expect("peer effect capability");
    PeerEmission::new(peer_id, message, block_hash, capability)
        .expect("supported announcement emission")
}

fn announcement_counts(network: &ManagedNetworkHandle) -> serde_json::Value {
    serde_json::to_value(
        network
            .block_relay_evidence_status()
            .expect("block relay evidence"),
    )
    .expect("serialize block relay evidence")
}

fn compact_message() -> WireNetworkMessage {
    WireNetworkMessage::CompactBlock(CompactBlockPayload {
        header: BlockHeader::default(),
        nonce: 7,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    })
}

fn headers_message() -> WireNetworkMessage {
    WireNetworkMessage::Headers(HeadersMessage {
        headers: vec![BlockHeader::default()],
    })
}

fn inventory_message(block_hash: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash.into(),
    }]))
}

#[test]
fn compact_success_receipt_records_achieved_effect_once() {
    // Arrange
    let peer_id = 128_210;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x21; 32]);
    let emission = prepared_emission(&network, peer_id, compact_message(), block_hash);
    let (_, _, capability) = emission.into_parts();

    // Act
    let completion = network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("complete compact write");

    // Assert
    assert_eq!(completion, EffectCompletion::Applied);
    let encoded = announcement_counts(&network);
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert!(
        network
            .peer_manager_snapshot()
            .expect("peer manager")
            .peer_state(peer_id)
            .expect("peer")
            .compact_announcements
            .contains(&block_hash)
    );
}

#[test]
fn headers_success_receipt_records_only_header_fallback() {
    // Arrange
    let peer_id = 128_211;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x22; 32]);
    let (_, _, capability) =
        prepared_emission(&network, peer_id, headers_message(), block_hash).into_parts();

    // Act
    network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("complete headers write");

    // Assert
    let encoded = announcement_counts(&network);
    assert_eq!(
        encoded["announcement"]["value"]["compact_headers_fallback_count"],
        1
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        0
    );
}

#[test]
fn inventory_success_receipt_records_only_inventory_fallback() {
    // Arrange
    let peer_id = 128_212;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x23; 32]);
    let (_, _, capability) =
        prepared_emission(&network, peer_id, inventory_message(block_hash), block_hash)
            .into_parts();

    // Act
    network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("complete inventory write");

    // Assert
    let encoded = announcement_counts(&network);
    assert_eq!(
        encoded["announcement"]["value"]["compact_inventory_fallback_count"],
        1
    );
    assert!(
        network
            .peer_manager_snapshot()
            .expect("peer manager")
            .peer_state(peer_id)
            .expect("peer")
            .compact_announcements
            .is_empty()
    );
}

#[test]
fn failed_or_unsent_emission_receives_no_achieved_effect_credit() {
    // Arrange
    let peer_id = 128_213;
    let network = compact_relay_enabled_managed_network(peer_id);
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block_hash = BlockHash::from_byte_array([0x24; 32]);
    let emission = prepared_emission(&network, peer_id, compact_message(), block_hash);
    let before = announcement_counts(&network);

    // Act
    drop(emission);

    // Assert
    assert_eq!(announcement_counts(&network), before);
}

#[test]
fn queue_full_preparation_returns_no_receipt_or_achieved_effect_credit() {
    // Arrange
    let peer_id = 128_214;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let block = Block::default();
    let outboxes = [PeerOutboxSnapshot::new(
        peer_id,
        open_bitcoin_network::PHASE94_MAX_PEER_QUEUED_MESSAGES,
        open_bitcoin_network::PHASE94_MAX_PEER_QUEUED_MESSAGES,
    )];
    let before = announcement_counts(&network);

    // Act
    let outcomes = network
        .prepare_block_announcements(&block, &outboxes)
        .expect("prepare announcements");

    // Assert
    assert!(matches!(
        outcomes.as_slice(),
        [AnnouncementPreparationOutcome::QueueFull { peer_id: 128_214 }]
    ));
    assert_eq!(announcement_counts(&network), before);
}

#[test]
fn partial_successful_prefix_credits_only_completed_receipts() {
    // Arrange
    let peer_id = 128_215;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect peer");
    let network = ManagedNetworkHandle::from_network_fixture(network);
    let hashes = [
        BlockHash::from_byte_array([0x25; 32]),
        BlockHash::from_byte_array([0x26; 32]),
        BlockHash::from_byte_array([0x27; 32]),
    ];
    let emissions = [
        prepared_emission(&network, peer_id, compact_message(), hashes[0]),
        prepared_emission(&network, peer_id, headers_message(), hashes[1]),
        prepared_emission(&network, peer_id, inventory_message(hashes[2]), hashes[2]),
    ];
    let [first, second, unsent] = emissions;
    let (_, _, first_capability) = first.into_parts();
    let (_, _, second_capability) = second.into_parts();

    // Act
    network
        .complete_peer_emission(first_capability.acknowledge_write())
        .expect("complete first write");
    network
        .complete_peer_emission(second_capability.acknowledge_write())
        .expect("complete second write");
    drop(unsent);

    // Assert
    let encoded = announcement_counts(&network);
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_headers_fallback_count"],
        1
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_inventory_fallback_count"],
        0
    );
}

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
    let replay = network
        .complete_peer_emission(duplicate)
        .expect("classify duplicate");

    // Assert
    assert_eq!(first, EffectCompletion::Applied);
    assert_eq!(replay, EffectCompletion::AlreadyApplied);
    assert_eq!(
        announcement_counts(&network)["announcement"]["value"]["compact_announced_count"],
        1
    );
}
