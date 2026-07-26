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
    consensus::{block_hash, transaction_txid},
    primitives::{
        Amount, BlockHash, InventoryType, InventoryVector, OutPoint, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    },
};
use open_bitcoin_mempool::{
    FinalMempoolMembership, MempoolAcceptanceTime, MempoolOrigin, MempoolOutcome,
    MempoolRemovalCause, MempoolRemovalRole, PolicyConfig, PolicyTime, RelayIntent,
};
use open_bitcoin_network::{
    InventoryList, OrphanPolicy, PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER,
    RelayActivationConfig, WireNetworkMessage,
};

use super::{
    assert_getdata, assert_targeted_getdata, build_block, consensus_params, local_config,
    p2sh_script, script, spend_transaction, transaction_relay_inventory, verify_flags,
};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn txid_inventory(txid: Txid) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: txid.into(),
    }])
}

fn network_with_chain(
    nonce: u64,
    block_count: u32,
    mempool_config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(nonce),
        mempool_config,
    );
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut coinbase_txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(previous_hash, height, 500_000_000);
        coinbase_txids.push(txid(&block.transactions[0]));
        previous_hash = block_hash(&block.header);
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("connect fixture block");
    }

    (network, coinbase_txids)
}

fn relay_enabled_network_with_chain(
    nonce: u64,
    block_count: u32,
    mempool_config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        mempool_config,
        RelayActivationConfig { enabled: true },
        true,
    );
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut coinbase_txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(previous_hash, height, 500_000_000);
        coinbase_txids.push(txid(&block.transactions[0]));
        previous_hash = block_hash(&block.header);
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("connect fixture block");
    }

    (network, coinbase_txids)
}

fn parent_and_child(previous_txid: Txid) -> (Transaction, Transaction) {
    let parent = spend_transaction(previous_txid, 499_999_000);
    let child = spend_transaction(txid(&parent), 499_998_000);
    (parent, child)
}

fn two_input_child(first_parent: Txid, second_parent: Txid, output_value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![
            TransactionInput {
                previous_output: OutPoint {
                    txid: first_parent,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
            TransactionInput {
                previous_output: OutPoint {
                    txid: second_parent,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
        ],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn low_fee_spend(previous_txid: Txid) -> Transaction {
    spend_transaction(previous_txid, 499_999_999)
}

fn test_orphan_policy(max_total_orphans: usize, max_orphans_per_peer: usize) -> OrphanPolicy {
    test_orphan_policy_with_reconsideration_cap(max_total_orphans, max_orphans_per_peer, 8)
}

fn test_orphan_policy_with_reconsideration_cap(
    max_total_orphans: usize,
    max_orphans_per_peer: usize,
    max_reconsiderations_per_parent: usize,
) -> OrphanPolicy {
    OrphanPolicy {
        max_total_orphans,
        max_orphans_per_peer,
        max_announcers_per_orphan: 8,
        orphan_ttl_seconds: 1,
        max_reconsiderations_per_parent,
    }
}

fn assert_mempool_contains(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    expected_txid: Txid,
) {
    assert!(network.mempool().mempool().entry(&expected_txid).is_some());
}

fn assert_not_stored(network: &ManagedPeerNetwork<MemoryChainstateStore>, rejected_txid: Txid) {
    assert!(!network.transactions_by_txid.contains_key(&rejected_txid));
}

#[test]
fn managed_admission_bridge_peer_tx_uses_download_boundary_before_mempool() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(610, 2, PolicyConfig::default());
    network.connect_outbound_peer(610, 0).expect("peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let inventory = transaction_relay_inventory(&transaction);

    // Act
    let inventory_outbound = network
        .receive_message(
            610,
            WireNetworkMessage::Inv(inventory.clone()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("inventory")
        .outbound;
    let count_before_tx = network.mempool_info().transaction_count;
    let result = network
        .receive_sync_message(
            610,
            WireNetworkMessage::Tx(transaction.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("tx");

    // Assert
    assert_getdata(&inventory_outbound, inventory);
    assert_eq!(count_before_tx, 0);
    assert!(result.outbound.is_empty());
    assert!(result.targeted_outbound.is_empty());
    assert_mempool_contains(&network, txid(&transaction));
}

#[test]
fn managed_admission_bridge_peer_missing_parent_stages_orphan_and_requests_parent() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(611, 2, PolicyConfig::default());
    network.connect_outbound_peer(611, 0).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let parent_txid = txid(&parent);

    // Act
    let result = network
        .process_peer_transaction_admission(
            611,
            child.clone(),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("orphan outcome");

    // Assert
    assert!(matches!(
        result.outcome,
        MempoolOutcome::Orphaned {
            missing_parents,
            ..
        } if missing_parents == vec![parent_txid]
    ));
    assert_eq!(network.orphan_count(), 1);
    assert_targeted_getdata(&result.targeted_outbound, 611, txid_inventory(parent_txid));
    assert_not_stored(&network, txid(&child));
}

#[test]
fn managed_admission_bridge_parent_acceptance_reconsiders_child() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(612, 2, PolicyConfig::default());
    network.add_inbound_peer(612).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let parent_txid = txid(&parent);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(612, child, 10, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    let result = network
        .process_peer_transaction_admission(612, parent, 11, verify_flags(), consensus_params())
        .expect("accept parent");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Accepted { txid, .. } if txid == parent_txid));
    assert!(result.reconsidered.iter().any(
        |outcome| matches!(outcome, MempoolOutcome::Accepted { txid, .. } if *txid == child_txid)
    ));
    assert_eq!(network.orphan_count(), 0);
    assert_mempool_contains(&network, parent_txid);
    assert_mempool_contains(&network, child_txid);
}

#[test]
fn managed_admission_bridge_peer_admission_preserves_receive_metadata() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(623, 2, PolicyConfig::default());
    network.add_inbound_peer(623).expect("peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .process_peer_transaction_admission(
            623,
            transaction,
            42,
            verify_flags(),
            consensus_params(),
        )
        .expect("peer admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted peer transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(42))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Peer);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_reconsidered_orphan_uses_reconsideration_metadata() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(624, 2, PolicyConfig::default());
    network.add_inbound_peer(624).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(624, child, 41, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    network
        .process_peer_transaction_admission(624, parent, 43, verify_flags(), consensus_params())
        .expect("accept parent and reconsider child");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&child_txid)
        .expect("reconsidered child")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(43))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Peer);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_peer_duplicate_preserves_first_metadata() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(625, 2, PolicyConfig::default());
    network.add_inbound_peer(625).expect("peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    network
        .process_peer_transaction_admission(
            625,
            transaction.clone(),
            42,
            verify_flags(),
            consensus_params(),
        )
        .expect("first admission");

    // Act
    network
        .process_peer_transaction_admission(
            625,
            transaction,
            44,
            verify_flags(),
            consensus_params(),
        )
        .expect("duplicate admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("original peer transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(42))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Peer);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_drains_ready_orphans_after_reconsideration_cap() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(620, 2, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy_with_reconsideration_cap(10, 10, 1));
    network.add_inbound_peer(620).expect("peer");
    let parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let parent_txid = txid(&parent);
    let children = vec![
        spend_transaction(parent_txid, 499_998_000),
        spend_transaction(parent_txid, 499_997_000),
        spend_transaction(parent_txid, 499_996_000),
    ];
    let child_txids: Vec<_> = children.iter().map(txid).collect();
    for (index, child) in children.into_iter().enumerate() {
        network
            .process_peer_transaction_admission(
                620,
                child,
                20 + index as i64,
                verify_flags(),
                consensus_params(),
            )
            .expect("stage child");
    }
    assert_eq!(network.orphan_count(), 3);

    // Act
    let result = network
        .process_peer_transaction_admission(620, parent, 30, verify_flags(), consensus_params())
        .expect("accept parent");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Accepted { txid, .. } if txid == parent_txid));
    for child_txid in child_txids {
        assert!(
            result
                .reconsidered
                .iter()
                .any(|outcome| outcome.txid() == child_txid),
            "missing reconsidered outcome for child {child_txid:?}"
        );
    }
    assert_eq!(result.reconsidered.len(), 3);
    assert_eq!(network.orphan_count(), 0);
}

#[test]
fn managed_admission_bridge_still_missing_parent_child_remains_staged() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(613, 3, PolicyConfig::default());
    network.add_inbound_peer(613).expect("peer");
    let first_parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let second_parent = spend_transaction(coinbase_txids[1], 499_999_000);
    let first_parent_txid = txid(&first_parent);
    let second_parent_txid = txid(&second_parent);
    let child = two_input_child(first_parent_txid, second_parent_txid, 499_998_000);
    let staged = network
        .process_peer_transaction_admission(
            613,
            child.clone(),
            20,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage child");
    let MempoolOutcome::Orphaned {
        missing_parents, ..
    } = staged.outcome
    else {
        panic!("expected staged child to be orphaned");
    };
    assert!(missing_parents.len() >= 2);
    let requested_parent = missing_parents[0];
    let accepted_parent = if requested_parent == first_parent_txid {
        first_parent
    } else {
        assert_eq!(requested_parent, second_parent_txid);
        second_parent
    };

    // Act
    let result = network
        .process_peer_transaction_admission(
            613,
            accepted_parent,
            21,
            verify_flags(),
            consensus_params(),
        )
        .expect("accept first parent");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Accepted { .. }));
    assert!(result.reconsidered.is_empty());
    assert_eq!(network.orphan_count(), 1);
    assert_not_stored(&network, txid(&child));
}

#[test]
fn managed_admission_bridge_rejected_child_is_removed_from_orphanage() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(614, 2, PolicyConfig::default());
    network.add_inbound_peer(614).expect("peer");
    let parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let child = spend_transaction(txid(&parent), 500_000_000);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(614, child, 30, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    let result = network
        .process_peer_transaction_admission(614, parent, 31, verify_flags(), consensus_params())
        .expect("accept parent");

    // Assert
    assert!(result.reconsidered.iter().any(
        |outcome| matches!(outcome, MempoolOutcome::Rejected { txid, .. } if *txid == child_txid)
    ));
    assert_eq!(network.orphan_count(), 0);
    assert_not_stored(&network, child_txid);
}

#[test]
fn managed_admission_bridge_orphan_expiry_returns_expired_outcome_without_sleep() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(615, 2, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy(4, 4));
    network.add_inbound_peer(615).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(615, child, 100, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    let outcomes = network.expire_orphan_transactions(102);

    // Assert
    assert!(outcomes.iter().any(
        |outcome| matches!(outcome, MempoolOutcome::Expired { txid, .. } if *txid == child_txid)
    ));
    assert_eq!(network.orphan_count(), 0);
    assert_not_stored(&network, txid(&parent));
}

#[test]
fn managed_admission_bridge_orphan_cap_eviction_returns_evicted_outcome() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(616, 3, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy(1, 1));
    network.add_inbound_peer(616).expect("peer");
    let (first_parent, first_child) = parent_and_child(coinbase_txids[0]);
    let (second_parent, second_child) = parent_and_child(coinbase_txids[1]);
    let first_child_txid = txid(&first_child);

    // Act
    network
        .process_peer_transaction_admission(
            616,
            first_child,
            200,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage first child");
    let result = network
        .process_peer_transaction_admission(
            616,
            second_child,
            201,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage second child");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Orphaned { .. }));
    assert!(
        result
            .reconsidered
            .iter()
            .any(|outcome| matches!(outcome, MempoolOutcome::Evicted { txid, .. } if *txid == first_child_txid))
    );
    assert_eq!(network.orphan_count(), 1);
    assert_not_stored(&network, txid(&first_parent));
    assert_not_stored(&network, txid(&second_parent));
}

#[test]
fn managed_admission_bridge_duplicate_and_rejected_peer_txs_do_not_store_transaction() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(617, 3, PolicyConfig::default());
    network.add_inbound_peer(617).expect("peer");
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let rejected = low_fee_spend(coinbase_txids[1]);
    let accepted_txid = txid(&accepted);
    let rejected_txid = txid(&rejected);
    network
        .process_peer_transaction_admission(
            617,
            accepted.clone(),
            300,
            verify_flags(),
            consensus_params(),
        )
        .expect("accepted");
    let stored_count = network.transactions_by_txid.len();

    // Act
    let duplicate = network
        .process_peer_transaction_admission(617, accepted, 301, verify_flags(), consensus_params())
        .expect("duplicate");
    let rejected_result = network
        .process_peer_transaction_admission(617, rejected, 302, verify_flags(), consensus_params())
        .expect("rejected");

    // Assert
    assert!(
        matches!(duplicate.outcome, MempoolOutcome::Duplicate { txid } if txid == accepted_txid)
    );
    assert!(
        matches!(rejected_result.outcome, MempoolOutcome::Rejected { txid, .. } if txid == rejected_txid)
    );
    assert_eq!(network.transactions_by_txid.len(), stored_count);
    assert_not_stored(&network, rejected_txid);
}

#[test]
fn managed_admission_bridge_replacement_removes_replaced_indexes() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(618, 2, PolicyConfig::default());
    network.add_inbound_peer(618).expect("peer");
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let replacement = spend_transaction(coinbase_txids[0], 499_996_000);
    let original_txid = txid(&original);
    let replacement_txid = txid(&replacement);
    network
        .process_peer_transaction_admission(618, original, 400, verify_flags(), consensus_params())
        .expect("original");

    // Act
    let result = network
        .process_peer_transaction_admission(
            618,
            replacement,
            401,
            verify_flags(),
            consensus_params(),
        )
        .expect("replacement");

    // Assert
    assert!(matches!(
        result.outcome,
        MempoolOutcome::Replaced { txid, ref replaced, .. }
            if txid == replacement_txid && replaced == &vec![original_txid]
    ));
    assert_not_stored(&network, original_txid);
    assert_mempool_contains(&network, replacement_txid);
    assert!(network.transactions_by_txid.contains_key(&replacement_txid));
    assert!(result.delta.removed.iter().any(|removal| {
        removal.member.txid == original_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(result.delta.final_membership.iter().any(|state| {
        state.member.txid == original_txid && state.membership == FinalMempoolMembership::Absent
    }));
    assert!(result.delta.final_membership.iter().any(|state| {
        state.member.txid == replacement_txid && state.membership == FinalMempoolMembership::Present
    }));
}

#[test]
fn managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(619, 2, PolicyConfig::default());
    for peer_id in 619..=621 {
        network
            .connect_outbound_peer(peer_id, 0)
            .expect("relay peer");
    }
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let child_inventory = transaction_relay_inventory(&child);
    network
        .receive_message(
            619,
            WireNetworkMessage::Inv(child_inventory.clone()),
            500,
            verify_flags(),
            consensus_params(),
        )
        .expect("delivering peer inventory");
    network
        .receive_message(
            620,
            WireNetworkMessage::Inv(child_inventory.clone()),
            501,
            verify_flags(),
            consensus_params(),
        )
        .expect("second announcer inventory");
    network
        .receive_sync_message(
            619,
            WireNetworkMessage::Tx(child),
            502,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage child through receipt provenance");
    let before_disconnect = network.peer_manager().transaction_request_snapshot(619);

    // Act
    let first_disconnect = network
        .disconnect_peer_at(619, 503)
        .expect("disconnect cleanup");
    let after_disconnect = network.peer_manager().transaction_request_snapshot(619);
    let late_inventory = network
        .receive_message(
            621,
            WireNetworkMessage::Inv(child_inventory),
            504,
            verify_flags(),
            consensus_params(),
        )
        .expect("late orphan announcer");
    network
        .disconnect_peer_at(620, 505)
        .expect("second announcer disconnect");

    // Assert
    assert_eq!(before_disconnect.in_flight_count, 1);
    assert!(first_disconnect.is_empty());
    assert_eq!(after_disconnect.in_flight_count, 0);
    assert!(late_inventory.outbound.is_empty());
    assert_eq!(network.orphan_count(), 1);
    network
        .disconnect_peer_at(621, 506)
        .expect("late announcer disconnect");
    assert_eq!(network.orphan_count(), 0);
    assert_eq!(network.network_info().connected_peers, 0);
    assert_not_stored(&network, txid(&parent));
}

#[test]
fn managed_admission_bridge_local_submission_uses_same_outcome_contract() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(620, 2, PolicyConfig::default());
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let accepted_txid = txid(&accepted);
    let orphan_parent = spend_transaction(coinbase_txids[0], 499_998_000);
    let orphan = spend_transaction(txid(&orphan_parent), 499_997_000);

    // Act
    let accepted_outcome = network
        .submit_local_transaction_outcome_at(
            accepted.clone(),
            verify_flags(),
            consensus_params(),
            40,
            RelayIntent::NotRequested,
        )
        .expect("accepted outcome");
    let duplicate_outcome = network
        .submit_local_transaction_outcome_at(
            accepted,
            verify_flags(),
            consensus_params(),
            41,
            RelayIntent::NotRequested,
        )
        .expect("duplicate outcome");
    let orphan_outcome = network
        .submit_local_transaction_outcome_at(
            orphan,
            verify_flags(),
            consensus_params(),
            42,
            RelayIntent::NotRequested,
        )
        .expect("orphan outcome");

    // Assert
    assert!(
        matches!(accepted_outcome, MempoolOutcome::Accepted { txid, .. } if txid == accepted_txid)
    );
    assert!(
        matches!(duplicate_outcome, MempoolOutcome::Duplicate { txid } if txid == accepted_txid)
    );
    assert!(matches!(orphan_outcome, MempoolOutcome::Orphaned { .. }));
    assert_eq!(network.orphan_count(), 0);
}

#[test]
fn managed_admission_bridge_local_not_requested_preserves_explicit_time() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(629, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            45,
            RelayIntent::NotRequested,
        )
        .expect("explicit local admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted explicit transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(45))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_local_requested_admission_preserves_metadata() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(626, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::Requested,
        )
        .expect("local admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted local transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(50))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::Requested);
}

#[test]
fn managed_admission_bridge_local_not_requested_admission_preserves_metadata() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(627, 2, PolicyConfig::default());
    network
        .connect_outbound_peer(630, 1)
        .expect("eligible peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::NotRequested,
        )
        .expect("local admission without relay");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted local transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(50))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
    assert_eq!(network.relay_fanout_info().queued_transactions, 0);
}

#[test]
fn managed_admission_bridge_local_duplicate_preserves_first_metadata() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(628, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    network
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::Requested,
        )
        .expect("first local admission");

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            51,
            RelayIntent::NotRequested,
        )
        .expect("duplicate local admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("original local transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(50))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::Requested);
}

#[test]
fn managed_admission_bridge_explicit_local_submission_preserves_outcome_contract() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(621, 3, PolicyConfig::default());
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let accepted_txid = txid(&accepted);
    let (parent, orphan) = parent_and_child(coinbase_txids[1]);

    // Act
    let accepted_outcome = network
        .submit_local_transaction_outcome_at(
            accepted,
            verify_flags(),
            consensus_params(),
            52,
            RelayIntent::NotRequested,
        )
        .expect("explicit accepted outcome");
    let orphan_outcome = network
        .submit_local_transaction_outcome_at(
            orphan,
            verify_flags(),
            consensus_params(),
            53,
            RelayIntent::NotRequested,
        )
        .expect("explicit orphan outcome");

    // Assert
    assert!(matches!(
        accepted_outcome,
        MempoolOutcome::Accepted { txid, .. } if txid == accepted_txid
    ));
    assert!(matches!(orphan_outcome, MempoolOutcome::Orphaned { .. }));
    assert_not_stored(&network, txid(&parent));
}

#[test]
fn managed_admission_bridge_resource_caps_preserved_under_orphan_burst() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(622, 6, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy(3, 3));
    network.add_inbound_peer(622).expect("peer");
    let orphan_children = coinbase_txids
        .iter()
        .take(5)
        .map(|coinbase_txid| parent_and_child(*coinbase_txid).1)
        .collect::<Vec<_>>();

    // Act
    let results = orphan_children
        .into_iter()
        .enumerate()
        .map(|(index, child)| {
            network
                .process_peer_transaction_admission(
                    622,
                    child,
                    600 + index as i64,
                    verify_flags(),
                    consensus_params(),
                )
                .expect("orphan burst")
        })
        .collect::<Vec<_>>();
    let request_snapshot = network.peer_manager().transaction_request_snapshot(622);
    let evicted_count = results
        .iter()
        .flat_map(|result| result.reconsidered.iter())
        .filter(|outcome| matches!(outcome, MempoolOutcome::Evicted { .. }))
        .count();

    // Assert
    assert_eq!(network.orphan_count(), 3);
    assert!(evicted_count >= 2);
    assert!(request_snapshot.in_flight_count <= PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER);
}

#[test]
fn managed_admission_bridge_orphaned_peer_tx_feeds_compact_extra_txn() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(119_301, 2, PolicyConfig::default());
    network.connect_outbound_peer(119_301, 0).expect("peer");
    let (_parent, child) = parent_and_child(coinbase_txids[0]);
    assert_eq!(network.compact_extra_txn_len(), 0);

    // Act
    let result = network
        .process_peer_transaction_admission(119_301, child, 10, verify_flags(), consensus_params())
        .expect("orphan outcome");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Orphaned { .. }));
    assert_eq!(
        network.compact_extra_txn_len(),
        1,
        "orphaned staged body must push into CompactExtraTxnBuffer"
    );
}

#[test]
fn managed_admission_bridge_rejected_peer_tx_feeds_compact_extra_txn_gated() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(119_302, 3, PolicyConfig::default());
    network.add_inbound_peer(119_302).expect("peer");
    let rejected = low_fee_spend(coinbase_txids[1]);
    assert_eq!(network.compact_extra_txn_len(), 0);

    // Act
    let result = network
        .process_peer_transaction_admission(
            119_302,
            rejected,
            20,
            verify_flags(),
            consensus_params(),
        )
        .expect("rejected");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Rejected { .. }));
    assert_eq!(
        network.compact_extra_txn_len(),
        1,
        "rejected body under per-tx size gate must push_gated into CompactExtraTxnBuffer"
    );
}

#[test]
fn managed_admission_bridge_replaced_victims_feed_compact_extra_txn() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(119_303, 2, PolicyConfig::default());
    network.add_inbound_peer(119_303).expect("peer");
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let replacement = spend_transaction(coinbase_txids[0], 499_996_000);
    network
        .process_peer_transaction_admission(
            119_303,
            original,
            30,
            verify_flags(),
            consensus_params(),
        )
        .expect("original");
    let len_before_replace = network.compact_extra_txn_len();

    // Act
    let result = network
        .process_peer_transaction_admission(
            119_303,
            replacement,
            31,
            verify_flags(),
            consensus_params(),
        )
        .expect("replacement");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Replaced { .. }));
    assert!(
        network.compact_extra_txn_len() > len_before_replace,
        "replaced victim bodies must push into CompactExtraTxnBuffer before demotion"
    );
}
