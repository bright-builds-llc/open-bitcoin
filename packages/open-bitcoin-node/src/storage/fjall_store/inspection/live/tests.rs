// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
mod unlinked;
use crate::storage::fjall_store::{
    FjallNodeStore,
    inspection::{InspectedRecords, copy},
};
use open_bitcoin_core::consensus::CanonicalChain;
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

type Rows = BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>;

fn source() -> PathBuf {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    fs::canonicalize(std::env::temp_dir())
        .expect("physical parent")
        .join(format!("phase167-live-{:x}", u128::from_le_bytes(nonce)))
}

fn with_store(test: impl FnOnce(&FjallNodeStore)) {
    let path = source();
    let (store, recovered) = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
        .expect("genuine canonical factory")
        .into_parts()
        .expect("original proved owners");
    recovered.check(&store).expect("original proof");
    TRACE.set(Trace::default());
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| test(&store)));
    MAYBE_HOOK.set(None);
    drop(recovered);
    drop(store);
    fs::remove_dir_all(path).expect("all owners released");
    if let Err(error) = outcome {
        std::panic::resume_unwind(error);
    }
}

fn rows(store: &FjallNodeStore) -> Rows {
    let records =
        InspectedRecords::read_existing(store.database()).expect("complete detached records");
    records
        .namespaces()
        .map(|name| {
            (
                name.to_owned(),
                records
                    .maybe_records(name)
                    .expect("present namespace")
                    .clone(),
            )
        })
        .collect()
}

fn assert_successful_pair() {
    TRACE.with_borrow(|trace| {
        assert!(trace.starts.len() >= 2 && trace.changed >= 1);
        assert_eq!(trace.equals.len(), 1);
        let l0 = *trace.starts.last().expect("actual successful attempt start");
        let l1 = *trace.captures.last().expect("actual successful capture");
        let l2 = trace.equals[0];
        assert_eq!(trace.rechecks.last().expect("real recheck capture").ledger, l2.ledger);
        assert!(l0.ledger.0 > 0, "failed attempts remain charged");
        for (stage, start, end) in [("successful-capture", l0, l1), ("successful-recheck-full-Eq", l1, l2)] {
            let (entries, bytes, files) = end.footprint;
            assert!(bytes > 0 && files > 0);
            assert_eq!((end.ledger.0-start.ledger.0, end.ledger.1-start.ledger.1, end.ledger.2-start.ledger.2), (entries as u64, bytes, files as u64));
            assert_eq!((end.io.0-start.io.0, end.io.1-start.io.1), (bytes, files as u64));
            eprintln!("LIVE_SUCCESSFUL_PAIR attempt={} stage={stage} Lbefore={start:?} Lafter={end:?} actual_full_hash_and_EOF=true", trace.starts.len());
        }
    });
}

fn record_action() {
    TRACE.with_borrow_mut(|trace| trace.actions.push(trace.starts.len()));
}

fn assert_actions_changed(count: usize) {
    TRACE.with_borrow(|trace| {
        assert_eq!(
            trace.actions.len(),
            count,
            "exact intended physical actions"
        );
        assert!(trace.actions.windows(2).all(|pair| pair[0] < pair[1]));
        for attempt in &trace.actions {
            assert!(
                trace.changed_attempts.contains(attempt),
                "actual action-associated Changed"
            );
        }
    });
}

fn assert_churn_exhausted(work: (u64, u64, u64)) {
    TRACE.with_borrow(|trace| {
        let starts = trace.starts.len();
        let actions = trace.actions.len();
        assert!(trace.changed > 1);
        assert!(trace.captures.is_empty() && trace.equals.is_empty());
        assert!(actions == starts || actions + 1 == starts);
        for attempt in &trace.changed_attempts {
            assert!(trace.actions.contains(attempt));
        }
        assert_eq!(
            trace.changed + 1,
            starts,
            "one terminal Refused, no continuation"
        );
        for pair in trace.starts.windows(2) {
            let before = pair[0].ledger;
            let after = pair[1].ledger;
            assert!(after.0 > before.0 && after.1 >= before.1 && after.2 >= before.2);
        }
        assert!(trace.hash_bytes > 0 && trace.hash_bytes <= work.1);
        assert!(trace.eof_reads > 0 && trace.eof_reads <= work.2);
        assert!(work.0 <= 8192 && work.1 <= 2 * copy::MAX_BYTES && work.2 <= 8192);
        let resource = if work.0 == 8192 {
            "visits"
        } else if work.2 == 8192 {
            "EOF"
        } else {
            assert!(
                2 * copy::MAX_BYTES - work.1 < 65536,
                "actual next content request cannot fit"
            );
            "requested-content"
        };
        eprintln!(
            "LIVE_DEFAULT_CHURN_EXHAUSTED resource={resource} work={work:?} starts={starts} captures={} changed={} completed_hash_bytes={} completed_EOF_reads={} all9_equal=true original_stamp_valid=true",
            trace.captures.len(),
            trace.changed,
            trace.hash_bytes,
            trace.eof_reads
        );
    });
}

#[test]
fn phase167_live_physical_work_independent_exact_limits_one_over_and_overflow() {
    // Arrange
    let mut work = copy::WorkLedger::default();
    // Act
    work.content(2 * copy::MAX_BYTES)
        .expect("exact content boundary");
    for _ in 0..2 * copy::MAX_ENTRIES {
        work.visit().expect("entry boundary");
        work.probe().expect("independent EOF boundary");
    }
    // Assert
    assert!(matches!(
        work.content(1),
        Err(copy::CopyFailure::Refused(_))
    ));
    assert!(matches!(work.visit(), Err(copy::CopyFailure::Refused(_))));
    assert!(matches!(work.probe(), Err(copy::CopyFailure::Refused(_))));
    assert_eq!(
        (work.visits, work.content, work.probes),
        (8192, 1073741824, 8192)
    );
    let mut overflow = u64::MAX;
    assert!(matches!(
        copy::WorkLedger::reserve(&mut overflow, 1, u64::MAX),
        Err(copy::CopyFailure::Refused(_))
    ));
    assert_eq!(overflow, u64::MAX);
}

#[test]
fn phase167_live_physical_work_partial_read_keeps_full_request_charge() {
    // Arrange
    let root = source();
    fs::create_dir(&root).expect("owned root");
    let path = root.join("partial");
    fs::write(&path, [7; 4096]).expect("initial file");
    MAYBE_HOOK.set(Some(Box::new(|point, path| {
        if point == Point::Opened {
            fs::OpenOptions::new()
                .write(true)
                .open(path)
                .expect("opened truncation")
                .set_len(19)
                .expect("actual partial source");
        }
    })));
    let mut work = copy::WorkLedger::default();
    // Act
    let result = copy::Inventory::capture_typed(&root, &mut Some(&mut work));
    MAYBE_HOOK.set(None);
    fs::remove_dir_all(root).expect("cleanup");
    // Assert
    assert!(matches!(result, Err(copy::CopyFailure::Changed(_))));
    assert_eq!((work.content, work.probes, work.visits), (4096, 0, 2));
    work.content(2 * copy::MAX_BYTES - 4096)
        .expect("only unused allowance");
    assert!(work.content(1).is_err(), "partial work never refunded");
}

#[test]
fn phase167_live_physical_first_native_change_then_stable() {
    with_store(|store| {
        // Arrange: retain an actually populated namespace before the original stamp.
        let before = rows(store);
        assert_eq!(before.len(), 9);
        assert!(!before["schema"].is_empty());
        let held = store.schema.clone();
        let database = store.database().clone();
        let stamp = database.stamp().expect("original stamp");
        let attempts = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&attempts);
        MAYBE_HOOK.set(Some(Box::new(move |point, root| {
            if point != Point::Captured { return; }
            let attempt = observed.fetch_add(1, Ordering::SeqCst);
            if attempt == 0 {
                held.rotate_memtable_and_wait().expect("actual populated native rotation");
                database.check_stamp(&stamp).expect("original stamp remains valid");
                record_action();
                eprintln!("LIVE_NATIVE actual_populated_rotation=true original_stamp_valid=true root={root:?}; actual_driver_classification_next=true");
            }
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        // Assert
        assert_eq!(
            rows(store),
            before,
            "complete nine maps and catalog unchanged"
        );
        eprintln!(
            "LIVE_NATIVE result={result:?} attempts={} all9_equal=true",
            attempts.load(Ordering::SeqCst)
        );
        result.expect("one native change followed by a complete stable observation");
        assert_actions_changed(1);
        assert_successful_pair();
    });
}

#[test]
fn phase167_live_physical_work_bounded_two_native_changes_stabilize() {
    with_store(|store| {
        // Arrange: both original populated memtables exist before the one stamp.
        let expected = rows(store);
        assert!(!expected["schema"].is_empty() && !expected["headers"].is_empty());
        let spaces = [store.schema.clone(), store.headers.clone()];
        let db = store.database().clone();
        let stamp = db.stamp().expect("one original stamp");
        let hook_expected = expected.clone();
        let attempts = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&attempts);
        MAYBE_HOOK.set(Some(Box::new(move |point, root| {
            if point != Point::Captured {
                return;
            }
            let index = observed.fetch_add(1, Ordering::SeqCst);
            let Some(space) = spaces.get(index) else {
                return;
            };
            space
                .rotate_memtable_and_wait()
                .expect("distinct populated rotation");
            db.check_stamp(&stamp).expect("same original stamp");
            let actual = InspectedRecords::read_existing(&db).expect("complete action records");
            assert_eq!(actual.namespaces().count(), hook_expected.len());
            for (name, records) in &hook_expected {
                assert_eq!(actual.maybe_records(name), Some(records));
            }
            record_action();
            let attempt = TRACE.with_borrow(|trace| trace.starts.len());
            eprintln!(
                "LIVE_TWO_NATIVE action={} attempt={attempt} root={root:?} actual_populated_rotation=true original_stamp_valid=true all9_equal=true; driver_classification_next=true",
                index + 1,
            );
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        // Assert
        assert_eq!(rows(store), expected);
        assert_actions_changed(2);
        eprintln!(
            "LIVE_TWO_NATIVE both_action_associated_changed=true original_stamp_valid=true all9_equal=true result={result:?}"
        );
        result.expect("two native changes followed by a complete stable pair");
        assert_successful_pair();
    });
}

#[test]
fn phase167_live_physical_root_metadata_churn_preserves_full_inventory_checks() {
    with_store(|store| {
        // Arrange
        let attempts = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&attempts);
        MAYBE_HOOK.set(Some(Box::new(move |point, root| {
            if point != Point::Captured {
                return;
            }
            if observed.fetch_add(1, Ordering::SeqCst) != 0 {
                return;
            }
            let before = copy::Inventory::capture(root).expect("full first tree");
            let anchor = Root::open(root).expect("held original root");
            let metadata = fs::metadata(root).expect("root metadata");
            fs::write(root.join("owned-churn"), b"x").expect("owned child create");
            fs::remove_file(root.join("owned-churn")).expect("owned child remove");
            fs::File::open(root)
                .expect("root fd")
                .set_times(fs::FileTimes::new().set_modified(
                    metadata.modified().expect("mtime") + std::time::Duration::from_secs(1),
                ))
                .expect("distinct actual root mtime");
            anchor
                .check(root)
                .expect("independent anchor excludes only len/mtime");
            assert!(matches!(
                before.recheck_typed(root, &mut Some(&mut copy::WorkLedger::default())),
                Err(copy::CopyFailure::Changed(_))
            ));
            record_action();
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        // Assert
        result.expect("complete equal inventory pair after root churn");
        assert_actions_changed(1);
        assert_successful_pair();
    });
}

#[test]
fn phase167_live_physical_retained_root_directory_requires_full_second_pair() {
    with_store(|store| {
        use std::os::unix::fs::MetadataExt;
        // Arrange
        let expected = rows(store);
        let db = store.database().clone();
        let stamp = db.stamp().expect("original root-change stamp");
        let mut first = true;
        MAYBE_HOOK.set(Some(Box::new(move |point, root| {
            if point != Point::Captured || !first {
                return;
            }
            first = false;
            let anchor = Root::open(root).expect("held readonly root");
            let before = fs::metadata(root).expect("before child");
            fs::create_dir(root.join("retained-owned-directory")).expect("retained child");
            let after = fs::metadata(root).expect("after child");
            assert_ne!(
                before.nlink(),
                after.nlink(),
                "actual host link delta, no portable formula"
            );
            anchor
                .check(root)
                .expect("same held fd/path/dev/inode/type/mode");
            db.check_stamp(&stamp).expect("original logical authority");
            record_action();
            eprintln!(
                "LIVE_ROOT_DIRECTORY before_links={} after_links={} child_retained=true",
                before.nlink(),
                after.nlink()
            );
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        let retained = store.datadir().join("retained-owned-directory");
        let remained = retained.is_dir();
        fs::remove_dir(retained).expect("owned child removed after complete observation");
        // Assert
        result.expect("stable complete pair after retained directory change");
        assert_actions_changed(1);
        assert!(remained);
        assert_eq!(rows(store), expected);
        TRACE.with_borrow(|trace| {
            assert!(
                !trace.root_links.is_empty(),
                "full Inventory root nlink differs"
            )
        });
        assert_successful_pair();
    });
}

#[test]
fn phase167_live_physical_descendant_directory_links_changed_and_mode_refused() {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    for individual in [false, true] {
        for change_mode in [false, true] {
            with_store(|store| {
                // Arrange
                let directory = store.datadir().join("owned-descendant");
                fs::create_dir(&directory).expect("owned descendant before stamp");
                let original_mode = fs::metadata(&directory).expect("mode").permissions();
                let selected = directory.clone();
                let expected = rows(store);
                let mut changed = false;
                MAYBE_HOOK.set(Some(Box::new(move |point, path| {
                    let boundary = if individual {
                        point == Point::DirectoryRead && path == selected
                    } else {
                        point == Point::Captured
                    };
                    if !boundary || changed {
                        return;
                    }
                    changed = true;
                    let before = fs::metadata(&selected).expect("directory before");
                    fs::create_dir(selected.join("child")).expect("actual link change");
                    if change_mode {
                        fs::set_permissions(&selected, fs::Permissions::from_mode(0o750))
                            .expect("still-readable directory chmod");
                    }
                    let after = fs::metadata(&selected).expect("directory after");
                    assert_ne!(before.nlink(), after.nlink());
                    assert_eq!((before.dev(), before.ino()), (after.dev(), after.ino()));
                    record_action();
                })));
                // Act
                let result = store.check_submission_physical_inventory();
                MAYBE_HOOK.set(None);
                fs::set_permissions(&directory, original_mode)
                    .expect("restore before assertion unwind");
                fs::remove_dir_all(directory).expect("owned paths removed");
                // Assert
                assert_eq!(result.is_err(), change_mode);
                assert_eq!(rows(store), expected);
                TRACE.with_borrow(|trace| {
                    assert_eq!(trace.actions.len(), 1);
                    if change_mode {
                        assert_eq!((trace.starts.len(), trace.changed), (1, 0));
                    }
                    if change_mode && individual {
                        assert!(trace.captures.is_empty());
                    }
                });
                if !change_mode {
                    assert_actions_changed(1);
                    assert_successful_pair();
                }
            });
        }
    }
}

#[test]
fn phase167_live_physical_same_value_and_aba_restore_all_maps_but_refuse() {
    for aba in [false, true] {
        with_store(|store| {
            // Arrange
            let expected = rows(store);
            let (key, value) = expected["schema"].first_key_value().expect("populated");
            let (key, value) = (key.clone(), value.clone());
            let held = store.schema.clone();
            let attempts = Arc::new(AtomicUsize::new(0));
            let observed = Arc::clone(&attempts);
            MAYBE_HOOK.set(Some(Box::new(move |point, _| {
                if point != Point::Captured {
                    return;
                }
                observed.fetch_add(1, Ordering::SeqCst);
                if aba {
                    held.insert(key.as_slice(), b"B").expect("actual B");
                }
                held.insert(key.as_slice(), value.as_slice())
                    .expect("actual A");
            })));
            // Act
            let result = store.check_submission_physical_inventory();
            MAYBE_HOOK.set(None);
            // Assert
            assert!(result.is_err());
            assert_eq!(rows(store), expected);
            assert_eq!(attempts.load(Ordering::SeqCst), 1);
        });
    }
}

#[test]
fn phase167_live_physical_foreign_original_stamp_refuses_before_filesystem() {
    with_store(|store| {
        with_store(|foreign| {
            // Arrange
            let stamp = foreign.database().stamp().expect("foreign genuine owner");
            // Act
            let result = check_original(store, &stamp);
            // Assert
            assert!(result.is_err());
            TRACE.with_borrow(|trace| assert!(trace.starts.is_empty()));
            assert_eq!(rows(store), rows(foreign));
        })
    });
}

mod refusals;

struct BlockedValue {
    entered: std::sync::mpsc::Sender<()>,
    resume: std::sync::mpsc::Receiver<()>,
    value: Vec<u8>,
}

impl From<BlockedValue> for fjall::UserValue {
    fn from(value: BlockedValue) -> Self {
        value
            .entered
            .send(())
            .expect("conversion inside native insert after BEGIN");
        value.resume.recv().expect("deterministic release");
        value.value.into()
    }
}

#[test]
fn phase167_live_physical_inflight_native_conversion_refuses_original_stamp() {
    with_store(|store| {
        // Arrange
        let expected = rows(store);
        let (key, value) = expected["schema"].first_key_value().expect("real schema");
        let (key, value) = (key.clone(), value.clone());
        let held = store.schema.clone();
        let (entered_tx, entered_rx) = std::sync::mpsc::channel();
        let (resume_tx, resume_rx) = std::sync::mpsc::channel();
        let mut maybe_scheduled = Some((key, value, entered_tx, resume_rx));
        let worker = Arc::new(std::sync::Mutex::new(None));
        let observed_worker = Arc::clone(&worker);
        let attempts = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&attempts);
        MAYBE_HOOK.set(Some(Box::new(move |point, _| {
            if point != Point::Captured {
                return;
            }
            observed.fetch_add(1, Ordering::SeqCst);
            let (key, value, entered, resume) =
                maybe_scheduled.take().expect("only original attempt");
            let held = held.clone();
            let handle = std::thread::spawn(move || {
                held.insert(
                    key,
                    BlockedValue {
                        entered,
                        resume,
                        value,
                    },
                )
            });
            *observed_worker.lock().expect("test handle") = Some(handle);
            entered_rx.recv().expect("actual native conversion blocked");
        })));
        // Act
        let result = store.check_submission_physical_inventory();
        MAYBE_HOOK.set(None);
        let maybe_handle = worker.lock().expect("take handle").take();
        if let Some(handle) = maybe_handle {
            resume_tx.send(()).expect("release before assertion unwind");
            handle
                .join()
                .expect("writer joined")
                .expect("same-value write completed");
        }
        // Assert: no live writer remains at any result assertion.
        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert_eq!(rows(store), expected);
    });
}

#[test]
fn phase167_live_physical_held_root_rename_missing_replacement_and_symlink_refuse() {
    for replacement in ["missing", "directory", "symlink"] {
        with_store(|store| {
            // Arrange
            let saved = store.datadir().with_extension("held");
            let observed_saved = saved.clone();
            MAYBE_HOOK.set(Some(Box::new(move |point, root| {
                if point != Point::Captured {
                    return;
                }
                fs::rename(root, &observed_saved).expect("original inode held alive");
                match replacement {
                    "directory" => fs::create_dir(root).expect("different inode"),
                    "symlink" => {
                        std::os::unix::fs::symlink(&observed_saved, root).expect("root link")
                    }
                    _ => {}
                }
            })));
            // Act
            let result = store.check_submission_physical_inventory();
            MAYBE_HOOK.set(None);
            // Restore selected path before any assertion unwinds the live owners.
            if replacement == "directory" {
                fs::remove_dir(store.datadir()).expect("empty replacement");
            }
            if replacement == "symlink" {
                fs::remove_file(store.datadir()).expect("unlink replacement");
            }
            fs::rename(saved, store.datadir()).expect("restore original owned inode");
            // Assert
            assert!(result.is_err());
        });
    }
}
