// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use std::{
    io::{Read, Seek},
    os::unix::fs::PermissionsExt,
    path::Path,
};

fn socket_at_owned_path(path: &Path) -> std::os::unix::net::UnixListener {
    use std::os::unix::fs::FileTypeExt;
    let parent = fs::canonicalize(path.parent().expect("owned socket parent"))
        .expect("canonical owned parent");
    let name = path.file_name().expect("owned socket filename");
    let selected = parent.join(name);
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("short alias nonce");
    let alias = fs::canonicalize("/tmp")
        .expect("canonical portable short alias parent")
        .join(format!("p167-s-{:x}", u128::from_le_bytes(nonce)));
    let address = alias.join(name);
    std::os::unix::fs::symlink(parent, &alias).expect("atomic unique owned alias, never overwrite");
    let result = std::os::unix::net::UnixListener::bind(address);
    let removed = fs::remove_file(alias);
    removed.expect("remove only owned alias before handling bind result");
    let listener = result.expect("actual socket bind through short owned alias");
    assert!(
        fs::symlink_metadata(selected)
            .expect("original canonical socket path")
            .file_type()
            .is_socket()
    );
    listener
}

fn with_tree(test: impl FnOnce(&Path)) {
    let root = source();
    fs::create_dir(&root).expect("owned tree");
    fs::write(root.join("payload"), b"AAAA").expect("real content");
    fs::set_permissions(root.join("payload"), fs::Permissions::from_mode(0o644))
        .expect("explicit readable baseline independent of umask");
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| test(&root)));
    MAYBE_HOOK.set(None);
    TRACE.with_borrow_mut(|trace| trace.active = false);
    fs::remove_dir_all(root).expect("owned tree cleanup");
    if let Err(error) = outcome {
        std::panic::resume_unwind(error);
    }
}

fn refused<T: std::fmt::Debug>(result: Result<T, copy::CopyFailure>) -> StorageError {
    let copy::CopyFailure::Refused(error) = result.expect_err("must refuse") else {
        panic!("unsafe or permanent error must not become Changed");
    };
    error
}

#[test]
fn phase167_live_physical_work_bounded_default_ledger_churn_refuses() {
    with_store(|store| {
        // Arrange: one genuine owner, original stamp and actual root metadata action.
        let expected = rows(store);
        let db = store.database().clone();
        let stamp = Arc::new(db.stamp().expect("original churn stamp"));
        let original_stamp = Arc::clone(&stamp);
        let root = store.datadir().to_path_buf();
        let held = fs::File::open(&root).expect("owned directory fd");
        MAYBE_HOOK.set(Some(Box::new(move |point, path| {
            if point != Point::DirectoryRead || path != root {
                return;
            }
            let before = held.metadata().expect("initial root identity");
            let modified =
                before.modified().expect("observed mtime") + std::time::Duration::from_secs(1);
            held.set_times(fs::FileTimes::new().set_modified(modified))
                .expect("actual safe root churn");
            let after = fs::metadata(path).expect("changed root metadata");
            assert_eq!(
                (before.dev(), before.ino(), before.mode()),
                (after.dev(), after.ino(), after.mode())
            );
            assert!(after.is_dir());
            assert_ne!(
                before.modified().expect("before mtime"),
                after.modified().expect("after mtime")
            );
            db.check_stamp(&original_stamp)
                .expect("same original logical stamp");
            record_action();
            TRACE.with_borrow(|trace| {
                eprintln!(
                    "LIVE_DEFAULT_CHURN action={} attempt={} before={:?} after={:?} start={:?}",
                    trace.actions.len(),
                    trace.starts.len(),
                    before.modified(),
                    after.modified(),
                    trace.starts.last()
                )
            });
        })));
        // Act: the public gate supplies its ONE default cumulative ledger.
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        let work = OBSERVED_WORK.get();
        // Assert: actual full-walker Changed results end only at an existing limit.
        assert!(
            matches!(result, Err(StorageError::BackendFailure { message, .. }) if message == "inspection work limit exceeded")
        );
        store
            .database()
            .check_stamp(&stamp)
            .expect("original stamp survives exhaustion");
        assert_eq!(rows(store), expected);
        assert_churn_exhausted(work);
    });
}

#[test]
fn phase167_live_physical_work_bounded_denied_root_precharge_has_one_final_start() {
    with_store(|store| {
        // Arrange: this terminal edge is separate from default-ledger real churn.
        let stamp = store.database().stamp().expect("genuine original stamp");
        let root = Root::open(store.datadir()).expect("held genuine root");
        let mut work = copy::WorkLedger::default();
        for _ in 0..8192 {
            work.visit().expect("existing nonrefundable reservation");
        }
        let metadata_calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&metadata_calls);
        MAYBE_HOOK.set(Some(Box::new(move |point, _| {
            if point == Point::BeforeMetadata {
                observed.fetch_add(1, Ordering::SeqCst);
            }
        })));
        TRACE.set(Trace {
            active: true,
            ..Default::default()
        });
        // Act
        let result = attempts(store, &stamp, &root, &mut work);
        MAYBE_HOOK.set(None);
        TRACE.with_borrow_mut(|trace| trace.active = false);
        // Assert: anchors may do I/O; no walker metadata/content/EOF is funded.
        root.check(store.datadir()).expect("same held root");
        store
            .database()
            .check_stamp(&stamp)
            .expect("same original stamp");
        assert!(
            matches!(result, Err(StorageError::BackendFailure { message, .. }) if message == "inspection work limit exceeded")
        );
        assert_eq!((work.visits, work.content, work.probes), (8192, 0, 0));
        assert_eq!(metadata_calls.load(Ordering::SeqCst), 0);
        TRACE.with_borrow(|trace| {
            assert_eq!(
                (
                    trace.starts.len(),
                    trace.changed,
                    trace.captures.len(),
                    trace.rechecks.len(),
                    trace.equals.len()
                ),
                (1, 0, 0, 0, 0)
            );
            assert_eq!((trace.hash_bytes, trace.eof_reads), (0, 0));
            eprintln!("LIVE_TERMINAL_PRECHARGE starts=1 unchanged_visits=8192 walker_hash_bytes=0 EOF_reads=0 anchor_checks_retained=true");
        });
    });
}

#[test]
fn phase167_live_physical_known_descendant_deletion_is_changed_and_charged() {
    with_tree(|root| {
        // Arrange: the walker already enumerated this path before the hook.
        MAYBE_HOOK.set(Some(Box::new(|point, path| {
            if point == Point::BeforeMetadata
                && path.file_name().is_some_and(|name| name == "payload")
            {
                fs::remove_file(path).expect("actual enumerated descendant deletion");
            }
        })));
        let mut work = copy::WorkLedger::default();
        // Act
        let result = copy::Inventory::capture_typed(root, &mut Some(&mut work));
        MAYBE_HOOK.set(None);
        // Assert
        let copy::CopyFailure::Changed(error) = result.expect_err("known descendant disappeared")
        else {
            panic!("known descendant must be Changed");
        };
        assert!(
            matches!(error, StorageError::BackendFailure { message, .. } if message == "inspection source metadata unavailable")
        );
        assert_eq!((work.visits, work.content, work.probes), (2, 0, 0));
    });
}

#[test]
fn phase167_live_physical_content_replacement_with_restored_metadata_changes_full_hash() {
    with_tree(|root| {
        // Arrange
        let payload = root.join("payload");
        let modified = fs::metadata(&payload)
            .expect("metadata")
            .modified()
            .expect("mtime");
        let before = copy::Inventory::capture(root).expect("full initial inventory");
        // Act
        fs::write(&payload, b"BBBB").expect("same-length replacement");
        fs::File::open(&payload)
            .expect("held file")
            .set_times(fs::FileTimes::new().set_modified(modified))
            .expect("restore mtime");
        let result = before.recheck_typed(root, &mut Some(&mut copy::WorkLedger::default()));
        let strict = before.recheck(root);
        // Assert
        assert!(matches!(result, Err(copy::CopyFailure::Changed(_))));
        assert!(
            matches!(strict, Err(StorageError::BackendFailure { message, .. }) if message == "inspection source changed")
        );
    });
}

#[test]
fn phase167_live_physical_actual_partial_read_progress_keeps_requested_charge() {
    with_tree(|root| {
        // Arrange
        let payload = root.join("payload");
        fs::write(&payload, [7; 4096]).expect("original length");
        let mut held = fs::File::open(&payload).expect("actual opened endpoint");
        fs::OpenOptions::new()
            .write(true)
            .open(&payload)
            .expect("truncate held source")
            .set_len(19)
            .expect("real truncation");
        let mut work = copy::WorkLedger::default();
        // Act
        let result = copy::read_requested(&mut held, &mut [0; 4096], &mut Some(&mut work));
        let actual_position = held.stream_position().expect("partial read position");
        drop(held);
        // Assert
        assert!(matches!(result, Err(copy::CopyFailure::Changed(_))));
        assert_eq!(
            actual_position, 19,
            "actual OS partial progress before UnexpectedEof"
        );
        assert_eq!((work.content, work.probes), (4096, 0));
        work.content(2 * copy::MAX_BYTES - 4096)
            .expect("only remaining content");
        assert!(work.content(1).is_err());
    });
}

#[test]
fn phase167_live_physical_real_opened_directory_read_is_nontransient_refusal() {
    with_tree(|root| {
        // Arrange: a directory fd opens readonly but its read is a real OS error.
        let mut held = fs::File::open(root).expect("real opened directory endpoint");
        let natural = held
            .read(&mut [0; 16])
            .expect_err("host must reject directory reads");
        let mut work = copy::WorkLedger::default();
        // Act: exactly the production requested-read implementation.
        let result = copy::read_requested(&mut held, &mut [0; 16], &mut Some(&mut work));
        drop(held);
        // Assert
        eprintln!(
            "LIVE_OS_IO kind={:?} raw_os_error={:?}",
            natural.kind(),
            natural.raw_os_error()
        );
        assert_ne!(natural.kind(), std::io::ErrorKind::UnexpectedEof);
        assert_ne!(natural.kind(), std::io::ErrorKind::NotFound);
        let error = refused(result);
        assert!(
            matches!(error, StorageError::BackendFailure { message, .. } if message == "inspection source read failed")
        );
        assert_eq!((work.content, work.probes), (16, 0));
    });
}

#[test]
fn phase167_live_physical_opened_file_safety_prioritizes_mode_links_and_type() {
    for case in ["mode", "hardlink", "symlink", "directory", "socket"] {
        with_tree(|root| {
            // Arrange
            MAYBE_HOOK.set(Some(Box::new(move |point, payload| {
                if point != Point::Opened {
                    return;
                }
                let root = payload.parent().expect("owned parent");
                match case {
                    "mode" => fs::set_permissions(payload, fs::Permissions::from_mode(0o640))
                        .expect("readable mode changed"),
                    "hardlink" => {
                        fs::hard_link(payload, root.join("linked")).expect("unsafe links")
                    }
                    _ => {
                        fs::rename(payload, root.join("saved")).expect("retain opened inode");
                        match case {
                            "symlink" => std::os::unix::fs::symlink(root.join("saved"), payload)
                                .expect("actual link"),
                            "directory" => fs::create_dir(payload).expect("type changed"),
                            _ => drop(socket_at_owned_path(payload)),
                        }
                    }
                }
            })));
            // Act
            let result =
                copy::Inventory::capture_typed(root, &mut Some(&mut copy::WorkLedger::default()));
            MAYBE_HOOK.set(None);
            let original = if root.join("saved").exists() {
                root.join("saved")
            } else {
                root.join("payload")
            };
            fs::set_permissions(original, fs::Permissions::from_mode(0o644))
                .expect("restore original mode before assertions");
            // Assert
            refused(result);
        });
    }
}

#[test]
fn phase167_live_physical_readable_common_file_chmod_refuses_without_retry() {
    with_store(|store| {
        // Arrange: file exists before original stamp; both captures stay internally stable.
        let payload = store.datadir().join("owned-readable");
        fs::write(&payload, b"original").expect("owned file");
        fs::set_permissions(&payload, fs::Permissions::from_mode(0o600))
            .expect("initial readable mode");
        let selected = payload.clone();
        let expected = rows(store);
        MAYBE_HOOK.set(Some(Box::new(move |point, _| {
            if point == Point::Captured {
                fs::set_permissions(&selected, fs::Permissions::from_mode(0o640))
                    .expect("still-readable changed mode");
            }
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        fs::set_permissions(&payload, fs::Permissions::from_mode(0o600))
            .expect("restore before assertion unwind");
        fs::remove_file(payload).expect("owned cleanup");
        // Assert
        assert!(result.is_err());
        assert_eq!(rows(store), expected);
        TRACE.with_borrow(|trace| {
            assert_eq!(
                (trace.starts.len(), trace.captures.len(), trace.changed),
                (1, 1, 0)
            )
        });
    });
}

#[test]
fn phase167_live_physical_common_safety_precedes_other_safe_content_and_directory_changes() {
    with_tree(|root| {
        // Arrange
        let before = copy::Inventory::capture(root).expect("complete first tree");
        // Act: safe differences must not conceal a readable mode change.
        fs::write(root.join("payload"), b"BBBB").expect("safe content difference");
        fs::create_dir(root.join("added-directory")).expect("directory links changed");
        fs::set_permissions(root.join("payload"), fs::Permissions::from_mode(0o640))
            .expect("readable mode difference");
        let result = before.recheck_typed(root, &mut Some(&mut copy::WorkLedger::default()));
        fs::set_permissions(root.join("payload"), fs::Permissions::from_mode(0o644))
            .expect("restore permissions");
        // Assert
        refused(result);
    });
}

#[test]
fn phase167_live_physical_initial_capture_refusals_have_one_start_zero_captures() {
    for case in [
        "permission",
        "symlink",
        "hardlink",
        "socket",
        "bytes",
        "depth",
        "entries",
    ] {
        with_store(|store| {
            // Arrange: all changes belong to this disposable proved owner's tree.
            let owned = store.datadir().join("owned-refusal");
            fs::create_dir(&owned).expect("owned fixture subtree");
            let payload = owned.join("payload");
            fs::write(&payload, b"x").expect("fixture file");
            match case {
                "permission" => fs::set_permissions(&payload, fs::Permissions::from_mode(0o0))
                    .expect("actual denied mode"),
                "symlink" => std::os::unix::fs::symlink("missing", owned.join("link"))
                    .expect("actual symlink"),
                "hardlink" => {
                    fs::hard_link(&payload, owned.join("linked")).expect("actual hardlink")
                }
                "socket" => drop(socket_at_owned_path(&owned.join("socket"))),
                "bytes" => fs::File::create(owned.join("huge"))
                    .expect("sparse fixture")
                    .set_len(copy::MAX_BYTES + 1)
                    .expect("one over byte cap"),
                "depth" => {
                    let mut nested = owned.clone();
                    for _ in 0..copy::MAX_DEPTH {
                        nested.push("d");
                        fs::create_dir(&nested).expect("depth fixture");
                    }
                }
                _ => {
                    for index in 0..copy::MAX_ENTRIES {
                        fs::write(owned.join(index.to_string()), b"").expect("entry fixture");
                    }
                }
            }
            let maybe_permission = if case == "permission" {
                Some(
                    fs::File::open(&payload)
                        .expect_err("actual OS denies read")
                        .kind(),
                )
            } else {
                None
            };
            // Act
            let result = store.check_submission_physical_inventory();
            fs::set_permissions(&payload, fs::Permissions::from_mode(0o600))
                .expect("restore before assertions");
            fs::remove_dir_all(owned).expect("owned subtree cleanup");
            // Assert
            assert!(result.is_err(), "case={case}");
            if let Some(kind) = maybe_permission {
                assert_eq!(kind, std::io::ErrorKind::PermissionDenied);
                eprintln!("LIVE_OS_PERMISSION actual_kind={kind:?}");
            }
            TRACE.with_borrow(|trace| {
                assert_eq!(
                    (trace.starts.len(), trace.captures.len(), trace.changed),
                    (1, 0, 0),
                    "case={case}"
                )
            });
        });
    }
}

#[test]
fn phase167_live_physical_missing_and_unsafe_initial_root_refuse_before_driver() {
    with_store(|store| {
        for symlink in [false, true] {
            // Arrange
            let selected = store.datadir();
            let saved = selected.with_extension("original");
            fs::rename(selected, &saved).expect("owned original root retained");
            if symlink {
                std::os::unix::fs::symlink(&saved, selected).expect("unsafe selected root");
            }
            // Act
            let result = store.check_submission_physical_inventory();
            if symlink {
                fs::remove_file(selected).expect("owned root link");
            }
            fs::rename(saved, selected).expect("restore before assertions/handle drops");
            // Assert
            assert!(result.is_err());
            TRACE.with_borrow(|trace| assert!(trace.starts.is_empty()));
        }
    });
}

#[test]
fn phase167_live_physical_path_per_source_exact_bounds_and_arithmetic_refuse() {
    // Arrange
    let mut budget = copy::Budget::default();
    // Act / Assert
    budget
        .add(copy::MAX_BYTES, copy::MAX_DEPTH)
        .expect("exact byte/depth boundary");
    assert!(budget.add(1, 0).is_err());
    assert!(budget.add(u64::MAX, 0).is_err());
    assert!(copy::Budget::default().add(0, copy::MAX_DEPTH + 1).is_err());
    let mut entries = copy::Budget::default();
    for _ in 0..copy::MAX_ENTRIES {
        entries.add(0, 0).expect("entry boundary");
    }
    assert!(entries.add(0, 0).is_err());
    assert!(copy::validate_path(Path::new(&"x".repeat(copy::MAX_PATH_BYTES))).is_ok());
    assert!(copy::validate_path(Path::new(&"x".repeat(copy::MAX_PATH_BYTES + 1))).is_err());
    assert!(copy::validate_path(Path::new("safe/../escape")).is_err());
}

#[test]
fn phase167_live_physical_cumulative_exhaustion_precedes_metadata_read_and_eof_io() {
    with_tree(|root| {
        // Arrange
        let observed = Arc::new(AtomicUsize::new(0));
        let count = Arc::clone(&observed);
        MAYBE_HOOK.set(Some(Box::new(move |point, _| {
            if point == Point::BeforeMetadata {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })));
        let mut visits = copy::WorkLedger {
            visits: 8192,
            ..Default::default()
        };
        // Act
        let result = copy::Inventory::capture_typed(root, &mut Some(&mut visits));
        MAYBE_HOOK.set(None);
        let mut held = fs::File::open(root.join("payload")).expect("real source");
        let mut content = copy::WorkLedger {
            content: 2 * copy::MAX_BYTES,
            ..Default::default()
        };
        let read = copy::read_requested(&mut held, &mut [0; 1], &mut Some(&mut content));
        let position = held.stream_position().expect("unread endpoint");
        drop(held);
        TRACE.set(Trace {
            active: true,
            ..Default::default()
        });
        let mut probes = copy::WorkLedger {
            probes: 8192,
            ..Default::default()
        };
        let eof = copy::Inventory::capture_typed(root, &mut Some(&mut probes));
        TRACE.with_borrow_mut(|trace| trace.active = false);
        // Assert
        refused(result);
        refused(read);
        refused(eof);
        assert_eq!(
            observed.load(Ordering::SeqCst),
            0,
            "visit reserved before metadata"
        );
        assert_eq!(
            position, 0,
            "failed content reservation precedes actual read"
        );
        assert_eq!(
            (visits.visits, content.content, probes.probes),
            (8192, 1073741824, 8192)
        );
        assert_eq!(
            probes.content, 4,
            "failed EOF never refunds completed content"
        );
        TRACE.with_borrow(|trace| assert_eq!((trace.hash_bytes, trace.eof_reads), (4, 0)));
        content
            .probe()
            .expect("EOF is independent, never content funding");
        assert!(content.content(1).is_err());
    });
}

#[test]
fn phase167_live_physical_strict_offline_first_native_change_returns_original_error_once() {
    with_store(|store| {
        // Arrange
        let expected = rows(store);
        let stamp = store.database().stamp().expect("original stamp");
        let held = store.schema.clone();
        let before = copy::Inventory::capture(store.datadir()).expect("complete strict before");
        held.rotate_memtable_and_wait()
            .expect("real populated native rotation");
        let traversals = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&traversals);
        let root = store.datadir().to_path_buf();
        MAYBE_HOOK.set(Some(Box::new(move |point, path| {
            if point == Point::BeforeMetadata && path == root {
                observed.fetch_add(1, Ordering::SeqCst);
            }
        })));
        // Act
        let result = before.recheck(store.datadir());
        MAYBE_HOOK.set(None);
        drop(held);
        // Assert
        eprintln!("LIVE_STRICT_FIRST_CHANGE result={result:?}");
        assert!(
            matches!(result, Err(StorageError::BackendFailure { message, .. }) if message == "inspection source changed")
        );
        assert_eq!(traversals.load(Ordering::SeqCst), 1);
        assert_eq!(rows(store), expected);
        store
            .database()
            .check_stamp(&stamp)
            .expect("original stamp remains valid");
        TRACE.with_borrow(|trace| {
            assert!(
                trace.starts.is_empty(),
                "strict path never enters live driver"
            )
        });
    });
}
