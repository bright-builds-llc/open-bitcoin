// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn lock_probe_missing_datadir_reports_unavailable_reason() {
    // Arrange
    let path = temp_store_path("lock-probe-missing");
    remove_dir_if_exists(&path);

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    assert_eq!(
        evidence,
        FieldAvailability::unavailable("lock probe unavailable: datadir does not exist")
    );
}

#[test]
fn lock_probe_datadir_without_lock_reports_no_artifact_without_creating_file() {
    // Arrange
    let path = temp_store_path("lock-probe-no-artifact");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("create temp datadir");

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("lock evidence should be available");
    };
    assert_eq!(evidence.kind, LockEvidenceKind::NoLockArtifact);
    assert_eq!(evidence.lock_path, lock_path(&path).display().to_string());
    assert_eq!(evidence.detail, "no Fjall lock artifact found");
    assert!(!lock_path(&path).exists());

    remove_dir_if_exists(&path);
}

#[test]
fn lock_probe_present_unheld_lock_reports_stale_evidence_and_keeps_file() {
    // Arrange
    let path = temp_store_path("lock-probe-stale");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("create temp datadir");
    fs::File::create(lock_path(&path)).expect("create lock artifact");

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("lock evidence should be available");
    };
    assert_eq!(evidence.kind, LockEvidenceKind::StaleLockEvidence);
    assert_eq!(evidence.lock_path, lock_path(&path).display().to_string());
    assert_eq!(
        evidence.detail,
        "Fjall lock artifact is present but not currently held"
    );
    assert!(lock_path(&path).exists());

    remove_dir_if_exists(&path);
}

#[test]
fn lock_probe_held_fjall_store_reports_active_contention_without_opening_store() {
    // Arrange
    let path = temp_store_path("lock-probe-active");
    remove_dir_if_exists(&path);
    let store_guard = FjallNodeStore::open(&path).expect("open store guard");

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("lock evidence should be available");
    };
    assert_eq!(evidence.kind, LockEvidenceKind::ActiveContention);
    assert_eq!(evidence.lock_path, lock_path(&path).display().to_string());
    assert_eq!(
        evidence.detail,
        "Fjall lock is currently held by another opener"
    );

    drop(store_guard);
    remove_dir_if_exists(&path);
}
