// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;

#[test]
fn phase161_disable_validation_healthy_faults_revoke_before_failure() {
    use super::super::super::{FilterPublicationFault, query::QueryRecordFault};
    for configured in [false, true] {
        for kind in 0..4 {
            // Arrange: no immutable query has discovered the external fault.
            let (path, store, positions, records) = retained_store("healthy-disable-validation", 3);
            recover(&store, &positions);
            let old = store
                .with_basic_filter_query(|reader| reader.integrity_epoch())
                .expect("healthy epoch");
            let epoch = store
                .filter_publication_guard()
                .expect("guard")
                .read_integrity_epoch();
            assert!(store.basic_filter_retention().is_ok());
            let scans = store.basic_filter_validation_scans_for_test();
            match kind {
                0 => store
                    .inject_query_record_fault_for_test(
                        records[0].identity().block_hash(),
                        QueryRecordFault::CorruptTarget,
                    )
                    .expect("hidden forest corruption"),
                1 => {
                    store
                        .block_index
                        .insert(codec::active_key(2), vec![255])
                        .expect("checkpoint projection corruption");
                }
                2 => {
                    store
                        .block_index
                        .insert(codec::ownership::OWNER_KEY, vec![255])
                        .expect("owner corruption");
                }
                _ => store.set_basic_filter_fault(FilterPublicationFault::BeforeDisable),
            }
            // Act
            let result = if configured {
                store.configure_basic_filter_index_before_prune(
                    Some(positions[2].block_hash),
                    BasicFilterStartupMode::Disabled,
                )
            } else {
                store.disable_basic_filter_index()
            };
            // Assert
            assert!(result.is_err());
            let control = store
                .filter_publication
                .lock()
                .expect("control even on publication poison");
            assert!(
                !control.read_integrity,
                "failed disable must revoke previously healthy read authority"
            );
            assert!(
                control.retention.measurement().is_err(),
                "failed validation cannot expose old complete growth"
            );
            assert_eq!(
                control.maybe_read_fault, None,
                "disable refusal is not a fabricated immutable query observation"
            );
            assert_eq!(
                control.read_integrity_epoch(),
                epoch + if configured { 2 } else { 1 }
            );
            drop(control);
            assert!(store.basic_filter_retention().is_err());
            assert!(
                store
                    .with_basic_filter_query(|reader| reader.integrity_epoch())
                    .is_err()
            );
            assert!(!matches!(store.basic_peer_epoch_is_current(&old), Ok(true)));
            assert_eq!(
                store.basic_filter_validation_scans_for_test() - scans,
                if kind == 0 || kind == 3 { 1 } else { 0 }
            );
            drop(old);
            cleanup(path, store);
        }
    }
}

#[test]
fn phase161_read_fault_callers_new_metadata_integrity_observation_dominates_backend() {
    use super::super::super::{FilterPublicationFault, publication::BasicFilterReadFault};
    // Arrange
    let (path, store, positions, records) = retained_store("sticky-source-merge", 3);
    recover(&store, &positions);
    store.set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
    assert!(
        store
            .maybe_basic_filter_point_query(records[2].identity().block_hash())
            .is_err()
    );
    let epoch = store
        .filter_publication_guard()
        .expect("guard")
        .read_integrity_epoch();
    store
        .block_index
        .insert(codec::ownership::OWNER_KEY, vec![255])
        .expect("external malformed ownership");
    // Act: a genuine decode source, rather than an invalidated-record refusal.
    assert!(
        store
            .with_basic_filter_query(|reader| reader.integrity_epoch())
            .is_err()
    );
    // Assert
    let control = store.filter_publication_guard().expect("guard");
    assert_eq!(
        control.maybe_read_fault,
        Some(BasicFilterReadFault::Integrity)
    );
    assert_eq!(control.read_integrity_epoch(), epoch);
    drop(control);
    cleanup(path, store);
}

#[test]
fn phase161_read_fault_callers_closing_epoch_exhaustion_preserves_sticky_fault() {
    use super::super::super::{FilterPublicationFault, publication::BasicFilterReadFault};
    // Arrange
    let (path, store, positions, records) = retained_store("sticky-closing-epoch", 3);
    recover(&store, &positions);
    store.set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
    assert!(
        store
            .maybe_basic_filter_point_query(records[2].identity().block_hash())
            .is_err()
    );
    {
        let mut control = store.filter_publication_guard().expect("guard");
        control.maybe_fault = None;
        control.reserve_last_integrity_epoch_for_test();
    }
    // Act: mandatory validation succeeds; the existing closing advance fails.
    assert!(
        store
            .recover_basic_filter_index_before_prune(Some(positions[2].block_hash))
            .is_err()
    );
    // Assert
    let control = store
        .filter_publication
        .lock()
        .expect("poisoned authority data");
    assert_eq!(
        control.maybe_read_fault,
        Some(BasicFilterReadFault::Backend)
    );
    assert!(!control.read_integrity);
    drop(control);
    assert!(store.basic_filter_retention().is_err());
    cleanup(path, store);
}

#[test]
fn phase161_read_fault_callers_failed_recovery_keeps_fault_and_optional_budget_can_reset() {
    use super::super::super::{
        FilterPublicationFault, publication::BasicFilterReadFault, query::QueryRecordFault,
    };
    for corrupt in [false, true] {
        // Arrange
        let (path, store, positions, records) = retained_store("sticky-recovery", 3);
        recover(&store, &positions);
        let hash = records[2].identity().block_hash();
        let old = store
            .with_basic_filter_query(|reader| reader.integrity_epoch())
            .expect("old epoch");
        if corrupt {
            store
                .inject_query_record_fault_for_test(hash, QueryRecordFault::CorruptParent)
                .expect("fault");
        } else {
            store.set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
        }
        assert!(store.maybe_basic_filter_point_query(hash).is_err());
        let expected = Some(if corrupt {
            BasicFilterReadFault::Integrity
        } else {
            BasicFilterReadFault::Backend
        });
        store.filter_publication_guard().expect("guard").maybe_fault = None;
        // Act: ordinary metadata and a partial append cannot refresh the forest.
        seed(&store, &positions);
        assert!(store.maybe_basic_filter_work(&fence(&positions)).is_err());
        assert_eq!(
            store
                .basic_filter_operator_observation()
                .expect("capture")
                .maybe_read_fault,
            expected
        );
        if corrupt {
            assert!(
                store
                    .recover_basic_filter_index_before_prune(Some(positions[2].block_hash))
                    .is_err()
            );
            assert_eq!(
                store
                    .basic_filter_operator_observation()
                    .expect("capture")
                    .maybe_read_fault,
                expected
            );
            assert!(store.basic_filter_retention().is_err());
        } else {
            store
                .configure_basic_filter_index_before_prune_with_retention_limits(
                    Some(positions[2].block_hash),
                    BasicFilterStartupMode::PreserveSaved,
                    RetentionObservationLimits {
                        rows: 0,
                        key_value_bytes: 0,
                    },
                )
                .expect("mandatory validation succeeds despite optional budget");
            assert!(
                store
                    .basic_filter_operator_observation()
                    .expect("capture")
                    .maybe_read_fault
                    .is_none()
            );
            assert_eq!(
                store.basic_filter_retention(),
                Err(BasicIndexUnavailableReason::RetentionBudgetExceeded)
            );
            assert!(
                store
                    .maybe_basic_filter_point_query(hash)
                    .expect("new authority")
                    .is_some()
            );
        }
        // Assert
        assert!(
            !store
                .basic_peer_epoch_is_current(&old)
                .expect("old token stays revoked")
        );
        drop(old);
        cleanup(path, store);
    }
}

#[test]
fn phase161_read_fault_callers_failed_host_transitions_never_clear_sticky_failure() {
    use super::super::super::{FilterPublicationFault as Fault, publication::BasicFilterReadFault};
    for fault in [
        Fault::BeforeEnable,
        Fault::AfterEnable,
        Fault::BeforeDisable,
        Fault::AfterDisable,
        Fault::AfterRelease,
    ] {
        for configured in [false, true] {
            // Arrange
            let (path, store, positions, records) = retained_store("sticky-host-fault", 3);
            for position in &positions {
                store
                    .save_block(
                        &fixture_block(position.previous_block_hash(), position.height),
                        PersistMode::Sync,
                    )
                    .expect("required body");
                if position.height > 0 {
                    store
                        .save_undo(
                            position.block_hash,
                            &BlockUndo::default(),
                            PersistMode::Sync,
                        )
                        .expect("required undo");
                }
            }
            recover(&store, &positions);
            let enable = matches!(fault, Fault::BeforeEnable | Fault::AfterEnable);
            if enable {
                store
                    .disable_basic_filter_index()
                    .expect("disabled baseline");
            }
            store.set_basic_filter_fault(Fault::BeforeQueryRead);
            // The test point adapter exercises the same source on disabled retained data.
            assert!(
                store
                    .maybe_basic_filter_point_query(records[2].identity().block_hash())
                    .is_err()
            );
            store.set_basic_filter_fault(fault);
            // Act
            let result = if configured {
                store.configure_basic_filter_index_before_prune(
                    Some(positions[2].block_hash),
                    if enable {
                        BasicFilterStartupMode::Enabled
                    } else {
                        BasicFilterStartupMode::Disabled
                    },
                )
            } else if enable {
                store.enable_basic_filter_index(&fence(&positions))
            } else {
                store.disable_basic_filter_index()
            };
            // Assert: publication poison may make capture itself unavailable.
            assert!(result.is_err());
            assert_eq!(
                store
                    .filter_publication
                    .lock()
                    .expect("fault consumed")
                    .maybe_fault,
                None,
                "actual lifecycle injection must be reached"
            );
            let control = store.filter_publication.lock().expect("control");
            assert_eq!(
                control.maybe_read_fault,
                Some(BasicFilterReadFault::Backend)
            );
            assert!(!control.read_integrity);
            drop(control);
            assert!(store.basic_filter_retention().is_err());
            cleanup(path, store);
        }
    }
}

#[test]
fn phase161_retention_recovery_startup_modes_reuse_one_existing_validation() {
    for mode in [
        BasicFilterStartupMode::PreserveSaved,
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, store, positions, _) = retained_store("retention-startup-single-scan", 3);
        let expected = observed_rows(&store);
        for limits in [
            RetentionObservationLimits {
                rows: 3,
                key_value_bytes: expected.retained_record_key_value_bytes,
            },
            RetentionObservationLimits {
                rows: 2,
                key_value_bytes: u64::MAX,
            },
            RetentionObservationLimits {
                rows: u64::MAX,
                key_value_bytes: expected.retained_record_key_value_bytes - 1,
            },
        ] {
            let scans = store.basic_filter_validation_scans_for_test();
            // Act
            store
                .configure_basic_filter_index_before_prune_with_retention_limits(
                    Some(positions[2].block_hash),
                    mode,
                    limits,
                )
                .expect("valid bounded startup");
            // Assert
            assert_eq!(
                store.basic_filter_validation_scans_for_test() - scans,
                1,
                "startup mode must not add an accounting traversal"
            );
            let result = if limits.rows == 3 {
                Ok(expected)
            } else {
                Err(BasicIndexUnavailableReason::RetentionBudgetExceeded)
            };
            assert_eq!(store.basic_filter_retention(), result);
            let scans = store.basic_filter_validation_scans_for_test();
            for _ in 0..64 {
                assert_eq!(store.basic_filter_retention(), result);
            }
            assert_eq!(store.basic_filter_validation_scans_for_test(), scans);
        }
        cleanup(path, store);
    }
}

#[test]
fn phase161_retention_recovery_proven_empty_enable_requires_no_scan() {
    // Arrange
    let path = temp_path("retention-startup-proven-empty");
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, _) = fixtures(1);
    seed(&store, &positions);
    store
        .save_block(&fixture_block(BlockHash::default(), 0), PersistMode::Sync)
        .expect("genesis body");
    let scans = store.basic_filter_validation_scans_for_test();
    // Act
    store
        .configure_basic_filter_index_before_prune_with_retention_limits(
            Some(positions[0].block_hash),
            BasicFilterStartupMode::Enabled,
            RetentionObservationLimits {
                rows: 0,
                key_value_bytes: 0,
            },
        )
        .expect("validated empty initialization");
    // Assert
    assert_eq!(store.basic_filter_validation_scans_for_test(), scans);
    assert_eq!(
        store
            .basic_filter_retention()
            .expect("measured empty")
            .retained_record_count,
        0
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_reconciliation_keeps_only_existing_two_validations() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-reconcile-scan-count", 3);
    seed(&store, &positions[..1]);
    let scans = store.basic_filter_validation_scans_for_test();
    // Act
    store
        .configure_basic_filter_index_before_prune_with_retention_limits(
            Some(positions[0].block_hash),
            BasicFilterStartupMode::PreserveSaved,
            RetentionObservationLimits {
                rows: 2,
                key_value_bytes: u64::MAX,
            },
        )
        .expect("reconciled recovery");
    // Assert
    assert_eq!(
        store.basic_filter_validation_scans_for_test() - scans,
        2,
        "recovery and existing checkpoint publisher each validate; accounting adds none"
    );
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionBudgetExceeded)
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(checkpoint(&records[0]))
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_each_mode_refuses_corruption_without_installing_totals() {
    for mode in [
        BasicFilterStartupMode::PreserveSaved,
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, store, positions, _) = retained_store("retention-startup-corruption", 3);
        store
            .write_raw_for_test(StorageNamespace::BlockIndex, &codec::active_key(8), vec![0])
            .expect("corrupt hidden projection");
        let scans = store.basic_filter_validation_scans_for_test();
        // Act
        let result = store.configure_basic_filter_index_before_prune_with_retention_limits(
            Some(positions[2].block_hash),
            mode,
            RetentionObservationLimits {
                rows: 0,
                key_value_bytes: 0,
            },
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(store.basic_filter_validation_scans_for_test() - scans, 1);
        assert_eq!(
            store.basic_filter_retention(),
            Err(BasicIndexUnavailableReason::RetentionFailed)
        );
        cleanup(path, store);
    }
}

#[test]
fn phase161_retention_recovery_each_mode_counts_unclaimed_existing_state_rows() {
    for mode in [
        BasicFilterStartupMode::PreserveSaved,
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let path = temp_path("retention-startup-unclaimed");
        let store = FjallNodeStore::open(&path).expect("store");
        let (positions, records) = fixtures(3);
        seed(&store, &positions);
        for position in &positions {
            store
                .save_block(
                    &fixture_block(position.previous_block_hash(), position.height),
                    PersistMode::Sync,
                )
                .expect("history body");
            if position.height != 0 {
                store
                    .save_undo(
                        position.block_hash,
                        &BlockUndo::default(),
                        PersistMode::Sync,
                    )
                    .expect("history undo");
            }
        }
        store
            .initialize_basic_filter_state(&fence(&positions))
            .expect("empty saved state");
        append_current(&store, &records).expect("unclaimed immutable rows");
        let expected = observed_rows(&store);
        let scans = store.basic_filter_validation_scans_for_test();
        // Act
        store
            .configure_basic_filter_index_before_prune(Some(positions[2].block_hash), mode)
            .expect("retained recovery");
        // Assert
        assert_eq!(store.basic_filter_validation_scans_for_test() - scans, 1);
        assert_eq!(store.basic_filter_retention(), Ok(expected));
        assert_eq!(expected.retained_record_count, 3);
        assert_eq!(
            store.maybe_basic_filter_checkpoint().expect("checkpoint"),
            Some(FilterCheckpoint::new(IndexPrefix::Empty))
        );
        cleanup(path, store);
    }
}

#[test]
fn phase161_retention_recovery_orphan_rows_never_authorize_empty_initialization() {
    for mode in [
        BasicFilterStartupMode::PreserveSaved,
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        for corrupt in [false, true] {
            // Arrange
            let path = temp_path("retention-startup-orphan-proof");
            let store = FjallNodeStore::open(&path).expect("store");
            let (positions, records) = fixtures(3);
            seed(&store, &positions);
            seed_orphan_records(&store, &records);
            if corrupt {
                store
                    .write_raw_for_test(
                        StorageNamespace::BlockIndex,
                        &codec::record_key(records[1].identity().block_hash()),
                        vec![0],
                    )
                    .expect("corrupt orphan");
            }
            // Act
            let result = store
                .configure_basic_filter_index_before_prune(Some(positions[2].block_hash), mode);
            // Assert
            assert!(
                result.is_err(),
                "bounded artifact presence refuses partial state"
            );
            assert_eq!(
                store.basic_filter_retention(),
                Err(BasicIndexUnavailableReason::RetentionFailed)
            );
            assert_eq!(
                store
                    .maybe_basic_filter_state()
                    .expect("state remains absent"),
                None
            );
            cleanup(path, store);
        }
    }
}
