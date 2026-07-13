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

use open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, SendCompactMessage};
use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, Transaction, Txid},
};
use open_bitcoin_mempool::MempoolOutcome;
use open_bitcoin_network::WireNetworkMessage;

use super::{
    build_block, compact_relay_enabled_managed_network, consensus_params, mine_header,
    spend_transaction, verify_flags,
};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

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

#[test]
fn live_compact_receive_uses_mempool_candidates_not_empty_facts() {
    // Arrange — tip + mempool tx whose short ID appears in the CompactBlock
    let mut network = compact_relay_enabled_managed_network(119_201);
    let peer_id = 119_201;
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");

    let mempool_tx = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![mempool_tx.clone()]);
    let payload = compact_payload_from_block(&announced, 42);

    let outcome = network
        .submit_local_transaction_outcome(mempool_tx, verify_flags(), consensus_params())
        .expect("admit mempool tx");
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));

    handshake_and_sendcmpct(&mut network, peer_id);

    // Act — live ManagedPeerNetwork receive must inject mempool candidates
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("live compact receive");

    // Assert — injected facts reconstruct without GetBlockTxn for the mempool-supplied short ID.
    // Empty-facts live receive would always request every unmatched short-id index.
    let live_requests_missing = outbound
        .iter()
        .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)));
    assert!(
        !live_requests_missing,
        "live CompactBlock receive with mempool candidates must not fall back to empty-facts GetBlockTxn; outbound={outbound:?}"
    );
}

#[test]
fn live_compact_receive_sync_message_also_injects_mempool_candidates() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(119_202);
    let peer_id = 119_202;
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");

    let mempool_tx = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![mempool_tx.clone()]);
    let payload = compact_payload_from_block(&announced, 42);
    let outcome = network
        .submit_local_transaction_outcome(mempool_tx, verify_flags(), consensus_params())
        .expect("admit mempool tx");
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let result = network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("live compact sync receive");

    // Assert
    let live_requests_missing = result
        .outbound
        .iter()
        .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)));
    assert!(
        !live_requests_missing,
        "receive_sync_message CompactBlock path must inject mempool candidates; outbound={:?}",
        result.outbound
    );
}
