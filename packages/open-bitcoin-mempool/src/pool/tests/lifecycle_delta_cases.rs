// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{BlockHash, Transaction, TransactionInput, Txid, Wtxid};

use crate::{
    AdmissionContext, BlockLifecycleContext, FinalMempoolMembership, Mempool, MempoolEntryMetadata,
    MempoolError, MempoolLifecycleDelta, MempoolLifecycleInvariantError, MempoolLifecycleRemoval,
    MempoolMemberIdentity, MempoolMemberState, MempoolOrigin, MempoolOutcome, MempoolRemovalCause,
    MempoolRemovalRole, MempoolRetryClear, MempoolRetryClearCause, PolicyConfig, PolicyTime,
    RelayIntent, RollingMempoolFeeRate, TransactionVirtualSize,
    transaction_weight_and_virtual_size,
};

use super::{
    build_block, non_standard_spend, sample_chainstate_snapshot, spend_transaction, submit,
};

fn identity(value: u8) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([value; 32]),
        wtxid: Wtxid::from_byte_array([value.saturating_add(32); 32]),
    }
}

fn submit_transition(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: Transaction,
    context: AdmissionContext,
) -> crate::MempoolTransition {
    mempool
        .accept_transaction_transition_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            context,
        )
        .expect("transition outcome")
}

fn build_retry_delta(
    first: MempoolRetryClearCause,
    second: MempoolRetryClearCause,
) -> MempoolLifecycleDelta {
    let member = identity(1);
    let mut builder = MempoolLifecycleDelta::builder();
    builder
        .record_final_membership(MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Present,
        })
        .expect("consistent final membership");
    builder
        .record_retry_clear(MempoolRetryClear {
            member,
            cause: first,
        })
        .expect("consistent first retry clear");
    builder
        .record_retry_clear(MempoolRetryClear {
            member,
            cause: second,
        })
        .expect("consistent second retry clear");
    builder.build().expect("complete lifecycle delta")
}

#[test]
fn lifecycle_delta_labels_are_fixed_and_complete() {
    // Arrange
    let causes = [
        (MempoolRemovalCause::Replacement, "replacement"),
        (MempoolRemovalCause::Expiry, "expiry"),
        (MempoolRemovalCause::Pressure, "pressure"),
        (MempoolRemovalCause::BlockConfirmation, "block_confirmation"),
        (MempoolRemovalCause::BlockConflict, "block_conflict"),
        (MempoolRemovalCause::Reorg, "reorg"),
    ];
    let roles = [
        (MempoolRemovalRole::Direct, "direct"),
        (MempoolRemovalRole::Descendant, "descendant"),
    ];
    let memberships = [
        (FinalMempoolMembership::Present, "present"),
        (FinalMempoolMembership::Absent, "absent"),
    ];
    let retry_causes = [
        (
            MempoolRetryClearCause::LifecycleRemoval,
            "lifecycle_removal",
        ),
        (MempoolRetryClearCause::EligibleServe, "eligible_serve"),
        (
            MempoolRetryClearCause::TransportWritten,
            "transport_written",
        ),
    ];

    // Act and Assert
    for (cause, expected) in causes {
        assert_eq!(cause.as_str(), expected);
    }
    for (role, expected) in roles {
        assert_eq!(role.as_str(), expected);
    }
    for (membership, expected) in memberships {
        assert_eq!(membership.as_str(), expected);
    }
    for (cause, expected) in retry_causes {
        assert_eq!(cause.as_str(), expected);
    }
}

#[test]
fn lifecycle_delta_deduplicates_and_orders_affected_members() {
    // Arrange
    let first = identity(1);
    let removed = identity(2);
    let admitted_first = identity(3);
    let mut builder = MempoolLifecycleDelta::builder();
    for member in [admitted_first, first, admitted_first] {
        builder
            .record_admitted(member)
            .expect("consistent admitted member");
    }
    builder
        .record_removal(MempoolLifecycleRemoval {
            member: removed,
            cause: MempoolRemovalCause::Pressure,
            role: MempoolRemovalRole::Descendant,
        })
        .expect("consistent descendant removal");
    builder
        .record_removal(MempoolLifecycleRemoval {
            member: removed,
            cause: MempoolRemovalCause::Replacement,
            role: MempoolRemovalRole::Direct,
        })
        .expect("consistent direct removal");
    for state in [
        MempoolMemberState {
            member: admitted_first,
            membership: FinalMempoolMembership::Present,
        },
        MempoolMemberState {
            member: removed,
            membership: FinalMempoolMembership::Absent,
        },
        MempoolMemberState {
            member: first,
            membership: FinalMempoolMembership::Present,
        },
    ] {
        builder
            .record_final_membership(state)
            .expect("consistent final membership");
    }
    builder
        .record_retry_clear(MempoolRetryClear {
            member: removed,
            cause: MempoolRetryClearCause::EligibleServe,
        })
        .expect("consistent eligibility clear");
    builder
        .record_retry_clear(MempoolRetryClear {
            member: removed,
            cause: MempoolRetryClearCause::LifecycleRemoval,
        })
        .expect("consistent lifecycle clear");

    // Act
    let delta = builder.build().expect("complete lifecycle delta");

    // Assert
    assert_eq!(delta.admitted, vec![admitted_first, first]);
    assert_eq!(
        delta.removed,
        vec![MempoolLifecycleRemoval {
            member: removed,
            cause: MempoolRemovalCause::Replacement,
            role: MempoolRemovalRole::Direct,
        }]
    );
    assert_eq!(
        delta.final_membership,
        vec![
            MempoolMemberState {
                member: first,
                membership: FinalMempoolMembership::Present,
            },
            MempoolMemberState {
                member: removed,
                membership: FinalMempoolMembership::Absent,
            },
            MempoolMemberState {
                member: admitted_first,
                membership: FinalMempoolMembership::Present,
            },
        ]
    );
    assert_eq!(
        delta.retry_clears,
        vec![MempoolRetryClear {
            member: removed,
            cause: MempoolRetryClearCause::LifecycleRemoval,
        }]
    );
}

#[test]
fn lifecycle_delta_keeps_cause_independent_from_direct_role_upgrade() {
    // Arrange
    let member = identity(4);
    let mut builder = MempoolLifecycleDelta::builder();
    builder
        .record_removal(MempoolLifecycleRemoval {
            member,
            cause: MempoolRemovalCause::BlockConflict,
            role: MempoolRemovalRole::Descendant,
        })
        .expect("consistent descendant removal");
    builder
        .record_removal(MempoolLifecycleRemoval {
            member,
            cause: MempoolRemovalCause::BlockConflict,
            role: MempoolRemovalRole::Direct,
        })
        .expect("consistent direct removal");
    builder
        .record_final_membership(MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Absent,
        })
        .expect("consistent final membership");

    // Act
    let delta = builder.build().expect("complete lifecycle delta");

    // Assert
    assert_eq!(delta.removed[0].cause, MempoolRemovalCause::BlockConflict);
    assert_eq!(delta.removed[0].role, MempoolRemovalRole::Direct);
}

#[test]
fn lifecycle_delta_applies_removal_cause_precedence_in_both_orders() {
    // Arrange
    let member = identity(7);
    let causes = [
        MempoolRemovalCause::Replacement,
        MempoolRemovalCause::Expiry,
        MempoolRemovalCause::Pressure,
        MempoolRemovalCause::BlockConfirmation,
        MempoolRemovalCause::BlockConflict,
        MempoolRemovalCause::Reorg,
    ];

    // Act
    let deltas = [causes.to_vec(), causes.into_iter().rev().collect()].map(|ordered_causes| {
        let mut builder = MempoolLifecycleDelta::builder();
        for cause in ordered_causes {
            builder
                .record_removal(MempoolLifecycleRemoval {
                    member,
                    cause,
                    role: MempoolRemovalRole::Descendant,
                })
                .expect("consistent removal identity");
        }
        builder
            .record_final_membership(MempoolMemberState {
                member,
                membership: FinalMempoolMembership::Absent,
            })
            .expect("consistent final membership");
        builder.build().expect("complete lifecycle delta")
    });

    // Assert
    for delta in deltas {
        assert_eq!(
            delta.removed[0].cause,
            MempoolRemovalCause::BlockConfirmation
        );
    }
}

#[test]
fn lifecycle_delta_absent_final_membership_wins_duplicate_state() {
    // Arrange
    let member = identity(8);
    let mut builder = MempoolLifecycleDelta::builder();
    builder
        .record_final_membership(MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Present,
        })
        .expect("consistent present state");

    // Act
    builder
        .record_final_membership(MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Absent,
        })
        .expect("consistent absent state");
    let delta = builder.build().expect("complete lifecycle delta");

    // Assert
    assert_eq!(
        delta.final_membership[0].membership,
        FinalMempoolMembership::Absent
    );
}

#[test]
fn lifecycle_delta_empty_contract_is_explicit() {
    // Arrange and Act
    let empty = MempoolLifecycleDelta::empty();
    let built_empty = MempoolLifecycleDelta::builder()
        .build()
        .expect("empty builder is complete");

    // Assert
    assert!(empty.is_empty());
    assert!(built_empty.is_empty());
    assert_eq!(empty, built_empty);
}

#[test]
fn lifecycle_delta_collapses_duplicate_retry_clear_cause() {
    // Arrange
    let cause = MempoolRetryClearCause::TransportWritten;

    // Act
    let delta = build_retry_delta(cause, cause);

    // Assert
    assert_eq!(delta.retry_clears.len(), 1);
    assert_eq!(delta.retry_clears[0].cause, cause);
}

#[test]
fn lifecycle_delta_applies_retry_clear_precedence_in_both_orders() {
    // Arrange
    let cases = [
        (
            MempoolRetryClearCause::EligibleServe,
            MempoolRetryClearCause::TransportWritten,
            MempoolRetryClearCause::TransportWritten,
        ),
        (
            MempoolRetryClearCause::EligibleServe,
            MempoolRetryClearCause::LifecycleRemoval,
            MempoolRetryClearCause::LifecycleRemoval,
        ),
        (
            MempoolRetryClearCause::TransportWritten,
            MempoolRetryClearCause::LifecycleRemoval,
            MempoolRetryClearCause::LifecycleRemoval,
        ),
    ];

    // Act and Assert
    for (lower, higher, expected) in cases {
        assert_eq!(
            build_retry_delta(lower, higher).retry_clears[0].cause,
            expected
        );
        assert_eq!(
            build_retry_delta(higher, lower).retry_clears[0].cause,
            expected
        );
    }
}

#[test]
fn lifecycle_delta_rejects_conflicting_txid_or_wtxid_identity() {
    // Arrange
    let original = identity(5);
    let conflicting_wtxid = MempoolMemberIdentity {
        txid: original.txid,
        wtxid: Wtxid::from_byte_array([99; 32]),
    };
    let conflicting_txid = MempoolMemberIdentity {
        txid: Txid::from_byte_array([100; 32]),
        wtxid: original.wtxid,
    };
    let mut txid_builder = MempoolLifecycleDelta::builder();
    txid_builder
        .record_admitted(original)
        .expect("first identity is consistent");
    let mut wtxid_builder = MempoolLifecycleDelta::builder();
    wtxid_builder
        .record_admitted(original)
        .expect("first identity is consistent");

    // Act
    let txid_error = txid_builder
        .record_retry_clear(MempoolRetryClear {
            member: conflicting_wtxid,
            cause: MempoolRetryClearCause::EligibleServe,
        })
        .expect_err("same txid with another wtxid must fail");
    let wtxid_error = wtxid_builder
        .record_retry_clear(MempoolRetryClear {
            member: conflicting_txid,
            cause: MempoolRetryClearCause::EligibleServe,
        })
        .expect_err("same wtxid with another txid must fail");

    // Assert
    assert!(matches!(
        txid_error,
        MempoolLifecycleInvariantError::IdentityConflict { .. }
    ));
    assert!(matches!(
        wtxid_error,
        MempoolLifecycleInvariantError::IdentityConflict { .. }
    ));
    assert!(
        txid_error
            .to_string()
            .contains("conflicts with a prior pair")
    );
    assert!(
        wtxid_error
            .to_string()
            .contains("conflicts with a prior pair")
    );
}

#[test]
fn lifecycle_delta_requires_final_membership_for_every_affected_identity() {
    // Arrange
    let member = identity(6);
    let mut builder = MempoolLifecycleDelta::builder();
    builder
        .record_admitted(member)
        .expect("consistent admitted member");

    // Act
    let error = builder
        .build()
        .expect_err("affected identity without final membership must fail");

    // Assert
    assert_eq!(
        error,
        MempoolLifecycleInvariantError::MissingFinalMembership { member }
    );
    assert!(error.to_string().contains("has no final membership"));
}

#[test]
fn lifecycle_invariant_errors_map_to_internal_mempool_errors() {
    // Arrange
    let source = MempoolLifecycleInvariantError::MissingFinalMembership {
        member: identity(9),
    };

    // Act
    let admission_error = super::super::admission::lifecycle_invariant_error(source);
    let lifecycle_error = super::super::lifecycle::lifecycle_invariant_error(source);

    // Assert
    assert!(matches!(
        admission_error,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(matches!(
        lifecycle_error,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(admission_error.to_string().contains("final membership"));
    assert!(lifecycle_error.to_string().contains("final membership"));
}

#[test]
fn accepted_admission_transition_reports_present_member() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let member = MempoolMemberIdentity { txid, wtxid };
    let mut mempool = Mempool::default();

    // Act
    let transition = submit_transition(
        &mut mempool,
        &snapshot,
        transaction,
        AdmissionContext::legacy_unknown(),
    );

    // Assert
    assert!(matches!(
        transition.outcome,
        MempoolOutcome::Accepted { .. }
    ));
    assert_eq!(transition.delta.admitted, vec![member]);
    assert_eq!(
        transition.delta.final_membership,
        vec![MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Present,
        }]
    );
    assert!(transition.delta.removed.is_empty());
}

#[test]
fn replacement_transition_preserves_direct_and_descendant_roles() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_txid = transaction_txid(&original).expect("original txid");
    let descendant = spend_transaction(
        original_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_996_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement_txid = transaction_txid(&replacement).expect("replacement txid");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original).expect("original admission");
    submit(&mut mempool, &snapshot, descendant).expect("descendant admission");

    // Act
    let transition = submit_transition(
        &mut mempool,
        &snapshot,
        replacement,
        AdmissionContext::legacy_unknown(),
    );

    // Assert
    assert!(matches!(
        transition.outcome,
        MempoolOutcome::Replaced { .. }
    ));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == original_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == descendant_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert!(transition.delta.final_membership.iter().any(|state| {
        state.member.txid == replacement_txid && state.membership == FinalMempoolMembership::Present
    }));
}

#[test]
fn legacy_trim_transition_reports_pressure_without_rolling_fee_change() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let low_fee_parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_900,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_parent_txid = transaction_txid(&low_fee_parent).expect("low fee parent txid");
    let low_fee_child = spend_transaction(
        low_fee_parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let low_fee_child_txid = transaction_txid(&low_fee_child).expect("low fee child txid");
    let (_parent_weight, parent_vsize) =
        transaction_weight_and_virtual_size(&low_fee_parent).expect("parent size");
    let (_child_weight, child_vsize) =
        transaction_weight_and_virtual_size(&low_fee_child).expect("child size");
    let high_fee = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        legacy_vsize_trim_limit: TransactionVirtualSize::new(parent_vsize + child_vsize),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, low_fee_parent).expect("low fee parent admission");
    submit(&mut mempool, &snapshot, low_fee_child).expect("low fee child admission");

    // Act
    let transition = submit_transition(
        &mut mempool,
        &snapshot,
        high_fee,
        AdmissionContext::legacy_unknown(),
    );

    // Assert
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == low_fee_parent_txid
            && removal.cause == MempoolRemovalCause::Pressure
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(transition.delta.removed.iter().any(|removal| {
        removal.member.txid == low_fee_child_txid
            && removal.cause == MempoolRemovalCause::Pressure
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert_eq!(
        mempool.rolling_mempool_fee_rate(),
        RollingMempoolFeeRate::ZERO
    );
}

#[test]
fn connected_block_transition_distinguishes_confirmation_from_conflict_descendants() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let confirmed = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let confirmed_txid = transaction_txid(&confirmed).expect("confirmed txid");
    let conflict = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let conflict_txid = transaction_txid(&conflict).expect("conflict txid");
    let descendant = spend_transaction(
        conflict_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let in_block_conflict = spend_transaction(
        coinbase_txids[1],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut block = build_block(BlockHash::from_byte_array([0; 32]), 3, 499_999_000);
    block.transactions.push(confirmed.clone());
    block.transactions.push(in_block_conflict);
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, confirmed).expect("confirmed admission");
    submit(&mut mempool, &snapshot, conflict).expect("conflict admission");
    submit(&mut mempool, &snapshot, descendant).expect("descendant admission");

    // Act
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block transition");

    // Assert
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == confirmed_txid
            && removal.cause == MempoolRemovalCause::BlockConfirmation
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == conflict_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == descendant_txid
            && removal.cause == MempoolRemovalCause::BlockConflict
            && removal.role == MempoolRemovalRole::Descendant
    }));
}

#[test]
fn noncommitting_attempts_return_empty_delta_and_preserve_metadata() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let initial_metadata = MempoolEntryMetadata::legacy_unknown();
    let mut mempool = Mempool::default();
    submit_transition(
        &mut mempool,
        &snapshot,
        transaction.clone(),
        AdmissionContext::new(initial_metadata),
    );
    let changed_metadata = MempoolEntryMetadata::new(
        crate::MempoolAcceptanceTime::Known(PolicyTime::new(123)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    );

    // Act
    let duplicate = submit_transition(
        &mut mempool,
        &snapshot,
        transaction,
        AdmissionContext::new(changed_metadata),
    );
    let orphan = submit_transition(
        &mut mempool,
        &snapshot,
        spend_transaction(
            Txid::from_byte_array([99; 32]),
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        AdmissionContext::new(changed_metadata),
    );
    let rejected = submit_transition(
        &mut Mempool::default(),
        &snapshot,
        non_standard_spend(coinbase_txids[1]),
        AdmissionContext::new(changed_metadata),
    );

    // Assert
    assert!(matches!(
        duplicate.outcome,
        MempoolOutcome::Duplicate { .. }
    ));
    assert!(duplicate.delta.is_empty());
    assert!(matches!(orphan.outcome, MempoolOutcome::Orphaned { .. }));
    assert!(orphan.delta.is_empty());
    assert!(matches!(rejected.outcome, MempoolOutcome::Rejected { .. }));
    assert!(rejected.delta.is_empty());
    assert_eq!(
        mempool.entry(&txid).expect("original entry").metadata,
        initial_metadata
    );
}
