// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Private, bounded inspection; never opens the original engine.

use super::open_lease::OperationLease;
use super::{StorageError, StorageNamespace, StorageRecoveryAction};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
    fs::{self, File, OpenOptions, TryLockError},
    path::{Path, PathBuf},
};

pub(super) mod canonical_history;
mod copy;
mod live;
mod protocol;
mod reader;
mod worker;
pub(in crate::storage) use reader::InspectedRecords;
pub(super) use worker::maybe_run;

pub(in crate::storage) fn require_existing_namespaces(
    records: &InspectedRecords,
) -> Result<(), StorageError> {
    require_existing_namespace_catalog(records.namespaces())
}

pub(in crate::storage) fn require_existing_namespace_catalog<'a>(
    names: impl Iterator<Item = &'a str>,
) -> Result<(), StorageError> {
    let names: std::collections::BTreeSet<_> = names.collect();
    for namespace in [
        StorageNamespace::Headers,
        StorageNamespace::BlockIndex,
        StorageNamespace::Chainstate,
        StorageNamespace::Coins,
        StorageNamespace::Wallet,
        StorageNamespace::Metrics,
        StorageNamespace::Mempool,
        StorageNamespace::Runtime,
        StorageNamespace::Schema,
    ] {
        if !names.contains(namespace.as_str()) {
            return Err(inspection_error("inspection live namespace missing"));
        }
    }
    Ok(())
}

impl super::FjallNodeStore {
    /// Observe the current physical copy envelope without reopening the held engine.
    pub(crate) fn check_submission_physical_inventory(&self) -> Result<(), StorageError> {
        live::check(self)
    }
}

/// Returns detached bytes only after isolated recovery and all parent gates.
pub(super) fn with_inspection<T>(
    lease: &OperationLease,
    validate: impl FnOnce(&InspectedRecords) -> Result<T, StorageError>,
) -> Result<T, StorageError> {
    inspect_owned_then(
        lease,
        worker::recover,
        |_, _, _| Ok(()),
        |records| validate(&records),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Inventoried,
    Copied,
    Validated,
    BeforeCleanup,
    Cleaned,
}

#[cfg(test)]
fn inspect_path<T>(
    lease: &OperationLease,
    validate: impl FnOnce(&Path) -> Result<T, StorageError>,
    hook: impl FnMut(Stage, &Path, &Path) -> Result<(), StorageError>,
) -> Result<T, StorageError> {
    inspect_owned(
        lease,
        |scratch| worker::Completion::Reaped(validate(&scratch.root.join("database"))),
        hook,
    )
}

#[cfg(test)]
fn inspect_owned<T>(
    lease: &OperationLease,
    recover: impl FnOnce(&ScratchGuard) -> worker::Completion<T>,
    hook: impl FnMut(Stage, &Path, &Path) -> Result<(), StorageError>,
) -> Result<T, StorageError> {
    inspect_owned_then(lease, recover, hook, Ok)
}

fn inspect_owned_then<T, U>(
    lease: &OperationLease,
    recover: impl FnOnce(&ScratchGuard) -> worker::Completion<T>,
    mut hook: impl FnMut(Stage, &Path, &Path) -> Result<(), StorageError>,
    validate: impl FnOnce(T) -> Result<U, StorageError>,
) -> Result<U, StorageError> {
    let source = lease.source();
    check_source_path(source)?;
    let _engine_lock = lock_source(source)?;
    let inventory = copy::Inventory::capture(source)?;
    hook(Stage::Inventoried, source, source)?;
    let scratch = ScratchGuard::create(source)?;
    let database_path = scratch.root.join("database");
    // All fallible work after allocation stays inside this closure so cleanup also
    // runs on copy, recovery, source-change and validator refusals.
    let mut reaped = true;
    let result = (|| {
        inventory.copy_to(source, &database_path)?;
        hook(Stage::Copied, source, &scratch.root)?;
        let value = match recover(&scratch) {
            worker::Completion::Reaped(result) => result?,
            worker::Completion::Uncertain(error) => {
                reaped = false;
                return Err(error);
            }
        };
        hook(Stage::Validated, source, &scratch.root)?;
        Ok(value)
    })();
    let source_check = inventory
        .recheck(source)
        .and_then(|()| check_source_path(source));
    // An unconfirmed child may still own this tree. Never remove or chmod it.
    if !reaped {
        if source_check.is_err() {
            return Err(inspection_error(
                "inspection child completion uncertain; source recheck failed",
            ));
        }
        return match result {
            Err(error) => Err(error),
            Ok(_) => Err(inspection_error("inspection child completion uncertain")),
        };
    }
    let recovered_check =
        scratch
            .check_identity()
            .and_then(|()| match fs::symlink_metadata(&database_path) {
                Ok(_) => copy::secure_recovered_tree(&database_path),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(_) => Err(inspection_error("inspection recovered tree unavailable")),
            });
    let cleanup_hook = hook(Stage::BeforeCleanup, source, &scratch.root);
    scratch.cleanup()?;
    cleanup_hook?;
    hook(Stage::Cleaned, source, &database_path)?;
    source_check?;
    recovered_check?;
    validate(result?)
}

fn check_source_path(source: &Path) -> Result<(), StorageError> {
    copy::validate_path(source)?;
    if fs::canonicalize(source).map_err(|_| inspection_error("inspection source unavailable"))?
        != source
    {
        return Err(inspection_error("unsafe inspection source path"));
    }
    Ok(())
}

fn lock_source(source: &Path) -> Result<File, StorageError> {
    let path = source.join("lock");
    let identity = copy::Identity::read(&path)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .map_err(|_| inspection_error("inspection existing engine lock unavailable"))?;
    identity.check_opened(&file, &path)?;
    file.try_lock().map_err(|error| match error {
        TryLockError::WouldBlock => inspection_error("database locked by another process"),
        TryLockError::Error(_) => inspection_error("inspection engine locking unavailable"),
    })?;
    identity.check_opened(&file, &path)?;
    Ok(file)
}

struct ScratchGuard {
    root: PathBuf,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
}

impl ScratchGuard {
    fn create(source: &Path) -> Result<Self, StorageError> {
        // Caller-controlled TMPDIR is not an authority for private inspection.
        let parent = fs::canonicalize("/tmp")
            .map_err(|_| inspection_error("inspection private root unavailable"))?;
        let mut nonce = [0_u8; 16];
        getrandom::fill(&mut nonce)
            .map_err(|_| inspection_error("inspection private identity unavailable"))?;
        let suffix: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
        let root = parent.join(format!("open-bitcoin-inspection-{suffix}"));
        if root.starts_with(source) || source.starts_with(&root) {
            return Err(inspection_error("inspection private root overlaps source"));
        }
        copy::create_private_directory(&root)?;
        // Directory creation is atomic; no payload is written before identity capture.
        let metadata = match fs::symlink_metadata(&root) {
            Ok(metadata) => metadata,
            Err(_) => {
                fs::remove_dir(&root)
                    .map_err(|_| inspection_error("inspection scratch cleanup failed"))?;
                return Err(inspection_error(
                    "inspection private root metadata unavailable",
                ));
            }
        };
        #[cfg(test)]
        OBSERVED_SCRATCH.set(Some(root.clone()));
        let guard = Self {
            root,
            #[cfg(unix)]
            device: metadata.dev(),
            #[cfg(unix)]
            inode: metadata.ino(),
        };
        #[cfg(all(test, unix))]
        if let Some(receipt) = MAYBE_ROOT_RECEIPT.take()
            && let Err(error) = tests::record_owned_scratch(source, &guard.root, &receipt)
        {
            guard.cleanup()?;
            return Err(error);
        }
        Ok(guard)
    }

    fn cleanup(self) -> Result<(), StorageError> {
        #[cfg(test)]
        if FAIL_CLEANUP.replace(false) {
            return Err(inspection_error("inspection scratch cleanup failed"));
        }
        self.check_identity()?;
        fs::remove_dir_all(&self.root)
            .map_err(|_| inspection_error("inspection scratch cleanup failed"))
    }

    fn check_identity(&self) -> Result<(), StorageError> {
        let metadata = fs::symlink_metadata(&self.root)
            .map_err(|_| inspection_error("inspection scratch cleanup failed"))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(inspection_error(
                "inspection scratch cleanup identity changed",
            ));
        }
        #[cfg(unix)]
        if metadata.dev() != self.device || metadata.ino() != self.inode {
            return Err(inspection_error(
                "inspection scratch cleanup identity changed",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
thread_local! {
    static FAIL_CLEANUP: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static OBSERVED_SCRATCH: std::cell::Cell<Option<PathBuf>> = const { std::cell::Cell::new(None) };
    static MAYBE_ROOT_RECEIPT: std::cell::Cell<Option<PathBuf>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(in crate::storage::fjall_store) fn inject_cleanup_failure_for_test() {
    FAIL_CLEANUP.set(true);
}

#[cfg(test)]
pub(in crate::storage::fjall_store) fn take_observed_scratch_for_test() -> Option<PathBuf> {
    OBSERVED_SCRATCH.take()
}

fn inspection_error(message: &'static str) -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::Runtime,
        message: message.into(),
        action: StorageRecoveryAction::Restart,
    }
}

#[cfg(all(test, unix))]
mod tests;

#[cfg(all(test, unix))]
mod phase167_tests;

#[cfg(all(test, unix))]
mod namespace_tests {
    use super::*;
    use crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS;
    use crate::storage::fjall_store::FjallNodeStore;
    use open_bitcoin_core::consensus::CanonicalChain;

    type Rows = std::collections::BTreeMap<String, std::collections::BTreeMap<Vec<u8>, Vec<u8>>>;

    fn genesis_rows(selected: CanonicalChain) -> Rows {
        let mut rows: Rows = crate::storage::canonical_bootstrap::prepare_genesis_rows(selected)
            .expect("complete canonical genesis rows")
            .into_iter()
            .map(|(name, rows)| (name.to_owned(), rows))
            .collect();
        for name in ["wallet", "metrics", "mempool", "runtime"] {
            rows.entry(name.to_owned()).or_default();
        }
        rows
    }

    fn assert_missing_refuses(source: &Path, missing: &str, history: bool) {
        // Arrange: remove one genuinely existing empty namespace, then drop all handles.
        {
            let db = fjall::Database::builder(source)
                .open()
                .expect("owned fixture mutation");
            assert!(db.keyspace_exists(missing));
            let records = InspectedRecords::read_existing(&db).expect("complete source");
            assert!(
                records
                    .maybe_records(missing)
                    .expect("present empty namespace")
                    .is_empty()
            );
            let space = db
                .keyspace(missing, fjall::KeyspaceCreateOptions::default)
                .expect("existing");
            db.delete_keyspace(space)
                .expect("controlled missing namespace");
            db.persist(fjall::PersistMode::SyncAll)
                .expect("durable missing source");
        }
        let before = copy::Inventory::capture(source).expect("full before manifest");
        let calls = BACKEND_BUILDER_CALLS.get();
        // Act: these public safe producers must refuse before an original backend.
        let result = if history {
            FjallNodeStore::reopen_canonical_history_existing(source, CanonicalChain::Regtest)
                .map(drop)
        } else {
            FjallNodeStore::reopen_canonical_genesis_existing(source, CanonicalChain::Regtest)
                .map(drop)
        };
        // Assert: old-source RED must print its actual child stage, delta1 and changed manifest.
        let after = copy::Inventory::capture(source).expect("full after manifest");
        eprintln!(
            "MISSING_NAMESPACE history={history} missing={missing} stage={:?} backend_delta={} manifest_equal={}",
            worker::HISTORY_OBSERVATION.get().0,
            BACKEND_BUILDER_CALLS.get() - calls,
            before == after
        );
        assert!(result.is_err());
        assert_eq!(BACKEND_BUILDER_CALLS.get(), calls);
        assert_eq!(before, after);
    }

    #[test]
    fn phase167_preeffect_history_worker_missing_empty_history_namespaces_refuse() {
        for missing in ["wallet", "metrics", "mempool", "runtime"] {
            phase167_tests::with_source(|source| assert_missing_refuses(source, missing, true));
        }
    }

    #[test]
    fn phase167_preeffect_history_missing_empty_genesis_namespaces_refuse() {
        for missing in ["wallet", "metrics", "mempool", "runtime"] {
            phase167_tests::with_rows_source(genesis_rows(CanonicalChain::Regtest), |source| {
                assert_missing_refuses(source, missing, false);
            });
        }
    }

    #[test]
    fn phase167_preeffect_history_complete_genesis_namespaces_keep_all_selectors() {
        for selected in [
            CanonicalChain::Mainnet,
            CanonicalChain::Testnet3,
            CanonicalChain::Signet,
            CanonicalChain::Regtest,
        ] {
            phase167_tests::with_rows_source(genesis_rows(selected), |source| {
                let opened = FjallNodeStore::reopen_canonical_genesis_existing(source, selected)
                    .expect("unchanged complete source remains supported");
                assert_eq!(opened.genesis().chain(), selected);
            });
        }
    }
}
