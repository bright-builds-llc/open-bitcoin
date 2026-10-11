// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::super::retention::{RetainedBasicFilterObservation, RetentionObservationLimits};
use super::*;
use crate::status::BasicIndexUnavailableReason;
use crate::status::{BasicRetentionBasis, BasicRetentionMeasurement};

fn observed_rows(store: &FjallNodeStore) -> BasicRetentionMeasurement {
    let mut result = BasicRetentionMeasurement {
        retained_record_count: 0,
        encoded_filter_payload_bytes: 0,
        retained_record_key_value_bytes: 0,
        basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
    };
    for row in store.block_index.prefix("basic_filter:v1:record:") {
        let (key, bytes) = row.into_inner().expect("independent raw row");
        let payload = u32::from_le_bytes(bytes[166..170].try_into().expect("payload length"));
        assert_eq!(bytes.len(), 170 + payload as usize);
        result.retained_record_count += 1;
        result.encoded_filter_payload_bytes += u64::from(payload);
        result.retained_record_key_value_bytes += (key.len() + bytes.len()) as u64;
    }
    result
}

fn retained_store(
    name: &str,
    count: u32,
) -> (
    PathBuf,
    FjallNodeStore,
    Vec<ChainPosition>,
    Vec<StoredFilterRecord>,
) {
    let path = temp_path(name);
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, records) = fixtures(count);
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("initialize");
    publish(&store, &positions, &records);
    (path, store, positions, records)
}

fn recover(store: &FjallNodeStore, positions: &[ChainPosition]) {
    store
        .recover_basic_filter_index_before_prune(Some(positions.last().expect("tip").block_hash))
        .expect("recover retained records");
}

fn cleanup(path: PathBuf, store: FjallNodeStore) {
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

mod publication;
mod recovery;

#[test]
fn phase161_operator_authority_retained_presence_survives_optional_measurement_budget() {
    // Arrange
    let (path, store, positions, _) = retained_store("operator-measurement-bound", 3);
    // Act
    store
        .configure_basic_filter_index_before_prune_with_retention_limits(
            Some(positions[2].block_hash),
            crate::chainstate::BasicFilterStartupMode::PreserveSaved,
            super::super::retention::RetentionObservationLimits {
                rows: 0,
                key_value_bytes: u64::MAX,
            },
        )
        .expect("mandatory validation finishes");
    let observation = store
        .basic_filter_operator_observation()
        .expect("coherent capture");
    // Assert
    assert_eq!(
        observation.retained_availability,
        Ok(crate::status::BasicRetainedAvailability::Observed)
    );
    assert_eq!(
        observation.retention,
        Err(BasicIndexUnavailableReason::RetentionBudgetExceeded)
    );
    assert!(observation.authority.is_ok());
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_empty_is_measured_zero() {
    // Arrange
    let path = temp_path("retention-empty");
    let store = FjallNodeStore::open(&path).expect("store");
    let scans = store.basic_filter_validation_scans_for_test();
    // Act
    store
        .recover_basic_filter_index_before_prune(None)
        .expect("empty recovery");
    let measured = store.basic_filter_retention();
    // Assert
    assert_eq!(store.basic_filter_validation_scans_for_test(), scans);
    assert_eq!(
        measured,
        Ok(BasicRetentionMeasurement {
            retained_record_count: 0,
            encoded_filter_payload_bytes: 0,
            retained_record_key_value_bytes: 0,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        })
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase161_retention_recovery_genesis_has_exact_independent_basis() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-genesis", 1);
    let expected = observed_rows(&store);
    // Act
    recover(&store, &positions);
    // Assert
    assert_eq!(expected.retained_record_count, 1);
    assert!(expected.encoded_filter_payload_bytes > 0);
    assert!(expected.retained_record_key_value_bytes > expected.encoded_filter_payload_bytes);
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_disabled_rows_survive_real_reopen() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-disabled", 3);
    let expected = observed_rows(&store);
    store.disable_basic_filter_index().expect("disable");
    drop(store);
    let store = FjallNodeStore::open(&path).expect("real reopen");
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionIncomplete)
    );
    // Act
    recover(&store, &positions);
    // Assert
    assert_eq!(expected.retained_record_count, 3);
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_displaced_and_ahead_rows_count_once() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-displaced", 3);
    seed(&store, &positions[..1]);
    publish_current(
        &store,
        &fence(&positions[..1]),
        checkpoint(&records[0]),
        IndexInputProtection::FromHeight(1),
        &[],
    )
    .expect("displace suffix");
    let mut replacement_block = fixture_block(positions[0].block_hash, 1);
    replacement_block.header.nonce = 42;
    let replacement_position = ChainPosition::new(replacement_block.header.clone(), 1, 3, 1001);
    let undo = BlockUndo::default();
    let inputs = BasicFilterInputs::from_historical(
        &replacement_block,
        &replacement_position,
        Some(HistoricalBlockUndo {
            block_hash: replacement_position.block_hash,
            undo: &undo,
        }),
    )
    .expect("inputs");
    let replacement =
        StoredFilterRecord::generate(&inputs, &replacement_position, Some(&records[0].identity()))
            .expect("fork record");
    // Retained rows do not claim checkpoint authority, including processed ahead.
    append_current(&store, std::slice::from_ref(&replacement)).expect("retain replacement");
    let expected = observed_rows(&store);
    drop(store);
    let store = FjallNodeStore::open(&path).expect("real reopen");
    // Act
    recover(&store, &positions[..1]);
    // Assert
    assert_eq!(expected.retained_record_count, 4);
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    assert_eq!(
        store
            .maybe_active_basic_filter_record(1)
            .expect("hidden suffix"),
        None
    );
    assert_eq!(
        store
            .basic_filter_projection(2)
            .expect("retained hidden projection"),
        records[2].identity().block_hash()
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_corrupt_parent_never_leaks_prior_totals() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-corrupt-parent", 3);
    recover(&store, &positions);
    assert!(store.basic_filter_retention().is_ok());
    store
        .block_index
        .remove(codec::record_key(records[1].identity().block_hash()))
        .expect("remove parent");
    // Act
    let result = store.recover_basic_filter_index_before_prune(Some(positions[2].block_hash));
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionFailed)
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_corrupt_hidden_projection_stops_complete_measurement() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-hidden-projection", 3);
    recover(&store, &positions);
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            &codec::active_key(8),
            codec::encode_projection(8, records[2].identity().block_hash()),
        )
        .expect("invalid hidden projection");
    // Act
    let result = store.recover_basic_filter_index_before_prune(Some(positions[2].block_hash));
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionFailed)
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_missing_active_projection_refuses() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-missing-projection", 3);
    recover(&store, &positions);
    store
        .block_index
        .remove(codec::active_key(1))
        .expect("remove projection");
    // Act
    let result = store.recover_basic_filter_index_before_prune(Some(positions[2].block_hash));
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionFailed)
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_incomplete_initialization_does_not_earn_zero() {
    // Arrange
    let path = temp_path("retention-failed-initialization");
    let store = FjallNodeStore::open(&path).expect("store");
    // Act
    let result = store.configure_basic_filter_index_before_prune(
        None,
        crate::chainstate::BasicFilterStartupMode::Enabled,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionFailed)
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_observation_reads_do_not_scan_small_or_many_rows() {
    for count in [1, 48] {
        // Arrange
        let (path, store, positions, _) = retained_store("retention-read-cost", count);
        recover(&store, &positions);
        let expected = observed_rows(&store);
        let reads = store.basic_filter_point_reads_for_test();
        let inspections = store
            .filter_publication_guard()
            .expect("control")
            .retention_inspections;
        // Act
        for _ in 0..64 {
            assert_eq!(store.basic_filter_retention(), Ok(expected));
        }
        // Assert
        assert_eq!(store.basic_filter_point_reads_for_test(), reads);
        assert_eq!(
            store
                .filter_publication_guard()
                .expect("control")
                .retention_inspections
                - inspections,
            64
        );
        cleanup(path, store);
    }
}

#[test]
fn phase161_retention_recovery_exact_and_one_over_row_and_byte_budgets() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-budgets", 3);
    let expected = observed_rows(&store);
    let limits = RetentionObservationLimits {
        rows: 3,
        key_value_bytes: expected.retained_record_key_value_bytes,
    };
    // Act
    let exact = store
        .measure_basic_filter_records(limits)
        .expect("valid integrity");
    let rows_over = store
        .measure_basic_filter_records(RetentionObservationLimits { rows: 2, ..limits })
        .expect("row budget must not reject valid integrity");
    let bytes_over = store
        .measure_basic_filter_records(RetentionObservationLimits {
            key_value_bytes: limits.key_value_bytes - 1,
            ..limits
        })
        .expect("byte budget must not reject valid integrity");
    // Assert
    assert_eq!(exact.measurement(), Ok(expected));
    for (observation, budget) in [
        (rows_over, RetentionObservationLimits { rows: 2, ..limits }),
        (
            bytes_over,
            RetentionObservationLimits {
                key_value_bytes: limits.key_value_bytes - 1,
                ..limits
            },
        ),
    ] {
        assert_eq!(
            observation.measurement(),
            Err(BasicIndexUnavailableReason::RetentionBudgetExceeded)
        );
        store
            .configure_basic_filter_index_before_prune_with_retention_limits(
                Some(positions[2].block_hash),
                crate::chainstate::BasicFilterStartupMode::PreserveSaved,
                budget,
            )
            .expect("over-budget startup stays valid");
        assert_eq!(
            store.basic_filter_retention(),
            Err(BasicIndexUnavailableReason::RetentionBudgetExceeded)
        );
    }
    recover(&store, &positions);
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_budget_exhaustion_does_not_skip_projection_integrity() {
    // Arrange
    let (path, store, _, _) = retained_store("retention-budget-integrity", 3);
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, &codec::active_key(8), vec![0])
        .expect("corrupt hidden projection");
    // Act
    let result = store.measure_basic_filter_records(RetentionObservationLimits {
        rows: 0,
        key_value_bytes: 0,
    });
    // Assert
    assert!(result.is_err());
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_checked_overflow_is_unavailable() {
    for field in 0..3 {
        // Arrange
        let mut measured = BasicRetentionMeasurement {
            retained_record_count: 0,
            encoded_filter_payload_bytes: 0,
            retained_record_key_value_bytes: 0,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        };
        match field {
            0 => measured.retained_record_count = u64::MAX,
            1 => measured.encoded_filter_payload_bytes = u64::MAX,
            _ => measured.retained_record_key_value_bytes = u64::MAX,
        }
        let mut observation = RetainedBasicFilterObservation::Complete(measured);
        // Act
        observation.observe_lengths(
            1,
            1,
            1,
            RetentionObservationLimits {
                rows: u64::MAX,
                key_value_bytes: u64::MAX,
            },
        );
        // Assert
        assert_eq!(
            observation.measurement(),
            Err(BasicIndexUnavailableReason::RetentionFailed)
        );
    }
}

#[test]
fn phase161_retention_recovery_raw_invalidation_cannot_revive_old_totals() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-raw-invalidation", 3);
    recover(&store, &positions);
    let key = codec::record_key(records[0].identity().block_hash());
    let bytes = store
        .get_bytes(StorageNamespace::BlockIndex, &key)
        .expect("bytes")
        .expect("record");
    let clone = store.clone();
    // Act
    clone
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, bytes)
        .expect("raw clone write");
    // Assert
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionIncomplete)
    );
    store
        .filter_publication_guard()
        .expect("control")
        .read_integrity = true;
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionIncomplete)
    );
    drop(clone);
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_poison_never_exposes_saved_measurement() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-poison", 1);
    recover(&store, &positions);
    store.set_basic_filter_revision_for_test(u64::MAX);
    // Act
    let result = store.invalidate_basic_filter_append();
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionFailed)
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_direct_enable_installs_already_validated_totals() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-direct-enable", 3);
    let expected = observed_rows(&store);
    // Act
    store
        .enable_basic_filter_index(&fence(&positions))
        .expect("direct recovered enable");
    // Assert
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    cleanup(path, store);
}

#[test]
fn phase161_retention_recovery_append_revision_and_disable_preserve_totals() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-metadata-revision", 3);
    recover(&store, &positions);
    let expected = observed_rows(&store);
    // Act
    store
        .invalidate_basic_filter_append()
        .expect("ordinary revision");
    store
        .disable_basic_filter_index()
        .expect("metadata-only disable");
    // Assert
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    cleanup(path, store);
}
