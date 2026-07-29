// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid};
use open_bitcoin_primitives::{BlockHash, Transaction, TransactionInput};

use super::{build_block, sample_chainstate_snapshot, spend_transaction};
use crate::{
    AdmissionContext, BlockLifecycleContext, FinalMempoolMembership, Mempool,
    MempoolAcceptanceTime, MempoolCapacity, MempoolEntryMetadata, MempoolError, MempoolOrigin,
    MempoolRemovalCause, PolicyConfig, PolicyTime, RelayIntent,
};

fn verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
}

fn consensus_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

fn admission_transaction(previous_txid: open_bitcoin_primitives::Txid) -> Transaction {
    spend_transaction(
        previous_txid,
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    )
}

fn admit(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: Transaction,
    context: AdmissionContext,
) {
    mempool
        .accept_transaction_with_context(
            transaction,
            snapshot,
            verify_flags(),
            consensus_params(),
            context,
        )
        .expect("fixture admission");
}

#[test]
fn expiry_preparation_is_pure_and_orders_descendants_before_ancestors() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = admission_transaction(coinbase_txids[0]);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let child_txid = transaction_txid(&child).expect("child txid");
    let accepted_at = PolicyTime::new(100);
    let context = AdmissionContext::new(MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(accepted_at),
        MempoolOrigin::Local,
        RelayIntent::NotRequested,
    ));
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_expiry_hours: 1,
        ..PolicyConfig::default()
    });
    for transaction in [parent, child] {
        admit(&mut mempool, &snapshot, transaction, context);
    }
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_expiry(PolicyTime::new(3_701))
        .expect("expiry preparation");

    // Assert
    assert_eq!(mempool.complete_snapshot(), before);
    assert!(
        prepared
            .facts()
            .removed()
            .iter()
            .all(|removed| removed.removal.cause == MempoolRemovalCause::Expiry)
    );
    assert_eq!(
        prepared
            .facts()
            .teardown_order()
            .iter()
            .map(|member| member.txid)
            .collect::<Vec<_>>(),
        [child_txid, parent_txid]
    );
}

#[test]
fn pressure_preparation_is_pure_and_orders_the_evicted_graph_for_teardown() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(4);
    let low_fee_parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_900,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&low_fee_parent).expect("parent txid");
    let low_fee_child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let child_txid = transaction_txid(&low_fee_child).expect("child txid");
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut staging = Mempool::default();
    for transaction in [low_fee_parent.clone(), low_fee_child.clone()] {
        admit(
            &mut staging,
            &snapshot,
            transaction,
            AdmissionContext::legacy_unknown(),
        );
    }
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(staging.accounted_memory().as_usize()),
        ..PolicyConfig::default()
    });
    for transaction in [low_fee_parent, low_fee_child] {
        admit(
            &mut mempool,
            &snapshot,
            transaction,
            AdmissionContext::legacy_unknown(),
        );
    }
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_transaction_with_context(
            high_fee,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("pressure preparation");

    // Assert
    assert_eq!(mempool.complete_snapshot(), before);
    assert!(
        prepared
            .facts()
            .removed()
            .iter()
            .all(|removed| removed.removal.cause == MempoolRemovalCause::Pressure)
    );
    assert_eq!(
        prepared
            .facts()
            .teardown_order()
            .iter()
            .map(|member| member.txid)
            .collect::<Vec<_>>(),
        [child_txid, parent_txid]
    );
}

#[test]
fn connected_block_preparation_is_pure_for_confirmation_and_conflict_removals() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let confirmed = admission_transaction(coinbase_txids[0]);
    let confirmed_txid = transaction_txid(&confirmed).expect("confirmed txid");
    let conflict = admission_transaction(coinbase_txids[1]);
    let conflict_txid = transaction_txid(&conflict).expect("conflict txid");
    let descendant = spend_transaction(
        conflict_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let replacement = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    for transaction in [confirmed.clone(), conflict, descendant] {
        admit(
            &mut mempool,
            &snapshot,
            transaction,
            AdmissionContext::legacy_unknown(),
        );
    }
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    block.transactions.extend([confirmed, replacement]);
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block preparation");

    // Assert
    assert_eq!(mempool.complete_snapshot(), before);
    assert!(prepared.facts().removed().iter().any(|removed| {
        removed.removal.member.txid == confirmed_txid
            && removed.removal.cause == MempoolRemovalCause::BlockConfirmation
    }));
    let teardown_txids = prepared
        .facts()
        .teardown_order()
        .iter()
        .map(|member| member.txid)
        .collect::<Vec<_>>();
    assert_eq!(teardown_txids.len(), 3);
    assert!(teardown_txids.contains(&confirmed_txid));
    assert!(
        teardown_txids
            .iter()
            .position(|txid| *txid == descendant_txid)
            < teardown_txids
                .iter()
                .position(|txid| *txid == conflict_txid)
    );
}

#[test]
fn reorg_steps_require_consuming_block_removal_before_reconsideration() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let txid = transaction_txid(&transaction).expect("transaction txid");
    let mut mempool = Mempool::default();
    admit(
        &mut mempool,
        &snapshot,
        transaction.clone(),
        AdmissionContext::legacy_unknown(),
    );
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 499_999_000);
    block.transactions.push(transaction.clone());
    let removal = mempool
        .prepare_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block preparation");

    // Act
    let early_error = mempool
        .prepare_transaction_with_context(
            transaction.clone(),
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::reorg(PolicyTime::new(71)),
        )
        .expect_err("reconsideration before removal must fail");
    mempool
        .commit_prepared_mempool_transition(removal)
        .expect("removal commit");
    let reconsideration = mempool
        .prepare_transaction_with_context(
            transaction,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::reorg(PolicyTime::new(71)),
        )
        .expect("reconsideration after removal");

    // Assert
    assert!(matches!(
        early_error,
        MempoolError::DuplicateTransaction { .. }
    ));
    assert_eq!(reconsideration.facts().admitted_order()[0].txid, txid);
    assert_eq!(
        reconsideration.facts().delta().final_membership[0].membership,
        FinalMempoolMembership::Present
    );
}
