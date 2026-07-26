// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::{BTreeMap, HashMap};

use open_bitcoin_consensus::{ConsensusParams, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{Amount, Transaction, TransactionInput, Txid, Wtxid};

use super::{sample_chainstate_snapshot, spend_transaction, submit};
use crate::pool::candidate::prepare_candidate;
use crate::pool::lifecycle::MempoolRemovalFact;
use crate::pool::prospective::{ProspectiveMempool, SubDelta};
use crate::{
    AdmissionContext, Mempool, MempoolEntry, MempoolError, MempoolMemberIdentity,
    MempoolRemovalCause, MempoolRemovalRole, ResourceAccountingError, TransactionVirtualSize,
};

fn prepared_spend(
    mempool: &Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    confirmed_txid: Txid,
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

fn removal_delta(member: MempoolMemberIdentity, role: MempoolRemovalRole) -> SubDelta {
    SubDelta::removals(BTreeMap::from([(
        member,
        MempoolRemovalFact {
            cause: MempoolRemovalCause::Pressure,
            role,
        },
    )]))
}

fn parent_child_mempool() -> (Mempool, MempoolMemberIdentity) {
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
    submit(&mut mempool, &snapshot, parent).expect("parent admission");
    submit(&mut mempool, &snapshot, child).expect("child admission");
    let member = MempoolMemberIdentity {
        txid: parent_txid,
        wtxid: mempool.entry(&parent_txid).expect("parent entry").wtxid,
    };
    (mempool, member)
}

#[test]
fn subdelta_identity_conflict_is_rejected() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mempool = Mempool::default();
    let left = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_999_000);
    let mut right = prepared_spend(&mempool, &snapshot, coinbase_txids[1], 499_999_000);
    right.entry.txid = left.entry.txid;

    // Act
    let error = SubDelta::from_entries([left.entry, right.entry])
        .expect_err("conflicting identity pair must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn subdelta_duplicate_wtxid_is_rejected() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mempool = Mempool::default();
    let left = prepared_spend(&mempool, &snapshot, coinbase_txids[0], 499_999_000);
    let mut right = prepared_spend(&mempool, &snapshot, coinbase_txids[1], 499_999_000);
    right.entry.wtxid = left.entry.wtxid;

    // Act
    let error =
        SubDelta::from_entries([left.entry, right.entry]).expect_err("duplicate wtxid must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn staged_wtxid_conflict_with_base_is_rejected() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mut mempool = Mempool::default();
    let existing = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let existing_txid = transaction_txid(&existing).expect("existing txid");
    submit(&mut mempool, &snapshot, existing).expect("existing admission");
    let mut candidate = prepared_spend(&mempool, &snapshot, coinbase_txids[1], 499_999_000);
    candidate.entry.wtxid = mempool.entry(&existing_txid).expect("existing entry").wtxid;
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    let error = prospective
        .stage_candidate(candidate)
        .expect_err("base wtxid conflict must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn missing_pressure_victim_is_rejected() {
    // Arrange
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    let error = prospective
        .stage_descendant_package_removal(
            Txid::from_byte_array([99; 32]),
            MempoolRemovalCause::Pressure,
        )
        .expect_err("missing victim must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn missing_descendant_is_rejected_without_mutation() {
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
    submit(&mut mempool, &snapshot, parent).expect("parent admission");
    mempool
        .entries
        .get_mut(&parent_txid)
        .expect("parent entry")
        .children
        .insert(Txid::from_byte_array([98; 32]));
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    let error = prospective
        .stage_descendant_package_removal(parent_txid, MempoolRemovalCause::Pressure)
        .expect_err("missing descendant must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert!(prospective.maybe_entry(&parent_txid).is_some());
}

#[test]
fn wrong_removal_identity_is_rejected() {
    // Arrange
    let (mempool, parent_member) = parent_child_mempool();
    let wrong_member = MempoolMemberIdentity {
        txid: parent_member.txid,
        wtxid: Wtxid::from_byte_array([97; 32]),
    };

    // Act
    let error = ProspectiveMempool::new(&mempool)
        .compose(removal_delta(wrong_member, MempoolRemovalRole::Direct))
        .expect_err("wrong removal identity must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn missing_removal_entry_is_rejected() {
    // Arrange
    let mempool = Mempool::default();
    let missing_member = MempoolMemberIdentity {
        txid: Txid::from_byte_array([96; 32]),
        wtxid: Wtxid::from_byte_array([95; 32]),
    };

    // Act
    let error = ProspectiveMempool::new(&mempool)
        .compose(removal_delta(missing_member, MempoolRemovalRole::Direct))
        .expect_err("missing removal must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn parent_only_removal_is_rejected() {
    // Arrange
    let (mempool, parent_member) = parent_child_mempool();

    // Act
    let error = ProspectiveMempool::new(&mempool)
        .compose(removal_delta(parent_member, MempoolRemovalRole::Direct))
        .expect_err("parent-only removal must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn repeated_removal_is_rejected() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    submit(&mut mempool, &snapshot, transaction).expect("admission");
    let member = MempoolMemberIdentity {
        txid,
        wtxid: mempool.entry(&txid).expect("entry").wtxid,
    };
    let delta = removal_delta(member, MempoolRemovalRole::Direct);
    let mut prospective = ProspectiveMempool::new(&mempool);
    prospective.compose(delta.clone()).expect("first removal");

    // Act
    let error = prospective
        .compose(delta)
        .expect_err("repeated removal must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn child_first_subdelta_links_a_later_parent() {
    // Arrange
    let (_, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent_txid = Txid::from_byte_array([2; 32]);
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
    let mut child = entry_for_transaction(
        spend_transaction(
            parent_txid,
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );
    child.txid = Txid::from_byte_array([1; 32]);
    let child_txid = child.txid;
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    prospective
        .compose(SubDelta::from_entries([parent, child]).expect("two-entry subdelta"))
        .expect("child-first composition");

    // Assert
    assert!(
        prospective
            .maybe_entry(&parent_txid)
            .expect("parent")
            .children
            .contains(&child_txid)
    );
    assert!(
        prospective
            .maybe_entry(&child_txid)
            .expect("child")
            .parents
            .contains(&parent_txid)
    );
}

#[test]
fn relation_cycles_are_visited_once() {
    // Arrange
    let (_, coinbase_txids) = sample_chainstate_snapshot(3);
    let left_txid = Txid::from_byte_array([1; 32]);
    let right_txid = Txid::from_byte_array([2; 32]);
    let mut left = entry_for_transaction(
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );
    left.txid = left_txid;
    left.children.insert(right_txid);
    let mut right = entry_for_transaction(
        spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );
    right.txid = right_txid;
    right.children.insert(left_txid);
    let mempool = Mempool {
        entries: HashMap::from([(left_txid, left), (right_txid, right)]),
        ..Mempool::default()
    };
    let prospective = ProspectiveMempool::new(&mempool);

    // Act
    let descendants = prospective.collect_descendants(left_txid);

    // Assert
    assert_eq!(descendants, [left_txid, right_txid].into_iter().collect());
}

#[test]
fn checked_resource_underflow_reports_component() {
    // Arrange
    let mut ledger = crate::MempoolResourceLedger::ZERO;
    let (_, coinbase_txids) = sample_chainstate_snapshot(2);
    let entry = entry_for_transaction(
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );

    // Act
    let error = ledger
        .checked_remove_entry(&entry)
        .expect_err("underflow must fail closed");

    // Assert
    assert!(matches!(
        error,
        ResourceAccountingError::Underflow {
            component: "total transaction virtual size"
        }
    ));
}

#[test]
fn checked_resource_memory_underflow_is_atomic() {
    // Arrange
    let (_, coinbase_txids) = sample_chainstate_snapshot(2);
    let entry = entry_for_transaction(
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        1_000,
    );
    let mut ledger =
        crate::MempoolResourceLedger::new(entry.virtual_size, crate::AccountedMempoolMemory::ZERO);
    let before = ledger;

    // Act
    let error = ledger
        .checked_remove_entry(&entry)
        .expect_err("memory underflow must fail closed");

    // Assert
    assert!(matches!(
        error,
        ResourceAccountingError::Underflow {
            component: "total entry accounted memory"
        }
    ));
    assert_eq!(ledger, before);
}

#[test]
fn resource_underflow_display_names_the_component() {
    // Arrange
    let error = ResourceAccountingError::Underflow {
        component: "fixture bytes",
    };

    // Act
    let rendered = error.to_string();

    // Assert
    assert_eq!(
        rendered,
        "mempool resource accounting underflow: fixture bytes"
    );
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
