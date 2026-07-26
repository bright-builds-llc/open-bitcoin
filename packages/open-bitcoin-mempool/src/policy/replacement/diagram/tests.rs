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

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid, Wtxid,
};

use super::{
    FeeChunk, Fraction, add_breakpoints, aggregate_entries, combine_chunks, compare_fraction,
    diagram_strictly_improves, enforce_diagram_improvement, fee_at_virtual_size, old_fee_chunks,
    rate_cmp, sort_chunks, total_chunk_size,
};
use crate::policy::replacement::{MempoolView, PackageReplacementError};
use crate::pool::candidate::PreparedCandidate;
use crate::{CandidateFees, MempoolEntry, MempoolEntryMetadata, TransactionVirtualSize};

#[derive(Default)]
struct DiagramView {
    entries: BTreeMap<Txid, MempoolEntry>,
}

impl MempoolView for DiagramView {
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

fn transaction(previous: Txid) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous,
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid output"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 0,
    }
}

fn entry(id: u8, fee_sats: i64, virtual_size: usize) -> MempoolEntry {
    MempoolEntry::new(
        transaction(txid(id.saturating_add(50))),
        txid(id),
        Wtxid::from_byte_array([id.saturating_add(100); 32]),
        Amount::from_sats(fee_sats).expect("valid fee"),
        TransactionVirtualSize::new(virtual_size),
        virtual_size.saturating_mul(4),
        0,
        MempoolEntryMetadata::legacy_unknown(),
    )
}

fn candidate(id: u8, fee_sats: i64, virtual_size: usize) -> PreparedCandidate {
    let fee = Amount::from_sats(fee_sats).expect("valid candidate fee");
    PreparedCandidate::for_policy_test(
        entry(id, fee_sats, virtual_size),
        CandidateFees {
            base: fee,
            modified: fee,
        },
    )
}

#[test]
fn old_diagram_chunks_cover_both_parent_child_orderings() {
    // Arrange
    let parent_id = txid(20);
    let child_id = txid(21);
    let removed = BTreeSet::from([parent_id, child_id]);
    let mut high_child = DiagramView::default();
    let mut parent = entry(20, 100, 100);
    let mut child = entry(21, 500, 100);
    parent.children.insert(child_id);
    child.parents.insert(parent_id);
    high_child.entries.insert(parent_id, parent.clone());
    high_child.entries.insert(child_id, child.clone());
    let mut low_child = DiagramView::default();
    parent.fee = Amount::from_sats(500).expect("valid fee");
    child.fee = Amount::from_sats(100).expect("valid fee");
    low_child.entries.insert(parent_id, parent);
    low_child.entries.insert(child_id, child);

    // Act
    let high_child_chunks = old_fee_chunks(&high_child, &removed).expect("high child chunks");
    let low_child_chunks = old_fee_chunks(&low_child, &removed).expect("low child chunks");

    // Assert
    assert_eq!(high_child_chunks.len(), 1);
    assert_eq!(low_child_chunks.len(), 2);
}

#[test]
fn retained_parent_paths_cover_present_and_missing_entries() {
    // Arrange
    let conflict_id = txid(20);
    let retained_id = txid(30);
    let mut view = DiagramView::default();
    let mut conflict = entry(20, 100, 100);
    conflict.parents.insert(retained_id);
    view.entries.insert(conflict_id, conflict);
    view.entries.insert(retained_id, entry(30, 100, 100));
    let package = vec![candidate(10, 300, 100), candidate(11, 500, 100)];
    let direct = BTreeSet::from([conflict_id]);
    let removed = BTreeSet::from([conflict_id]);

    // Act
    let present = enforce_diagram_improvement(&view, &package, &direct, &removed);
    view.entries.remove(&retained_id);
    let missing = enforce_diagram_improvement(&view, &package, &direct, &removed);

    // Assert
    assert!(present.is_ok());
    assert_eq!(
        missing,
        Err(PackageReplacementError::RemovalEntryMissing { txid: retained_id })
    );
}

#[test]
fn ancestor_and_aggregate_lookup_failures_are_typed() {
    // Arrange
    let parent_id = txid(20);
    let child_id = txid(21);
    let mut view = DiagramView::default();
    let mut child = entry(21, 100, 100);
    child.parents.insert(parent_id);
    view.entries.insert(child_id, child);
    let removed = BTreeSet::from([parent_id, child_id]);

    // Act
    let old_result = old_fee_chunks(&view, &removed);
    let aggregate_result = aggregate_entries(&view, &BTreeSet::from([txid(99)]));

    // Assert
    assert_eq!(
        old_result,
        Err(PackageReplacementError::RemovalEntryMissing { txid: parent_id })
    );
    assert_eq!(
        aggregate_result,
        Err(PackageReplacementError::RemovalEntryMissing { txid: txid(99) })
    );
}

#[test]
fn chunk_arithmetic_sorting_and_size_overflow_are_checked() {
    // Arrange
    let fee_overflow_left = FeeChunk {
        fee_sats: i64::MAX,
        virtual_size: 1,
    };
    let fee_overflow_right = FeeChunk {
        fee_sats: 1,
        virtual_size: 1,
    };
    let size_overflow_left = FeeChunk {
        fee_sats: 1,
        virtual_size: usize::MAX,
    };
    let mut chunks = vec![
        FeeChunk {
            fee_sats: 2,
            virtual_size: 2,
        },
        FeeChunk {
            fee_sats: 1,
            virtual_size: 1,
        },
    ];

    // Act
    let fee_overflow = combine_chunks(fee_overflow_left, fee_overflow_right);
    let size_overflow = combine_chunks(size_overflow_left, fee_overflow_right);
    let total_overflow = total_chunk_size(&[size_overflow_left, fee_overflow_right]);
    sort_chunks(&mut chunks);

    // Assert
    assert_eq!(fee_overflow, Err(PackageReplacementError::FeeOverflow));
    assert_eq!(
        size_overflow,
        Err(PackageReplacementError::VirtualSizeOverflow)
    );
    assert_eq!(
        total_overflow,
        Err(PackageReplacementError::VirtualSizeOverflow)
    );
    assert_eq!(chunks[0].virtual_size, 1);
    assert_eq!(rate_cmp(2, 1, 1, 1), std::cmp::Ordering::Greater);
}

#[test]
fn diagram_partial_order_covers_short_equal_better_and_worse_curves() {
    // Arrange
    let old = [FeeChunk {
        fee_sats: 100,
        virtual_size: 100,
    }];
    let shorter = [FeeChunk {
        fee_sats: 200,
        virtual_size: 99,
    }];
    let equal = old;
    let better = [FeeChunk {
        fee_sats: 101,
        virtual_size: 100,
    }];
    let worse = [FeeChunk {
        fee_sats: 99,
        virtual_size: 100,
    }];
    let longer = [
        old[0],
        FeeChunk {
            fee_sats: 1,
            virtual_size: 1,
        },
    ];

    // Act
    let results = [
        diagram_strictly_improves(&old, &shorter),
        diagram_strictly_improves(&old, &equal),
        diagram_strictly_improves(&old, &better),
        diagram_strictly_improves(&old, &worse),
        diagram_strictly_improves(&old, &longer),
    ];

    // Assert
    assert_eq!(
        results,
        [Ok(false), Ok(false), Ok(true), Ok(false), Ok(true)]
    );
}

#[test]
fn breakpoint_and_fraction_helpers_cover_partial_terminal_and_overflow_paths() {
    // Arrange
    let chunks = [
        FeeChunk {
            fee_sats: 10,
            virtual_size: 10,
        },
        FeeChunk {
            fee_sats: 20,
            virtual_size: 10,
        },
    ];
    let mut breakpoints = BTreeSet::new();
    let overflow_chunks = [
        FeeChunk {
            fee_sats: 1,
            virtual_size: usize::MAX,
        },
        FeeChunk {
            fee_sats: 1,
            virtual_size: 1,
        },
    ];

    // Act
    let added = add_breakpoints(&mut breakpoints, &chunks, 15);
    let partial = fee_at_virtual_size(&chunks, 15);
    let terminal = fee_at_virtual_size(&chunks, 25);
    let breakpoint_overflow = add_breakpoints(&mut BTreeSet::new(), &overflow_chunks, usize::MAX);
    let fraction_overflow = compare_fraction(
        Fraction {
            numerator: i128::MAX,
            denominator: 2,
        },
        Fraction {
            numerator: 1,
            denominator: 2,
        },
    );

    // Assert
    assert!(added.is_ok());
    assert_eq!(breakpoints, BTreeSet::from([10, 15]));
    assert_eq!(partial.expect("partial fraction").denominator, 10);
    assert_eq!(terminal.expect("terminal fraction").denominator, 1);
    assert_eq!(
        breakpoint_overflow,
        Err(PackageReplacementError::VirtualSizeOverflow)
    );
    assert_eq!(fraction_overflow, Err(PackageReplacementError::FeeOverflow));
}

#[test]
fn fee_curve_reports_virtual_size_and_numerator_overflow() {
    // Arrange
    let size_overflow = [
        FeeChunk {
            fee_sats: 1,
            virtual_size: usize::MAX,
        },
        FeeChunk {
            fee_sats: 1,
            virtual_size: 1,
        },
    ];
    let numerator_overflow = [
        FeeChunk {
            fee_sats: i64::MAX,
            virtual_size: 1,
        },
        FeeChunk {
            fee_sats: i64::MAX,
            virtual_size: usize::MAX - 1,
        },
    ];

    // Act
    let size_result = fee_at_virtual_size(&size_overflow, usize::MAX);
    let numerator_result = fee_at_virtual_size(&numerator_overflow, usize::MAX);

    // Assert
    assert_eq!(
        size_result,
        Ok(Fraction {
            numerator: usize::MAX as i128,
            denominator: usize::MAX as i128,
        })
    );
    assert_eq!(numerator_result, Err(PackageReplacementError::FeeOverflow));
}
