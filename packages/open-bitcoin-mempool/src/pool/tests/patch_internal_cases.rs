// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::{BTreeMap, BTreeSet, HashMap};

use open_bitcoin_consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{Amount, TransactionInput, Txid};

use super::{
    ProspectiveGraph, aggregate_stats, checked_count, checked_fee_sum, closure, validate_stat_limit,
};
use crate::{
    AggregateStats, LimitDirection, Mempool, MempoolEntry, MempoolError, PolicyConfig,
    TransactionVirtualSize,
};

use super::super::tests::spend_transaction;

fn entry(previous: Txid, vout: u32, marker: u8, vsize: usize, fee: i64) -> MempoolEntry {
    let transaction = spend_transaction(
        previous,
        vout,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).unwrap_or(Txid::from_byte_array([marker; 32]));
    let wtxid = transaction_wtxid(&transaction).expect("fixture transaction serializes");
    MempoolEntry::new(
        transaction,
        txid,
        wtxid,
        Amount::from_sats(fee).expect("fixture fee is in range"),
        TransactionVirtualSize::new(vsize),
        vsize.saturating_mul(4),
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    )
}

#[test]
fn graph_helpers_cover_empty_invalid_link_and_cycle_safe_paths() {
    // Arrange
    let empty = ProspectiveGraph {
        updates: BTreeMap::new(),
        self_rates: BTreeMap::new(),
    };
    let parent_txid = Txid::from_byte_array([1; 32]);
    let child_txid = Txid::from_byte_array([2; 32]);
    let mut parent = entry(Txid::from_byte_array([9; 32]), 0, 1, 1, 1);
    parent.txid = parent_txid;
    let mut child = entry(parent_txid, 99, 2, 1, 1);
    child.txid = child_txid;
    let mempool = Mempool {
        entries: HashMap::from([(parent_txid, parent), (child_txid, child)]),
        ..Mempool::default()
    };

    // Act
    let empty_error = empty
        .eviction_package()
        .expect_err("empty graph has no eviction candidate");
    let missing_candidate = empty
        .validate_limits(&PolicyConfig::default(), child_txid)
        .expect_err("missing candidate is an invariant error");
    let graph = ProspectiveGraph::build(&mempool, &BTreeMap::new(), &BTreeSet::new())
        .expect("graph construction");
    let diamond_edges = BTreeMap::from([
        (
            parent_txid,
            BTreeSet::from([child_txid, Txid::from_byte_array([3; 32])]),
        ),
        (child_txid, BTreeSet::from([Txid::from_byte_array([4; 32])])),
        (
            Txid::from_byte_array([3; 32]),
            BTreeSet::from([Txid::from_byte_array([4; 32])]),
        ),
    ]);

    // Assert
    assert!(matches!(
        empty_error,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(matches!(
        missing_candidate,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(
        graph
            .updates
            .get(&child_txid)
            .expect("child update")
            .parents
            .is_empty()
    );
    assert_eq!(closure(&diamond_edges, parent_txid).len(), 3);
}

#[test]
fn aggregate_and_limit_helpers_cover_checked_failure_paths() {
    // Arrange
    let base_txid = Txid::from_byte_array([5; 32]);
    let related_txid = Txid::from_byte_array([6; 32]);
    let mut base = entry(Txid::from_byte_array([7; 32]), 0, 5, usize::MAX, 1);
    base.txid = base_txid;
    let mut related = entry(Txid::from_byte_array([8; 32]), 0, 6, 1, 1);
    related.txid = related_txid;
    let upserts = BTreeMap::from([(base_txid, base), (related_txid, related)]);
    let mempool = Mempool::default();

    // Act
    let missing_entry = aggregate_stats(&mempool, &BTreeMap::new(), base_txid, &BTreeSet::new())
        .expect_err("missing aggregate entry");
    let virtual_size_overflow = aggregate_stats(
        &mempool,
        &upserts,
        base_txid,
        &BTreeSet::from([related_txid]),
    )
    .expect_err("aggregate virtual size overflow");
    let missing_related = aggregate_stats(
        &mempool,
        &upserts,
        related_txid,
        &BTreeSet::from([Txid::from_byte_array([10; 32])]),
    )
    .expect("missing related entry is ignored");
    let fee_overflow = checked_fee_sum(i64::MAX, 1).expect_err("fee overflow");
    let count_overflow =
        checked_count(usize::MAX, "fixture count overflow").expect_err("count overflow");
    let size_limit = validate_stat_limit(
        AggregateStats::new(1, TransactionVirtualSize::new(2), 1),
        2,
        1,
        LimitDirection::Ancestor,
        None,
    )
    .expect_err("virtual size limit");

    // Assert
    for error in [
        missing_entry,
        virtual_size_overflow,
        fee_overflow,
        count_overflow,
        size_limit,
    ] {
        assert!(matches!(
            error,
            MempoolError::InternalInvariant { .. } | MempoolError::LimitExceeded { .. }
        ));
    }
    assert_eq!(missing_related.count, 2);
}
