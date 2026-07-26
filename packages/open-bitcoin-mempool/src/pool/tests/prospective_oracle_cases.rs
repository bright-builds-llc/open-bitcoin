// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;

use open_bitcoin_consensus::{ConsensusParams, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{Amount, Transaction, TransactionInput, Txid};

use super::{sample_chainstate_snapshot, spend_transaction, submit};
use crate::pool::candidate::prepare_candidate;
use crate::pool::prospective::{ProspectiveMempool, SubDelta};
use crate::{
    AdmissionContext, FeeRate, Mempool, MempoolEntry, MempoolError, MempoolLifecycleDelta,
    MempoolRemovalCause, TransactionVirtualSize, recompute_resource_ledger,
};

fn prepared_spend(
    mempool: &Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    confirmed_txid: open_bitcoin_primitives::Txid,
    output_value: i64,
) -> crate::pool::candidate::PreparedCandidate {
    let transaction = spend_transaction(
        confirmed_txid,
        0,
        output_value,
        TransactionInput::SEQUENCE_FINAL,
    );
    prepare_candidate(
        mempool,
        transaction,
        snapshot,
        ConsensusParams {
            coinbase_maturity: 1,
            ..ConsensusParams::default()
        },
        AdmissionContext::legacy_unknown(),
    )
    .expect("candidate preparation")
}

fn entry_for_transaction(transaction: Transaction, fee_sats: i64) -> MempoolEntry {
    MempoolEntry::new(
        transaction.clone(),
        transaction_txid(&transaction).expect("entry txid"),
        transaction_wtxid(&transaction).expect("entry wtxid"),
        Amount::from_sats(fee_sats).expect("entry fee"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    )
}

#[test]
fn prospective_insertion_is_overlay_first_and_matches_full_recomputation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mut mempool = Mempool::default();
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    submit(&mut mempool, &snapshot, parent).expect("parent admission");
    let child = prepared_spend(&mempool, &snapshot, parent_txid, 499_998_000);
    let child_txid = child.entry.txid;
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    prospective
        .stage_candidate(child)
        .expect("prospective child insertion");
    let materialized = prospective
        .materialize_for_test()
        .expect("test-only materialization");
    let resource_oracle =
        recompute_resource_ledger(&materialized.entries, &materialized.spent_outpoints)
            .expect("resource oracle");

    // Assert
    assert!(prospective.maybe_entry(&child_txid).is_some());
    assert!(
        prospective
            .maybe_entry(&parent_txid)
            .expect("updated parent")
            .children
            .contains(&child_txid)
    );
    assert_eq!(materialized.resource_ledger, resource_oracle);
    assert_eq!(materialized.entries[&parent_txid].descendant_stats.count, 2);
    assert_eq!(materialized.entries[&child_txid].ancestor_stats.count, 2);
}

#[test]
fn composing_duplicate_addition_fails_without_changing_the_overlay() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mempool = Mempool::default();
    let candidate = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_999_000);
    let candidate_txid = candidate.entry.txid;
    let mut prospective = ProspectiveMempool::new(&mempool);
    let applied = prospective
        .stage_candidate(candidate)
        .expect("initial stage");
    let before = prospective.materialize_for_test().expect("before snapshot");

    // Act
    let error = prospective
        .compose(applied)
        .expect_err("duplicate addition must fail closed");
    let after = prospective.materialize_for_test().expect("after snapshot");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(before.entries, after.entries);
    assert_eq!(
        prospective.maybe_entry(&candidate_txid),
        after.entries.get(&candidate_txid)
    );
}

#[test]
fn conflicting_spent_update_fails_before_overlay_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mempool = Mempool::default();
    let first = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_999_000);
    let second = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_998_000);
    let second_txid = second.entry.txid;
    let mut prospective = ProspectiveMempool::new(&mempool);
    prospective
        .stage_candidate(first)
        .expect("first candidate stage");

    // Act
    let error = prospective
        .stage_candidate(second)
        .expect_err("double-spend edit must fail closed");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(prospective.maybe_entry(&second_txid).is_none());
}

#[test]
fn descendant_package_removal_matches_recompute_state_and_resource_ledger() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let child_txid = transaction_txid(&child).expect("child txid");
    submit(&mut mempool, &snapshot, parent).expect("parent admission");
    submit(&mut mempool, &snapshot, child).expect("child admission");
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    prospective
        .stage_descendant_package_removal(parent_txid, MempoolRemovalCause::Pressure)
        .expect("descendant package removal");
    let materialized = prospective
        .materialize_for_test()
        .expect("test-only materialization");
    let resource_oracle =
        recompute_resource_ledger(&materialized.entries, &materialized.spent_outpoints)
            .expect("resource oracle");

    // Assert
    assert!(prospective.maybe_entry(&parent_txid).is_none());
    assert!(prospective.maybe_entry(&child_txid).is_none());
    assert!(materialized.entries.is_empty());
    assert_eq!(materialized.resource_ledger, resource_oracle);
}

#[test]
fn prepared_patch_contains_only_sparse_facts_and_applies_once() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mempool = Mempool::default();
    let candidate = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_999_000);
    let candidate_txid = candidate.entry.txid;
    let mut prospective = ProspectiveMempool::new(&mempool);
    prospective
        .stage_candidate(candidate)
        .expect("candidate stage");
    let patch = prospective
        .prepare_patch(MempoolLifecycleDelta::empty())
        .expect("sparse patch");
    let mut applied = mempool.clone();

    // Act
    applied.apply_prepared(patch).expect("guarded apply");

    // Assert
    assert!(applied.entry(&candidate_txid).is_some());
    assert_eq!(
        applied.resource_ledger(),
        recompute_resource_ledger(applied.entries(), &applied.spent_outpoints)
            .expect("resource oracle")
    );
}

#[test]
fn leaf_removal_detaches_from_visible_parent() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let child_txid = transaction_txid(&child).expect("child txid");
    submit(&mut mempool, &snapshot, parent).expect("parent admission");
    submit(&mut mempool, &snapshot, child).expect("child admission");
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    prospective
        .stage_descendant_package_removal(child_txid, MempoolRemovalCause::Pressure)
        .expect("leaf removal");

    // Assert
    assert!(
        prospective
            .maybe_entry(&parent_txid)
            .expect("visible parent")
            .children
            .is_empty()
    );
}

#[test]
fn ordered_package_removal_skips_an_already_removed_parent() {
    // Arrange
    let (_, coinbase_txids) = sample_chainstate_snapshot(3);
    let parent_txid = Txid::from_byte_array([1; 32]);
    let child_txid = Txid::from_byte_array([2; 32]);
    let mut parent = entry_for_transaction(
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );
    parent.txid = parent_txid;
    parent.children.insert(child_txid);
    let mut child = entry_for_transaction(
        spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );
    child.txid = child_txid;
    child.parents.insert(parent_txid);
    let mut mempool = Mempool::default();
    mempool.entries = HashMap::from([(parent_txid, parent), (child_txid, child)]);
    mempool.resource_ledger = recompute_resource_ledger(&mempool.entries, &mempool.spent_outpoints)
        .expect("fixture resource ledger");
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    prospective
        .stage_descendant_package_removal(parent_txid, MempoolRemovalCause::Pressure)
        .expect("ordered package removal");

    // Assert
    assert!(prospective.maybe_entry(&parent_txid).is_none());
    assert!(prospective.maybe_entry(&child_txid).is_none());
}

#[test]
fn staged_addition_then_removal_returns_to_baseline_resources() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mempool = Mempool::default();
    let candidate = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_999_000);
    let candidate_txid = candidate.entry.txid;
    let mut prospective = ProspectiveMempool::new(&mempool);
    prospective
        .stage_candidate(candidate)
        .expect("candidate stage");

    // Act
    prospective
        .stage_descendant_package_removal(candidate_txid, MempoolRemovalCause::Pressure)
        .expect("staged candidate removal");

    // Assert
    assert_eq!(prospective.accounted_memory().as_usize(), 0);
    assert!(
        prospective
            .materialize_for_test()
            .expect("baseline oracle")
            .entries
            .is_empty()
    );
}

#[test]
fn staged_replacement_preserves_spent_index_accounting() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let original_txid = transaction_txid(&original).expect("original txid");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original).expect("original admission");
    let replacement = prepared_spend(
        &Mempool::default(),
        &snapshot,
        coinbase_txids[0],
        499_998_000,
    );
    let replacement_txid = replacement.entry.txid;
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    prospective
        .stage_descendant_package_removal(original_txid, MempoolRemovalCause::Replacement)
        .expect("original removal");
    prospective
        .stage_candidate(replacement)
        .expect("replacement stage");
    let materialized = prospective
        .materialize_for_test()
        .expect("replacement oracle");

    // Assert
    assert!(prospective.maybe_entry(&original_txid).is_none());
    assert!(prospective.maybe_entry(&replacement_txid).is_some());
    assert_eq!(materialized.entries.len(), 1);
    assert_eq!(
        materialized.resource_ledger,
        recompute_resource_ledger(&materialized.entries, &materialized.spent_outpoints)
            .expect("resource oracle")
    );
}

#[test]
fn generated_graph_recomputation_oracle_covers_twenty_five_sparse_additions() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(26);
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    for (index, confirmed_txid) in coinbase_txids.into_iter().take(25).enumerate() {
        prospective
            .stage_candidate(prepared_spend(
                &mempool,
                &snapshot,
                confirmed_txid,
                499_999_000 - index as i64,
            ))
            .expect("bounded generated insertion");
    }
    let materialized = prospective
        .materialize_for_test()
        .expect("test-only recompute_state oracle");
    let resource_oracle =
        recompute_resource_ledger(&materialized.entries, &materialized.spent_outpoints)
            .expect("recompute_resource_ledger oracle");

    // Assert
    assert_eq!(materialized.entries.len(), 25);
    assert_eq!(materialized.resource_ledger, resource_oracle);
    assert_eq!(
        prospective.accounted_memory(),
        materialized.resource_ledger.accounted_memory()
    );
    prospective
        .rolling_fee_state_mut()
        .track_package_removed(FeeRate::from_sats_per_kvb(1_000));
    assert_eq!(
        prospective
            .rolling_fee_state_mut()
            .rolling_fee_rate()
            .fee_rate()
            .sats_per_kvb(),
        1_000
    );
    assert_eq!(prospective.full_recompute_count_for_test(), 0);
    assert_eq!(prospective.full_clone_count_for_test(), 0);
}

#[test]
fn empty_subdelta_composition_is_a_noop() {
    // Arrange
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);
    let before = prospective
        .materialize_for_test()
        .expect("before materialization");

    // Act
    prospective
        .compose(SubDelta::from_entries(Vec::new()).expect("empty subdelta"))
        .expect("empty composition");
    let after = prospective
        .materialize_for_test()
        .expect("after materialization");

    // Assert
    assert_eq!(before.entries, after.entries);
    assert_eq!(before.spent_outpoints, HashMap::new());
}
