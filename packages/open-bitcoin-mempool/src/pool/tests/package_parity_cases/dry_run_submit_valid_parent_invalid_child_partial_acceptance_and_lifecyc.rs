use super::*;

#[test]
fn dry_run_submit_valid_parent_invalid_child_partial_acceptance_and_lifecycle_match() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        confirmed_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let mut invalid_child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    invalid_child.inputs[0].script_sig = script(&[0x01, 0x52]);
    let package =
        WellFormedPackage::try_from(vec![parent.clone(), invalid_child.clone()]).expect("package");
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![parent, invalid_child]).expect("submission package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::default();
    let before = mempool.complete_snapshot();

    // Act
    let dry_run = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry run");
    assert_eq!(mempool.complete_snapshot(), before);
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("submit report");

    // Assert
    assert_eq!(dry_run.report, submitted.report);
    assert_eq!(submitted.report.status(), &PackageStatus::Partial);
    assert_eq!(submitted.delta.admitted.len(), 1);
    assert_eq!(submitted.delta.admitted[0].txid, parent_txid);
    assert!(mempool.entry(&parent_txid).is_some());
    assert!(matches!(
        submitted.report.members(),
        [
            PackageMemberResult::FinallyPresent(_),
            PackageMemberResult::HardRejected(_)
        ]
    ));
}

#[test]
fn stale_revision_sparse_patch_rejects_before_apply_without_mutation() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(3);
    let package_transaction = spend_transaction(
        confirmed_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let package = WellFormedPackage::try_from(vec![package_transaction.clone()]).expect("package");
    let mut mempool = Mempool::default();
    let evaluation = evaluate_package_for_test(
        &mempool,
        &package,
        AdmissionContext::legacy_unknown(),
        &snapshot,
        verify_flags(),
        consensus_params(),
    )
    .expect("prepared evaluation");
    let patch = evaluation.patch.expect("sparse patch");
    submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            confirmed_txids[1],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("revision-changing admission");
    let before_apply = mempool.complete_snapshot();

    // Act
    let stale = mempool.apply_prepared(patch);

    // Assert
    assert!(matches!(
        stale,
        Err(MempoolError::StalePreparedTransition { .. })
    ));
    assert_eq!(mempool.complete_snapshot(), before_apply);
    assert!(
        mempool
            .entry(&transaction_txid(&package_transaction).expect("txid"))
            .is_none()
    );
}

#[test]
fn static_truc_rolling_limits_replacement_ephemeral_late_script_order_is_exact() {
    // Arrange
    let expected = vec![
        PackagePolicyStage::Static,
        PackagePolicyStage::Truc,
        PackagePolicyStage::Rolling,
        PackagePolicyStage::Limits,
        PackagePolicyStage::Replacement,
        PackagePolicyStage::Ephemeral,
        PackagePolicyStage::Scripts,
        PackagePolicyStage::Trim,
    ];

    // Act
    let complete = package_policy_probe_for_test(None);

    // Assert
    assert_eq!(complete, (expected.clone(), 1, 1));
    for pre_script_failure in expected.into_iter().take(6) {
        let (trace, script_calls, trim_calls) =
            package_policy_probe_for_test(Some(pre_script_failure));
        assert_eq!(trace.last(), Some(&pre_script_failure));
        assert_eq!(script_calls, 0, "pre-script failure invoked checker");
        assert_eq!(trim_calls, 0);
    }
}

#[test]
fn overlap_over_100_conservative_limited_rbf_count_precedes_union() {
    // Arrange
    let first_spent = OutPoint {
        txid: Txid::from_byte_array([1; 32]),
        vout: 0,
    };
    let second_spent = OutPoint {
        txid: Txid::from_byte_array([2; 32]),
        vout: 0,
    };
    let mut view = PolicyView::default();
    for (id, spent) in [(20, first_spent.clone()), (21, second_spent.clone())] {
        let conflict = policy_entry(id, 2, vec![spent.clone()], 100, 100, 51);
        view.spenders.insert(spent, conflict.txid);
        view.entries.insert(conflict.txid, conflict);
    }
    let parent = policy_candidate(10, 2, vec![first_spent, second_spent], 300, 100);
    let child = policy_candidate(
        11,
        2,
        vec![OutPoint {
            txid: parent.entry.txid,
            vout: 0,
        }],
        400,
        100,
    );

    // Act
    let result = evaluate_limited_package_replacement(
        &view,
        &[parent, child],
        IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
    );

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::TooManyPotentialReplacements {
            count: 102,
            limit: 100,
        })
    );
    assert_eq!(view.descendant_calls.get(), 0);
}

#[test]
fn truc_child_replacement_and_sibling_eviction_use_pre_replacement_facts() {
    // Arrange
    let parent_txid = Txid::from_byte_array([1; 32]);
    let old_child_txid = Txid::from_byte_array([2; 32]);
    let mut view = PolicyView::default();
    let mut parent = policy_entry(1, 3, vec![], 1_000, 100, 2);
    let mut old_child = policy_entry(
        2,
        3,
        vec![OutPoint {
            txid: parent_txid,
            vout: 0,
        }],
        1_000,
        100,
        1,
    );
    parent.children.insert(old_child_txid);
    old_child.parents.insert(parent_txid);
    old_child.ancestor_stats.count = 2;
    view.entries.insert(parent_txid, parent);
    view.entries.insert(old_child_txid, old_child);
    let replacement = [policy_candidate(
        3,
        3,
        vec![OutPoint {
            txid: parent_txid,
            vout: 0,
        }],
        1_000,
        100,
    )];

    // Act
    let child_replacement = evaluate_truc_package(
        &view,
        &replacement,
        TrucPolicy::Enforce,
        &BTreeSet::from([old_child_txid]),
    );
    let sibling_eviction =
        evaluate_truc_package(&view, &replacement, TrucPolicy::Enforce, &BTreeSet::new());
    view.entries
        .get_mut(&old_child_txid)
        .expect("old child")
        .descendant_stats
        .count = 2;
    let ineligible_sibling =
        evaluate_truc_package(&view, &replacement, TrucPolicy::Enforce, &BTreeSet::new());

    // Assert
    assert_eq!(child_replacement, Ok(None));
    assert_eq!(
        sibling_eviction
            .expect("eligible sibling eviction")
            .map(|intent| intent.sibling),
        Some(old_child_txid)
    );
    assert!(matches!(
        ineligible_sibling,
        Err(TrucPolicyError::IneligibleSibling { .. })
    ));
}

#[test]
fn pay_to_anchor_anchor_send_dust_and_ephemeral_predicates_are_exact() {
    // Arrange
    let pay_to_anchor = script(&[0x51, 0x02, 0x4e, 0x73]);
    let ordinary = script(&[
        0xa9, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x87,
    ]);

    // Act / Assert
    let defaults = PolicyConfig::default();
    assert_eq!(
        defaults.ephemeral_policy,
        EphemeralPolicy {
            anchor: true,
            send: false,
            dust: false,
        }
    );
    for anchor in [false, true] {
        for send in [false, true] {
            for dust in [false, true] {
                let permissions = EphemeralPolicy { anchor, send, dust };
                assert_eq!(
                    output_policy_result(pay_to_anchor.clone(), 0, permissions).is_ok(),
                    anchor
                );
                assert_eq!(
                    output_policy_result(pay_to_anchor.clone(), 1, permissions).is_ok(),
                    anchor && dust
                );
                assert_eq!(
                    output_policy_result(ordinary.clone(), 0, permissions).is_ok(),
                    send
                );
                assert_eq!(
                    output_policy_result(ordinary.clone(), 1, permissions).is_ok(),
                    send && dust
                );
            }
        }
    }
}

#[test]
fn max_bound_generated_sparse_overlay_recompute_has_zero_clone_and_one_trim() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(MAX_PACKAGE_COUNT as u32 + 1);
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    for (index, confirmed_txid) in confirmed_txids
        .into_iter()
        .take(MAX_PACKAGE_COUNT)
        .enumerate()
    {
        let transaction = spend_transaction(
            confirmed_txid,
            0,
            499_999_000 - index as i64,
            TransactionInput::SEQUENCE_FINAL,
        );
        let prepared = prepare_candidate(
            &mempool,
            transaction,
            &snapshot,
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("generated candidate");
        prospective
            .stage_candidate(prepared)
            .expect("sparse generated addition");
    }
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(
            prospective.accounted_memory().as_usize().saturating_sub(1),
        ),
        ..PolicyConfig::default()
    };
    let removed = trim_prospective_to_capacity(&mut prospective, &config).expect("one final trim");
    let recomputed = prospective
        .materialize_for_test()
        .expect("test-only full recompute");
    let ledger = recompute_resource_ledger(&recomputed.entries, &recomputed.spent_outpoints)
        .expect("resource recompute");
    let full_clone_count = prospective.full_clone_count_for_test();
    let full_recompute_count = prospective.full_recompute_count_for_test();
    let trim_invocations = prospective.trim_invocations_for_test();
    let patch = prospective
        .prepare_patch(MempoolLifecycleDelta::empty())
        .expect("revision-bound patch");

    // Assert
    assert_eq!(removed.len(), 1);
    assert_eq!(recomputed.entries.len(), MAX_PACKAGE_COUNT - 1);
    assert_eq!(recomputed.resource_ledger, ledger);
    assert_eq!(full_clone_count, 0, "zero clone");
    assert_eq!(full_recompute_count, 0, "zero recompute");
    assert_eq!(trim_invocations, 1, "one trim");
    let mut applied = mempool.clone();
    applied.apply_prepared(patch).expect("sparse patch apply");
    assert_eq!(applied.entries(), &recomputed.entries);
    assert_eq!(applied.resource_ledger(), recomputed.resource_ledger);
}

#[test]
fn max_bound_package_path_runs_one_final_trim_and_keeps_ordered_report() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(MAX_PACKAGE_COUNT as u32 + 1);
    let transactions = confirmed_txids
        .into_iter()
        .take(MAX_PACKAGE_COUNT)
        .enumerate()
        .map(|(index, txid)| {
            spend_transaction(
                txid,
                0,
                499_999_000 - index as i64,
                TransactionInput::SEQUENCE_FINAL,
            )
        })
        .collect::<Vec<_>>();
    let package = WellFormedPackage::try_from(transactions).expect("max-bound package");
    let mempool = Mempool::new(PolicyConfig {
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::ZERO),
        mempool_capacity: MempoolCapacity::new(usize::MAX),
        ..PolicyConfig::default()
    });

    // Act
    reset_package_trim_count_for_test();
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("max-bound dry run");

    // Assert
    assert_eq!(result.report.members().len(), MAX_PACKAGE_COUNT);
    assert_eq!(package_trim_count_for_test(), 1);
    assert!(
        result
            .report
            .members()
            .iter()
            .all(|member| matches!(member, PackageMemberResult::FinallyPresent(_)))
    );
    assert!(mempool.entries().is_empty());
}
