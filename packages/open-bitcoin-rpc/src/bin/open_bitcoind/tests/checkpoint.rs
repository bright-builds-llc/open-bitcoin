// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::sync::{Arc, Barrier, Mutex};

use open_bitcoin_mempool::PolicyTime;
use open_bitcoin_node::{
    ManagedNetworkAuthorityError, StorageError, StorageNamespace, StorageRecoveryAction,
    network::{MempoolCheckpointError, MempoolCheckpointOutcome},
    storage::fjall_store::{MempoolSnapshotDecodeLimits, SnapshotWriteExecutionError},
};

use super::*;
use crate::checkpoint::{
    CheckpointDrive, CheckpointWait, DaemonCheckpointError, MEMPOOL_CHECKPOINT_INTERVAL,
    checkpoint_failure_class, checkpoint_worker_loop, checkpoint_worker_loop_with,
    settle_and_mark_clean, start_mempool_checkpoint_worker,
};

#[test]
fn durable_worker_requests_one_checkpoint_at_private_five_minute_boundary() {
    // Arrange
    let waits = Arc::new(Mutex::new(Vec::new()));
    let observed_waits = Arc::clone(&waits);
    let drives = Arc::new(Mutex::new(Vec::new()));
    let observed_drives = Arc::clone(&drives);
    let mut wait_count = 0;

    // Act
    let outcome = checkpoint_worker_loop_with(
        move |duration| {
            observed_waits
                .lock()
                .expect("wait observations")
                .push(duration);
            wait_count += 1;
            if wait_count == 1 {
                CheckpointWait::Elapsed
            } else {
                CheckpointWait::Shutdown
            }
        },
        move |drive| {
            observed_drives
                .lock()
                .expect("drive observations")
                .push(drive);
            Ok::<_, ()>(drive)
        },
        |_| panic!("successful cadence must not report a periodic failure"),
    )
    .expect("checkpoint loop");

    // Assert
    assert_eq!(
        waits.lock().expect("wait observations").as_slice(),
        [MEMPOOL_CHECKPOINT_INTERVAL, MEMPOOL_CHECKPOINT_INTERVAL]
    );
    assert_eq!(
        drives.lock().expect("drive observations").as_slice(),
        [CheckpointDrive::Periodic, CheckpointDrive::Shutdown]
    );
    assert_eq!(outcome, CheckpointDrive::Shutdown);
}

#[test]
fn no_store_runtime_starts_no_checkpoint_worker() {
    // Arrange
    let data_dir = temp_store_path("checkpoint-no-store");
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("checkpoint store");
    let runtime =
        DurableSyncRuntime::open(store, SyncRuntimeConfig::default()).expect("checkpoint runtime");

    // Act
    let maybe_worker = start_mempool_checkpoint_worker(runtime.network_handle(), None);

    // Assert
    assert!(maybe_worker.is_none());
    remove_dir_if_exists(&data_dir);
}

#[test]
fn periodic_failure_is_reported_once_and_shutdown_settlement_still_runs() {
    // Arrange
    let failures = Arc::new(Mutex::new(Vec::new()));
    let observed_failures = Arc::clone(&failures);
    let mut wait_count = 0;
    let mut drive_count = 0;

    // Act
    let outcome = checkpoint_worker_loop_with(
        move |_| {
            wait_count += 1;
            if wait_count == 1 {
                CheckpointWait::Elapsed
            } else {
                CheckpointWait::Shutdown
            }
        },
        move |drive| {
            drive_count += 1;
            if drive == CheckpointDrive::Periodic {
                return Err("execution");
            }
            Ok(drive_count)
        },
        move |error| {
            observed_failures
                .lock()
                .expect("failure observations")
                .push(*error);
        },
    )
    .expect("shutdown settlement");

    // Assert
    assert_eq!(outcome, 2);
    assert_eq!(
        failures.lock().expect("failure observations").as_slice(),
        ["execution"]
    );
}

#[test]
fn producer_is_quiesced_before_shutdown_capture_and_clean_marker() {
    // Arrange
    let data_dir = temp_store_path("checkpoint-shutdown-order");
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("checkpoint store");
    let runtime = DurableSyncRuntime::open(store.clone(), SyncRuntimeConfig::default())
        .expect("checkpoint runtime");
    let handle = runtime.network_handle();
    let producer_entered = Arc::new(Barrier::new(2));
    let release_producer = Arc::new(Barrier::new(2));
    let producer_generation = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let worker_generation = Arc::clone(&producer_generation);
    let worker_entered = Arc::clone(&producer_entered);
    let worker_release = Arc::clone(&release_producer);
    let producer = std::thread::spawn(move || {
        worker_entered.wait();
        worker_release.wait();
        worker_generation.store(1, std::sync::atomic::Ordering::SeqCst);
    });
    producer_entered.wait();
    let checkpoint_worker = start_mempool_checkpoint_worker(handle.clone(), Some(store.clone()))
        .expect("durable checkpoint worker");

    // Act
    release_producer.wait();
    producer.join().expect("producer join");
    assert_eq!(
        producer_generation.load(std::sync::atomic::Ordering::SeqCst),
        1
    );
    checkpoint_worker
        .shutdown_and_mark_clean()
        .expect("clean checkpoint shutdown");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(301), 1)
        .expect("checkpoint evidence");
    let metadata = store
        .load_runtime_metadata()
        .expect("runtime metadata")
        .expect("clean metadata");

    // Assert
    assert_eq!(evidence.maybe_last_durable_generation, Some(0));
    assert_eq!(
        evidence.maybe_last_durable_generation,
        Some(evidence.current_generation)
    );
    assert!(metadata.last_clean_shutdown);
    remove_dir_if_exists(&data_dir);
}

#[test]
fn checkpoint_failure_prevents_clean_marker() {
    // Arrange
    let calls = Arc::new(Mutex::new(Vec::new()));
    let observed_calls = Arc::clone(&calls);
    let marker_calls = Arc::clone(&calls);

    // Act
    let result = settle_and_mark_clean(
        move || {
            observed_calls
                .lock()
                .expect("call observations")
                .push("settle");
            Err(DaemonCheckpointError::ShutdownCheckpoint)
        },
        move || {
            marker_calls
                .lock()
                .expect("call observations")
                .push("mark-clean");
            Ok(())
        },
    );

    // Assert
    assert!(matches!(
        result,
        Err(DaemonCheckpointError::ShutdownCheckpoint)
    ));
    assert_eq!(
        calls.lock().expect("call observations").as_slice(),
        ["settle"]
    );
}

#[test]
fn encode_write_and_completion_failures_have_fixed_classes_and_never_mark_clean() {
    for error in [
        MempoolCheckpointError::Execution(SnapshotWriteExecutionError::Encode(
            injected_storage_error(StorageNamespace::Mempool),
        )),
        MempoolCheckpointError::Execution(SnapshotWriteExecutionError::Storage(
            injected_storage_error(StorageNamespace::Mempool),
        )),
        MempoolCheckpointError::CompletionDispatch(ManagedNetworkAuthorityError::Poisoned),
    ] {
        // Arrange
        let marker_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let observed_marker_calls = Arc::clone(&marker_calls);
        let expected_class = checkpoint_failure_class(&error);

        // Act
        let result = settle_and_mark_clean(
            || Err(DaemonCheckpointError::ShutdownCheckpoint),
            move || {
                observed_marker_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok(())
            },
        );

        // Assert
        assert!(matches!(
            result,
            Err(DaemonCheckpointError::ShutdownCheckpoint)
        ));
        assert_eq!(marker_calls.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(matches!(
            expected_class,
            "execution" | "completion-dispatch"
        ));
    }
}

#[test]
fn clean_marker_failure_has_fixed_class_and_follows_settlement() {
    // Arrange
    let calls = Arc::new(Mutex::new(Vec::new()));
    let settle_calls = Arc::clone(&calls);
    let marker_calls = Arc::clone(&calls);
    let storage_error = StorageError::BackendFailure {
        namespace: StorageNamespace::Runtime,
        message: "injected private backend detail".to_string(),
        action: StorageRecoveryAction::Restart,
    };

    // Act
    let result = settle_and_mark_clean(
        move || {
            settle_calls
                .lock()
                .expect("call observations")
                .push("settle");
            Ok(())
        },
        move || {
            marker_calls
                .lock()
                .expect("call observations")
                .push("mark-clean");
            Err(storage_error)
        },
    );
    let error = result.expect_err("clean marker failure");

    // Assert
    assert_eq!(
        calls.lock().expect("call observations").as_slice(),
        ["settle", "mark-clean"]
    );
    assert!(matches!(error, DaemonCheckpointError::CleanMarker));
    assert_eq!(
        error.to_string(),
        "open-bitcoind checkpoint shutdown failed: clean-marker"
    );
    assert!(!error.to_string().contains("injected"));
}

#[test]
fn clean_skip_still_forces_current_generation_before_marking_clean() {
    // Arrange
    let data_dir = temp_store_path("checkpoint-clean-skip");
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("checkpoint store");
    let runtime = DurableSyncRuntime::open(store.clone(), SyncRuntimeConfig::default())
        .expect("checkpoint runtime");
    let handle = runtime.network_handle();
    let checkpoint_worker = start_mempool_checkpoint_worker(handle.clone(), Some(store.clone()))
        .expect("durable checkpoint worker");

    // Act
    checkpoint_worker
        .shutdown_and_mark_clean()
        .expect("clean checkpoint shutdown");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(302), 1)
        .expect("checkpoint evidence");

    // Assert
    assert_eq!(evidence.current_generation, 0);
    assert_eq!(evidence.maybe_last_durable_generation, Some(0));
    let limits =
        MempoolSnapshotDecodeLimits::for_persisted_input().expect("snapshot decode limits");
    assert!(
        store
            .load_mempool_snapshot_with_limits(limits)
            .expect("stored snapshot")
            .is_some()
    );
    assert!(
        store
            .load_runtime_metadata()
            .expect("runtime metadata")
            .expect("clean metadata")
            .last_clean_shutdown
    );
    remove_dir_if_exists(&data_dir);
}

#[test]
fn exact_current_runtime_skips_redundant_shutdown_write() {
    // Arrange
    let data_dir = temp_store_path("checkpoint-redundant-skip");
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("checkpoint store");
    let runtime = DurableSyncRuntime::open(store.clone(), SyncRuntimeConfig::default())
        .expect("checkpoint runtime");
    let handle = runtime.network_handle();
    start_mempool_checkpoint_worker(handle.clone(), Some(store.clone()))
        .expect("first checkpoint worker")
        .shutdown_and_mark_clean()
        .expect("first shutdown");

    // Act
    let outcome = checkpoint_worker_loop(
        handle,
        store,
        |_| CheckpointWait::Shutdown,
        || PolicyTime::new(303),
    )
    .expect("redundant shutdown settle");

    // Assert
    assert_eq!(outcome, MempoolCheckpointOutcome::SkippedClean);
    remove_dir_if_exists(&data_dir);
}

#[test]
fn daemon_shutdown_orders_all_producer_joins_before_checkpoint_settlement() {
    // Arrange
    let daemon_source = include_str!("../../open-bitcoind.rs");

    // Act
    let http_end = daemon_source
        .find("let serve_result = axum::serve")
        .expect("HTTP serve");
    let inbound_join = daemon_source
        .find("inbound_listener.shutdown().await")
        .expect("inbound join");
    let metrics_join = daemon_source
        .find("worker.shutdown();")
        .expect("metrics join");
    let coordinator = daemon_source
        .find("    settle_daemon_shutdown(\n")
        .expect("actual shutdown coordinator");
    let sync_join = daemon_source
        .find("maybe_sync_worker.map_or(Ok(()), |worker| worker.shutdown())")
        .expect("sync join callback");
    let retry_join = daemon_source
        .find("|| Ok(retry_worker.shutdown()?),")
        .expect("retry producer join callback");
    let checkpoint_settle = daemon_source
        .find("maybe_checkpoint_worker.map_or(Ok(()), |worker| worker.shutdown_settle())")
        .expect("checkpoint settlement callback");
    let checkpoint_source = include_str!("../checkpoint.rs");

    // Assert
    assert!(http_end < inbound_join);
    assert!(inbound_join < metrics_join);
    assert!(metrics_join < coordinator);
    assert!(coordinator < sync_join);
    assert!(sync_join < retry_join);
    assert!(retry_join < checkpoint_settle);
    assert!(
        checkpoint_source
            .contains("let results = [sync(), coins(), retry(), checkpoint(), serve_result];")
    );
    assert!(
        checkpoint_source
            .find("return Err(error);")
            .expect("failure gate")
            < checkpoint_source
                .find("mark_clean().map_err")
                .expect("clean after success")
    );
}

#[test]
fn phase157_shutdown_attempts_every_settlement_before_returning_first_failure() {
    // Arrange
    let calls = std::cell::RefCell::new(Vec::new());
    let failure = || -> crate::checkpoint::ShutdownResult {
        Err(Box::new(DaemonCheckpointError::ProducerJoin))
    };
    // Act
    let result = crate::checkpoint::settle_daemon_shutdown(
        Ok(()),
        || {
            calls.borrow_mut().push("sync");
            failure()
        },
        || {
            calls.borrow_mut().push("coins");
            Err(Box::new(DaemonCheckpointError::ShutdownCheckpoint))
        },
        || {
            calls.borrow_mut().push("retry");
            Ok(())
        },
        || {
            calls.borrow_mut().push("checkpoint");
            Ok(())
        },
        || {
            calls.borrow_mut().push("clean");
            Ok(())
        },
    );
    // Assert
    assert_eq!(*calls.borrow(), ["sync", "coins", "retry", "checkpoint"]);
    assert_eq!(
        result
            .expect_err("first failure returned")
            .downcast_ref::<DaemonCheckpointError>(),
        Some(&DaemonCheckpointError::ProducerJoin)
    );
}

#[test]
fn phase157_shutdown_http_failure_withholds_clean_marker_after_settlement() {
    // Arrange
    let calls = std::cell::RefCell::new(Vec::new());
    // Act
    let result = crate::checkpoint::settle_daemon_shutdown(
        Err(Box::new(std::io::Error::other("serve failed"))),
        || {
            calls.borrow_mut().push("sync");
            Ok(())
        },
        || {
            calls.borrow_mut().push("coins");
            Ok(())
        },
        || {
            calls.borrow_mut().push("retry");
            Ok(())
        },
        || {
            calls.borrow_mut().push("checkpoint");
            Ok(())
        },
        || {
            calls.borrow_mut().push("clean");
            Ok(())
        },
    );
    // Assert
    assert_eq!(*calls.borrow(), ["sync", "coins", "retry", "checkpoint"]);
    assert_eq!(
        result.expect_err("HTTP failure visible").to_string(),
        "serve failed"
    );
}

#[test]
fn phase157_shutdown_success_marks_clean_only_after_all_settlement() {
    // Arrange
    let calls = std::cell::RefCell::new(Vec::new());
    // Act
    crate::checkpoint::settle_daemon_shutdown(
        Ok(()),
        || {
            calls.borrow_mut().push("sync");
            Ok(())
        },
        || {
            calls.borrow_mut().push("coins");
            Ok(())
        },
        || {
            calls.borrow_mut().push("retry");
            Ok(())
        },
        || {
            calls.borrow_mut().push("checkpoint");
            Ok(())
        },
        || {
            calls.borrow_mut().push("clean");
            Ok(())
        },
    )
    .expect("clean shutdown");
    // Assert
    assert_eq!(
        *calls.borrow(),
        ["sync", "coins", "retry", "checkpoint", "clean"]
    );
}

fn injected_storage_error(namespace: StorageNamespace) -> StorageError {
    StorageError::BackendFailure {
        namespace,
        message: "injected private backend detail".to_string(),
        action: StorageRecoveryAction::Restart,
    }
}
