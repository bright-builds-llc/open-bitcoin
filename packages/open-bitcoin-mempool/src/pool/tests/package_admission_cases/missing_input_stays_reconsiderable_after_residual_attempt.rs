// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn missing_input_stays_reconsiderable_after_residual_attempt() {
    // Arrange
    let (snapshot, _coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        Txid::from_byte_array([94; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = Mempool::default();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![transaction]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("missing input report");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::Reconsiderable(
            ReconsiderableMemberFailure::MissingInputs { .. }
        )]
    ));
}

#[test]
fn residual_pre_script_policy_failure_discards_working_overlay() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = non_standard_spend(parent_txid);
    let mempool = rolling_only_mempool(PolicyConfig::default());
    let before = mempool.complete_snapshot();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![parent, child]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("policy failure report");

    // Assert
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn residual_limit_failure_discards_working_overlay() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_994_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = rolling_only_mempool(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });
    let before = mempool.complete_snapshot();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![parent, child]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("limit failure report");

    // Assert
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn residual_script_failure_discards_working_overlay() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = invalid_script_transaction(spend_transaction(
        parent_txid,
        0,
        499_994_000,
        TransactionInput::SEQUENCE_FINAL,
    ));
    let mempool = rolling_only_mempool(PolicyConfig::default());
    let before = mempool.complete_snapshot();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![parent, child]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("script failure report");

    // Assert
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn prospective_limit_checks_cover_all_directions_and_kinds() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let configs = [
        PolicyConfig {
            max_ancestor_count: 0,
            ..PolicyConfig::default()
        },
        PolicyConfig {
            max_ancestor_virtual_size: 0,
            ..PolicyConfig::default()
        },
        PolicyConfig {
            max_descendant_count: 1,
            ..PolicyConfig::default()
        },
        PolicyConfig {
            max_descendant_virtual_size: 1,
            ..PolicyConfig::default()
        },
    ];

    // Act
    let reports = configs.map(|config| {
        Mempool::new(config)
            .dry_run_package(
                DryRunPackageCommand {
                    package: package(vec![parent.clone(), child.clone()]),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .expect("typed limit report")
            .report
    });

    // Assert
    assert!(reports.iter().all(|report| {
        report
            .members()
            .iter()
            .any(|member| matches!(member, PackageMemberResult::HardRejected(_)))
    }));
}

#[test]
fn prospective_limit_checks_fail_closed_for_missing_members() {
    // Arrange
    let mempool = Mempool::default();
    let prospective = super::super::prospective::ProspectiveMempool::new(&mempool);
    let missing_txid = Txid::from_byte_array([95; 32]);

    // Act
    let missing_candidate = prospective
        .validate_candidate_limits(missing_txid)
        .expect_err("missing candidate");

    // Assert
    assert!(matches!(
        missing_candidate,
        MempoolError::InternalInvariant { .. }
    ));
}

#[test]
fn prospective_limit_checks_fail_closed_for_missing_ancestor() {
    // Arrange
    let transaction = spend_transaction(
        Txid::from_byte_array([96; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let mut entry = MempoolEntry::new(
        transaction,
        txid,
        wtxid,
        Amount::from_sats(1_000).expect("fee"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    );
    entry.parents.insert(Txid::from_byte_array([97; 32]));
    let mut mempool = Mempool::default();
    mempool.entries.insert(txid, entry);
    let prospective = super::super::prospective::ProspectiveMempool::new(&mempool);

    // Act
    let error = prospective
        .validate_candidate_limits(txid)
        .expect_err("missing ancestor");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn package_invariant_mappers_remain_typed() {
    // Arrange
    let report_error = PackageReportError::MemberCountMismatch {
        expected: 1,
        actual: 0,
    };

    // Act
    let empty_group = super::super::package_admission::empty_fee_group_error_for_test();
    let group_invariant = super::super::package_admission::group_invariant_for_test();
    let report_invariant =
        super::super::package_admission::report_invariant_error_for_test(report_error);

    // Assert
    assert!(matches!(
        empty_group,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(matches!(
        group_invariant,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(matches!(
        report_invariant,
        MempoolError::InternalInvariant { .. }
    ));
}
