// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid},
    primitives::{Block, BlockHash, InventoryType, InventoryVector, Transaction, Txid},
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{
    InventoryList, RelayActivationConfig, TxServingRecordStatus, WireNetworkMessage,
};

use super::{
    build_block, consensus_params, local_config, mine_header, spend_transaction, verify_flags,
};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn tx_inventory(txid: Txid) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: txid.into(),
    }])
}

fn block_inventory(block: &Block) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    }])
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

fn relay_enabled_network(
    nonce: u64,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>, Block) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
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

    (network, coinbase_txids, spendable)
}

#[test]
fn managed_getdata_serves_only_accepted_relay_eligible_transaction() {
    // Arrange
    let (mut network, coinbase_txids, _spendable) = relay_enabled_network(801);
    network
        .connect_outbound_peer(801, 1)
        .expect("connect outbound");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    network
        .submit_local_transaction_outcome(transaction.clone(), verify_flags(), consensus_params())
        .expect("accepted transaction");

    // Act
    let outbound = network
        .receive_message(
            801,
            WireNetworkMessage::GetData(tx_inventory(transaction_txid)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("getdata");

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::Tx(transaction)]);
    assert_eq!(
        network.relay_serving_info().latest_outcomes[0].label,
        "served",
    );
}

#[test]
fn managed_getdata_reports_unknown_confirmed_replaced_evicted_expired_notfound() {
    // Arrange
    let (mut network, coinbase_txids, spendable) = relay_enabled_network(802);
    network
        .connect_outbound_peer(802, 1)
        .expect("connect outbound");
    let unknown = Txid::from_byte_array([42_u8; 32]);
    let confirmed = spend_transaction(coinbase_txids[0], 499_999_000);
    let confirmed_txid = txid(&confirmed);
    network
        .submit_local_transaction_outcome(confirmed.clone(), verify_flags(), consensus_params())
        .expect("accepted confirmed fixture");
    let block_with_confirmed =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![confirmed]);
    network
        .connect_local_block(&block_with_confirmed, verify_flags(), consensus_params())
        .expect("connect confirmed transaction");
    let original = spend_transaction(coinbase_txids[1], 499_999_000);
    let replacement = spend_transaction(coinbase_txids[1], 499_996_000);
    let original_txid = txid(&original);
    network
        .submit_local_transaction_outcome(original, verify_flags(), consensus_params())
        .expect("original accepted");
    network
        .submit_local_transaction_outcome(replacement, verify_flags(), consensus_params())
        .expect("replacement accepted");
    let evicted = Txid::from_byte_array([43_u8; 32]);
    let expired = Txid::from_byte_array([44_u8; 32]);
    let rejected = Txid::from_byte_array([45_u8; 32]);
    network
        .relay_serving
        .record_status(evicted, None, TxServingRecordStatus::Evicted);
    network
        .relay_serving
        .record_status(expired, None, TxServingRecordStatus::Expired);
    network
        .relay_serving
        .record_status(rejected, None, TxServingRecordStatus::Rejected);

    // Act
    let outbound = network
        .receive_message(
            802,
            WireNetworkMessage::GetData(InventoryList::new(vec![
                tx_inventory(unknown).inventory[0].clone(),
                tx_inventory(confirmed_txid).inventory[0].clone(),
                tx_inventory(original_txid).inventory[0].clone(),
                tx_inventory(evicted).inventory[0].clone(),
                tx_inventory(expired).inventory[0].clone(),
                tx_inventory(rejected).inventory[0].clone(),
            ])),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("getdata");

    // Assert
    assert_eq!(outbound.len(), 1);
    assert!(matches!(
        &outbound[0],
        WireNetworkMessage::NotFound(inventory) if inventory.inventory.len() == 6
    ));
    let labels = network
        .relay_serving_info()
        .latest_outcomes
        .iter()
        .map(|outcome| outcome.label)
        .collect::<Vec<_>>();
    assert_eq!(
        labels,
        vec![
            "unknown",
            "confirmed",
            "replaced",
            "evicted",
            "expired",
            "rejected"
        ],
    );
}

#[test]
fn managed_getdata_preserves_block_serving_branch() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(803),
        PolicyConfig::default(),
    );
    network
        .connect_outbound_peer(803, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let outbound = network
        .receive_message(
            803,
            WireNetworkMessage::GetData(block_inventory(&genesis)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("block getdata");

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::Block(genesis)]);
    assert!(network.relay_serving_info().latest_outcomes.is_empty());
}
