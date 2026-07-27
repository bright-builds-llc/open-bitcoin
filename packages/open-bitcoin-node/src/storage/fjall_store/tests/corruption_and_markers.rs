// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn incompatible_schema_version_returns_schema_mismatch() {
    // Arrange
    let path = temp_store_path("schema-mismatch");
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
    assert!(matches!(
        error,
        StorageError::SchemaMismatch {
            expected: SchemaVersion::CURRENT,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn malformed_snapshot_maps_to_corruption() {
    // Arrange
    let path = temp_store_path("corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            SNAPSHOT_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed record");

    // Act
    let error = store
        .load_chainstate_snapshot()
        .expect_err("malformed chainstate");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn recovery_marker_round_trips_and_clean_shutdown_clears_it() {
    // Arrange
    let path = temp_store_path("recovery-marker");
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
    let loaded = store
        .load_recovery_marker()
        .expect("load recovery marker")
        .expect("recovery marker");
    store
        .mark_clean_shutdown(PersistMode::Sync)
        .expect("mark clean shutdown");

    // Assert
    assert_eq!(loaded, marker);
    assert_eq!(store.load_recovery_marker().expect("reload marker"), None);
    assert!(
        store
            .load_runtime_metadata()
            .expect("load runtime metadata")
            .expect("runtime metadata")
            .last_clean_shutdown
    );

    remove_dir_if_exists(&path);
}

#[test]
fn malformed_recovery_marker_maps_to_runtime_corruption() {
    // Arrange
    let path = temp_store_path("recovery-marker-corruption");
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
    assert!(matches!(
        error,
        StorageError::RecoveryMarkerCorruption {
            namespace: StorageNamespace::Runtime,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}
