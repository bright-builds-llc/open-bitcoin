// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/policy/truc_policy.h
// - packages/bitcoin-knots/src/policy/truc_policy.cpp
// - packages/bitcoin-knots/test/functional/mempool_truc.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid, Wtxid,
};

use super::{
    EligibleSiblingEviction, MAX_TRUC_ANCESTOR_COUNT, MAX_TRUC_CHILD_VIRTUAL_SIZE,
    MAX_TRUC_DESCENDANT_COUNT, MAX_TRUC_VIRTUAL_SIZE, TrucPolicyError, evaluate_truc_package,
    prospective_ancestor_count, prospective_descendant_count, validate_truc_package,
};
use crate::policy::replacement::MempoolView;
use crate::pool::candidate::PreparedCandidate;
use crate::{
    CandidateFees, MempoolEntry, MempoolEntryMetadata, TransactionVirtualSize, TrucPolicy,
};

#[derive(Default)]
struct FixtureView {
    entries: BTreeMap<Txid, MempoolEntry>,
}

impl MempoolView for FixtureView {
    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.entries.get(txid)
    }

    fn maybe_spender(&self, _outpoint: &OutPoint) -> Option<Txid> {
        None
    }

    fn collect_descendants(&self, _txid: Txid) -> BTreeSet<Txid> {
        BTreeSet::new()
    }
}

fn txid(byte: u8) -> Txid {
    Txid::from_byte_array([byte; 32])
}

fn outpoint(byte: u8) -> OutPoint {
    OutPoint {
        txid: txid(byte),
        vout: 0,
    }
}

fn entry(id: u8, version: i32, inputs: Vec<OutPoint>, virtual_size: usize) -> MempoolEntry {
    let fee = Amount::from_sats(1_000).expect("valid fee");
    MempoolEntry::new(
        Transaction {
            version,
            inputs: inputs
                .into_iter()
                .map(|previous_output| TransactionInput {
                    previous_output,
                    script_sig: ScriptBuf::default(),
                    sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                    witness: Default::default(),
                })
                .collect(),
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid output"),
                script_pubkey: ScriptBuf::default(),
            }],
            lock_time: 0,
        },
        txid(id),
        Wtxid::from_byte_array([id.wrapping_add(100); 32]),
        fee,
        TransactionVirtualSize::new(virtual_size),
        virtual_size.saturating_mul(4),
        0,
        MempoolEntryMetadata::legacy_unknown(),
    )
}

fn candidate(
    id: u8,
    version: i32,
    inputs: Vec<OutPoint>,
    virtual_size: usize,
) -> PreparedCandidate {
    let fee = Amount::from_sats(1_000).expect("valid fee");
    PreparedCandidate::for_policy_test(
        entry(id, version, inputs, virtual_size),
        CandidateFees {
            base: fee,
            modified: fee,
        },
    )
}

#[test]
fn truc_reject_accept_and_enforce_modes_are_distinct() {
    // Arrange
    let view = FixtureView::default();
    let oversized = [candidate(1, 3, vec![], MAX_TRUC_VIRTUAL_SIZE + 1)];

    // Act
    let rejected = evaluate_truc_package(&view, &oversized, TrucPolicy::Reject, &BTreeSet::new());
    let accepted = evaluate_truc_package(&view, &oversized, TrucPolicy::Accept, &BTreeSet::new());
    let enforced = evaluate_truc_package(&view, &oversized, TrucPolicy::Enforce, &BTreeSet::new());

    // Assert
    assert!(matches!(rejected, Err(TrucPolicyError::Rejected { .. })));
    assert_eq!(accepted, Ok(None));
    assert!(matches!(enforced, Err(TrucPolicyError::TooLarge { .. })));
    assert_eq!(
        evaluate_truc_package(
            &view,
            &[candidate(2, 2, vec![], 100)],
            TrucPolicy::Reject,
            &BTreeSet::new(),
        ),
        Ok(None)
    );
}

#[test]
fn truc_child_size_and_bidirectional_inheritance_are_enforced() {
    // Arrange
    let view = FixtureView::default();
    let truc_parent = candidate(1, 3, vec![], 100);
    let oversized_child = candidate(2, 3, vec![outpoint(1)], MAX_TRUC_CHILD_VIRTUAL_SIZE + 1);
    let ordinary_child = candidate(3, 2, vec![outpoint(1)], 100);
    let ordinary_parent = candidate(4, 2, vec![], 100);
    let truc_child = candidate(5, 3, vec![outpoint(4)], 100);

    // Act / Assert
    assert!(matches!(
        evaluate_truc_package(
            &view,
            &[truc_parent.clone(), oversized_child],
            TrucPolicy::Enforce,
            &BTreeSet::new()
        ),
        Err(TrucPolicyError::ChildTooLarge { .. })
    ));
    assert!(matches!(
        evaluate_truc_package(
            &view,
            &[truc_parent, ordinary_child],
            TrucPolicy::Enforce,
            &BTreeSet::new()
        ),
        Err(TrucPolicyError::VersionInheritance { .. })
    ));
    assert!(matches!(
        evaluate_truc_package(
            &view,
            &[ordinary_parent, truc_child],
            TrucPolicy::Enforce,
            &BTreeSet::new()
        ),
        Err(TrucPolicyError::VersionInheritance { .. })
    ));
}

#[test]
fn truc_parent_and_child_and_ancestor_limit_reject() {
    // Arrange
    let view = FixtureView::default();
    let grandparent = candidate(1, 3, vec![], 100);
    let parent = candidate(2, 3, vec![outpoint(1)], 100);
    let child = candidate(3, 3, vec![outpoint(2)], 100);

    // Act
    let result = evaluate_truc_package(
        &view,
        &[grandparent, parent, child],
        TrucPolicy::Enforce,
        &BTreeSet::new(),
    );

    // Assert
    assert!(matches!(
        result,
        Err(TrucPolicyError::DescendantLimit {
            count,
            ..
        }) if count > MAX_TRUC_DESCENDANT_COUNT && MAX_TRUC_ANCESTOR_COUNT == 2
    ));
}

#[test]
fn direct_conflict_child_replacement_is_hypothetical() {
    // Arrange
    let mut view = FixtureView::default();
    let mut parent = entry(1, 3, vec![], 100);
    let mut old_child = entry(2, 3, vec![outpoint(1)], 100);
    parent.children.insert(old_child.txid);
    parent.descendant_stats.count = 2;
    old_child.parents.insert(parent.txid);
    old_child.ancestor_stats.count = 2;
    view.entries.insert(parent.txid, parent);
    view.entries.insert(old_child.txid, old_child);
    let replacement = [candidate(3, 3, vec![outpoint(1)], 100)];
    let direct_conflicts = BTreeSet::from([txid(2)]);

    // Act
    let result = evaluate_truc_package(&view, &replacement, TrucPolicy::Enforce, &direct_conflicts);

    // Assert
    assert_eq!(result, Ok(None));
}

#[test]
fn sole_leaf_sibling_yields_eviction_intent_but_descendant_is_ineligible() {
    // Arrange
    let mut view = FixtureView::default();
    let mut parent = entry(1, 3, vec![], 100);
    let mut sibling = entry(2, 3, vec![outpoint(1)], 100);
    parent.children.insert(sibling.txid);
    parent.descendant_stats.count = 2;
    sibling.parents.insert(parent.txid);
    sibling.ancestor_stats.count = 2;
    sibling.descendant_stats.count = 1;
    view.entries.insert(parent.txid, parent);
    view.entries.insert(sibling.txid, sibling.clone());
    let child = [candidate(3, 3, vec![outpoint(1)], 100)];

    // Act
    let eligible = evaluate_truc_package(&view, &child, TrucPolicy::Enforce, &BTreeSet::new())
        .expect("eligible sibling");
    view.entries
        .get_mut(&sibling.txid)
        .expect("sibling")
        .descendant_stats
        .count = 2;
    let ineligible = evaluate_truc_package(&view, &child, TrucPolicy::Enforce, &BTreeSet::new());

    // Assert
    assert_eq!(eligible.map(|intent| intent.sibling), Some(sibling.txid));
    assert!(matches!(
        ineligible,
        Err(TrucPolicyError::IneligibleSibling { .. })
    ));
}

#[test]
fn truc_errors_have_stable_nonempty_diagnostics() {
    // Arrange
    let errors = [
        TrucPolicyError::Rejected { txid: txid(1) },
        TrucPolicyError::TooLarge {
            txid: txid(2),
            virtual_size: MAX_TRUC_VIRTUAL_SIZE + 1,
        },
        TrucPolicyError::ChildTooLarge {
            txid: txid(3),
            virtual_size: MAX_TRUC_CHILD_VIRTUAL_SIZE + 1,
        },
        TrucPolicyError::AncestorLimit {
            txid: txid(4),
            count: MAX_TRUC_ANCESTOR_COUNT + 1,
        },
        TrucPolicyError::DescendantLimit {
            txid: txid(5),
            count: MAX_TRUC_DESCENDANT_COUNT + 1,
        },
        TrucPolicyError::VersionInheritance {
            parent: txid(6),
            child: txid(7),
        },
        TrucPolicyError::SiblingTopology { parent: txid(8) },
        TrucPolicyError::ParentAndChild { txid: txid(9) },
        TrucPolicyError::IneligibleSibling { sibling: txid(10) },
    ];

    // Act
    let diagnostics = errors.map(|error| error.to_string());

    // Assert
    assert!(diagnostics.iter().all(|diagnostic| !diagnostic.is_empty()));
}

#[test]
fn direct_validation_covers_non_enforced_and_ordinary_paths() {
    // Arrange
    let view = FixtureView::default();
    let ordinary = candidate(1, 2, vec![], 100);
    let truc = candidate(2, 3, vec![], 100);

    // Act / Assert
    assert!(
        validate_truc_package(
            &view,
            std::slice::from_ref(&ordinary),
            TrucPolicy::Accept,
            &BTreeSet::new(),
            None,
        )
        .is_ok()
    );
    assert!(
        validate_truc_package(
            &view,
            std::slice::from_ref(&ordinary),
            TrucPolicy::Reject,
            &BTreeSet::new(),
            None,
        )
        .is_ok()
    );
    assert!(matches!(
        validate_truc_package(&view, &[truc], TrucPolicy::Reject, &BTreeSet::new(), None,),
        Err(TrucPolicyError::Rejected { .. })
    ));
    assert!(
        validate_truc_package(
            &view,
            &[ordinary],
            TrucPolicy::Enforce,
            &BTreeSet::new(),
            None,
        )
        .is_ok()
    );
}

#[test]
fn ancestor_parent_child_and_unconsumed_sibling_paths_are_typed() {
    // Arrange
    let mut view = FixtureView::default();
    let grandparent = entry(1, 3, vec![], 100);
    let mut parent = entry(2, 3, vec![outpoint(1)], 100);
    let sibling = entry(4, 3, vec![outpoint(2)], 100);
    parent.children.insert(sibling.txid);
    view.entries.insert(grandparent.txid, grandparent);
    view.entries.insert(parent.txid, parent);
    view.entries.insert(sibling.txid, sibling);
    let child = candidate(3, 3, vec![outpoint(2)], 100);
    let mut sibling_view = FixtureView::default();
    let mut sibling_parent = entry(8, 3, vec![], 100);
    let sibling_entry = entry(9, 3, vec![outpoint(8)], 100);
    sibling_parent.children.insert(sibling_entry.txid);
    sibling_view
        .entries
        .insert(sibling_parent.txid, sibling_parent);
    sibling_view
        .entries
        .insert(sibling_entry.txid, sibling_entry);
    let sibling_child = candidate(10, 3, vec![outpoint(8)], 100);
    let chain = [
        candidate(5, 3, vec![], 100),
        candidate(6, 3, vec![outpoint(5)], 100),
        candidate(7, 3, vec![outpoint(6)], 100),
    ];

    // Act
    let ancestor = validate_truc_package(
        &view,
        std::slice::from_ref(&child),
        TrucPolicy::Enforce,
        &BTreeSet::new(),
        Some(EligibleSiblingEviction { sibling: txid(4) }),
    );
    let sibling = validate_truc_package(
        &sibling_view,
        &[sibling_child],
        TrucPolicy::Enforce,
        &BTreeSet::new(),
        None,
    );
    let parent_and_child = validate_truc_package(
        &FixtureView::default(),
        &chain,
        TrucPolicy::Enforce,
        &BTreeSet::new(),
        None,
    );

    // Assert
    assert!(matches!(
        ancestor,
        Err(TrucPolicyError::AncestorLimit { count: 3, .. })
    ));
    assert!(matches!(
        sibling,
        Err(TrucPolicyError::SiblingTopology { .. })
    ));
    assert!(matches!(
        parent_and_child,
        Err(TrucPolicyError::DescendantLimit { .. }) | Err(TrucPolicyError::ParentAndChild { .. })
    ));
}

#[test]
fn missing_and_multiple_sibling_entries_are_ineligible() {
    // Arrange
    let mut missing_view = FixtureView::default();
    let mut missing_parent = entry(1, 3, vec![], 100);
    missing_parent.children.insert(txid(2));
    missing_view
        .entries
        .insert(missing_parent.txid, missing_parent);
    let child = [candidate(3, 3, vec![outpoint(1)], 100)];

    let mut multiple_view = FixtureView::default();
    let mut parent = entry(4, 3, vec![], 100);
    let mut first = entry(5, 3, vec![outpoint(4)], 100);
    let second = entry(6, 3, vec![outpoint(4)], 100);
    first.ancestor_stats.count = 2;
    parent.children.extend([first.txid, second.txid]);
    multiple_view.entries.insert(parent.txid, parent);
    multiple_view.entries.insert(first.txid, first);
    multiple_view.entries.insert(second.txid, second);
    let multiple_child = [candidate(7, 3, vec![outpoint(4)], 100)];

    // Act
    let missing =
        evaluate_truc_package(&missing_view, &child, TrucPolicy::Enforce, &BTreeSet::new());
    let multiple = evaluate_truc_package(
        &multiple_view,
        &multiple_child,
        TrucPolicy::Enforce,
        &BTreeSet::new(),
    );

    // Assert
    assert!(matches!(
        missing,
        Err(TrucPolicyError::IneligibleSibling { sibling }) if sibling == txid(2)
    ));
    assert!(matches!(
        multiple,
        Err(TrucPolicyError::IneligibleSibling { .. })
    ));
}

#[test]
fn mempool_parent_inheritance_and_middle_member_topology_reject() {
    // Arrange
    let mut view = FixtureView::default();
    let ordinary_parent = entry(1, 2, vec![], 100);
    view.entries.insert(ordinary_parent.txid, ordinary_parent);
    let truc_child = [candidate(2, 3, vec![outpoint(1)], 100)];
    let grandparent = candidate(3, 3, vec![], 100);
    let middle = candidate(4, 3, vec![outpoint(3)], 100);
    let child = candidate(5, 3, vec![outpoint(4)], 100);

    // Act
    let inheritance =
        evaluate_truc_package(&view, &truc_child, TrucPolicy::Enforce, &BTreeSet::new());
    let middle_topology = validate_truc_package(
        &FixtureView::default(),
        &[middle, grandparent, child],
        TrucPolicy::Enforce,
        &BTreeSet::new(),
        None,
    );

    // Assert
    assert!(matches!(
        inheritance,
        Err(TrucPolicyError::VersionInheritance { .. })
    ));
    assert!(matches!(
        middle_topology,
        Err(TrucPolicyError::ParentAndChild { .. })
    ));
}

#[test]
fn candidate_sibling_skip_and_cycle_safe_counts_are_covered() {
    // Arrange
    let mut view = FixtureView::default();
    let mut parent = entry(1, 3, vec![], 100);
    parent.children.insert(txid(2));
    view.entries.insert(parent.txid, parent);
    let candidate_child = [candidate(2, 3, vec![outpoint(1)], 100)];

    let first = candidate(3, 3, vec![outpoint(4)], 100);
    let second = candidate(4, 3, vec![outpoint(3)], 100);
    let entries = BTreeMap::from([
        (first.entry.txid, &first.entry),
        (second.entry.txid, &second.entry),
    ]);

    // Act
    let skip = evaluate_truc_package(
        &view,
        &candidate_child,
        TrucPolicy::Enforce,
        &BTreeSet::new(),
    );
    let ancestor_count = prospective_ancestor_count(&view, &first.entry, &entries);
    let descendant_count = prospective_descendant_count(&first.entry, &entries);

    // Assert
    assert_eq!(skip, Ok(None));
    assert_eq!(ancestor_count, 3);
    assert_eq!(descendant_count, 3);
}
