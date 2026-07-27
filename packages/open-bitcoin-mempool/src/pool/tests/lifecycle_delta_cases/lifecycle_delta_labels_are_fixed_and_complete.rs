// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

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
