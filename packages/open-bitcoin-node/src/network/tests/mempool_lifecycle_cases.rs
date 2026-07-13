// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, SendCompactMessage};
use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, BlockHeader, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    MempoolCapacityStatus, MempoolOutcome, PolicyConfig, RollingFeeParityStatus,
};
use open_bitcoin_network::WireNetworkMessage;

use super::{
    EASY_BITS, build_block, coinbase_transaction, compact_relay_enabled_managed_network,
    consensus_params, local_config, mine_header, spend_transaction, verify_flags,
};
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn wtxid(transaction: &Transaction) -> Wtxid {
    transaction_wtxid(transaction).expect("wtxid")
}

fn snapshot_from_transactions(transactions: Vec<Transaction>) -> MempoolSnapshot {
    MempoolSnapshot {
        records: transactions
            .into_iter()
            .map(|transaction| MempoolSnapshotRecord {
                txid: txid(&transaction),
                wtxid: wtxid(&transaction),
                transaction,
                fee_sats: 1_000,
                virtual_size: 100,
            })
            .collect(),
    }
}

fn build_block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    extra_transactions: Vec<Transaction>,
) -> Block {
    let mut transactions = vec![coinbase_transaction(height, 500_000_000)];
    transactions.extend(extra_transactions);
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

fn network_with_chain() -> (
    ManagedPeerNetwork<MemoryChainstateStore>,
    Block,
    Block,
    Vec<Txid>,
) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(701),
        PolicyConfig::default(),
    );
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);
    let spendable = build_block_with_transactions(block_hash(&genesis.header), 1, vec![]);
    let coinbase_txids = vec![
        txid(&genesis.transactions[0]),
        txid(&spendable.transactions[0]),
    ];
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");

    (network, genesis, spendable, coinbase_txids)
}

#[test]
fn managed_block_connect_removes_confirmed_mempool_transaction_and_runtime_caches() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = transaction_wtxid(&transaction).expect("wtxid");
    network
        .submit_local_transaction(transaction.clone(), verify_flags(), consensus_params())
        .expect("submit local transaction");
    let connected_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![transaction]);

    // Act
    network
        .connect_local_block(&connected_block, verify_flags(), consensus_params())
        .expect("connect block with mempool transaction");

    // Assert
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .is_none()
    );
    assert!(!network.transactions_by_txid.contains_key(&transaction_txid));
    assert!(
        !network
            .transactions_by_wtxid
            .contains_key(&transaction_wtxid)
    );
    let info = network.mempool_info();
    assert_eq!(info.transaction_count, 0);
    assert_eq!(info.capacity_status, MempoolCapacityStatus::Empty);
    assert_eq!(info.rolling_fee_parity, RollingFeeParityStatus::Deferred);
}

#[test]
fn recovered_confirmed_transaction_is_removed_from_serving_and_fanout_after_block_connect() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = wtxid(&transaction);
    let snapshot = snapshot_from_transactions(vec![transaction.clone()]);
    network
        .recover_mempool_snapshot(&snapshot, verify_flags(), consensus_params())
        .expect("recover transaction");
    assert_eq!(network.relay_serving_info().serveable_transactions, 1);
    assert_eq!(network.relay_fanout_info().known_transactions, 1);
    let connected_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![transaction]);

    // Act
    network
        .connect_local_block(&connected_block, verify_flags(), consensus_params())
        .expect("connect recovered transaction block");

    // Assert
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .is_none()
    );
    assert!(!network.transactions_by_txid.contains_key(&transaction_txid));
    assert!(
        !network
            .transactions_by_wtxid
            .contains_key(&transaction_wtxid)
    );
    assert_eq!(network.relay_serving_info().serveable_transactions, 0);
    assert_eq!(network.relay_fanout_info().known_transactions, 0);
}

#[test]
fn managed_block_connect_removes_conflict_and_descendant_caches() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let original_txid = txid(&original);
    let descendant = spend_transaction(original_txid, 499_998_000);
    let descendant_txid = txid(&descendant);
    let replacement = spend_transaction(coinbase_txids[0], 499_997_000);
    network
        .submit_local_transaction(original, verify_flags(), consensus_params())
        .expect("submit original");
    network
        .submit_local_transaction(descendant, verify_flags(), consensus_params())
        .expect("submit descendant");
    let connected_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![replacement]);

    // Act
    network
        .connect_local_block(&connected_block, verify_flags(), consensus_params())
        .expect("connect conflict block");

    // Assert
    assert!(network.mempool().mempool().entry(&original_txid).is_none());
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&descendant_txid)
            .is_none()
    );
    assert!(!network.transactions_by_txid.contains_key(&original_txid));
    assert!(!network.transactions_by_txid.contains_key(&descendant_txid));
    assert_eq!(network.mempool_info().transaction_count, 0);
}

#[test]
fn recovered_conflicting_transaction_removes_descendant_serving_and_fanout_state() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let original_txid = txid(&original);
    let original_wtxid = wtxid(&original);
    let descendant = spend_transaction(original_txid, 499_998_000);
    let descendant_txid = txid(&descendant);
    let descendant_wtxid = wtxid(&descendant);
    let replacement = spend_transaction(coinbase_txids[0], 499_997_000);
    network
        .recover_mempool_snapshot(
            &snapshot_from_transactions(vec![original, descendant]),
            verify_flags(),
            consensus_params(),
        )
        .expect("recover parent and descendant");
    assert_eq!(network.relay_serving_info().serveable_transactions, 2);
    assert_eq!(network.relay_fanout_info().known_transactions, 2);
    let connected_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![replacement]);

    // Act
    network
        .connect_local_block(&connected_block, verify_flags(), consensus_params())
        .expect("connect conflicting block");

    // Assert
    assert!(network.mempool().mempool().entry(&original_txid).is_none());
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&descendant_txid)
            .is_none()
    );
    assert!(!network.transactions_by_txid.contains_key(&original_txid));
    assert!(!network.transactions_by_txid.contains_key(&descendant_txid));
    assert!(!network.transactions_by_wtxid.contains_key(&original_wtxid));
    assert!(
        !network
            .transactions_by_wtxid
            .contains_key(&descendant_wtxid)
    );
    assert_eq!(network.relay_serving_info().serveable_transactions, 0);
    assert_eq!(network.relay_fanout_info().known_transactions, 0);
}

#[test]
fn recovered_replacement_cleans_old_txid_and_preserves_new_accepted_identity() {
    // Arrange
    let (mut network, _genesis, _spendable, coinbase_txids) = network_with_chain();
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let original_txid = txid(&original);
    let original_wtxid = wtxid(&original);
    let replacement = spend_transaction(coinbase_txids[0], 499_997_000);
    let replacement_txid = txid(&replacement);
    let replacement_wtxid = wtxid(&replacement);
    network
        .recover_mempool_snapshot(
            &snapshot_from_transactions(vec![original]),
            verify_flags(),
            consensus_params(),
        )
        .expect("recover original");

    // Act
    network
        .submit_local_transaction_outcome(replacement, verify_flags(), consensus_params())
        .expect("replace recovered transaction");

    // Assert
    assert!(network.mempool().mempool().entry(&original_txid).is_none());
    assert!(!network.transactions_by_txid.contains_key(&original_txid));
    assert!(!network.transactions_by_wtxid.contains_key(&original_wtxid));
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&replacement_txid)
            .is_some()
    );
    assert!(network.transactions_by_txid.contains_key(&replacement_txid));
    assert!(
        network
            .transactions_by_wtxid
            .contains_key(&replacement_wtxid)
    );
    assert_eq!(network.relay_serving_info().serveable_transactions, 1);
    assert_eq!(network.relay_fanout_info().known_transactions, 1);
}

#[test]
fn managed_reorg_reconsiders_eligible_disconnected_transaction() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let disconnected_transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let disconnected_txid = txid(&disconnected_transaction);
    let old_tip = build_block_with_transactions(
        block_hash(&spendable.header),
        2,
        vec![disconnected_transaction.clone()],
    );
    let replacement_tip = build_block_with_transactions(block_hash(&spendable.header), 2, vec![]);
    network
        .connect_local_block(&old_tip, verify_flags(), consensus_params())
        .expect("connect old tip");
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&disconnected_txid)
            .is_none()
    );

    // Act
    network
        .reorg_to_branch(
            &[old_tip],
            &[AnchoredBlock {
                block: replacement_tip,
                chain_work: 3,
            }],
            verify_flags(),
            consensus_params(),
        )
        .expect("reorg to replacement tip");

    // Assert
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&disconnected_txid)
            .is_some()
    );
    assert!(
        network
            .transactions_by_txid
            .contains_key(&disconnected_txid)
    );
}

fn compact_payload_matched_and_missing(
    announced: &Block,
    matched: &Transaction,
    missing: &Transaction,
    nonce: u64,
) -> CompactBlockPayload {
    let matched_wtxid = transaction_wtxid(matched).expect("matched wtxid");
    let missing_wtxid = transaction_wtxid(missing).expect("missing wtxid");
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&announced.header, nonce);
    let matched_short_id =
        open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &matched_wtxid);
    let missing_short_id =
        open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &missing_wtxid);

    CompactBlockPayload {
        header: announced.header.clone(),
        nonce,
        short_ids: vec![matched_short_id, missing_short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: announced.transactions[0].clone(),
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
fn connected_block_mempool_removal_clears_matched_compact_partial_slot() {
    // Arrange — live CompactBlock leaves an in-flight partial with one mempool-matched slot
    let mut network = compact_relay_enabled_managed_network(119_301);
    let peer_id = 119_301;
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");

    let matched = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let matched_wtxid = wtxid(&matched);
    // Missing short-id body is not in the announced merkle tree; we never complete reconstruction.
    let still_missing = spend_transaction(txid(&genesis.transactions[0]), 499_998_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![matched.clone()]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_matched_and_missing(&announced, &matched, &still_missing, 42);

    let outcome = network
        .submit_local_transaction_outcome(matched.clone(), verify_flags(), consensus_params())
        .expect("admit matched tx");
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));

    handshake_and_sendcmpct(&mut network, peer_id);
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live compact receive with one match + one missing")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
        "expected GetBlockTxn so partial stays in-flight; outbound={outbound:?}"
    );

    let download_state = network
        .peer_manager()
        .compact_download_peer_state(peer_id)
        .expect("download state after receive");
    let in_flight = download_state
        .in_flight
        .get(&announced_hash)
        .expect("in-flight partial");
    assert!(in_flight.partial.is_transaction_available(1));
    assert!(!in_flight.partial.is_transaction_available(2));

    // Conflict block confirms removal of the matched mempool tx without connecting announced
    let conflict = spend_transaction(txid(&spendable.transactions[0]), 499_997_000);
    let conflict_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![conflict]);

    // Act — connected-block lifecycle must forward removal.wtxid into PeerManager
    network
        .connect_local_block(&conflict_block, verify_flags(), consensus_params())
        .expect("connect conflict block removing matched mempool tx");

    // Assert — matched volatile slot cleared; missing index remains
    let download_state = network
        .peer_manager()
        .compact_download_peer_state(peer_id)
        .expect("download state after lifecycle");
    let in_flight = download_state
        .in_flight
        .get(&announced_hash)
        .expect("in-flight partial retained");
    assert!(
        !in_flight.partial.is_transaction_available(1),
        "matched slot for wtxid {matched_wtxid:?} must clear after connected-block removal"
    );
    assert_eq!(in_flight.partial.missing_transaction_indexes(), vec![1, 2]);
}
