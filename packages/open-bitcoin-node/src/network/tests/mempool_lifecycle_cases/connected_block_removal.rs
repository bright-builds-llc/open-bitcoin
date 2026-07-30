// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;
use crate::network::mempool_lifecycle;

#[test]
fn connected_block_reconciles_more_than_the_orphan_policy_limit() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let mut funding = spend_transaction(coinbase_txids[0], 450_000_000);
    let funding_output = funding.outputs[0].clone();
    funding.outputs = vec![funding_output; 127];
    for output in &mut funding.outputs {
        output.value = open_bitcoin_core::primitives::Amount::from_sats(3_000_000)
            .expect("bounded funding output");
    }
    let funding_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![funding.clone()]);
    network
        .connect_local_block(&funding_block, verify_flags(), consensus_params())
        .expect("connect funding block");
    let funding_txid = txid(&funding);
    let mut confirmed = Vec::with_capacity(127);
    for output_index in 0_u32..127 {
        let mut transaction = spend_transaction(funding_txid, 2_999_000);
        transaction.inputs[0].previous_output.vout = output_index;
        transaction.lock_time = output_index;
        network
            .submit_local_transaction_outcome_at(
                transaction.clone(),
                verify_flags(),
                consensus_params(),
                100 + i64::from(output_index),
                RelayIntent::NotRequested,
            )
            .expect("submit independent lifecycle member");
        confirmed.push(transaction);
    }
    let confirmation_block =
        build_block_with_transactions(block_hash(&funding_block.header), 3, confirmed);

    // Act
    network
        .connect_local_block(&confirmation_block, verify_flags(), consensus_params())
        .expect("valid whole-mempool lifecycle delta should commit");

    // Assert
    assert_eq!(network.mempool_info().transaction_count, 0);
    assert_eq!(network.relay_serving_info().serveable_transactions, 0);
    assert_eq!(network.relay_fanout_info().known_transactions, 0);
    assert_eq!(network.reconcile_lifecycle_projection().counts(), [0; 7]);
    assert_lifecycle_authority(&network, 128);
}

#[test]
fn managed_block_connect_removes_confirmed_mempool_transaction_and_runtime_caches() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = transaction_wtxid(&transaction).expect("wtxid");
    network
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            10,
            RelayIntent::NotRequested,
        )
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
    assert_eq!(info.rolling_fee_parity, RollingFeeParityStatus::Active);
    assert_lifecycle_authority(&network, 2);
}

#[test]
fn managed_block_connect_uses_explicit_context_and_typed_delta() {
    // Arrange
    let (mut network, _genesis, spendable, coinbase_txids) = network_with_chain();
    let confirmed = spend_transaction(coinbase_txids[0], 499_999_000);
    let confirmed_txid = txid(&confirmed);
    let conflict = spend_transaction(coinbase_txids[1], 499_999_000);
    let conflict_txid = txid(&conflict);
    let descendant = spend_transaction(conflict_txid, 499_998_000);
    let descendant_txid = txid(&descendant);
    let in_block_conflict = spend_transaction(coinbase_txids[1], 499_997_000);
    for (timestamp, transaction) in [(10, confirmed.clone()), (11, conflict), (12, descendant)] {
        network
            .submit_local_transaction_outcome_at(
                transaction,
                verify_flags(),
                consensus_params(),
                timestamp,
                RelayIntent::NotRequested,
            )
            .expect("submit lifecycle fixture");
    }
    let confirmation_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![confirmed]);
    let conflict_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![in_block_conflict]);
    let context = mempool_lifecycle::block_lifecycle_context(70, 2);

    // Act
    let confirmation_lifecycle = network
        .apply_connected_block_mempool_lifecycle(&confirmation_block, context)
        .expect("apply confirmation lifecycle");
    let conflict_lifecycle = network
        .apply_connected_block_mempool_lifecycle(&conflict_block, context)
        .expect("apply conflict lifecycle");

    // Assert
    assert_eq!(confirmation_lifecycle.context, context);
    assert_eq!(conflict_lifecycle.context, context);
    assert!(confirmation_lifecycle.delta.removed.iter().any(|removal| {
        removal.member.txid == confirmed_txid
            && removal.cause == MempoolRemovalCause::BlockConfirmation
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(conflict_lifecycle.delta.removed.iter().any(|removal| {
        removal.member.txid == conflict_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(conflict_lifecycle.delta.removed.iter().any(|removal| {
        removal.member.txid == descendant_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert!(
        confirmation_lifecycle
            .delta
            .final_membership
            .iter()
            .chain(&conflict_lifecycle.delta.final_membership)
            .all(|state| { state.membership == FinalMempoolMembership::Absent })
    );
    assert!(!network.transactions_by_txid.contains_key(&confirmed_txid));
    assert!(!network.transactions_by_txid.contains_key(&conflict_txid));
    assert!(!network.transactions_by_txid.contains_key(&descendant_txid));
    assert_lifecycle_authority(&network, 5);
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
    assert_lifecycle_authority(&network, 1);
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
        .submit_local_transaction_outcome_at(
            original,
            verify_flags(),
            consensus_params(),
            20,
            RelayIntent::NotRequested,
        )
        .expect("submit original");
    network
        .submit_local_transaction_outcome_at(
            descendant,
            verify_flags(),
            consensus_params(),
            21,
            RelayIntent::NotRequested,
        )
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
    assert_lifecycle_authority(&network, 3);
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
    assert_lifecycle_authority(&network, 1);
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
        .submit_local_transaction_outcome_at(
            replacement,
            verify_flags(),
            consensus_params(),
            30,
            RelayIntent::NotRequested,
        )
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
    assert_lifecycle_authority(&network, 1);
}
