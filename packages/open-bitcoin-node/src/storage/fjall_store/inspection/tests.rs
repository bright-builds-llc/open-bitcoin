// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{copy as inventory, *};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::{fs, path::PathBuf};

#[test]
fn phase166_inspection_foundation_valid_records_reach_validator_after_cleanup() {
    // Arrange
    let source = raw_source();
    let before = inventory::Inventory::capture(&source).expect("source before");
    let lease = OperationLease::acquire(&source).expect("source lease");
    let mut called = false;
    // Act
    let result = with_inspection(&lease, |_| {
        called = true;
        Ok(())
    });
    let after = inventory::Inventory::capture(&source).expect("source after");
    let scratch = OBSERVED_SCRATCH.take().expect("owned copy");
    drop(lease);
    fs::remove_dir_all(&source).expect("fixture cleanup");
    // Assert
    assert!(
        called,
        "isolated records must reach validator after parent gates"
    );
    assert!(result.is_ok());
    assert_eq!(before, after);
    assert!(!scratch.exists());
}

mod copy;

#[derive(serde::Serialize, serde::Deserialize)]
struct OwnedRoots {
    source: PathBuf,
    scratch: PathBuf,
    device: u64,
    inode: u64,
}

pub(super) fn record_owned_scratch(
    source: &Path,
    scratch: &Path,
    receipt: &Path,
) -> Result<(), StorageError> {
    use std::io::Write;
    let metadata = fs::symlink_metadata(scratch)
        .map_err(|_| inspection_error("test receipt metadata unavailable"))?;
    let bytes = serde_json::to_vec(&OwnedRoots {
        source: source.to_path_buf(),
        scratch: scratch.to_path_buf(),
        device: metadata.dev(),
        inode: metadata.ino(),
    })
    .map_err(|_| inspection_error("test receipt encoding failed"))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(receipt)
        .map_err(|_| inspection_error("test receipt creation failed"))?;
    file.write_all(&bytes)
        .and_then(|()| file.sync_all())
        .map_err(|_| inspection_error("test receipt write failed"))
}

#[test]
fn phase166_inspection_malformed_engine_child() {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let Some(source) = std::env::var_os("PHASE166_INSPECTION_CHILD_SOURCE") else {
        return;
    };
    let receipt =
        std::env::var_os("PHASE166_INSPECTION_CHILD_RECEIPT").expect("controlled receipt");
    let lease = OperationLease::acquire(Path::new(&source)).expect("child lease");
    MAYBE_ROOT_RECEIPT.set(Some(PathBuf::from(receipt)));
    let outcome = catch_unwind(AssertUnwindSafe(|| with_inspection(&lease, |_| Ok(()))));
    assert!(
        matches!(outcome, Ok(Err(_))),
        "malformed input must return refusal"
    );
    let error = outcome
        .expect("inspecting parent survived")
        .expect_err("malformed refused");
    assert!(
        matches!(error, StorageError::BackendFailure { message, .. } if message == "inspection isolated recovery refused")
    );
    assert!(!OBSERVED_SCRATCH.take().expect("owned scratch").exists());
}

#[test]
fn phase166_inspection_raw_pin_characterization_child() {
    // Arrange: this exact child is the sole raw scratch recovery caller.
    let Some(source) = std::env::var_os("PHASE166_INSPECTION_CHILD_SOURCE") else {
        return;
    };
    let receipt =
        std::env::var_os("PHASE166_INSPECTION_CHILD_RECEIPT").expect("controlled receipt");
    let lease = OperationLease::acquire(Path::new(&source)).expect("child lease");
    MAYBE_ROOT_RECEIPT.set(Some(PathBuf::from(receipt)));
    // Act / Assert: the live parent records the actual abort, then owns cleanup.
    let result = inspect_path(&lease, worker::raw_recover, |_, _, _| Ok(()));
    assert!(result.is_err(), "malformed raw pin must not return records");
}

#[test]
fn phase166_inspection_malformed_engine_names_refuse_without_panics_or_retained_scratch() {
    // Arrange / Act: refusal alone cannot stand in for successful isolated reading.
    let source = raw_source();
    let lease = OperationLease::acquire(&source).expect("valid control lease");
    let valid = with_inspection(&lease, |_| Ok(())).is_ok();
    drop(lease);
    fs::remove_dir_all(source).expect("valid control cleanup");
    // Assert: this positive control must recover before malformed refusals count.
    assert!(
        valid,
        "valid control requires process-isolated record recovery"
    );
    assert_malformed_children("phase166_inspection_malformed_engine_child", false);
}

#[test]
fn phase166_inspection_raw_pin_abort_characterization() {
    assert_malformed_children("phase166_inspection_raw_pin_characterization_child", true);
}

fn assert_malformed_children(child: &str, characterize_abort: bool) {
    #[cfg(target_os = "linux")]
    use std::os::unix::ffi::OsStringExt;
    use std::{ffi::OsString, os::unix::process::ExitStatusExt, process::Command};
    let mut observations = Vec::new();
    let names = [
        ("integer", OsString::from("not-an-integer")),
        ("overflow", OsString::from("18446744073709551616")),
    ];
    // APFS rejects non-UTF8 names during fixture creation; Linux can exercise
    // this distinct backend parser path without claiming it ran on macOS.
    #[cfg(target_os = "linux")]
    let names = names
        .into_iter()
        .chain(std::iter::once(("utf8", OsString::from_vec(vec![0xff]))));
    for (kind, name) in names {
        // Arrange: a live parent owns the receipt directory and a closed valid source.
        let parent = fixture();
        fs::set_permissions(&parent, fs::Permissions::from_mode(0o700))
            .expect("private test parent");
        let source = parent.join("source");
        fs::create_dir(&source).expect("child source");
        seed_raw_source(&source);
        fs::create_dir(source.join("keyspaces").join(name)).expect("bounded malformed directory");
        let before = inventory::Inventory::capture(&source).expect("source before");
        let receipt = parent.join("owned-root.json");
        // Act: process completion is the barrier, including an aborting backend.
        let child_name = format!("storage::fjall_store::inspection::tests::{child}");
        let output = Command::new(std::env::current_exe().expect("test executable"))
            .args(["--exact", &child_name, "--nocapture"])
            .env("PHASE166_INSPECTION_CHILD_SOURCE", &source)
            .env("PHASE166_INSPECTION_CHILD_RECEIPT", &receipt)
            .output()
            .expect("controlled child completion");
        let receipt_meta = fs::symlink_metadata(&receipt).expect("receipt exists");
        assert!(
            receipt_meta.is_file()
                && receipt_meta.mode() & 0o777 == 0o600
                && receipt_meta.len() < 16_384
        );
        let owned: OwnedRoots =
            serde_json::from_slice(&fs::read(&receipt).expect("bounded receipt"))
                .expect("owned receipt");
        assert_eq!(
            owned.source,
            fs::canonicalize(&source).expect("source identity")
        );
        let retained = owned.scratch.exists();
        let preserved = before == inventory::Inventory::capture(&source).expect("source after");
        eprintln!(
            "phase166 malformed {kind}: child_success={}, signal={:?}, scratch_retained={retained}, source_preserved={preserved}",
            output.status.success(),
            output.status.signal()
        );
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        // Cleanup only the exact private inode recorded by this controlled invocation.
        if retained {
            let metadata = fs::symlink_metadata(&owned.scratch).expect("recorded root");
            assert!(metadata.is_dir() && metadata.mode() & 0o777 == 0o700);
            assert_eq!(
                (metadata.dev(), metadata.ino()),
                (owned.device, owned.inode)
            );
            assert_eq!(
                owned.scratch.parent(),
                Some(
                    fs::canonicalize("/tmp")
                        .expect("fixed scratch parent")
                        .as_path()
                )
            );
            fs::remove_dir_all(&owned.scratch).expect("owned aborted scratch cleanup");
        }
        assert!(OperationLease::acquire(&source).is_ok());
        let engine = File::open(source.join("lock")).expect("source engine lock");
        engine.try_lock().expect("engine immediately reacquired");
        drop(engine);
        fs::remove_dir_all(parent).expect("owned fixture cleanup");
        observations.push(if characterize_abort {
            (output.status.signal() == Some(6), retained, preserved)
        } else {
            (output.status.success(), !retained, preserved)
        });
    }
    // Assert
    assert!(
        observations
            .iter()
            .all(|(refused, cleaned, preserved)| *refused && *cleaned && *preserved),
        "child outcome must match the explicitly selected behavior or raw-pin characterization"
    );
}

fn fixture() -> PathBuf {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("fixture nonce");
    let name: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
    let root = fs::canonicalize(std::env::temp_dir()).expect("canonical test root");
    let source = root.join(format!("obi-{name}"));
    fs::create_dir(&source).expect("fixture directory");
    source
}

#[test]
fn phase166_inspection_foundation_reads_only_existing_logical_namespaces_and_records() {
    // Arrange
    let source = fixture();
    let records = {
        let db = fjall::Database::builder(&source)
            .open()
            .expect("raw source fixture");
        let present = db
            .keyspace("schema", fjall::KeyspaceCreateOptions::default)
            .expect("fixture namespace");
        present.insert("schema_version", b"1").expect("fixture row");
        db.keyspace("empty", fjall::KeyspaceCreateOptions::default)
            .expect("empty namespace");
        db.persist(fjall::PersistMode::SyncAll)
            .expect("fixture durability");
        // Act: enumerate this known-valid fixture without any recovery/open.
        InspectedRecords::read_existing(&db).expect("existing bytes")
    };
    // Assert: these detached records include no synthesized namespace or row.
    {
        assert_eq!(
            records.namespaces().collect::<Vec<_>>(),
            vec!["empty", "schema"]
        );
        assert!(
            records
                .maybe_records("empty")
                .expect("genuinely empty namespace")
                .is_empty()
        );
        assert!(records.maybe_records("block_index").is_none());
        let schema = records.maybe_records("schema").expect("present schema");
        assert_eq!(schema.len(), 1);
        assert_eq!(
            schema.get(b"schema_version".as_slice()),
            Some(&b"1".to_vec())
        );
    }
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

fn raw_source() -> PathBuf {
    let source = fixture();
    seed_raw_source(&source);
    source
}

fn seed_raw_source(source: &Path) {
    {
        let db = fjall::Database::builder(source)
            .worker_threads(1)
            .open()
            .expect("raw fixture");
        let keyspace = db
            .keyspace("runtime", fjall::KeyspaceCreateOptions::default)
            .expect("runtime");
        keyspace
            .insert("secret", b"wallet-private-sentinel")
            .expect("sentinel");
        db.persist(fjall::PersistMode::SyncAll).expect("durability");
    }
}

#[test]
fn phase166_inspection_foundation_engine_and_app_locks_cover_copy_cleanup() {
    // Arrange
    let source = raw_source();
    let lease = OperationLease::acquire(&source).expect("lease");
    let mut observed = Vec::new();
    // Act
    let result = inspect_path(
        &lease,
        |scratch| {
            assert!(
                inventory::Inventory::capture(scratch)
                    .expect("copied tree")
                    .private_modes()
            );
            Ok(())
        },
        |stage, source, scratch| {
            if stage == Stage::Inventoried {
                return Ok(());
            }
            let engine = File::open(source.join("lock")).expect("source lock");
            assert!(matches!(engine.try_lock(), Err(TryLockError::WouldBlock)));
            std::thread::scope(|scope| {
                scope
                    .spawn(|| {
                        assert!(OperationLease::acquire(source).is_err());
                        assert!(super::super::FjallNodeStore::open(source).is_err());
                    })
                    .join()
                    .expect("concurrent contender");
            });
            if stage == Stage::Cleaned {
                assert!(!scratch.exists());
            }
            observed.push(stage);
            Ok(())
        },
    );
    // Assert
    result.expect("bounded recovery");
    assert_eq!(observed.len(), 4);
    let engine = File::open(source.join("lock")).expect("source lock");
    engine.try_lock().expect("engine released after cleanup");
    assert!(OperationLease::acquire(&source).is_err());
    drop(engine);
    drop(lease);
    let reopened =
        super::super::FjallNodeStore::open(&source).expect("admission lease now available");
    drop(reopened);
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_foundation_failure_paths_remove_owned_scratch_and_preserve_source() {
    // Arrange
    let source = raw_source();
    let before = inventory::Inventory::capture(&source).expect("before");
    let lease = OperationLease::acquire(&source).expect("lease");
    for failure in ["read", "write", "validation", "final-check", "cleanup-hook"] {
        let mut maybe_scratch = None;
        // Act
        let result = inspect_path(
            &lease,
            |_| {
                if failure == "validation" {
                    return Err(inspection_error("inspection validation refused"));
                }
                Ok(())
            },
            |stage, _, scratch| {
                if stage == Stage::Inventoried {
                    match failure {
                        "read" => inventory::COPY_IO_FAULT.set(Some(inventory::CopyIoFault::Read)),
                        "write" => {
                            inventory::COPY_IO_FAULT.set(Some(inventory::CopyIoFault::Write))
                        }
                        _ => {}
                    }
                }
                if stage == Stage::BeforeCleanup {
                    maybe_scratch = Some(scratch.to_path_buf());
                }
                if stage == Stage::Copied {
                    maybe_scratch = Some(scratch.to_path_buf());
                }
                if failure == "final-check" && stage == Stage::Validated {
                    return Err(inspection_error("inspection injected read failure"));
                }
                if failure == "cleanup-hook" && stage == Stage::BeforeCleanup {
                    return Err(inspection_error(
                        "inspection injected cleanup boundary failure",
                    ));
                }
                Ok(())
            },
        );
        // Assert
        assert!(result.is_err(), "{failure}");
        assert!(!maybe_scratch.expect("scratch").exists(), "{failure}");
        assert_eq!(
            before,
            inventory::Inventory::capture(&source).expect("after")
        );
    }
    drop(lease);
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_foundation_cleanup_failure_is_visible_and_blocks_result() {
    // Arrange
    let source = raw_source();
    let before = inventory::Inventory::capture(&source).expect("before");
    let lease = OperationLease::acquire(&source).expect("lease");
    let mut maybe_scratch = None;
    // Act
    let result = inspect_path(
        &lease,
        |_| Ok(()),
        |stage, _, scratch| {
            if stage == Stage::BeforeCleanup {
                maybe_scratch = Some(scratch.to_path_buf());
                FAIL_CLEANUP.set(true);
            }
            Ok(())
        },
    );
    // Assert
    let error = result.expect_err("cleanup must fail success");
    assert!(
        matches!(error, StorageError::BackendFailure { ref message, .. } if message == "inspection scratch cleanup failed")
    );
    let scratch = maybe_scratch.expect("owned scratch");
    assert!(scratch.is_dir());
    assert!(!format!("{error:?}").contains("wallet-private-sentinel"));
    assert!(!format!("{error:?}").contains(&source.to_string_lossy().to_string()));
    assert_eq!(
        before,
        inventory::Inventory::capture(&source).expect("after")
    );
    assert!(OperationLease::acquire(&source).is_err());
    fs::remove_dir_all(scratch).expect("test removes injected leftover");
    drop(lease);
    fs::remove_dir_all(&source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_foundation_source_path_replacement_and_unsafe_lock_refuse() {
    // Arrange
    let source = fixture();
    let lease = OperationLease::acquire(&source).expect("lease");
    std::os::unix::fs::symlink("absent", source.join("lock")).expect("lock link");
    // Act / Assert
    assert!(with_inspection(&lease, |_| Ok(())).is_err());
    fs::remove_file(source.join("lock")).expect("remove link");
    fs::remove_dir(&source).expect("remove directory");
    std::os::unix::fs::symlink("absent", &source).expect("root link");
    assert!(with_inspection(&lease, |_| Ok(())).is_err());
    fs::remove_file(&source).expect("fixture cleanup");
    drop(lease);
}

#[test]
fn phase166_inspection_foundation_cleanup_never_follows_replaced_owned_root() {
    // Arrange
    let source = fixture();
    fs::write(source.join("private"), b"preserve source").expect("source");
    let before = inventory::Inventory::capture(&source).expect("before");
    let scratch = ScratchGuard::create(&source).expect("owned root");
    let root = scratch.root.clone();
    let saved = root.with_extension("saved");
    fs::rename(&root, &saved).expect("replace owned identity");
    std::os::unix::fs::symlink(&source, &root).expect("replacement points to source");
    // Act
    let result = scratch.cleanup();
    // Assert
    assert!(result.is_err());
    assert_eq!(
        before,
        inventory::Inventory::capture(&source).expect("after")
    );
    assert!(
        fs::symlink_metadata(&root)
            .expect("replacement remains")
            .is_symlink()
    );
    fs::remove_file(root).expect("remove fixture replacement");
    fs::remove_dir(saved).expect("remove relocated owned fixture");
    fs::remove_dir_all(source).expect("source cleanup");
}

#[test]
fn phase166_inspection_foundation_canonical_genesis_basic_payload_fits_fresh_and_reopened_bounds() {
    use crate::storage::{
        FjallNodeStore, PersistMode, coins_codec, filter_index::StoredFilterRecord,
    };
    use open_bitcoin_core::{
        chainstate::{
            BasicFilterInputs, ChainPosition, FilterCheckpoint, IndexPrefix,
            VerifiedChainstateFence,
        },
        consensus::{CanonicalChain, chain_spec::CanonicalGenesisProof},
    };
    // Arrange: real canonical payload and actual BASIC storage, solely a sizing
    // fixture; this raw fixture is not claimed to be admitted canonical storage.
    let source = fixture();
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("genesis");
    let position = ChainPosition::genesis(
        proof.header().clone(),
        proof.work(),
        i64::from(proof.header().time),
    )
    .expect("position");
    let store = FjallNodeStore::open(&source).expect("fixture store");
    store
        .save_block(proof.block(), PersistMode::Sync)
        .expect("body");
    let headers =
        open_bitcoin_network::HeaderStore::from_canonical_genesis(&proof).expect("headers");
    store
        .save_header_entries(
            &headers.entries().cloned().collect::<Vec<_>>(),
            PersistMode::Sync,
        )
        .expect("header rows");
    store
        .save_chain_meta(std::slice::from_ref(&position), PersistMode::Sync)
        .expect("chain meta");
    store
        .coins_view()
        .write_raw_bytes(
            &coins_codec::encode_best_block_key(),
            coins_codec::encode_best_block_value(proof.hash()),
        )
        .expect("B fixture");
    let positions = [position];
    let fence = VerifiedChainstateFence::new(Some(proof.hash()), Some(&positions)).expect("fence");
    store
        .initialize_basic_filter_state(&fence)
        .expect("BASIC initialize");
    let input = BasicFilterInputs::from_historical(proof.block(), &positions[0], None)
        .expect("BASIC input");
    let record = StoredFilterRecord::generate(&input, &positions[0], None).expect("BASIC record");
    let checkpoint = FilterCheckpoint::new(IndexPrefix::Committed(record.identity()));
    let work = store
        .maybe_basic_filter_work(&fence)
        .expect("work")
        .expect("active work");
    store
        .publish_basic_filter_checkpoint(
            &work,
            &fence,
            checkpoint,
            checkpoint.input_protection(),
            &[record],
        )
        .expect("BASIC publish");
    drop(work);
    drop(store);
    for observation in ["fresh", "reopened"] {
        // Act
        let inventory = inventory::Inventory::capture(&source).expect("bounded complete inventory");
        let lease = OperationLease::acquire(&source).expect("lease");
        inspect_path(
            &lease,
            |scratch| {
                assert!(inventory::Inventory::capture(scratch)?.private_modes());
                Ok(())
            },
            |_, _, _| Ok(()),
        )
        .expect("copy actual payload");
        drop(lease);
        // Assert
        let (entries, bytes) = inventory.size();
        assert!(entries < inventory::MAX_ENTRIES && bytes < inventory::MAX_BYTES);
        assert_eq!(
            inventory,
            inventory::Inventory::capture(&source).expect("preserved")
        );
        eprintln!(
            "phase166 inspection {observation} genesis+BASIC: {entries} entries, {bytes} bytes"
        );
        if observation == "fresh" {
            let reopened = FjallNodeStore::open(&source).expect("actual reopened raw fixture");
            assert!(
                reopened
                    .load_basic_filter_record(proof.hash())
                    .expect("BASIC row")
                    .is_some()
            );
            drop(reopened);
        }
    }
    fs::remove_dir_all(source).expect("fixture cleanup");
}
