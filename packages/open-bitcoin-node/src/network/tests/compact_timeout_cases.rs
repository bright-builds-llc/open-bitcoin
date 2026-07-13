// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

//! ManagedPeerNetwork live-path proofs for Phase 120 compact-download timeout → GetData (RCN-07)
//! and Timeout volatile cleanup evidence (GOV-03).

use super::{
    build_block, compact_relay_enabled_managed_network, consensus_params, mine_header,
    spend_transaction, verify_flags,
};
use crate::status::FieldAvailability;
use crate::{ManagedPeerNetwork, MemoryChainstateStore};
use open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, SendCompactMessage};
use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, InventoryType, Transaction, Txid},
};
use open_bitcoin_network::{COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, WireNetworkMessage};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn build_block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    extra_transactions: Vec<Transaction>,
) -> Block {
    let mut block = build_block(previous_block_hash, height, 500_000_000);
    block.transactions.extend(extra_transactions);
    let (merkle_root, maybe_mutated) = block_merkle_root(&block.transactions).expect("merkle root");
    assert!(!maybe_mutated);
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);
    block
}

fn compact_payload_from_block(block: &Block, nonce: u64) -> CompactBlockPayload {
    assert!(block.transactions.len() >= 2);
    let wtxid = transaction_wtxid(&block.transactions[1]).expect("wtxid");
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, nonce);
    let short_id = open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &wtxid);

    CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids: vec![short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: block.transactions[0].clone(),
        }],
    }
}

fn handshake_and_sendcmpct(network: &mut ManagedPeerNetwork<MemoryChainstateStore>, peer_id: u64) {
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
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
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendcmpct");
}

fn tip_chain(network: &mut ManagedPeerNetwork<MemoryChainstateStore>) -> (Block, Block) {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    (genesis, spendable)
}

fn start_in_flight_compact_download(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    start_time: i64,
) -> BlockHash {
    let (_genesis, spendable) = tip_chain(network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 17);
    handshake_and_sendcmpct(network, peer_id);

    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            start_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("start compact download")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
        "missing short IDs must leave in-flight via GetBlockTxn; outbound={outbound:?}"
    );
    let maybe_state = network.peer_manager().compact_download_peer_state(peer_id);
    let state = maybe_state.expect("compact download state after start");
    assert!(
        state.in_flight.contains_key(&announced_hash),
        "expected in-flight entry for {announced_hash:?}"
    );
    announced_hash
}

fn assert_getdata_block(message: &WireNetworkMessage, expected_hash: BlockHash) -> bool {
    matches!(
        message,
        WireNetworkMessage::GetData(inventory)
            if inventory.inventory.len() == 1
                && inventory.inventory[0].inventory_type == InventoryType::Block
                && inventory.inventory[0].object_hash == expected_hash.into()
    )
}

#[test]
fn expire_compact_download_timeouts_returns_getdata_and_clears_in_flight() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(120_101);
    let peer_id = 120_101;
    let start_time = 1_000;
    let block_hash = start_in_flight_compact_download(&mut network, peer_id, start_time);

    // Act
    let expired = network
        .expire_compact_download_timeouts(start_time + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1)
        .expect("expire compact downloads");

    // Assert
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].0, peer_id);
    assert!(
        assert_getdata_block(&expired[0].1, block_hash),
        "expire must return peer-targeted GetData(Block); got={:?}",
        expired[0].1
    );
    let maybe_state = network.peer_manager().compact_download_peer_state(peer_id);
    let state = maybe_state.expect("download state retained after expire");
    assert!(
        state.in_flight.is_empty(),
        "timeout expire must clear volatile in_flight"
    );
}

#[test]
fn receive_sync_message_past_timeout_emits_getdata_and_timeout_evidence() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(120_102);
    let peer_id = 120_102;
    let start_time = 2_000;
    let block_hash = start_in_flight_compact_download(&mut network, peer_id, start_time);
    let timeout_before = match network.block_relay_evidence_status().cleanup {
        FieldAvailability::Available(cleanup) => cleanup.compact_download_timeout_count,
        FieldAvailability::Unavailable { .. } => 0,
    };

    // Act — benign live traffic advances the caller clock past compact timeout
    let result = network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::SendHeaders,
            start_time + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
            verify_flags(),
            consensus_params(),
        )
        .expect("live receive after timeout");

    // Assert
    assert!(
        result
            .outbound
            .iter()
            .any(|message| assert_getdata_block(message, block_hash)),
        "receive_sync_message must emit same-peer GetData(Block) after timeout; outbound={:?}",
        result.outbound
    );
    let timeout_after = match network.block_relay_evidence_status().cleanup {
        FieldAvailability::Available(cleanup) => cleanup.compact_download_timeout_count,
        FieldAvailability::Unavailable { reason } => {
            panic!("cleanup evidence unavailable after timeout: {reason}")
        }
    };
    assert!(
        timeout_after > timeout_before,
        "Timeout cleanup evidence must increment; before={timeout_before} after={timeout_after}"
    );
    assert_eq!(timeout_after, timeout_before + 1);
}

#[test]
fn receive_message_preserves_other_peer_timeout_getdata() {
    // Arrange — peer A holds in-flight; peer B's receive advances the clock
    let mut network = compact_relay_enabled_managed_network(120_103);
    let peer_a = 120_103;
    let peer_b = 120_104;
    let start_time = 3_000;
    let block_hash = start_in_flight_compact_download(&mut network, peer_a, start_time);
    handshake_and_sendcmpct(&mut network, peer_b);

    // Act
    let result = network
        .receive_message(
            peer_b,
            WireNetworkMessage::SendHeaders,
            start_time + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
            verify_flags(),
            consensus_params(),
        )
        .expect("peer B receive after peer A timeout");

    // Assert — other-peer GetData must land in targeted_outbound, never dropped
    assert!(
        result
            .targeted_outbound
            .iter()
            .any(|(target_peer_id, message)| {
                *target_peer_id == peer_a && assert_getdata_block(message, block_hash)
            }),
        "receive_message must preserve other-peer timeout GetData; targeted={:?}",
        result.targeted_outbound
    );
    assert!(
        !result
            .outbound
            .iter()
            .any(|message| assert_getdata_block(message, block_hash)),
        "peer A's GetData must not be mis-attributed to peer B outbound"
    );
}

#[test]
fn compact_timeout_cleanup_leaves_durable_chainstate_unchanged() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(120_105);
    let peer_id = 120_105;
    let start_time = 4_000;
    let _block_hash = start_in_flight_compact_download(&mut network, peer_id, start_time);
    let tip_before = network.maybe_chain_tip().expect("tip before timeout");
    let chain_len_before = network.chainstate_snapshot().active_chain.len();
    let durable_blocks_before = network.blocks_by_hash.len();

    // Act
    let _expired = network
        .expire_compact_download_timeouts(start_time + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1)
        .expect("expire compact downloads");

    // Assert — GOV-03 volatile-only cleanup
    let tip_after = network.maybe_chain_tip().expect("tip after timeout");
    assert_eq!(tip_after.block_hash, tip_before.block_hash);
    assert_eq!(tip_after.height, tip_before.height);
    assert_eq!(
        network.chainstate_snapshot().active_chain.len(),
        chain_len_before
    );
    assert_eq!(network.blocks_by_hash.len(), durable_blocks_before);
}
