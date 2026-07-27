// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn fjall_recovery_evidence_lock_contention_maps_typed_backend_failure() {
    // Arrange
    let path = temp_store_path("recovery-lock-contention");
    remove_dir_if_exists(&path);
    let store_guard = FjallNodeStore::open(&path).expect("open store guard");

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("second open should hit lock contention"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            action: StorageRecoveryAction::Restart,
            ..
        }
    ));
    assert_eq!(
        error.recovery_category(),
        SyncRecoveryCategory::StorageLockContention
    );
    if let StorageError::BackendFailure { message, .. } = &error {
        assert_eq!(message, "database locked by another process");
    }

    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(
        evidence.category,
        SyncRecoveryCategory::StorageLockContention
    );
    assert_eq!(evidence.cause, RecoveryCause::ActiveLock);
    assert_eq!(evidence.action_class, RecoveryActionClass::SafeRetry);

    drop(store_guard);
    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_path_as_file_maps_backend_open_failure() {
    // Arrange
    let path = temp_store_path("recovery-path-as-file");
    remove_file_if_exists(&path);
    remove_dir_if_exists(&path);
    fs::File::create(&path).expect("create path-as-file fixture");

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("file path should fail store open"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            ..
        }
    ));
    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(
        evidence.category,
        SyncRecoveryCategory::StorageBackendFailure
    );
    assert_eq!(evidence.cause, RecoveryCause::BackendOpenFailure);
    assert_eq!(evidence.action_class, RecoveryActionClass::StopAndEscalate);

    remove_file_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_schema_mismatch_maps_classifier_cause() {
    // Arrange
    let path = temp_store_path("recovery-schema-mismatch");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(SchemaVersion::CURRENT.get() + 1)
            .expect("write schema version");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("expected schema mismatch"),
        Err(error) => error,
    };

    // Assert
    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(evidence.category, SyncRecoveryCategory::IncompatibleSchema);
    assert_eq!(evidence.cause, RecoveryCause::SchemaMismatch);
    assert_eq!(
        evidence.action_class,
        RecoveryActionClass::BackupThenRebuild
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_corruption_marker_maps_classifier_cause() {
    // Arrange
    let path = temp_store_path("recovery-corruption-marker");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Runtime,
            RECOVERY_MARKER_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed marker");

    // Act
    let error = store
        .load_recovery_marker()
        .expect_err("malformed recovery marker");

    // Assert
    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(evidence.category, SyncRecoveryCategory::StoreCorruption);
    assert_eq!(evidence.cause, RecoveryCause::CorruptionMarker);
    assert_eq!(
        evidence.action_class,
        RecoveryActionClass::BackupThenRebuild
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_partial_write_maps_classifier_cause() {
    // Arrange
    let path = temp_store_path("recovery-partial-write");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");

    // Act
    let marker = store
        .mark_interrupted_write(
            StorageNamespace::BlockIndex,
            StorageRecoveryAction::Reindex,
            "block index write interrupted",
            PersistMode::Sync,
        )
        .expect("write recovery marker");

    // Assert
    let mut input = base_recovery_classifier_input();
    input.maybe_recovery_marker = Some(marker);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(evidence.cause, RecoveryCause::PartialWrite);
    assert_eq!(
        evidence.action_class,
        RecoveryActionClass::BackupThenRebuild
    );
    assert_eq!(
        evidence.maybe_affected_namespace,
        Some("block_index".to_string())
    );

    remove_dir_if_exists(&path);
}
