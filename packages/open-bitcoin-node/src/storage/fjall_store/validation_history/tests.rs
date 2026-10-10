// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

// Logically nested in the codec owner to inspect private capabilities directly:
// no test factory is exposed to production or to other fixture modules.
use super::*;
use crate::storage::{PersistMode, fjall_store::validation_history::HistoryPublicationFault};
use std::time::{SystemTime, UNIX_EPOCH};

fn store_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "phase159-history-{name}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

#[test]
fn phase159_validation_history_store_real_reopen_retains_monotonic_history() {
    // Arrange
    let dir = store_path("reopen");
    let id = BlockValidationIdentity::new(
        BlockHash::from_byte_array([1; 32]),
        BlockHash::from_byte_array([0; 32]),
        0,
    )
    .expect("identity");
    {
        let store = FjallNodeStore::open(&dir).expect("fresh store");
        let known = AdmittedValidationHeaders {
            store: store.clone(),
            identities: vec![id],
        };
        store
            .publish_admitted_validation_headers(&known)
            .expect("known");
        assert_eq!(
            store.validation_provenance(id.hash()).expect("read"),
            ValidationProvenance::NeverConnected
        );
        let accepted = AcceptedValidationBatch {
            store: store.clone(),
            identities: vec![id],
        };
        store
            .publish_validation_history(&accepted)
            .expect("accepted");
        assert!(store.publish_admitted_validation_headers(&known).is_err());
    }
    // Act
    let store = FjallNodeStore::open(&dir).expect("exclusive reopen");
    // Assert
    assert_eq!(
        store.validation_provenance(id.hash()).expect("retained"),
        ValidationProvenance::ScriptsValid
    );
    assert!(
        store
            .recovered_validation_history()
            .expect("recovery capability")
            .is_complete()
            .expect("coverage")
    );
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_legacy_absence_is_not_backfilled() {
    // Arrange
    let dir = store_path("legacy");
    {
        let store = FjallNodeStore::open_without_ensure_schema_for_test(&dir).expect("legacy seed");
        store
            .write_raw_for_test(StorageNamespace::Runtime, "legacy", vec![1])
            .expect("old data");
    }
    // Act
    let store = FjallNodeStore::open(&dir).expect("legacy open");
    // Assert
    assert_eq!(
        store
            .validation_provenance(BlockHash::from_byte_array([3; 32]))
            .expect("absence"),
        ValidationProvenance::UnknownLegacy
    );
    assert!(
        !store
            .recovered_validation_history()
            .expect("recovery")
            .is_complete()
            .expect("coverage")
    );
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_raw_clone_metadata_loses_coverage_not_acceptance() {
    // Arrange
    let dir = store_path("raw");
    let store = FjallNodeStore::open(&dir).expect("store");
    let id = BlockValidationIdentity::new(
        BlockHash::from_byte_array([1; 32]),
        BlockHash::from_byte_array([0; 32]),
        0,
    )
    .expect("identity");
    let accepted = AcceptedValidationBatch {
        store: store.clone(),
        identities: vec![id],
    };
    store
        .publish_validation_history(&accepted)
        .expect("accepted");
    let recovered = store.recovered_validation_history().expect("capability");
    assert!(recovered.belongs_to(&store.clone()));
    // Act
    store
        .clone()
        .save_chain_meta(&[], PersistMode::Sync)
        .expect("raw seed");
    // Assert
    assert!(recovered.is_complete().is_err());
    assert_eq!(
        store.validation_provenance(id.hash()).expect("retained"),
        ValidationProvenance::ScriptsValid
    );
    assert_eq!(
        store
            .validation_provenance(BlockHash::from_byte_array([4; 32]))
            .expect("absent"),
        ValidationProvenance::UnknownLegacy
    );
    drop(accepted);
    drop(recovered);
    drop(store);
    let reopened = FjallNodeStore::open(&dir).expect("reopen");
    assert_eq!(
        reopened
            .validation_provenance(BlockHash::from_byte_array([4; 32]))
            .expect("absent"),
        ValidationProvenance::UnknownLegacy
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_rejects_foreign_and_conflicting_batches() {
    // Arrange
    let dir = store_path("owned");
    let other_dir = store_path("foreign");
    let store = FjallNodeStore::open(&dir).expect("store");
    let foreign = FjallNodeStore::open(&other_dir).expect("foreign");
    let id = BlockValidationIdentity::new(
        BlockHash::from_byte_array([1; 32]),
        BlockHash::from_byte_array([0; 32]),
        0,
    )
    .expect("identity");
    let batch = AcceptedValidationBatch {
        store: foreign.clone(),
        identities: vec![id],
    };
    // Act / Assert
    assert!(store.publish_validation_history(&batch).is_err());
    let batch = AcceptedValidationBatch {
        store: store.clone(),
        identities: vec![id],
    };
    store.publish_validation_history(&batch).expect("first");
    let conflicting =
        BlockValidationIdentity::new(id.hash(), BlockHash::from_byte_array([9; 32]), 1)
            .expect("conflicting");
    let conflict = AcceptedValidationBatch {
        store: store.clone(),
        identities: vec![conflicting],
    };
    assert!(store.publish_validation_history(&conflict).is_err());
    assert_eq!(
        store.validation_provenance(id.hash()).expect("unchanged"),
        ValidationProvenance::ScriptsValid
    );
    drop(conflict);
    drop(batch);
    drop(store);
    drop(foreign);
    std::fs::remove_dir_all(dir).expect("cleanup");
    std::fs::remove_dir_all(other_dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_rejects_conflicting_admitted_batch_before_effects() {
    // Arrange
    let dir = store_path("duplicate-admitted");
    let store = FjallNodeStore::open(&dir).expect("store");
    let first = BlockValidationIdentity::new(
        BlockHash::from_byte_array([1; 32]),
        BlockHash::from_byte_array([2; 32]),
        1,
    )
    .expect("first identity");
    let conflict =
        BlockValidationIdentity::new(first.hash(), BlockHash::from_byte_array([3; 32]), 2)
            .expect("conflicting identity");
    let admitted = AdmittedValidationHeaders {
        store: store.clone(),
        identities: vec![first, conflict],
    };
    // Act
    let result = store.publish_admitted_validation_headers(&admitted);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store
            .validation_provenance(first.hash())
            .expect("unchanged"),
        ValidationProvenance::NeverConnected
    );
    drop(admitted);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_corruption_blocks_exclusive_open_without_basic() {
    // Arrange / Act / Assert
    for (label, key, bytes) in [
        ("coverage", COVERAGE_KEY.to_string(), vec![9, 1]),
        (
            "record",
            record_key(BlockHash::from_byte_array([1; 32])),
            vec![1],
        ),
        ("version", "validated_coverage:v2".to_string(), vec![1, 1]),
    ] {
        let dir = store_path(label);
        {
            let store = FjallNodeStore::open(&dir).expect("store");
            store
                .write_raw_for_test(StorageNamespace::BlockIndex, &key, bytes)
                .expect("corruption injection");
        }
        assert!(matches!(
            FjallNodeStore::open(&dir),
            Err(StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                ..
            })
        ));
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}

#[test]
fn phase159_validation_history_store_faults_distinguish_before_and_after_commit() {
    // Arrange / Act / Assert
    for (label, fault, expected) in [
        (
            "before",
            HistoryPublicationFault::BeforeCommit,
            ValidationProvenance::NeverConnected,
        ),
        (
            "after",
            HistoryPublicationFault::AfterCommit,
            ValidationProvenance::ScriptsValid,
        ),
    ] {
        let dir = store_path(label);
        let id = BlockValidationIdentity::new(
            BlockHash::from_byte_array([1; 32]),
            BlockHash::from_byte_array([0; 32]),
            0,
        )
        .expect("identity");
        {
            let store = FjallNodeStore::open(&dir).expect("store");
            let batch = AcceptedValidationBatch {
                store: store.clone(),
                identities: vec![id],
            };
            store.set_validation_history_fault(fault);
            assert!(store.publish_validation_history(&batch).is_err());
            if fault == HistoryPublicationFault::AfterCommit {
                assert!(store.clone().validation_provenance(id.hash()).is_err());
                assert!(store.recovered_validation_history().is_err());
                assert!(store.publish_validation_history(&batch).is_err());
            }
        }
        let store = FjallNodeStore::open(&dir).expect("exclusive recover");
        assert_eq!(
            store.validation_provenance(id.hash()).expect("disk state"),
            expected
        );
        drop(store);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}
