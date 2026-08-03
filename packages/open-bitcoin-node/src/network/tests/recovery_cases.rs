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

use std::collections::BTreeSet;

use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, InventoryType, InventoryVector, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    AdmissionContext, Mempool, MempoolAcceptanceTime, MempoolCapacity, MempoolEntryMetadata,
    MempoolMemberIdentity, MempoolOrigin, PolicyConfig, PolicyTime, RelayIntent,
    transaction_weight_and_virtual_size,
};
use open_bitcoin_network::{InventoryList, RelayActivationConfig, WireNetworkMessage};

use super::{
    build_block, consensus_params, local_config, mine_header, spend_transaction, verify_flags,
};
use crate::network::ManagedMempoolRecoverySummary;
use crate::network::recovery::topology::{RecoveryTopologyLimits, prepare_recovery_topology};
use crate::status::relay_evidence::RelayEvidenceField;
use crate::storage::mempool_snapshot::CapturedMempoolGeneration;
use crate::storage::{MempoolRecoveryStatus, MempoolSnapshot, MempoolSnapshotRecord};
use crate::{ManagedNetworkHandle, ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn wtxid(transaction: &Transaction) -> Wtxid {
    transaction_wtxid(transaction).expect("wtxid")
}

fn snapshot_record(transaction: Transaction) -> MempoolSnapshotRecord {
    snapshot_record_with_metadata(transaction, MempoolEntryMetadata::legacy_unknown())
}

fn snapshot_record_with_metadata(
    transaction: Transaction,
    metadata: MempoolEntryMetadata,
) -> MempoolSnapshotRecord {
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = wtxid(&transaction);
    let (_, virtual_size) =
        transaction_weight_and_virtual_size(&transaction).expect("transaction size");
    MempoolSnapshotRecord::try_from_compatibility(
        transaction,
        transaction_txid,
        transaction_wtxid,
        1_000,
        virtual_size,
        metadata,
    )
    .expect("valid compatibility record")
}

fn snapshot_from_transactions(transactions: Vec<Transaction>) -> MempoolSnapshot {
    MempoolSnapshot::from_legacy_v1(transactions.into_iter().map(snapshot_record).collect())
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

fn prepare_and_install_mempool_recovery(
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    snapshot: &MempoolSnapshot,
    startup_at: PolicyTime,
) -> (ManagedNetworkHandle, ManagedMempoolRecoverySummary) {
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let prepared = handle
        .prepare_mempool_recovery_at(snapshot, verify_flags(), consensus_params(), startup_at)
        .expect("prepare recovery outside authority");
    let summary = handle
        .install_mempool_recovery(prepared)
        .expect("install prepared recovery");
    (handle, summary)
}

fn topology_identities(transactions: Vec<Transaction>) -> Vec<(Txid, Wtxid)> {
    let snapshot = snapshot_from_transactions(transactions);
    prepare_recovery_topology(&snapshot.records, RecoveryTopologyLimits::standard())
        .expect("valid recovery topology")
        .ordered_identities()
        .collect()
}

mod metadata;
mod staging;
#[test]
fn staged_recovery_classifies_later_capacity_trim_from_final_membership() {
    // Arrange
    let (measurement_network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_095, 3, PolicyConfig::default());
    let lower_fee = spend_transaction(coinbase_txids[0], 499_999_000);
    let higher_fee = spend_transaction(coinbase_txids[1], 499_998_000);
    let mut measuring_pool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(usize::MAX),
        ..PolicyConfig::default()
    });
    measuring_pool
        .accept_transaction_transition_with_context(
            lower_fee.clone(),
            &measurement_network.chainstate_snapshot(),
            verify_flags(),
            consensus_params(),
            AdmissionContext::recovery(MempoolEntryMetadata::legacy_unknown()),
        )
        .expect("measure one entry");
    let one_entry_capacity = measuring_pool.accounted_memory().as_usize();
    let (network, replay_coinbase_txids, _latest_block) = relay_enabled_network_with_chain(
        1_096,
        3,
        PolicyConfig {
            mempool_capacity: MempoolCapacity::new(one_entry_capacity),
            ..PolicyConfig::default()
        },
    );
    let replay_lower_fee = spend_transaction(replay_coinbase_txids[0], 499_999_000);
    let replay_higher_fee = spend_transaction(replay_coinbase_txids[1], 499_998_000);
    assert_eq!(txid(&replay_lower_fee), txid(&lower_fee));
    assert_eq!(txid(&replay_higher_fee), txid(&higher_fee));
    let snapshot = snapshot_from_transactions(vec![replay_lower_fee, replay_higher_fee]);

    // Act
    let prepared = network
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(6_000),
        )
        .expect("prepare capacity-trimmed recovery");
    let summary = ManagedMempoolRecoverySummary::from_records(prepared.recovery_records().to_vec());

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(summary.dropped_evicted_count, 1);
    assert!(prepared.staged_mempool().entry(&txid(&lower_fee)).is_none());
    assert!(
        prepared
            .staged_mempool()
            .entry(&txid(&higher_fee))
            .is_some()
    );
    assert_eq!(
        prepared.staged_mempool().rolling_mempool_fee_rate(),
        open_bitcoin_mempool::RollingMempoolFeeRate::ZERO
    );
}

#[test]
fn staged_recovery_classifies_confirmed_duplicate_and_policy_records() {
    // Arrange
    let (mut network, coinbase_txids, latest_block) =
        relay_enabled_network_with_chain(1_097, 5, PolicyConfig::default());
    let confirmed = spend_transaction(coinbase_txids[0], 499_999_000);
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
    let duplicate = spend_transaction(coinbase_txids[1], 499_999_000);
    let policy_incompatible = spend_transaction(coinbase_txids[2], 499_999_999);
    let snapshot = snapshot_from_transactions(vec![
        policy_incompatible,
        duplicate.clone(),
        confirmed,
        duplicate,
    ]);

    // Act
    let prepared = network
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(7_000),
        )
        .expect("prepare classified recovery");
    let summary = ManagedMempoolRecoverySummary::from_records(prepared.recovery_records().to_vec());

    // Assert
    assert_eq!(summary.recovered_count, 1);
    assert_eq!(summary.dropped_confirmed_count, 1);
    assert_eq!(summary.dropped_duplicate_count, 1);
    assert_eq!(summary.dropped_policy_incompatible_count, 1);
}

#[test]
fn staged_recovery_classifies_a_fully_spent_confirmed_transaction() {
    // Arrange
    let (mut network, coinbase_txids, latest_block) =
        relay_enabled_network_with_chain(1_109, 5, PolicyConfig::default());
    let confirmed = spend_transaction(coinbase_txids[0], 499_999_000);
    let mut confirming_block = build_block(block_hash(&latest_block.header), 5, 500_000_000);
    confirming_block.transactions.push(confirmed.clone());
    let (merkle_root, maybe_mutated) =
        block_merkle_root(&confirming_block.transactions).expect("confirming merkle root");
    assert!(!maybe_mutated);
    confirming_block.header.merkle_root = merkle_root;
    mine_header(&mut confirming_block);
    network
        .connect_local_block(&confirming_block, verify_flags(), consensus_params())
        .expect("connect confirming block");

    let spending = spend_transaction(txid(&confirmed), 499_998_000);
    let mut spending_block = build_block(block_hash(&confirming_block.header), 6, 500_000_000);
    spending_block.transactions.push(spending);
    let (merkle_root, maybe_mutated) =
        block_merkle_root(&spending_block.transactions).expect("spending merkle root");
    assert!(!maybe_mutated);
    spending_block.header.merkle_root = merkle_root;
    mine_header(&mut spending_block);
    network
        .connect_local_block(&spending_block, verify_flags(), consensus_params())
        .expect("spend every confirmed output");
    let snapshot = snapshot_from_transactions(vec![confirmed]);

    // Act
    let prepared = network
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(7_100),
        )
        .expect("prepare classified recovery");
    let summary = ManagedMempoolRecoverySummary::from_records(prepared.recovery_records().to_vec());

    // Assert
    assert_eq!(summary.dropped_confirmed_count, 1);
    assert_eq!(summary.dropped_missing_parent_count, 0);
}

#[test]
fn recovery_topology_orders_parent_before_child_independent_of_stored_order() {
    // Arrange
    let parent = spend_transaction(Txid::from_byte_array([70_u8; 32]), 9_000);
    let child = spend_transaction(txid(&parent), 8_000);
    let independent = spend_transaction(Txid::from_byte_array([71_u8; 32]), 7_000);
    let expected = topology_identities(vec![parent.clone(), child.clone(), independent.clone()]);

    // Act
    let reversed = topology_identities(vec![independent.clone(), child.clone(), parent.clone()]);
    let shuffled = topology_identities(vec![child.clone(), independent, parent.clone()]);

    // Assert
    assert_eq!(reversed, expected);
    assert_eq!(shuffled, expected);
    let parent_position = expected
        .iter()
        .position(|(identity_txid, _)| *identity_txid == txid(&parent))
        .expect("parent identity");
    let child_position = expected
        .iter()
        .position(|(identity_txid, _)| *identity_txid == txid(&child))
        .expect("child identity");
    assert!(parent_position < child_position);
}

#[test]
fn recovery_topology_keeps_external_prevouts_and_contains_edge_limit_failure() {
    // Arrange
    let first_parent = spend_transaction(Txid::from_byte_array([72_u8; 32]), 9_000);
    let second_parent = spend_transaction(Txid::from_byte_array([73_u8; 32]), 9_000);
    let mut over_limit = spend_transaction(txid(&first_parent), 8_000);
    over_limit.inputs.push(second_parent.inputs[0].clone());
    over_limit.inputs[1].previous_output.txid = txid(&second_parent);
    let dependent = spend_transaction(txid(&over_limit), 7_000);
    let external = spend_transaction(Txid::from_byte_array([74_u8; 32]), 6_000);
    let snapshot = snapshot_from_transactions(vec![
        dependent,
        external.clone(),
        over_limit.clone(),
        second_parent,
        first_parent,
    ]);

    // Act
    let topology = prepare_recovery_topology(
        &snapshot.records,
        RecoveryTopologyLimits::new(snapshot.records.len(), 1),
    )
    .expect("bounded topology");

    // Assert
    assert!(
        topology
            .ordered_identities()
            .any(|(identity_txid, _)| identity_txid == txid(&external))
    );
    assert_eq!(
        topology.status_for(txid(&over_limit)),
        Some(MempoolRecoveryStatus::DroppedMissingParent)
    );
    assert_eq!(
        topology.status_for(txid(&snapshot.records[0].transaction)),
        Some(MempoolRecoveryStatus::DroppedMissingParent)
    );
}

#[test]
fn managed_recovery_rehydrates_serving_cache_and_fanout_identity_without_socket_io() {
    // Arrange
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_080, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let snapshot = snapshot_from_transactions(vec![transaction]);

    // Act
    let (handle, summary) = prepare_and_install_mempool_recovery(
        network,
        &snapshot,
        PolicyTime::from_unix_seconds(8_000),
    );
    let network = handle
        .authority_snapshot_for_test()
        .expect("recovered authority");

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
    let (network, coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_081, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = wtxid(&transaction);
    let snapshot = snapshot_from_transactions(vec![transaction.clone()]);
    let (handle, summary) = prepare_and_install_mempool_recovery(
        network,
        &snapshot,
        PolicyTime::from_unix_seconds(8_100),
    );
    assert_eq!(summary.recovered_count, 1);
    handle
        .connect_outbound_peer(1_081, 1)
        .expect("connect txid peer");
    handle
        .connect_outbound_peer(1_082, 1)
        .expect("connect wtxid peer");
    handle
        .receive_message(
            1_082,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("negotiate wtxid relay");

    // Act
    let txid_response = handle
        .receive_message(
            1_081,
            WireNetworkMessage::GetData(tx_inventory(transaction_txid)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("txid getdata")
        .outbound;
    let wtxid_response = handle
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
    let dropped_snapshot =
        snapshot_from_transactions(vec![confirmed, missing_parent, policy_incompatible]);
    let (network_handle, summary) = prepare_and_install_mempool_recovery(
        network,
        &dropped_snapshot,
        PolicyTime::from_unix_seconds(8_300),
    );

    let (evicting_network, evicting_coinbase_txids, _latest_block) =
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
    let evicted_snapshot = snapshot_from_transactions(vec![evicted]);
    let (evicting_handle, evicted_summary) = prepare_and_install_mempool_recovery(
        evicting_network,
        &evicted_snapshot,
        PolicyTime::from_unix_seconds(8_400),
    );

    let (duplicate_network, duplicate_coinbase_txids, _latest_block) =
        relay_enabled_network_with_chain(1_085, 2, PolicyConfig::default());
    let duplicate = spend_transaction(duplicate_coinbase_txids[0], 499_999_000);
    let duplicate_txid = txid(&duplicate);
    let duplicate_snapshot = snapshot_from_transactions(vec![duplicate.clone(), duplicate]);
    let (duplicate_handle, duplicate_summary) = prepare_and_install_mempool_recovery(
        duplicate_network,
        &duplicate_snapshot,
        PolicyTime::from_unix_seconds(8_500),
    );
    let network = network_handle
        .authority_snapshot_for_test()
        .expect("dropped-record authority");
    let evicting_network = evicting_handle
        .authority_snapshot_for_test()
        .expect("evicted-record authority");
    let duplicate_network = duplicate_handle
        .authority_snapshot_for_test()
        .expect("duplicate-record authority");

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
