// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/ephemeral_policy.h
// - packages/bitcoin-knots/src/policy/ephemeral_policy.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid, Wtxid,
};

use super::{EphemeralPolicyError, validate_ephemeral_spends};
use crate::policy::replacement::MempoolView;
use crate::pool::candidate::PreparedCandidate;
use crate::{
    AggregateStats, CandidateFees, DustRelayFeeRate, EphemeralPolicy, MempoolEntry,
    MempoolEntryMetadata, TransactionVirtualSize,
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

fn p2a_output(value_sats: i64) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(value_sats).expect("valid value"),
        script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0x02, 0x4e, 0x73])
            .expect("valid anchor script"),
    }
}

fn ordinary_output(value_sats: i64) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(value_sats).expect("valid value"),
        script_pubkey: ScriptBuf::from_bytes(vec![
            0xa9, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x87,
        ])
        .expect("valid P2SH script"),
    }
}

fn entry(byte: u8, inputs: Vec<OutPoint>, outputs: Vec<TransactionOutput>) -> MempoolEntry {
    let transaction = Transaction {
        version: 3,
        inputs: inputs
            .into_iter()
            .map(|previous_output| TransactionInput {
                previous_output,
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            })
            .collect(),
        outputs,
        lock_time: 0,
    };
    let virtual_size = TransactionVirtualSize::new(100);
    let mut entry = MempoolEntry::new(
        transaction,
        txid(byte),
        Wtxid::from_byte_array([byte.wrapping_add(1); 32]),
        Amount::ZERO,
        virtual_size,
        400,
        0,
        MempoolEntryMetadata::legacy_unknown(),
    );
    entry.ancestor_stats = AggregateStats::new(1, virtual_size, 0);
    entry.descendant_stats = AggregateStats::new(1, virtual_size, 0);
    entry
}

fn prepared(entry: MempoolEntry, base: i64, modified: i64) -> PreparedCandidate {
    PreparedCandidate::for_policy_test(
        entry,
        CandidateFees {
            base: Amount::from_sats(base).expect("valid base fee"),
            modified: Amount::from_sats(modified).expect("valid modified fee"),
        },
    )
}

fn allow_all() -> EphemeralPolicy {
    EphemeralPolicy {
        anchor: true,
        send: true,
        dust: true,
    }
}

#[test]
fn nonzero_base_or_nonzero_modified_fee_rejects_permitted_dust_independently() {
    // Arrange
    let dusty = entry(1, Vec::new(), vec![p2a_output(0)]);

    // Act
    let base_error = validate_ephemeral_spends(
        &FixtureView::default(),
        &[prepared(dusty.clone(), 1, 0)],
        allow_all(),
        DustRelayFeeRate::default(),
    );
    let modified_error = validate_ephemeral_spends(
        &FixtureView::default(),
        &[prepared(dusty.clone(), 0, 1)],
        allow_all(),
        DustRelayFeeRate::default(),
    );
    let accepted = validate_ephemeral_spends(
        &FixtureView::default(),
        &[prepared(dusty, 0, 0)],
        allow_all(),
        DustRelayFeeRate::default(),
    );

    // Assert
    assert!(matches!(
        base_error,
        Err(EphemeralPolicyError::DustyTransactionHasFee {
            base_fee_sats: 1,
            modified_fee_sats: 0,
            ..
        })
    ));
    assert!(matches!(
        modified_error,
        Err(EphemeralPolicyError::DustyTransactionHasFee {
            base_fee_sats: 0,
            modified_fee_sats: 1,
            ..
        })
    ));
    assert!(accepted.is_ok());
}

#[test]
fn child_must_spend_multiple_dust_outputs_from_multiple_candidate_parents() {
    // Arrange
    let first = entry(10, Vec::new(), vec![ordinary_output(0), ordinary_output(0)]);
    let second = entry(11, Vec::new(), vec![ordinary_output(0)]);
    let incomplete_child = entry(
        12,
        vec![
            OutPoint {
                txid: txid(10),
                vout: 0,
            },
            OutPoint {
                txid: txid(11),
                vout: 0,
            },
        ],
        vec![ordinary_output(1_000)],
    );
    let complete_child = entry(
        13,
        vec![
            OutPoint {
                txid: txid(10),
                vout: 0,
            },
            OutPoint {
                txid: txid(10),
                vout: 1,
            },
            OutPoint {
                txid: txid(11),
                vout: 0,
            },
        ],
        vec![ordinary_output(1_000)],
    );

    // Act
    let incomplete = validate_ephemeral_spends(
        &FixtureView::default(),
        &[
            prepared(first.clone(), 0, 0),
            prepared(second.clone(), 0, 0),
            prepared(incomplete_child, 1_000, 1_000),
        ],
        allow_all(),
        DustRelayFeeRate::default(),
    );
    let complete = validate_ephemeral_spends(
        &FixtureView::default(),
        &[
            prepared(first, 0, 0),
            prepared(second, 0, 0),
            prepared(complete_child, 1_000, 1_000),
        ],
        allow_all(),
        DustRelayFeeRate::default(),
    );

    // Assert
    assert!(matches!(
        incomplete,
        Err(EphemeralPolicyError::MissingEphemeralSpends { missing, .. })
            if missing == BTreeSet::from([OutPoint { txid: txid(10), vout: 1 }])
    ));
    assert!(complete.is_ok());
}

#[test]
fn child_must_completely_spend_an_in_mempool_dusty_parent() {
    // Arrange
    let parent = entry(20, Vec::new(), vec![p2a_output(0), p2a_output(1)]);
    let mut view = FixtureView::default();
    view.entries.insert(parent.txid, parent);
    let partial = prepared(
        entry(
            21,
            vec![OutPoint {
                txid: txid(20),
                vout: 0,
            }],
            vec![ordinary_output(1_000)],
        ),
        1_000,
        1_000,
    );
    let complete = prepared(
        entry(
            22,
            vec![
                OutPoint {
                    txid: txid(20),
                    vout: 0,
                },
                OutPoint {
                    txid: txid(20),
                    vout: 1,
                },
            ],
            vec![ordinary_output(1_000)],
        ),
        1_000,
        1_000,
    );

    // Act / Assert
    assert!(matches!(
        validate_ephemeral_spends(&view, &[partial], allow_all(), DustRelayFeeRate::default()),
        Err(EphemeralPolicyError::MissingEphemeralSpends { .. })
    ));
    assert!(
        validate_ephemeral_spends(&view, &[complete], allow_all(), DustRelayFeeRate::default())
            .is_ok()
    );
}

#[test]
fn permission_matrix_selects_only_already_permitted_ephemeral_dust() {
    // Arrange
    let candidates = [
        (
            entry(30, Vec::new(), vec![p2a_output(0)]),
            EphemeralPolicy {
                anchor: false,
                send: true,
                dust: true,
            },
        ),
        (
            entry(31, Vec::new(), vec![ordinary_output(0)]),
            EphemeralPolicy {
                anchor: true,
                send: false,
                dust: true,
            },
        ),
        (
            entry(32, Vec::new(), vec![ordinary_output(1)]),
            EphemeralPolicy {
                anchor: true,
                send: true,
                dust: false,
            },
        ),
    ];

    // Act / Assert
    for (candidate, permissions) in candidates {
        assert!(
            validate_ephemeral_spends(
                &FixtureView::default(),
                &[prepared(candidate, 1, 1)],
                permissions,
                DustRelayFeeRate::default(),
            )
            .is_ok()
        );
    }
    assert!(matches!(
        validate_ephemeral_spends(
            &FixtureView::default(),
            &[prepared(
                entry(33, Vec::new(), vec![ordinary_output(1)]),
                1,
                1,
            )],
            allow_all(),
            DustRelayFeeRate::default(),
        ),
        Err(EphemeralPolicyError::DustyTransactionHasFee { .. })
    ));
}

#[test]
fn errors_have_stable_nonempty_diagnostics() {
    // Arrange
    let errors = [
        EphemeralPolicyError::DustyTransactionHasFee {
            txid: txid(40),
            base_fee_sats: 1,
            modified_fee_sats: 2,
        },
        EphemeralPolicyError::MissingEphemeralSpends {
            child: txid(41),
            missing: BTreeSet::from([OutPoint {
                txid: txid(42),
                vout: 0,
            }]),
        },
    ];

    // Act
    let diagnostics = errors.map(|error| error.to_string());

    // Assert
    assert!(diagnostics.iter().all(|diagnostic| !diagnostic.is_empty()));
}
