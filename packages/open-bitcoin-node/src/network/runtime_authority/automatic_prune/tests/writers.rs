// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use std::{
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

struct TempStore(PathBuf);

impl TempStore {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        Self::at_nanos(nanos)
    }

    fn at_nanos(nanos: u128) -> Self {
        let mut attempt = 0_u64;
        loop {
            let path = std::env::temp_dir().join(format!(
                "automatic-prune-writers-{}-{nanos}-{attempt}",
                std::process::id()
            ));
            // Wall-clock resolution cannot establish exclusive fixture ownership.
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    attempt = attempt.checked_add(1).expect("fixture suffix exhausted");
                }
                Err(error) => panic!("reserve test store {}: {error}", path.display()),
            }
        }
    }
}

impl Drop for TempStore {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).expect("remove closed test store");
    }
}

fn real_fixture() -> (
    TempStore,
    ManagedNetworkHandle<TestStore>,
    Arc<Mutex<Facts>>,
    crate::FjallNodeStore,
) {
    real_fixture_in(TempStore::new())
}

fn real_fixture_in(
    temp: TempStore,
) -> (
    TempStore,
    ManagedNetworkHandle<TestStore>,
    Arc<Mutex<Facts>>,
    crate::FjallNodeStore,
) {
    let store = crate::FjallNodeStore::open(&temp.0).expect("real store");
    store
        .mutate_payload_for_test(position(10).block_hash, vec![1, 2, 3], || {}, None)
        .expect("seed actual value");
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, 0);
    facts.lock().expect("facts").maybe_real_store = Some(store.clone());
    (temp, handle, facts, store)
}

#[test]
fn automatic_prune_equal_clock_fixture_cleanup_preserves_live_store() {
    // Arrange
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let (temp, _handle, _facts, store) = real_fixture_in(TempStore::at_nanos(nanos));

    // Act
    let closed_path = thread::spawn(move || {
        let (temp, handle, facts, store) = real_fixture_in(TempStore::at_nanos(nanos));
        let path = temp.0.clone();
        drop(store);
        drop(facts);
        drop(handle);
        drop(temp);
        path
    })
    .join()
    .expect("equal-clock fixture opens and closes independently");

    // Assert
    assert_ne!(closed_path, temp.0);
    assert!(!closed_path.exists());
    assert!(temp.0.is_dir());
    assert_eq!(
        store
            .retained_payload_usage(&[position(10)])
            .expect("surviving fixture remains usable")
            .current_usage_bytes,
        3
    );
}

fn paused_clone_writer(fail_after_insert: bool) {
    // Arrange
    let (_temp, handle, facts, store) = real_fixture();
    let start = Instant::now();
    tick(&handle, FlushMode::Periodic, 100).expect("first real pass");
    let first_duration = start.elapsed();
    let before = store.payload_usage_revision().expect("first revision");
    let writer_store = store.clone();
    let reader = handle.clone();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (revision_tx, revision_rx) = mpsc::channel();
    let (completed_tx, completed_rx) = mpsc::channel();
    facts.lock().expect("facts").maybe_revision_started = Some(revision_tx);
    // Act
    thread::scope(|scope| {
        let writer = scope.spawn(move || {
            writer_store.mutate_payload_for_test(
                position(10).block_hash,
                vec![4, 5, 6],
                || {
                    entered_tx.send(()).expect("guarded writer entered");
                    release_rx.recv().expect("writer released");
                },
                fail_after_insert.then(error),
            )
        });
        entered_rx.recv().expect("writer acquired and invalidated");
        scope.spawn(move || {
            completed_tx
                .send(tick(&reader, FlushMode::Periodic, 160))
                .expect("reader result");
        });
        revision_rx
            .recv()
            .expect("owner began synchronized revision read");
        let blocked = completed_rx.try_recv().is_err();
        let scans_while_paused = facts.lock().expect("facts").scans;
        release_tx.send(()).expect("release before assertions");
        let writer_result = writer.join().expect("writer thread");
        completed_rx
            .recv()
            .expect("owner completes")
            .expect("fresh measurement");
        // Assert
        assert!(blocked);
        assert_eq!(scans_while_paused, 1);
        assert_eq!(writer_result.is_err(), fail_after_insert);
    });
    let measured = store
        .retained_payload_usage(&[position(10)])
        .expect("finished facts");
    assert_ne!(measured.revision, before);
    assert_eq!(measured.current_usage_bytes, 3);
    assert_eq!(measured.height_sizes.len(), 1);
    assert_eq!(facts.lock().expect("facts").scans, 2);
    eprintln!(
        "automatic real scan: active positions=5, payload keys=1, candidate keys={}, first pass={first_duration:?}, error={fail_after_insert}",
        measured.height_sizes.len()
    );
}

#[test]
fn automatic_prune_paused_equal_sized_clone_writer_requires_same_tip_fresh_measurement() {
    paused_clone_writer(false);
}

#[test]
fn automatic_prune_clone_writer_partial_effect_error_cannot_reuse_old_measurement() {
    paused_clone_writer(true);
}

#[test]
fn automatic_prune_overlapping_clone_writers_cannot_publish_false_idle() {
    // Arrange
    let (_temp, handle, facts, store) = real_fixture();
    tick(&handle, FlushMode::Periodic, 100).expect("first pass");
    let writer_one = store.clone();
    let writer_two = store.clone();
    let reader = handle.clone();
    let (entered_one_tx, entered_one_rx) = mpsc::channel();
    let (release_one_tx, release_one_rx) = mpsc::channel();
    let (entered_two_tx, entered_two_rx) = mpsc::channel();
    let (attempted_two_tx, attempted_two_rx) = mpsc::channel();
    let (release_two_tx, release_two_rx) = mpsc::channel();
    let (revision_tx, revision_rx) = mpsc::channel();
    let (completed_tx, completed_rx) = mpsc::channel();
    // Act
    thread::scope(|scope| {
        scope.spawn(move || {
            writer_one
                .mutate_payload_for_test(
                    position(10).block_hash,
                    vec![7, 8, 9],
                    || {
                        entered_one_tx.send(()).expect("first writer entered");
                        release_one_rx.recv().expect("first released");
                    },
                    None,
                )
                .expect("first write")
        });
        entered_one_rx.recv().expect("first guard acquired");
        scope.spawn(move || {
            attempted_two_tx.send(()).expect("second attempts");
            writer_two
                .mutate_payload_for_test(
                    position(10).block_hash,
                    vec![10, 11, 12],
                    || {
                        entered_two_tx.send(()).expect("second entered");
                        release_two_rx.recv().expect("second released");
                    },
                    None,
                )
                .expect("second write");
        });
        attempted_two_rx
            .recv()
            .expect("second attempts acquisition");
        let second_blocked = entered_two_rx.try_recv().is_err();
        release_one_tx.send(()).expect("first release");
        entered_two_rx.recv().expect("second now holds guard");
        facts.lock().expect("facts").maybe_revision_started = Some(revision_tx);
        scope.spawn(move || {
            completed_tx
                .send(tick(&reader, FlushMode::Periodic, 160))
                .expect("reader result")
        });
        revision_rx.recv().expect("owner revision read starts");
        let reader_blocked = completed_rx.try_recv().is_err();
        release_two_tx.send(()).expect("release before assertions");
        completed_rx
            .recv()
            .expect("reader completes")
            .expect("fresh owner pass");
        // Assert
        assert!(second_blocked);
        assert!(reader_blocked);
    });
    assert_eq!(facts.lock().expect("facts").scans, 2);
    assert_eq!(
        store
            .retained_payload_usage(&[position(10)])
            .expect("latest values")
            .current_usage_bytes,
        3
    );
}

#[test]
fn automatic_prune_poisoned_real_payload_guard_refuses_idle_and_delete() {
    // Arrange
    let (_temp, handle, facts, store) = real_fixture();
    tick(&handle, FlushMode::Periodic, 100).expect("first pass");
    let writer_store = store.clone();
    let panic = thread::spawn(move || {
        writer_store.mutate_payload_for_test(
            position(10).block_hash,
            vec![4, 5, 6],
            || panic!("poison actual guarded writer"),
            None,
        )
    })
    .join();
    // Act
    let result = tick(&handle, FlushMode::Periodic, 160);
    // Assert
    assert!(panic.is_err());
    assert!(result.is_err());
    assert_eq!(facts.lock().expect("facts").scans, 1);
    assert!(facts.lock().expect("facts").deletes.is_empty());
}
