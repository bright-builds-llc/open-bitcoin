// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::super::{FAIL_CLEANUP, OBSERVED_SCRATCH, Stage, inspect_owned, with_inspection};
use super::*;
use std::{
    cell::Cell, collections::BTreeMap, fs::File, os::unix::fs::PermissionsExt, sync::Barrier,
};

const SPAWN: u8 = 1;
const MISSING: u8 = 2;
const PANIC: u8 = 3;
const TIMEOUT: u8 = 4;
const BAD: u8 = 5;
const TRUNCATED: u8 = 6;
const OVERSIZED: u8 = 7;
const VALID_BUT_LIVE: u8 = 8;
const NONZERO: u8 = 9;
const SIGNAL: u8 = 10;
pub(super) const UNCERTAIN: u8 = 11;
const REAP_UNCERTAIN: u8 = 12;
const NONPRIVATE: u8 = 13;
const REPLACED_RESULT: u8 = 14;
const VALID_NONZERO: u8 = 15;
const INPUT_STALLED: u8 = 16;
thread_local! { pub(super) static FAULT: Cell<u8> = const { Cell::new(0) }; }

pub(super) fn configure_command(command: &mut Command, fault: u8) {
    if fault == SPAWN {
        command.current_dir("/open-bitcoin-absent-worker-directory");
    }
    if fault == MISSING {
        command.args(["--skip", ARGS[1]]);
    }
}
pub(super) fn deadline(fault: u8, normal: Duration) -> Duration {
    if fault == VALID_BUT_LIVE || fault == INPUT_STALLED {
        return Duration::from_millis(250);
    }
    normal
}
pub(super) fn input_stalled(fault: u8) -> bool {
    fault == INPUT_STALLED
}
pub(super) fn child_before_read(job: &OwnedScratchJob) -> Result<(), StorageError> {
    match job.fault {
        PANIC => panic!("private-panic-sentinel"),
        TIMEOUT | UNCERTAIN | REAP_UNCERTAIN => thread::sleep(Duration::from_secs(60)),
        NONZERO => std::process::exit(79),
        SIGNAL => std::process::abort(),
        _ => {}
    }
    Ok(())
}
pub(super) fn child_output(job: &OwnedScratchJob, output: &mut File) -> Result<bool, StorageError> {
    match job.fault {
        BAD => output.write_all(b"bad").expect("fault write"),
        TRUNCATED => output.write_all(b"OBIR\x01\0\0\0").expect("fault write"),
        OVERSIZED => output
            .set_len(protocol::MAX_RESULT + 1)
            .expect("sparse overflow"),
        VALID_BUT_LIVE => {
            protocol::write_result(
                output,
                job,
                &InspectedRecords {
                    namespaces: BTreeMap::new(),
                },
            )?;
            output.sync_all().expect("published but live");
            thread::sleep(Duration::from_secs(60));
        }
        NONPRIVATE => {
            fs::set_permissions(job.root.join("result"), fs::Permissions::from_mode(0o644))
                .expect("fault mode")
        }
        REPLACED_RESULT => {
            fs::remove_file(job.root.join("result")).expect("unlink result");
            std::os::unix::fs::symlink("database/lock", job.root.join("result"))
                .expect("replace result");
        }
        VALID_NONZERO => {
            protocol::write_result(
                output,
                job,
                &InspectedRecords {
                    namespaces: BTreeMap::new(),
                },
            )?;
            output.sync_all().expect("complete result before refusal");
            std::process::exit(79);
        }
        _ => return Ok(false),
    }
    Ok(true)
}
pub(super) fn is_uncertain(fault: u8) -> bool {
    matches!(fault, UNCERTAIN | REAP_UNCERTAIN)
}
pub(super) fn uncertain<T>(child: &mut Child, fault: u8) -> Completion<T> {
    // The seam reports uncertainty to production orchestration, but the test
    // supervisor reaps its actual child so fixture cleanup is safe afterwards.
    child.kill().expect("test supervisor kill");
    child.wait().expect("test supervisor reap");
    Completion::Uncertain(inspection_error(if fault == UNCERTAIN {
        "inspection worker kill failed; completion uncertain"
    } else {
        "inspection worker reap failed; completion uncertain"
    }))
}

fn fixture() -> std::path::PathBuf {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    let suffix: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
    let source = fs::canonicalize(std::env::temp_dir())
        .expect("canonical tmp")
        .join(format!("obi-worker-{suffix}"));
    fs::create_dir(&source).expect("fixture");
    {
        let database = fjall::Database::builder(&source)
            .worker_threads(1)
            .open()
            .expect("fixture database");
        let rows = database
            .keyspace("rows", fjall::KeyspaceCreateOptions::default)
            .expect("rows");
        rows.insert([0, 255], b"").expect("empty value");
        rows.insert([1], b"wallet-private-sentinel").expect("value");
        database
            .keyspace("empty", fjall::KeyspaceCreateOptions::default)
            .expect("empty namespace");
        database
            .persist(fjall::PersistMode::SyncAll)
            .expect("durable");
    }
    // Real all-handle reopen folds the preallocated journal before repeatedly
    // hashing failure controls; the fresh physical sizing case stays in tests.rs.
    drop(
        fjall::Database::builder(&source)
            .worker_threads(1)
            .open()
            .expect("fixture reopen"),
    );
    source
}
fn assert_preserved_and_unlock(source: &Path, before: &copy::Inventory, lease: OperationLease) {
    assert_eq!(
        *before,
        copy::Inventory::capture(source).expect("source after")
    );
    assert!(!OBSERVED_SCRATCH.take().expect("owned root").exists());
    let engine = File::open(source.join("lock")).expect("engine");
    engine.try_lock().expect("engine released");
    drop(engine);
    assert!(OperationLease::acquire(source).is_err());
    drop(lease);
    drop(OperationLease::acquire(source).expect("application released"));
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_inspection_worker_exact_route_only() {
    // Arrange / Act / Assert
    assert!(route(ARGS.map(OsString::from).into_iter()));
    for args in [
        vec![],
        vec![ARGS[0]],
        ARGS[..3].to_vec(),
        vec![ARGS[0], ARGS[1], ARGS[2], "--test-threads=2"],
    ] {
        assert!(!route(args.into_iter().map(OsString::from)));
    }
    assert_eq!(DEADLINE, Duration::from_secs(30));
    assert!(maybe_run().is_none());
}

#[test]
fn phase166_inspection_worker_valid_records_only_after_cleanup_and_retained_locks() {
    // Arrange
    let source = fixture();
    let before = copy::Inventory::capture(&source).expect("source before");
    let lease = OperationLease::acquire(&source).expect("lease");
    let mut stages = Vec::new();
    // Act
    let records = inspect_owned(&lease, recover, |stage, source, scratch| {
        assert!(OperationLease::acquire(source).is_err());
        let lock = File::open(source.join("lock")).expect("engine lock");
        assert!(lock.try_lock().is_err());
        if stage == Stage::Cleaned {
            assert!(!scratch.exists());
        }
        stages.push(stage);
        Ok(())
    })
    .expect("isolated complete bytes");
    // Assert
    assert_eq!(stages.len(), 5);
    assert!(records.maybe_records("missing").is_none());
    assert!(records.maybe_records("empty").expect("empty").is_empty());
    let rows = records.maybe_records("rows").expect("rows");
    assert_eq!(rows.get([0, 255].as_slice()), Some(&vec![]));
    assert_eq!(
        rows.get([1].as_slice()),
        Some(&b"wallet-private-sentinel".to_vec())
    );
    assert!(rows.get([2].as_slice()).is_none());
    assert_preserved_and_unlock(&source, &before, lease);
}

#[test]
fn phase166_inspection_worker_proof_callback_keeps_locks_after_checked_cleanup() {
    // Arrange
    let source = fixture();
    let lease = OperationLease::acquire(&source).expect("lease");
    // Act
    with_inspection(&lease, |_| {
        assert!(!OBSERVED_SCRATCH.take().expect("owned root").exists());
        assert!(OperationLease::acquire(&source).is_err());
        let lock = File::open(source.join("lock")).expect("engine lock");
        assert!(lock.try_lock().is_err());
        Ok(())
    })
    .expect("proof after cleanup");
    // Assert
    let lock = File::open(source.join("lock")).expect("engine lock");
    lock.try_lock().expect("engine released after proof");
    drop(lock);
    drop(lease);
    fs::remove_dir_all(source).expect("source cleanup");
}

#[test]
fn phase166_inspection_worker_changed_source_blocks_proof_after_real_recovery() {
    // Arrange
    let source = fixture();
    let lease = OperationLease::acquire(&source).expect("lease");
    let mut called = false;
    // Act: deliberately unsupported external mutation after genuine child completion.
    let result = super::super::inspect_owned_then(
        &lease,
        recover,
        |stage, source, _| {
            if stage == Stage::Validated {
                fs::write(source.join("external-change"), b"changed").expect("fault");
            }
            Ok(())
        },
        |_| {
            called = true;
            Ok(())
        },
    );
    // Assert
    assert!(result.is_err());
    assert!(!called);
    assert!(!OBSERVED_SCRATCH.take().expect("owned root").exists());
    drop(lease);
    fs::remove_dir_all(source).expect("owned mutation fixture cleanup");
}

#[test]
fn phase166_inspection_worker_changed_root_cannot_chmod_source_or_run_proof() {
    // Arrange
    let source = fixture();
    let before = copy::Inventory::capture(&source).expect("source before");
    let lease = OperationLease::acquire(&source).expect("lease");
    let mut called = false;
    let mut maybe_saved = None;
    // Act
    let result = super::super::inspect_owned_then(
        &lease,
        recover,
        |stage, source, root| {
            if stage == Stage::Validated {
                let saved = root.with_extension("saved");
                fs::rename(root, &saved).expect("move owned root");
                std::os::unix::fs::symlink(source, root).expect("replaced root");
                maybe_saved = Some(saved);
            }
            Ok(())
        },
        |_| {
            called = true;
            Ok(())
        },
    );
    // Assert
    assert!(result.is_err());
    assert!(!called);
    assert_eq!(
        before,
        copy::Inventory::capture(&source).expect("source after")
    );
    fs::remove_file(OBSERVED_SCRATCH.take().expect("exact replaced path")).expect("unlink fixture");
    fs::remove_dir_all(maybe_saved.expect("exact moved root")).expect("owned fixture cleanup");
    drop(lease);
    fs::remove_dir_all(source).expect("source cleanup");
}

#[test]
fn phase166_inspection_worker_transport_exit_protocol_failures_block_callback() {
    for fault in [
        SPAWN,
        MISSING,
        PANIC,
        BAD,
        TRUNCATED,
        OVERSIZED,
        VALID_BUT_LIVE,
        NONZERO,
        SIGNAL,
        NONPRIVATE,
        REPLACED_RESULT,
        VALID_NONZERO,
        INPUT_STALLED,
    ] {
        // Arrange
        let source = fixture();
        let before = copy::Inventory::capture(&source).expect("before");
        let lease = OperationLease::acquire(&source).expect("lease");
        let mut called = false;
        FAULT.set(fault);
        // Act
        let result = with_inspection(&lease, |_| {
            called = true;
            Ok(())
        });
        FAULT.set(0);
        // Assert
        let error = result.expect_err("worker failure");
        if fault == INPUT_STALLED {
            assert!(format!("{error:?}").contains("deadline exceeded"));
        }
        assert!(!called, "fault {fault}");
        assert!(!format!("{error:?}").contains("wallet-private-sentinel"));
        assert!(!format!("{error:?}").contains(&source.to_string_lossy().to_string()));
        assert_preserved_and_unlock(&source, &before, lease);
    }
}

#[test]
fn phase166_inspection_worker_deadline_includes_input_and_reaps_child() {
    // Arrange
    let source = fixture();
    let before = copy::Inventory::capture(&source).expect("before");
    let lease = OperationLease::acquire(&source).expect("lease");
    FAULT.set(TIMEOUT);
    let started = Instant::now();
    let mut called = false;
    // Act
    let result = with_inspection(&lease, |_| {
        called = true;
        Ok(())
    });
    FAULT.set(0);
    // Assert
    assert!(started.elapsed() >= DEADLINE && started.elapsed() < Duration::from_secs(40));
    assert!(format!("{:?}", result.expect_err("deadline")).contains("deadline exceeded"));
    assert!(!called);
    assert_preserved_and_unlock(&source, &before, lease);
}

#[test]
fn phase166_inspection_worker_uncertain_completion_never_removes_root() {
    for fault in [UNCERTAIN, REAP_UNCERTAIN] {
        // Arrange
        let source = fixture();
        let before = copy::Inventory::capture(&source).expect("before");
        let lease = OperationLease::acquire(&source).expect("lease");
        FAULT.set(fault);
        let mut called = false;
        // Act
        let result = with_inspection(&lease, |_| {
            called = true;
            Ok(())
        });
        FAULT.set(0);
        // Assert
        assert!(format!("{:?}", result.expect_err("uncertain")).contains("completion uncertain"));
        assert!(!called);
        let root = OBSERVED_SCRATCH.take().expect("owned receipt");
        assert!(root.exists());
        assert_eq!(
            before,
            copy::Inventory::capture(&source).expect("preserved")
        );
        fs::remove_dir_all(root).expect("supervisor confirmed actual child reaped");
        drop(lease);
        fs::remove_dir_all(source).expect("source cleanup");
    }
}

#[test]
fn phase166_inspection_worker_cleanup_failure_blocks_successful_records() {
    // Arrange
    let source = fixture();
    let lease = OperationLease::acquire(&source).expect("lease");
    FAIL_CLEANUP.set(true);
    let mut called = false;
    // Act
    let result = with_inspection(&lease, |_| {
        called = true;
        Ok(())
    });
    // Assert
    assert!(format!("{:?}", result.expect_err("cleanup")).contains("cleanup failed"));
    assert!(!called);
    fs::remove_dir_all(OBSERVED_SCRATCH.take().expect("exact owned root"))
        .expect("injected leftover");
    drop(lease);
    fs::remove_dir_all(source).expect("source cleanup");
}

#[test]
fn phase166_inspection_worker_root_receipt_private_result_validation() {
    // Arrange
    let source = fixture();
    let scratch = ScratchGuard::create(&source).expect("scratch");
    let mut job = job(&scratch).expect("job");
    // Act / Assert
    job.check_root().expect("identity");
    job.result_file(false).expect("private result");
    job.inode ^= 1;
    assert!(job.check_root().is_err());
    job.inode ^= 1;
    fs::hard_link(
        scratch.root.join("result"),
        scratch.root.join("linked-result"),
    )
    .expect("hardlink control");
    assert!(job.result_file(false).is_err());
    fs::remove_file(scratch.root.join("linked-result")).expect("unlink control");
    fs::set_permissions(
        scratch.root.join("result"),
        fs::Permissions::from_mode(0o644),
    )
    .expect("nonprivate");
    assert!(job.result_file(false).is_err());
    fs::remove_file(scratch.root.join("result")).expect("remove");
    std::os::unix::fs::symlink(source.join("lock"), scratch.root.join("result")).expect("link");
    assert!(job.result_file(false).is_err());
    scratch.cleanup().expect("owned cleanup");
    fs::remove_dir_all(source).expect("source cleanup");
}

#[test]
fn phase166_inspection_worker_concurrent_roots_and_results_are_independent() {
    // Arrange
    let barrier = Barrier::new(2);
    // Act
    let roots = thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..2 {
            handles.push(scope.spawn(|| {
                let source = fixture();
                let lease = OperationLease::acquire(&source).expect("lease");
                barrier.wait();
                with_inspection(&lease, |records| {
                    assert!(records.maybe_records("rows").is_some());
                    Ok(())
                })
                .expect("concurrent recovery");
                let root = OBSERVED_SCRATCH.take().expect("root receipt");
                assert!(!root.exists());
                drop(lease);
                fs::remove_dir_all(source).expect("source cleanup");
                root
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().expect("inspection thread"))
            .collect::<Vec<_>>()
    });
    // Assert
    assert_ne!(roots[0], roots[1]);
}
