// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

//! Fake-clock coverage for the receive-independent retry worker.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use open_bitcoin_network::RetryJitterSeconds;

use super::*;
use crate::retry::{RetryWait, retry_worker_loop, start_initial_broadcast_retry_worker};

#[test]
fn retry_worker_fires_maintenance_tick_on_elapsed_without_sleep() {
    // Arrange
    let (runtime, data_dir) = retry_runtime("retry-elapsed");
    let waits = Arc::new(Mutex::new(Vec::new()));
    let observed_waits = Arc::clone(&waits);
    let mut wait_count = 0;
    let jitter = RetryJitterSeconds::new(10).expect("valid jitter");

    // Act
    retry_worker_loop(
        runtime.network_handle(),
        move |duration| {
            observed_waits
                .lock()
                .expect("wait observations")
                .push(duration);
            wait_count += 1;
            if wait_count == 1 {
                RetryWait::Elapsed
            } else {
                RetryWait::Shutdown
            }
        },
        || 1_000,
        move || Some(jitter),
    )
    .expect("retry loop");

    // Assert
    assert_eq!(
        waits.lock().expect("wait observations").as_slice(),
        [Duration::from_secs(600), Duration::from_secs(610)]
    );
    remove_dir_if_exists(&data_dir);
}

#[test]
fn retry_worker_shutdown_stops_before_next_tick() {
    // Arrange
    let (runtime, data_dir) = retry_runtime("retry-shutdown");
    let waits = Arc::new(Mutex::new(Vec::new()));
    let observed_waits = Arc::clone(&waits);
    let ticks = Arc::new(Mutex::new(0_u32));
    let observed_ticks = Arc::clone(&ticks);

    // Act
    retry_worker_loop(
        runtime.network_handle(),
        move |duration| {
            observed_waits
                .lock()
                .expect("wait observations")
                .push(duration);
            RetryWait::Shutdown
        },
        move || {
            *observed_ticks.lock().expect("tick observations") += 1;
            1_000
        },
        || RetryJitterSeconds::new(1).ok(),
    )
    .expect("retry loop");

    // Assert
    assert_eq!(
        waits.lock().expect("wait observations").as_slice(),
        [Duration::from_secs(600)]
    );
    assert_eq!(*ticks.lock().expect("tick observations"), 0);
    remove_dir_if_exists(&data_dir);
}

#[test]
fn retry_worker_jitter_unavailable_does_not_use_silent_constant_jitter() {
    // Arrange
    let (runtime, data_dir) = retry_runtime("retry-jitter");
    let waits = Arc::new(Mutex::new(Vec::new()));
    let observed_waits = Arc::clone(&waits);
    let mut wait_count = 0;
    let source = include_str!("../retry.rs");

    // Act
    retry_worker_loop(
        runtime.network_handle(),
        move |duration| {
            observed_waits
                .lock()
                .expect("wait observations")
                .push(duration);
            wait_count += 1;
            if wait_count == 1 {
                RetryWait::Elapsed
            } else {
                RetryWait::Shutdown
            }
        },
        || 1_000,
        || None,
    )
    .expect("retry loop");

    // Assert
    assert_eq!(
        waits.lock().expect("wait observations").as_slice(),
        [Duration::from_secs(600), Duration::from_secs(600)]
    );
    assert!(!source.contains("RetryJitterSeconds::new(0)"));
    assert!(!source.contains("RetryJitterSeconds::new(300)"));
    remove_dir_if_exists(&data_dir);
}

#[test]
fn retry_worker_is_not_started_from_durable_sync_runtime() {
    // Arrange
    let sync_runtime = include_str!("../../../../../open-bitcoin-node/src/sync.rs");
    let sync_seed = include_str!("../sync_seed.rs");
    let worker_start = start_initial_broadcast_retry_worker;

    // Act / Assert
    assert!(!sync_runtime.contains("start_initial_broadcast_retry_worker"));
    assert!(!sync_seed.contains("start_initial_broadcast_retry_worker"));
    let _ = worker_start;
}

fn retry_runtime(label: &str) -> (DurableSyncRuntime, std::path::PathBuf) {
    let data_dir = temp_store_path(label);
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("retry store");
    let runtime =
        DurableSyncRuntime::open(store, SyncRuntimeConfig::default()).expect("retry runtime");
    (runtime, data_dir)
}
