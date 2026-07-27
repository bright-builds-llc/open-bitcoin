// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

use super::*;

#[test]
fn outcome_labels_are_fixed_low_cardinality_values() {
    // Arrange
    let outcome_labels = [
        (MempoolOutcomeLabel::Accepted, "accepted"),
        (MempoolOutcomeLabel::Rejected, "rejected"),
        (MempoolOutcomeLabel::Duplicate, "duplicate"),
        (MempoolOutcomeLabel::Replaced, "replaced"),
        (MempoolOutcomeLabel::Orphaned, "orphaned"),
        (MempoolOutcomeLabel::Evicted, "evicted"),
        (MempoolOutcomeLabel::Expired, "expired"),
    ];
    let rejection_categories = [
        (MempoolRejectionCategory::Validation, "validation"),
        (MempoolRejectionCategory::NonStandard, "non_standard"),
        (
            MempoolRejectionCategory::RelayFeeTooLow,
            "relay_fee_too_low",
        ),
        (
            MempoolRejectionCategory::ConflictNotAllowed,
            "conflict_not_allowed",
        ),
        (
            MempoolRejectionCategory::ReplacementRejected,
            "replacement_rejected",
        ),
        (MempoolRejectionCategory::LimitExceeded, "limit_exceeded"),
        (
            MempoolRejectionCategory::InternalInvariant,
            "internal_invariant",
        ),
    ];

    // Act
    let outcome_values = outcome_labels
        .iter()
        .map(|(label, _expected)| label.as_str())
        .collect::<Vec<_>>();
    let category_values = rejection_categories
        .iter()
        .map(|(category, _expected)| category.as_str())
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        outcome_values,
        outcome_labels
            .iter()
            .map(|(_label, expected)| *expected)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        category_values,
        rejection_categories
            .iter()
            .map(|(_category, expected)| *expected)
            .collect::<Vec<_>>()
    );
}

#[test]
fn rejection_category_from_error_classifies_admission_errors() {
    // Arrange
    let txid = Txid::from_byte_array([1_u8; 32]);
    let outpoint = OutPoint { txid, vout: 0 };
    let errors = vec![
        (MempoolError::DuplicateTransaction { txid }, None),
        (
            MempoolError::MissingInput {
                outpoint: outpoint.clone(),
            },
            None,
        ),
        (MempoolError::CandidateEvicted { txid }, None),
        (
            MempoolError::Validation {
                reason: "bad-tx".to_string(),
            },
            Some(MempoolRejectionCategory::Validation),
        ),
        (
            MempoolError::NonStandard {
                reason: "scriptpubkey".to_string(),
            },
            Some(MempoolRejectionCategory::NonStandard),
        ),
        (
            MempoolError::RelayFeeTooLow {
                fee: amount(1),
                required_fee_sats: 2,
                virtual_size: 100,
            },
            Some(MempoolRejectionCategory::RelayFeeTooLow),
        ),
        (
            MempoolError::ConflictNotAllowed {
                conflicting: vec![txid],
                policy: RbfPolicy::Never,
            },
            Some(MempoolRejectionCategory::ConflictNotAllowed),
        ),
        (
            MempoolError::ReplacementRejected {
                reason: "fee bump".to_string(),
            },
            Some(MempoolRejectionCategory::ReplacementRejected),
        ),
        (
            MempoolError::LimitExceeded {
                direction: LimitDirection::Ancestor,
                kind: LimitKind::Count,
                txid: Some(txid),
                attempted: 2,
                max: 1,
            },
            Some(MempoolRejectionCategory::LimitExceeded),
        ),
        (
            MempoolError::InternalInvariant {
                reason: "missing candidate".to_string(),
            },
            Some(MempoolRejectionCategory::InternalInvariant),
        ),
        (
            MempoolError::StalePreparedTransition {
                expected_revision: 1,
                actual_revision: 2,
            },
            Some(MempoolRejectionCategory::InternalInvariant),
        ),
        (
            MempoolError::RevisionExhausted,
            Some(MempoolRejectionCategory::InternalInvariant),
        ),
    ];

    // Act
    let categories = errors
        .iter()
        .map(|(error, _expected)| MempoolRejectionCategory::from_error(error))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        categories,
        errors
            .iter()
            .map(|(_error, expected)| *expected)
            .collect::<Vec<_>>()
    );
}

#[test]
fn outcome_accessors_cover_every_variant_contract() {
    // Arrange
    let accepted_txid = Txid::from_byte_array([1_u8; 32]);
    let rejected_txid = Txid::from_byte_array([2_u8; 32]);
    let duplicate_txid = Txid::from_byte_array([3_u8; 32]);
    let replaced_txid = Txid::from_byte_array([4_u8; 32]);
    let orphaned_txid = Txid::from_byte_array([5_u8; 32]);
    let evicted_txid = Txid::from_byte_array([6_u8; 32]);
    let expired_txid = Txid::from_byte_array([7_u8; 32]);
    let evicted_package_txid = Txid::from_byte_array([8_u8; 32]);
    let replaced_conflict_txid = Txid::from_byte_array([9_u8; 32]);
    let missing_parent_txid = Txid::from_byte_array([10_u8; 32]);
    let wtxid = Wtxid::from_byte_array([42_u8; 32]);
    let accepted = MempoolOutcome::Accepted {
        txid: accepted_txid,
        wtxid,
        evicted: vec![evicted_package_txid],
    };
    let rejected = MempoolOutcome::Rejected {
        txid: rejected_txid,
        wtxid,
        category: MempoolRejectionCategory::Validation,
    };
    let duplicate = MempoolOutcome::Duplicate {
        txid: duplicate_txid,
    };
    let replaced = MempoolOutcome::Replaced {
        txid: replaced_txid,
        wtxid,
        replaced: vec![replaced_conflict_txid],
        evicted: vec![evicted_package_txid],
    };
    let orphaned = MempoolOutcome::Orphaned {
        txid: orphaned_txid,
        wtxid,
        missing_parents: vec![missing_parent_txid],
    };
    let evicted = MempoolOutcome::Evicted {
        txid: evicted_txid,
        wtxid,
    };
    let expired = MempoolOutcome::Expired {
        txid: expired_txid,
        wtxid,
    };

    // Act
    let observed = [
        (
            accepted.label(),
            accepted.txid(),
            accepted.maybe_wtxid(),
            accepted.missing_parents().to_vec(),
            accepted.replaced().to_vec(),
            accepted.evicted().to_vec(),
            accepted.maybe_rejection_category(),
        ),
        (
            rejected.label(),
            rejected.txid(),
            rejected.maybe_wtxid(),
            rejected.missing_parents().to_vec(),
            rejected.replaced().to_vec(),
            rejected.evicted().to_vec(),
            rejected.maybe_rejection_category(),
        ),
        (
            duplicate.label(),
            duplicate.txid(),
            duplicate.maybe_wtxid(),
            duplicate.missing_parents().to_vec(),
            duplicate.replaced().to_vec(),
            duplicate.evicted().to_vec(),
            duplicate.maybe_rejection_category(),
        ),
        (
            replaced.label(),
            replaced.txid(),
            replaced.maybe_wtxid(),
            replaced.missing_parents().to_vec(),
            replaced.replaced().to_vec(),
            replaced.evicted().to_vec(),
            replaced.maybe_rejection_category(),
        ),
        (
            orphaned.label(),
            orphaned.txid(),
            orphaned.maybe_wtxid(),
            orphaned.missing_parents().to_vec(),
            orphaned.replaced().to_vec(),
            orphaned.evicted().to_vec(),
            orphaned.maybe_rejection_category(),
        ),
        (
            evicted.label(),
            evicted.txid(),
            evicted.maybe_wtxid(),
            evicted.missing_parents().to_vec(),
            evicted.replaced().to_vec(),
            evicted.evicted().to_vec(),
            evicted.maybe_rejection_category(),
        ),
        (
            expired.label(),
            expired.txid(),
            expired.maybe_wtxid(),
            expired.missing_parents().to_vec(),
            expired.replaced().to_vec(),
            expired.evicted().to_vec(),
            expired.maybe_rejection_category(),
        ),
    ];

    // Assert
    assert_eq!(
        observed,
        [
            (
                MempoolOutcomeLabel::Accepted,
                accepted_txid,
                Some(wtxid),
                vec![],
                vec![],
                vec![evicted_package_txid],
                None,
            ),
            (
                MempoolOutcomeLabel::Rejected,
                rejected_txid,
                Some(wtxid),
                vec![],
                vec![],
                vec![],
                Some(MempoolRejectionCategory::Validation),
            ),
            (
                MempoolOutcomeLabel::Duplicate,
                duplicate_txid,
                None,
                vec![],
                vec![],
                vec![],
                None,
            ),
            (
                MempoolOutcomeLabel::Replaced,
                replaced_txid,
                Some(wtxid),
                vec![],
                vec![replaced_conflict_txid],
                vec![evicted_package_txid],
                None,
            ),
            (
                MempoolOutcomeLabel::Orphaned,
                orphaned_txid,
                Some(wtxid),
                vec![missing_parent_txid],
                vec![],
                vec![],
                None,
            ),
            (
                MempoolOutcomeLabel::Evicted,
                evicted_txid,
                Some(wtxid),
                vec![],
                vec![],
                vec![],
                None,
            ),
            (
                MempoolOutcomeLabel::Expired,
                expired_txid,
                Some(wtxid),
                vec![],
                vec![],
                vec![],
                None,
            ),
        ]
    );
}

#[test]
fn accepted_duplicate_orphan_replaced_evicted_and_rejected_outcomes_are_typed() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(4);
    let mut mempool = Mempool::default();
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let duplicate = original.clone();
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let orphan = spend_transaction(
        Txid::from_byte_array([8_u8; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let rejected = {
        let mut transaction = spend_transaction(
            coinbase_txids[1],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        transaction.outputs[0].script_pubkey = script(&[0x51]);
        transaction
    };
    let mut evicting_mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    });

    // Act
    let accepted = mempool
        .accept_transaction_outcome_with_context(
            original,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("accepted outcome");
    let duplicate = mempool
        .accept_transaction_outcome_with_context(
            duplicate,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("duplicate outcome");
    let replaced = mempool
        .accept_transaction_outcome_with_context(
            replacement,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("replaced outcome");
    let orphaned = Mempool::default()
        .accept_transaction_outcome_with_context(
            orphan,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("orphan outcome");
    let rejection = Mempool::default()
        .accept_transaction_outcome_with_context(
            rejected,
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("rejected outcome");
    let evicted = evicting_mempool
        .accept_transaction_outcome_with_context(
            spend_transaction(
                coinbase_txids[2],
                0,
                499_999_000,
                TransactionInput::SEQUENCE_FINAL,
            ),
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            crate::AdmissionContext::legacy_unknown(),
        )
        .expect("evicted outcome");

    // Assert
    assert_eq!(accepted.label(), MempoolOutcomeLabel::Accepted);
    assert_eq!(duplicate.label(), MempoolOutcomeLabel::Duplicate);
    assert_eq!(replaced.label(), MempoolOutcomeLabel::Replaced);
    assert_eq!(orphaned.label(), MempoolOutcomeLabel::Orphaned);
    assert_eq!(rejection.label(), MempoolOutcomeLabel::Rejected);
    assert_eq!(evicted.label(), MempoolOutcomeLabel::Evicted);
    assert_eq!(
        rejection.maybe_rejection_category(),
        Some(MempoolRejectionCategory::NonStandard)
    );
    assert_eq!(
        orphaned.missing_parents(),
        &[Txid::from_byte_array([8_u8; 32])]
    );
    assert_eq!(replaced.replaced().len(), 1);
    assert!(matches!(accepted, MempoolOutcome::Accepted { .. }));
}
