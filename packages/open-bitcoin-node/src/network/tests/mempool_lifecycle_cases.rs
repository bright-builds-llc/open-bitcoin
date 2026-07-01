// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp

use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, BlockHeader, Transaction, Txid},
};
use open_bitcoin_mempool::{MempoolCapacityStatus, PolicyConfig, RollingFeeParityStatus};

use super::{
    EASY_BITS, coinbase_transaction, consensus_params, local_config, mine_header,
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
