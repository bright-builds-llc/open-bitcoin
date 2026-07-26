// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/rbf.h
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/feefrac_tests.cpp
// - packages/bitcoin-knots/src/test/rbf_tests.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/util/feefrac.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_package_rbf.py

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid, Wtxid,
};

use super::{
    MAX_REPLACEMENT_CANDIDATES, MempoolView, PackageReplacementError, checked_fee_for_virtual_size,
    enforce_conservative_candidate_bound, enforce_replacement_fees,
    evaluate_limited_package_replacement, evaluate_limited_package_replacement_with_intent,
    package_virtual_size, removal_facts,
};
use crate::pool::candidate::PreparedCandidate;
use crate::{
    CandidateFees, FeeRate, IncrementalRelayFeeRate, MempoolEntry, MempoolEntryMetadata,
    MempoolRemovalRole, TransactionVirtualSize,
};

#[derive(Default)]
struct FixtureView {
    entries: BTreeMap<Txid, MempoolEntry>,
    spenders: BTreeMap<OutPoint, Txid>,
    descendant_calls: Cell<usize>,
}

impl MempoolView for FixtureView {
    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.entries.get(txid)
    }

    fn maybe_spender(&self, outpoint: &OutPoint) -> Option<Txid> {
        self.spenders.get(outpoint).copied()
    }

    fn collect_descendants(&self, txid: Txid) -> BTreeSet<Txid> {
        self.descendant_calls
            .set(self.descendant_calls.get().saturating_add(1));
        let mut descendants = BTreeSet::new();
        let mut pending = self
            .entries
            .get(&txid)
            .map(|entry| entry.children.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        while let Some(descendant) = pending.pop() {
            if !descendants.insert(descendant) {
                continue;
            }
            if let Some(entry) = self.entries.get(&descendant) {
                pending.extend(entry.children.iter().copied());
            }
        }
        descendants
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

fn transaction(inputs: Vec<OutPoint>) -> Transaction {
    Transaction {
        version: 2,
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
            value: Amount::from_sats(1).expect("valid output amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 0,
    }
}

fn entry(
    id: u8,
    inputs: Vec<OutPoint>,
    fee_sats: i64,
    virtual_size: usize,
    descendant_count: usize,
) -> MempoolEntry {
    let fee = Amount::from_sats(fee_sats).expect("valid fee");
    let mut entry = MempoolEntry::new(
        transaction(inputs),
        txid(id),
        Wtxid::from_byte_array([id.saturating_add(100); 32]),
        fee,
        TransactionVirtualSize::new(virtual_size),
        virtual_size.saturating_mul(4),
        0,
        MempoolEntryMetadata::legacy_unknown(),
    );
    entry.descendant_stats.count = descendant_count;
    entry
}

fn candidate(id: u8, inputs: Vec<OutPoint>, fee_sats: i64) -> PreparedCandidate {
    let entry = entry(id, inputs, fee_sats, 100, 1);
    let fee = Amount::from_sats(fee_sats).expect("valid candidate fee");
    PreparedCandidate::for_policy_test(
        entry,
        CandidateFees {
            base: fee,
            modified: fee,
        },
    )
}

fn fixture(
    original_fee_sats: i64,
    descendant_count: usize,
    parent_fee_sats: i64,
    child_fee_sats: i64,
) -> (FixtureView, Vec<PreparedCandidate>) {
    let spent = outpoint(1);
    let conflict = entry(
        20,
        vec![spent.clone()],
        original_fee_sats,
        100,
        descendant_count,
    );
    let mut view = FixtureView::default();
    view.spenders.insert(spent.clone(), conflict.txid);
    view.entries.insert(conflict.txid, conflict);
    let parent = candidate(10, vec![spent], parent_fee_sats);
    let child = candidate(
        11,
        vec![OutPoint {
            txid: parent.entry.txid,
            vout: 0,
        }],
        child_fee_sats,
    );
    (view, vec![parent, child])
}

fn incremental_relay_fee() -> IncrementalRelayFeeRate {
    IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000))
}

#[test]
fn valid_bounded_parent_child_replacement_returns_typed_direct_removal() {
    // Arrange
    let (view, package) = fixture(100, 1, 100, 300);

    // Act
    let replacement =
        evaluate_limited_package_replacement(&view, &package, incremental_relay_fee())
            .expect("replacement should pass");

    // Assert
    assert_eq!(replacement.removals.len(), 1);
    assert_eq!(
        replacement.removals.values().copied().collect::<Vec<_>>(),
        vec![MempoolRemovalRole::Direct]
    );
    assert_eq!(view.descendant_calls.get(), 1);
}

#[test]
fn eligible_truc_sibling_intent_uses_singleton_replacement_rules() {
    // Arrange
    let spent = outpoint(1);
    let conflict = entry(20, vec![spent.clone()], 100, 100, 1);
    let conflict_txid = conflict.txid;
    let mut view = FixtureView::default();
    view.entries.insert(conflict_txid, conflict);
    let package = [candidate(10, vec![spent], 400)];

    // Act
    let replacement = evaluate_limited_package_replacement_with_intent(
        &view,
        &package,
        incremental_relay_fee(),
        &BTreeSet::from([conflict_txid]),
    )
    .expect("eligible sibling replacement");

    // Assert
    assert_eq!(replacement.removals.len(), 1);
    assert_eq!(
        replacement.removals.values().next(),
        Some(&MempoolRemovalRole::Direct)
    );
}

#[test]
fn descendant_limit_fails_before_descendant_union_allocation() {
    // Arrange
    let (view, package) = fixture(100, MAX_REPLACEMENT_CANDIDATES + 1, 200, 300);

    // Act
    let result = evaluate_limited_package_replacement(&view, &package, incremental_relay_fee());

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::TooManyPotentialReplacements {
            count: MAX_REPLACEMENT_CANDIDATES + 1,
            limit: MAX_REPLACEMENT_CANDIDATES,
        })
    );
    assert_eq!(view.descendant_calls.get(), 0);
}

#[test]
fn overlapping_descendants_are_counted_conservatively_before_union() {
    // Arrange
    let first_spent = outpoint(1);
    let second_spent = outpoint(2);
    let mut view = FixtureView::default();
    for (id, spent) in [(20, first_spent.clone()), (21, second_spent.clone())] {
        let conflict = entry(id, vec![spent.clone()], 100, 100, 51);
        view.spenders.insert(spent, conflict.txid);
        view.entries.insert(conflict.txid, conflict);
    }
    let parent = candidate(10, vec![first_spent, second_spent], 300);
    let child = candidate(11, vec![outpoint(10)], 400);
    let package = vec![parent, child];

    // Act
    let result = evaluate_limited_package_replacement(&view, &package, incremental_relay_fee());

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::TooManyPotentialReplacements {
            count: 102,
            limit: MAX_REPLACEMENT_CANDIDATES,
        })
    );
    assert_eq!(view.descendant_calls.get(), 0);
}

#[test]
fn replacement_fee_must_cover_originals_plus_incremental_relay_fee() {
    // Arrange
    let (view, package) = fixture(100, 1, 100, 100);

    // Act
    let result = evaluate_limited_package_replacement(&view, &package, incremental_relay_fee());

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::InsufficientReplacementFee {
            replacement_fee_sats: 200,
            required_fee_sats: 300,
        })
    );
}

#[test]
fn replacement_package_feerate_must_be_strictly_above_parent() {
    // Arrange
    let (view, package) = fixture(100, 1, 300, 100);

    // Act
    let result = evaluate_limited_package_replacement(&view, &package, incremental_relay_fee());

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::PackageFeeRateNotAboveParent)
    );
}

#[test]
fn replacement_feerate_diagram_must_strictly_improve() {
    // Arrange
    let (view, package) = fixture(1_000, 1, 500, 700);

    // Act
    let result = evaluate_limited_package_replacement(&view, &package, incremental_relay_fee());

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::FeeRateDiagramNotImproved)
    );
}

#[test]
fn package_shape_conflict_and_ancestor_gates_return_typed_errors() {
    // Arrange
    let (mut view, package) = fixture(100, 1, 100, 300);
    let wrong_topology = vec![package[0].clone(), candidate(12, vec![outpoint(9)], 300)];
    let no_conflict_view = FixtureView::default();
    let empty_package = Vec::new();

    // Act
    let wrong_size =
        evaluate_limited_package_replacement(&view, &empty_package, incremental_relay_fee());
    let wrong_topology_result =
        evaluate_limited_package_replacement(&view, &wrong_topology, incremental_relay_fee());
    let no_conflicts =
        evaluate_limited_package_replacement(&no_conflict_view, &package, incremental_relay_fee());
    let ancestor = entry(1, vec![outpoint(99)], 10, 100, 1);
    view.entries.insert(ancestor.txid, ancestor);
    let in_mempool_ancestor =
        evaluate_limited_package_replacement(&view, &package, incremental_relay_fee());

    // Assert
    assert_eq!(
        wrong_size,
        Err(PackageReplacementError::WrongPackageSize { actual: 0 })
    );
    assert_eq!(
        wrong_topology_result,
        Err(PackageReplacementError::WrongTopology)
    );
    assert_eq!(
        no_conflicts,
        Err(PackageReplacementError::NoDirectConflicts)
    );
    assert_eq!(
        in_mempool_ancestor,
        Err(PackageReplacementError::InMempoolAncestor {
            candidate: txid(10),
            ancestor: txid(1),
        })
    );
}

#[test]
fn conservative_guard_reports_missing_zero_and_overflowing_counts() {
    // Arrange
    let missing = BTreeSet::from([txid(20)]);
    let mut zero_view = FixtureView::default();
    zero_view
        .entries
        .insert(txid(20), entry(20, vec![outpoint(1)], 100, 100, 0));
    let mut overflow_view = FixtureView::default();
    overflow_view.entries.insert(
        txid(20),
        entry(20, vec![outpoint(1)], 100, 100, MAX_REPLACEMENT_CANDIDATES),
    );
    overflow_view
        .entries
        .insert(txid(21), entry(21, vec![outpoint(2)], 100, 100, usize::MAX));
    let overflow = BTreeSet::from([txid(20), txid(21)]);

    // Act
    let missing_result = enforce_conservative_candidate_bound(&FixtureView::default(), &missing);
    let zero_result = enforce_conservative_candidate_bound(&zero_view, &missing);
    let overflow_result = enforce_conservative_candidate_bound(&overflow_view, &overflow);

    // Assert
    assert_eq!(
        missing_result,
        Err(PackageReplacementError::MissingConflict { txid: txid(20) })
    );
    assert_eq!(
        zero_result,
        Err(PackageReplacementError::InvalidDescendantCount {
            txid: txid(20),
            count: 0,
        })
    );
    assert_eq!(
        overflow_result,
        Err(PackageReplacementError::PotentialCountOverflow)
    );
}

#[test]
fn descendant_union_is_deduplicated_and_preserves_removal_roles() {
    // Arrange
    let (mut view, package) = fixture(100, 2, 100, 400);
    let conflict_txid = txid(20);
    let descendant = entry(21, vec![outpoint(20)], 50, 100, 1);
    view.entries
        .get_mut(&conflict_txid)
        .expect("conflict fixture")
        .children
        .insert(descendant.txid);
    view.entries.insert(descendant.txid, descendant);

    // Act
    let replacement =
        evaluate_limited_package_replacement(&view, &package, incremental_relay_fee())
            .expect("bounded descendant replacement");

    // Assert
    assert_eq!(replacement.removals.len(), 2);
    assert_eq!(
        replacement
            .removals
            .iter()
            .find(|(member, _role)| member.txid == txid(21))
            .map(|(_member, role)| *role),
        Some(MempoolRemovalRole::Descendant)
    );
}

#[test]
fn helper_arithmetic_and_lookup_failures_remain_typed() {
    // Arrange
    let (view, package) = fixture(100, 1, 100, 300);
    let missing = BTreeSet::from([txid(99)]);
    let direct = BTreeSet::new();
    let overflowing_package = vec![
        candidate(30, vec![outpoint(30)], 1),
        candidate(31, vec![outpoint(30)], 1),
    ];
    let mut overflowing_package = overflowing_package;
    overflowing_package[0].entry.virtual_size = TransactionVirtualSize::new(usize::MAX);
    overflowing_package[1].entry.virtual_size = TransactionVirtualSize::new(1);

    // Act
    let missing_role = removal_facts(&view, &direct, &missing);
    let missing_fee = enforce_replacement_fees(&view, &package, &missing, incremental_relay_fee());
    let invalid_incremental = enforce_replacement_fees(
        &view,
        &package,
        &BTreeSet::from([txid(20)]),
        IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(-1)),
    );
    let fee_overflow =
        checked_fee_for_virtual_size(i64::MAX, TransactionVirtualSize::new(usize::MAX));
    let vsize_overflow = package_virtual_size(&overflowing_package);

    // Assert
    assert_eq!(
        missing_role,
        Err(PackageReplacementError::RemovalEntryMissing { txid: txid(99) })
    );
    assert_eq!(
        missing_fee,
        Err(PackageReplacementError::RemovalEntryMissing { txid: txid(99) })
    );
    assert_eq!(
        invalid_incremental,
        Err(PackageReplacementError::InvalidIncrementalRelayFee)
    );
    assert_eq!(fee_overflow, Err(PackageReplacementError::FeeOverflow));
    assert_eq!(
        vsize_overflow,
        Err(PackageReplacementError::VirtualSizeOverflow)
    );
}

#[test]
fn replacement_errors_have_stable_nonempty_diagnostics() {
    // Arrange
    let errors = [
        PackageReplacementError::WrongPackageSize { actual: 3 },
        PackageReplacementError::WrongTopology,
        PackageReplacementError::InMempoolAncestor {
            candidate: txid(1),
            ancestor: txid(2),
        },
        PackageReplacementError::NoDirectConflicts,
        PackageReplacementError::MissingConflict { txid: txid(3) },
        PackageReplacementError::InvalidDescendantCount {
            txid: txid(4),
            count: 0,
        },
        PackageReplacementError::PotentialCountOverflow,
        PackageReplacementError::TooManyPotentialReplacements {
            count: 101,
            limit: 100,
        },
        PackageReplacementError::RemovalEntryMissing { txid: txid(5) },
        PackageReplacementError::FeeOverflow,
        PackageReplacementError::VirtualSizeOverflow,
        PackageReplacementError::InvalidIncrementalRelayFee,
        PackageReplacementError::InsufficientReplacementFee {
            replacement_fee_sats: 1,
            required_fee_sats: 2,
        },
        PackageReplacementError::PackageFeeRateNotAboveParent,
        PackageReplacementError::FeeRateDiagramNotImproved,
    ];

    // Act
    let messages = errors.map(|error| error.to_string());

    // Assert
    assert!(messages.iter().all(|message| !message.is_empty()));
}
