// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp

use std::collections::BTreeMap;

use open_bitcoin_codec::BlockTransactionsRequest;
use open_bitcoin_core::chainstate::PruneMode;
use open_bitcoin_network::PermissionEffectLabel;

use crate::network::{AnnouncementPreparationOutcome, PeerOutboxSnapshot};

use super::*;

fn height_prefix(value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = u64::from(value);
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }
    if encoded.last().is_some_and(|byte| byte & 0x80 != 0) {
        encoded.push(0x00);
    }

    let mut script_bytes = Vec::with_capacity(encoded.len() + 2);
    script_bytes.push(encoded.len() as u8);
    script_bytes.extend(encoded);
    script_bytes.push(0x51);
    script_bytes
}

fn limited_serve_block(previous_block_hash: BlockHash, height: u32) -> Block {
    let mut script_sig = height_prefix(height);
    script_sig.push(0x51);
    let transactions = vec![Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(500_000_000).expect("coinbase amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn connect_heights(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    tip_height: u32,
) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut previous = BlockHash::from_byte_array([0_u8; 32]);
    for height in 0..=tip_height {
        let block = limited_serve_block(previous, height);
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("connect active block");
        previous = block_hash(&block.header);
        blocks.push(block);
    }
    blocks
}

fn request_block(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    block: &Block,
) -> Vec<WireNetworkMessage> {
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(block_getdata_inventory(block)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("block getdata")
        .outbound
}

fn finish_handshake(network: &mut ManagedPeerNetwork<MemoryChainstateStore>, peer_id: u64) {
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Verack,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("verack");
}

#[test]
fn manual_mode_distance_291_getdata_is_notfound_and_removes_the_ordinary_peer() {
    // Arrange
    let peer_id = 149_301;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);
    let height_zero = &blocks[0];
    let height_zero_hash = block_hash(&height_zero.header);

    // Act
    let outbound = request_block(&mut network, peer_id, height_zero);

    // Assert
    assert_eq!(
        outbound,
        vec![WireNetworkMessage::NotFound(block_getdata_inventory(
            height_zero
        ))]
    );
    assert!(network.blocks_by_hash.contains_key(&height_zero_hash));
    assert!(network.peer_manager().peer_state(peer_id).is_none());
}

#[test]
fn manual_mode_distance_290_getdata_serves_the_block_and_keeps_the_peer() {
    // Arrange
    let peer_id = 149_302;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);

    // Act
    let outbound = request_block(&mut network, peer_id, &blocks[1]);

    // Assert
    assert!(matches!(
        outbound.as_slice(),
        [WireNetworkMessage::Block(_)]
    ));
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn download_permission_keeps_the_peer_on_an_out_of_window_getdata() {
    // Arrange
    let peer_id = 149_303;
    let mut network = block_serving_enabled_managed_network(peer_id);
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        peer_id,
        "203.0.113.7:18444",
        &["in", "download"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let effects = network
        .peer_manager()
        .peer_state(peer_id)
        .and_then(|peer| peer.maybe_inbound_record.as_ref())
        .expect("download peer")
        .permission_decision
        .active_effects();
    assert!(effects.contains(&PermissionEffectLabel::DownloadServingPolicyInput));
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);

    // Act
    let outbound = request_block(&mut network, peer_id, &blocks[0]);

    // Assert
    assert_eq!(
        outbound,
        vec![WireNetworkMessage::NotFound(block_getdata_inventory(
            &blocks[0]
        ))]
    );
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn noban_permission_does_not_keep_an_out_of_window_peer() {
    // Arrange
    let peer_id = 149_304;
    let mut network = block_serving_enabled_managed_network(peer_id);
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        peer_id,
        "203.0.113.7:18444",
        &["in", "noban"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);

    // Act
    let outbound = request_block(&mut network, peer_id, &blocks[0]);

    // Assert
    assert_eq!(
        outbound,
        vec![WireNetworkMessage::NotFound(block_getdata_inventory(
            &blocks[0]
        ))]
    );
    assert!(network.peer_manager().peer_state(peer_id).is_none());
}

#[test]
fn disabled_prune_still_serves_a_cached_height_zero_block() {
    // Arrange
    let peer_id = 149_305;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let blocks = connect_heights(&mut network, 291);

    // Act
    let outbound = request_block(&mut network, peer_id, &blocks[0]);

    // Assert
    assert!(matches!(
        outbound.as_slice(),
        [WireNetworkMessage::Block(_)]
    ));
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn in_window_missing_payload_stays_unavailable_and_keeps_the_peer() {
    // Arrange
    let peer_id = 149_306;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let blocks = connect_heights(&mut network, 291);
    network.set_serving_have_pruned(false);
    let tip = blocks.last().expect("tip");
    let tip_hash = block_hash(&tip.header);
    network.blocks_by_hash.remove(&tip_hash);
    let inventory = block_getdata_inventory(tip);

    // Act
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("in-window miss")
        .outbound;
    let request = inventory.inventory.first().cloned().expect("request");
    let input = network.managed_block_serve_input(peer_id, &request, tip_hash, false, false);
    let decision =
        serve_managed_block_request(input, |hash| network.blocks_by_hash.get(&hash).cloned());
    let FieldAvailability::Available(status) =
        network.block_relay_evidence_status().block_serving.status
    else {
        panic!("block-serving status counters should be available after getdata");
    };

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert_eq!(decision.status_label, BlockServingStatusLabel::Unavailable);
    assert_eq!(status.pruned_count, 0);
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn in_window_have_pruned_gap_is_pruned_and_keeps_the_peer() {
    // Arrange
    let peer_id = 149_307;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let blocks = connect_heights(&mut network, 291);
    network.set_serving_have_pruned(true);
    let tip = blocks.last().expect("tip");
    let tip_hash = block_hash(&tip.header);
    network.blocks_by_hash.remove(&tip_hash);
    let inventory = block_getdata_inventory(tip);

    // Act
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("pruned gap getdata")
        .outbound;
    let request = inventory.inventory.first().cloned().expect("request");
    let input = network.managed_block_serve_input(peer_id, &request, tip_hash, false, false);
    let decision =
        serve_managed_block_request(input, |hash| network.blocks_by_hash.get(&hash).cloned());

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert_eq!(decision.status_label, BlockServingStatusLabel::Pruned);
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn unknown_hash_with_have_pruned_is_not_labeled_pruned() {
    // Arrange
    let peer_id = 149_308;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let _blocks = connect_heights(&mut network, 1);
    network.set_serving_have_pruned(true);
    let unknown = BlockHash::from_byte_array([9_u8; 32]);
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: unknown.into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("unknown getdata")
        .outbound;
    let request = inventory.inventory.first().cloned().expect("request");
    let input = network.managed_block_serve_input(peer_id, &request, unknown, false, false);
    let decision =
        serve_managed_block_request(input, |hash| network.blocks_by_hash.get(&hash).cloned());

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert_ne!(decision.status_label, BlockServingStatusLabel::Pruned);
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn historical_block_announcement_is_suppressed_and_keeps_the_peer() {
    // Arrange
    let peer_id = 149_309;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    finish_handshake(&mut network, peer_id);
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);

    // Act
    let outcomes = network.prepare_block_announcements(
        &blocks[0],
        &[PeerOutboxSnapshot::new(peer_id, 0, 8)],
        &BTreeMap::new(),
    );

    // Assert
    assert!(matches!(
        outcomes.as_slice(),
        [AnnouncementPreparationOutcome::Suppressed {
            reason: CompactAnnouncementReason::CompactBlockUnavailable,
            ..
        }]
    ));
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn recent_tip_announcement_is_a_ready_block_inv() {
    // Arrange
    let peer_id = 149_310;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    finish_handshake(&mut network, peer_id);
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);
    let tip = blocks.last().expect("tip");
    let tip_hash = block_hash(&tip.header);

    // Act
    let outcomes = network.prepare_block_announcements(
        tip,
        &[PeerOutboxSnapshot::new(peer_id, 0, 8)],
        &BTreeMap::new(),
    );

    // Assert
    let [AnnouncementPreparationOutcome::Ready(emission)] = outcomes.as_slice() else {
        panic!("recent announcement should be ready");
    };
    match emission.message() {
        WireNetworkMessage::Inv(list) => {
            assert_eq!(list.inventory.len(), 1);
            assert_eq!(list.inventory[0].inventory_type, InventoryType::Block);
            assert_eq!(list.inventory[0].object_hash, tip_hash.into());
        }
        other => panic!("recent announcement should be an inv, got {other:?}"),
    }
    assert!(network.peer_manager().peer_state(peer_id).is_some());
}

#[test]
fn out_of_window_getblocktxn_is_notfound_and_removes_the_ordinary_peer() {
    // Arrange
    let peer_id = 149_311;
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let blocks = connect_heights(&mut network, 291);
    network.set_prune_mode(PruneMode::ManualOnly);
    let height_zero_hash = block_hash(&blocks[0].header);
    network
        .peer_manager_mut()
        .record_compact_block_announcement(peer_id, height_zero_hash)
        .expect("record compact announcement");

    // Act
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
                block_hash: height_zero_hash,
                index_deltas: vec![0],
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("out-of-window getblocktxn")
        .outbound;

    // Assert
    assert!(outbound.iter().any(|message| matches!(
        message,
        WireNetworkMessage::NotFound(inventory)
            if inventory.inventory.iter().any(|vector| vector.object_hash == height_zero_hash.into())
    )));
    assert!(
        outbound
            .iter()
            .all(|message| !matches!(message, WireNetworkMessage::BlockTxn(_)))
    );
    assert!(network.peer_manager().peer_state(peer_id).is_none());
}
