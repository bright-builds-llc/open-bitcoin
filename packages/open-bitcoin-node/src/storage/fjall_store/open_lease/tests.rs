// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{super::FjallNodeStore, OperationLease};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

// Keep our process-spawn barriers from inheriting another scenario's open fd.
static PROCESS_SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());

thread_local! {
    static CHECK_ENGINE_DROP: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub(super) fn assert_backend_closed_when_armed(source: &Path) {
    if !CHECK_ENGINE_DROP.replace(false) {
        return;
    }
    let lock = fs::File::open(source.join("lock")).expect("existing engine lock");
    lock.try_lock()
        .expect("engine objects must drop before the final application lease");
}

fn source(name: &str) -> PathBuf {
    let root = fs::canonicalize(std::env::temp_dir()).expect("canonical temporary directory");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    root.join(format!(
        "open-bitcoin-phase166-{name}-{}-{nonce}",
        std::process::id()
    ))
}

fn contender(source: &Path, expected: &str) {
    contender_kind(source, expected, "lease");
}

fn contender_kind(source: &Path, expected: &str, kind: &str) {
    let output = Command::new(std::env::current_exe().expect("test executable"))
        .args([
            "--exact",
            "storage::fjall_store::open_lease::tests::phase166_lease_child",
            "--nocapture",
        ])
        .env("PHASE166_LEASE_SOURCE", source)
        .env("PHASE166_LEASE_EXPECT", expected)
        .env("PHASE166_LEASE_KIND", kind)
        .output()
        .expect("child barrier completion");
    assert!(
        output.status.success(),
        "child result: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn phase166_lease_child() {
    let Some(source) = std::env::var_os("PHASE166_LEASE_SOURCE") else {
        return;
    };
    let source = Path::new(&source);
    let result = match std::env::var("PHASE166_LEASE_KIND")
        .expect("opener kind")
        .as_str()
    {
        "lease" => OperationLease::acquire(source).map(drop),
        "generic" => FjallNodeStore::open(source).map(drop),
        "fixture" => FjallNodeStore::open_without_ensure_schema_for_test(source).map(drop),
        _ => panic!("unknown opener"),
    };
    match std::env::var("PHASE166_LEASE_EXPECT")
        .expect("expectation")
        .as_str()
    {
        "busy" => assert!(
            matches!(result, Err(super::StorageError::BackendFailure {ref message, ..}) if message == "database locked by another process")
        ),
        "free" => assert!(result.is_ok()),
        _ => panic!("unknown expectation"),
    }
}

#[test]
fn phase166_lease_external_handoff_and_normalized_identity() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("identity");
    let lease = OperationLease::acquire(&source).expect("lease");
    // Act / Assert: acquisition never creates anything in a missing source.
    assert!(!source.exists());
    contender(&source.join("."), "busy");
    fs::create_dir_all(&source).expect("create source after acquisition");
    contender(&source, "busy");
    drop(lease);
    contender(&source, "free");
    fs::remove_dir(&source).expect("cleanup");
}

#[test]
fn phase166_lease_relative_and_absolute_source_share_identity() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let absolute = std::env::current_dir().expect("current directory");
    let lease = OperationLease::acquire(Path::new(".")).expect("relative lease");
    // Act / Assert
    assert_eq!(lease.source(), absolute);
    contender(&absolute, "busy");
    drop(lease);
    contender(&absolute, "free");
}

#[test]
fn phase166_lease_rejects_parent_components_without_source_effects() {
    // Arrange
    let source = source("parent");
    // Act
    let result = OperationLease::acquire(&source.join("..").join("other"));
    // Assert
    assert!(result.is_err());
    assert!(!source.exists());
}

#[cfg(unix)]
#[test]
fn phase166_lease_rejects_symlink_ancestors_and_final_aliases() {
    // Arrange
    let source = source("symlink");
    fs::create_dir(&source).expect("source");
    let alias = source.with_extension("alias");
    std::os::unix::fs::symlink(&source, &alias).expect("alias");
    // Act / Assert
    for selected in [alias.clone(), alias.join("."), alias.join("missing")] {
        assert!(OperationLease::acquire(&selected).is_err());
    }
    assert_eq!(fs::read_dir(&source).expect("source inventory").count(), 0);
    fs::remove_file(alias).expect("alias cleanup");
    fs::remove_dir(source).expect("source cleanup");
}

#[test]
fn phase166_lease_external_location_must_not_be_inside_selected_source() {
    // Arrange
    let source = fs::canonicalize("/tmp").expect("platform scratch root");
    // Act
    let result = OperationLease::acquire(&source);
    // Assert
    assert!(
        result.is_err(),
        "lease may never create entries in the selected tree"
    );
}

#[test]
fn phase166_lease_generic_and_fixture_openers_share_guard() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("entrypoints");
    let lease = OperationLease::acquire(&source).expect("lease");
    // Act / Assert
    assert!(FjallNodeStore::open(&source).is_err());
    assert!(FjallNodeStore::open_without_ensure_schema_for_test(&source).is_err());
    assert!(!source.exists());
    drop(lease);
    let store = FjallNodeStore::open(&source).expect("open after handoff");
    contender(&source, "busy");
    drop(store);
    contender(&source, "free");
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_lease_coins_view_final_drop_releases_engine_before_lease() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("coins-drop-order");
    let store = FjallNodeStore::open(&source).expect("store");
    let view = store.coins_view();
    drop(store);
    contender(&source, "busy");
    // Act: observe the actual instant when the last application lease drops.
    CHECK_ENGINE_DROP.set(true);
    drop(view);
    // Assert
    contender(&source, "free");
    let reopened = FjallNodeStore::open(&source).expect("same-datadir reopen");
    drop(reopened);
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_lease_store_clones_and_chainstate_adapter_retain_until_final_drop() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("clones");
    let store = FjallNodeStore::open(&source).expect("store");
    let clone = store.clone();
    let adapter = crate::FjallChainstateStore::from_store(clone.clone());
    // Act / Assert
    drop(store);
    contender_kind(&source, "busy", "generic");
    drop(clone);
    contender_kind(&source, "busy", "fixture");
    CHECK_ENGINE_DROP.set(true);
    drop(adapter);
    contender_kind(&source, "free", "generic");
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_lease_snapshot_reads_keep_live_parent_and_return_only_owned_data() {
    use open_bitcoin_core::chainstate::CoinsView;
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("snapshot");
    let store = FjallNodeStore::open(&source).expect("store");
    let view = store.coins_view();
    drop(store);
    // Act
    let snapshot = view.complete_snapshot().expect("complete coins snapshot");
    contender_kind(&source, "busy", "generic");
    // Assert: snapshot is detached domain data, with no escaped Fjall capability.
    assert!(snapshot.coins.is_empty());
    assert!(snapshot.maybe_best_block.is_none());
    CHECK_ENGINE_DROP.set(true);
    drop(view);
    contender_kind(&source, "free", "fixture");
    drop(snapshot);
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_lease_failed_construction_releases_before_repeated_reopen() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("construction");
    let store = FjallNodeStore::open(&source).expect("store");
    store
        .write_schema_version_for_test(u32::MAX)
        .expect("unsupported schema fixture");
    drop(store);
    // Act / Assert
    for _ in 0..3 {
        CHECK_ENGINE_DROP.set(true);
        assert!(FjallNodeStore::open(&source).is_err());
        contender(&source, "free");
    }
    let store =
        FjallNodeStore::open_without_ensure_schema_for_test(&source).expect("fixture reopen");
    store
        .write_schema_version_for_test(super::super::SchemaVersion::CURRENT.get())
        .expect("restore fixture schema");
    drop(store);
    for _ in 0..3 {
        let store = FjallNodeStore::open(&source).expect("repeat actual reopen");
        CHECK_ENGINE_DROP.set(true);
        drop(store);
        contender_kind(&source, "free", "fixture");
    }
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_lease_contention_preserves_existing_source_and_redacts_diagnostics() {
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("private-source");
    fs::create_dir(&source).expect("source");
    let marker = source.join("private-wallet-payload");
    fs::write(&marker, b"private sentinel").expect("sentinel");
    let before = fs::metadata(&marker).expect("before");
    let directory_before = fs::metadata(&source).expect("directory before");
    let lease = OperationLease::acquire(&source).expect("lease");
    // Act
    let error = FjallNodeStore::open(&source)
        .err()
        .expect("contention")
        .to_string();
    contender_kind(&source, "busy", "generic");
    // Assert
    assert!(error.len() <= 240);
    assert!(!error.contains("private-source"));
    assert!(!error.contains("private sentinel"));
    assert_eq!(fs::read(&marker).expect("sentinel"), b"private sentinel");
    let after = fs::metadata(&marker).expect("after");
    assert_eq!(
        before.modified().expect("mtime"),
        after.modified().expect("mtime")
    );
    assert_eq!(before.permissions(), after.permissions());
    assert_eq!(
        directory_before.modified().expect("directory mtime"),
        fs::metadata(&source)
            .expect("directory after")
            .modified()
            .expect("directory mtime")
    );
    assert_eq!(fs::read_dir(&source).expect("inventory").count(), 1);
    drop(lease);
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_lease_unsafe_external_file_is_bounded_and_source_free() {
    // Arrange
    let source = source("lock-error");
    let lock = super::external_lock_path(&source).expect("lock identity");
    fs::write(&lock, b"private invalid lock payload").expect("invalid fixture");
    // Act
    let error = OperationLease::acquire(&source)
        .err()
        .expect("unsafe lock")
        .to_string();
    // Assert
    assert!(!source.exists());
    assert!(error.len() <= 240);
    assert!(!error.contains("private invalid"));
    assert_eq!(
        fs::read(&lock).expect("lock preserved"),
        b"private invalid lock payload"
    );
    fs::remove_file(lock).expect("inactive invalid fixture cleanup");
}

#[test]
fn phase166_lease_ordinary_refusal_never_acquires_or_creates() {
    use open_bitcoin_core::consensus::CanonicalChain;
    // Arrange
    let source = source("ordinary");
    let lock = super::external_lock_path(&source).expect("lock identity");
    // Act / Assert
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ] {
        assert!(FjallNodeStore::open_ordinary(&source, chain).is_err());
    }
    assert!(!source.exists());
    assert!(!lock.exists());
}

#[cfg(unix)]
#[test]
fn phase166_lease_reuses_one_empty_private_inode_across_creation_and_reopen() {
    use std::os::unix::fs::MetadataExt;
    // Arrange
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let source = source("inode").join("missing").join("store");
    let lease = OperationLease::acquire(&source).expect("missing suffix lease");
    let lock = super::external_lock_path(&source).expect("lock identity");
    let before = fs::metadata(&lock).expect("initial lock");
    assert_eq!(before.len(), 0);
    assert_eq!(before.mode() & 0o777, 0o600);
    drop(lease);
    // Act / Assert
    for _ in 0..3 {
        let store = FjallNodeStore::open(&source).expect("create or reopen");
        let after = fs::metadata(&lock).expect("retained lock");
        assert_eq!((before.dev(), before.ino()), (after.dev(), after.ino()));
        contender(&source, "busy");
        drop(store);
        contender(&source, "free");
    }
    let root = source
        .parent()
        .expect("missing parent")
        .parent()
        .expect("owned root");
    fs::remove_dir_all(root).expect("cleanup");
    assert!(lock.exists(), "stable inactive lease inode remains");
}
