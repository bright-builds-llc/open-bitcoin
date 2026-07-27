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
    primitives::{Block, BlockHash, BlockHeader, MerkleRoot, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    FeeRate, FinalMempoolMembership, MempoolAcceptanceTime, MempoolCapacityStatus,
    MempoolEntryMetadata, MempoolOrigin, MempoolOutcome, MempoolRemovalCause, MempoolRemovalRole,
    PolicyConfig, PolicyTime, ROLLING_FEE_HALFLIFE_SECONDS, RelayIntent, ReorgLifecycleContext,
    RollingFeeParityStatus,
};
use open_bitcoin_network::WireNetworkMessage;

use super::{
    EASY_BITS, build_block, coinbase_transaction, compact_relay_enabled_managed_network,
    consensus_params, local_config, mine_header, spend_transaction, verify_flags,
};
use crate::network::BlockConnectDisposition;
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{ManagedNetworkHandle, ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn wtxid(transaction: &Transaction) -> Wtxid {
    transaction_wtxid(transaction).expect("wtxid")
}

fn seed_reject_evidence(network: &mut ManagedPeerNetwork<MemoryChainstateStore>) -> (Wtxid, Wtxid) {
    let hard_reject = Wtxid::from_byte_array([0x41; 32]);
    let reconsiderable = Wtxid::from_byte_array([0x42; 32]);
    network.peer_manager_mut().record_hard_reject(hard_reject);
    network
        .peer_manager_mut()
        .record_reconsiderable_transaction(reconsiderable);
    (hard_reject, reconsiderable)
}

fn assert_reject_evidence(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    hard_reject: Wtxid,
    reconsiderable: Wtxid,
    expected_present: bool,
) {
    assert_eq!(
        network.peer_manager().hard_reject_contains(hard_reject),
        expected_present
    );
    assert_eq!(
        network
            .peer_manager()
            .reconsiderable_transaction_contains(reconsiderable),
        expected_present
    );
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
                metadata: MempoolEntryMetadata::legacy_unknown(),
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

mod compact_cache_and_fee;
mod connected_block_removal;
mod expiry;
mod reorg_reject_evidence;
