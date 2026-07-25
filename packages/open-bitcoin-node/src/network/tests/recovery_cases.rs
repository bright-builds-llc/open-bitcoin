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

use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, InventoryType, InventoryVector, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolCapacity, MempoolEntryMetadata, MempoolOrigin, PolicyConfig,
    PolicyTime, RelayIntent,
};
use open_bitcoin_network::{InventoryList, RelayActivationConfig, WireNetworkMessage};

use super::{
    build_block, consensus_params, local_config, mine_header, spend_transaction, verify_flags,
};
use crate::network::ManagedMempoolRecoverySummary;
use crate::status::relay_evidence::RelayEvidenceField;
use crate::storage::{MempoolRecoveryStatus, MempoolSnapshot, MempoolSnapshotRecord};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn wtxid(transaction: &Transaction) -> Wtxid {
    transaction_wtxid(transaction).expect("wtxid")
}

fn snapshot_record(transaction: Transaction) -> MempoolSnapshotRecord {
    MempoolSnapshotRecord {
        txid: txid(&transaction),
        wtxid: wtxid(&transaction),
        transaction,
        fee_sats: 1_000,
        virtual_size: 100,
        metadata: MempoolEntryMetadata::legacy_unknown(),
    }
}

fn snapshot_record_with_metadata(
    transaction: Transaction,
    metadata: MempoolEntryMetadata,
) -> MempoolSnapshotRecord {
    let mut record = snapshot_record(transaction);
    record.metadata = metadata;
    record
}

fn snapshot_from_transactions(transactions: Vec<Transaction>) -> MempoolSnapshot {
    MempoolSnapshot {
        records: transactions.into_iter().map(snapshot_record).collect(),
    }
}

fn tx_inventory(transaction_txid: Txid) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: transaction_txid.into(),
    }])
}

fn wtx_inventory(transaction_wtxid: Wtxid) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::WitnessTransaction,
        object_hash: transaction_wtxid.into(),
    }])
}

fn relay_enabled_network_with_chain(
    nonce: u64,
    block_count: u32,
    mempool_config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>, Block) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        mempool_config,
        RelayActivationConfig { enabled: true },
        true,
    );
    let mut previous_block_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut coinbase_txids = Vec::new();
    let mut latest_block = build_block(previous_block_hash, 0, 500_000_000);

    for height in 0..block_count {
        let block = build_block(previous_block_hash, height, 500_000_000);
        coinbase_txids.push(txid(&block.transactions[0]));
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("connect fixture block");
        previous_block_hash = block_hash(&block.header);
        latest_block = block;
    }

    (network, coinbase_txids, latest_block)
}

fn assert_recovery_status(
    summary: &ManagedMempoolRecoverySummary,
    index: usize,
) -> MempoolRecoveryStatus {
    summary.records.get(index).expect("recovery record").status
}

#[test]
fn managed_recovery_rehydrates_serving_cache_and_fanout_identity_without_socket_io() {
    // Arrange
    let (mut network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_080, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let snapshot = snapshot_from_transactions(vec![transaction]);

    // Act
    let summary = network
        .recover_mempool_snapshot(&snapshot, verify_flags(), consensus_params())
        .expect("recover snapshot");

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(
        assert_recovery_status(&summary, 0),
        MempoolRecoveryStatus::Recovered
    );
    assert_eq!(network.latest_mempool_recovery_summary(), Some(summary));
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .is_some()
    );
    assert!(network.transactions_by_txid.contains_key(&transaction_txid));
    assert_eq!(network.relay_serving_info().serveable_transactions, 1);
    let fanout_info = network.relay_fanout_info();
    assert_eq!(fanout_info.known_transactions, 1);
    assert_eq!(fanout_info.queued_transactions, 0);
    assert!(fanout_info.latest_actions.is_empty());
    let RelayEvidenceField::Implemented(recovery_counters) =
        network.relay_evidence_status().recovery_counters
    else {
        panic!("expected implemented recovery counters");
    };
    assert_eq!(recovery_counters.recovered_count, 1);
    assert_eq!(recovery_counters.dropped_evicted_count, 0);
}

#[test]
fn managed_recovery_serves_recovered_txid_and_wtxid_for_eligible_peers() {
    // Arrange
    let (mut network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_081, 2, PolicyConfig::default());
    network
        .connect_outbound_peer(1_081, 1)
        .expect("connect txid peer");
    network
        .connect_outbound_peer(1_082, 1)
        .expect("connect wtxid peer");
    network
        .receive_message(
            1_082,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("negotiate wtxid relay");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = wtxid(&transaction);
    let snapshot = snapshot_from_transactions(vec![transaction.clone()]);
    network
        .recover_mempool_snapshot(&snapshot, verify_flags(), consensus_params())
        .expect("recover snapshot");

    // Act
    let txid_response = network
        .receive_message(
            1_081,
            WireNetworkMessage::GetData(tx_inventory(transaction_txid)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("txid getdata")
        .outbound;
    let wtxid_response = network
        .receive_message(
            1_082,
            WireNetworkMessage::GetData(wtx_inventory(transaction_wtxid)),
            3,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxid getdata")
        .outbound;

    // Assert
    assert_eq!(
        txid_response,
        vec![WireNetworkMessage::Tx(transaction.clone())]
    );
    assert_eq!(wtxid_response, vec![WireNetworkMessage::Tx(transaction)]);
}

#[test]
fn managed_recovery_drops_non_accepted_records_from_serving_and_fanout() {
    // Arrange
    let (mut network, coinbase_txids, latest_block) =
        relay_enabled_network_with_chain(1_083, 5, PolicyConfig::default());
    let confirmed = spend_transaction(coinbase_txids[0], 499_999_000);
    let confirmed_txid = txid(&confirmed);
    let confirmed_block = {
        let mut block = build_block(block_hash(&latest_block.header), 5, 500_000_000);
        block.transactions.push(confirmed.clone());
        let (merkle_root, maybe_mutated) =
            block_merkle_root(&block.transactions).expect("merkle root");
        assert!(!maybe_mutated);
        block.header.merkle_root = merkle_root;
        mine_header(&mut block);
        block
    };
    network
        .connect_local_block(&confirmed_block, verify_flags(), consensus_params())
        .expect("connect confirmed transaction");
    let missing_parent = spend_transaction(Txid::from_byte_array([77_u8; 32]), 499_999_000);
    let missing_parent_txid = txid(&missing_parent);
    let policy_incompatible = spend_transaction(coinbase_txids[2], 499_999_999);
    let policy_incompatible_txid = txid(&policy_incompatible);
    let summary = network
        .recover_mempool_snapshot(
            &snapshot_from_transactions(vec![confirmed, missing_parent, policy_incompatible]),
            verify_flags(),
            consensus_params(),
        )
        .expect("recover dropped records");

    let (mut evicting_network, evicting_coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(
            1_084,
            2,
            PolicyConfig {
                mempool_capacity: MempoolCapacity::new(0),
                ..PolicyConfig::default()
            },
        );
    let evicted = spend_transaction(evicting_coinbase_txids[0], 499_999_000);
    let evicted_txid = txid(&evicted);
    let evicted_summary = evicting_network
        .recover_mempool_snapshot(
            &snapshot_from_transactions(vec![evicted]),
            verify_flags(),
            consensus_params(),
        )
        .expect("recover evicted record");

    let (mut duplicate_network, duplicate_coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_085, 2, PolicyConfig::default());
    let duplicate = spend_transaction(duplicate_coinbase_txids[0], 499_999_000);
    let duplicate_txid = txid(&duplicate);
    let duplicate_summary = duplicate_network
        .recover_mempool_snapshot(
            &snapshot_from_transactions(vec![duplicate.clone(), duplicate]),
            verify_flags(),
            consensus_params(),
        )
        .expect("recover duplicate record");

    // Assert
    assert_eq!(summary.dropped_confirmed_count, 1);
    assert_eq!(summary.dropped_missing_parent_count, 1);
    assert_eq!(summary.dropped_policy_incompatible_count, 1);
    assert_eq!(evicted_summary.dropped_evicted_count, 1);
    assert_eq!(duplicate_summary.recovered_count, 1);
    assert_eq!(duplicate_summary.dropped_duplicate_count, 1);
    assert!(!network.transactions_by_txid.contains_key(&confirmed_txid));
    assert!(
        !network
            .transactions_by_txid
            .contains_key(&missing_parent_txid)
    );
    assert!(
        !network
            .transactions_by_txid
            .contains_key(&policy_incompatible_txid)
    );
    assert!(
        !evicting_network
            .transactions_by_txid
            .contains_key(&evicted_txid)
    );
    assert!(
        duplicate_network
            .transactions_by_txid
            .contains_key(&duplicate_txid)
    );
    assert_eq!(network.relay_serving_info().serveable_transactions, 0);
    assert_eq!(network.relay_fanout_info().known_transactions, 0);
    assert_eq!(
        evicting_network.relay_serving_info().serveable_transactions,
        0
    );
    assert_eq!(evicting_network.relay_fanout_info().known_transactions, 0);
    assert_eq!(
        duplicate_network
            .relay_serving_info()
            .serveable_transactions,
        1
    );
    assert_eq!(duplicate_network.relay_fanout_info().known_transactions, 1);
}

#[test]
fn recovery_metadata_managed_local_requested_preserves_facts_and_fanout() {
    // Arrange
    let (mut source, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_090, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    source
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            90,
            RelayIntent::Requested,
        )
        .expect("admit local requested");
    let snapshot = MempoolSnapshot::from_mempool(source.mempool().mempool());
    let expected = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );
    let (mut recovered, _coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_091, 2, PolicyConfig::default());

    // Act
    let summary = recovered
        .recover_mempool_snapshot(&snapshot, verify_flags(), consensus_params())
        .expect("recover known local");

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(snapshot.records[0].metadata, expected);
    let entry = recovered
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("recovered entry");
    assert_eq!(entry.metadata, expected);
    assert!(entry.metadata.is_retry_eligible(true));
    assert_eq!(recovered.relay_fanout_info().known_transactions, 1);
    assert_eq!(recovered.relay_fanout_info().queued_transactions, 0);
}

#[test]
fn recovery_metadata_managed_duplicate_preserves_original_canonical_metadata() {
    // Arrange
    let (mut network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_092, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let original = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );
    let conflicting = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(999)),
        MempoolOrigin::Peer,
        RelayIntent::NotRequested,
    );
    let snapshot = MempoolSnapshot {
        records: vec![
            snapshot_record_with_metadata(transaction.clone(), original),
            snapshot_record_with_metadata(transaction, conflicting),
        ],
    };

    // Act
    let summary = network
        .recover_mempool_snapshot(&snapshot, verify_flags(), consensus_params())
        .expect("recover duplicate metadata");

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(summary.dropped_duplicate_count, 1);
    assert_eq!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .expect("original")
            .metadata,
        original
    );
}
