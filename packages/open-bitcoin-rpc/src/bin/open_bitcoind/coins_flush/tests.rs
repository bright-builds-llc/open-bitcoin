// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::{PERIODIC_WRITE_MAX_SECS, PERIODIC_WRITE_MIN_SECS, resample_periodic_next_write};

#[path = "tests/filter_index.rs"]
mod filter_index;

#[test]
fn periodic_jitter_is_between_50_and_70_minutes() {
    // Arrange
    let now = 1_700_000_000_u64;

    // Act
    let next = resample_periodic_next_write(now).expect("getrandom fill");

    // Assert
    let seconds = next.unix_seconds();
    assert!(
        (now + PERIODIC_WRITE_MIN_SECS..=now + PERIODIC_WRITE_MAX_SECS).contains(&seconds),
        "jitter {seconds} must land in [{}, {}]",
        now + PERIODIC_WRITE_MIN_SECS,
        now + PERIODIC_WRITE_MAX_SECS
    );
    let source = include_str!("../coins_flush.rs");
    assert!(source.contains("getrandom"));
    assert!(!source.contains("open-bitcoin-chainstate"));
}

#[test]
fn coins_flush_source_uses_periodic_and_always() {
    // Arrange
    let source = include_str!("../coins_flush.rs");

    // Act / Assert
    assert!(source.contains("FlushMode::Periodic"));
    assert!(source.contains("FlushMode::Always"));
    assert!(source.contains("ManagedNetworkHandle"));
}

#[test]
fn coins_flush_worker_does_not_own_a_second_lifecycle() {
    // Arrange
    let source = include_str!("../coins_flush.rs");

    // Act / Assert
    assert!(!source.contains("FlushLifecycle {"));
    assert!(!source.contains("initialize("));
    assert!(source.contains("handle.flush_coins") || source.contains("flush_coins("));
    assert!(source.contains("set_coins_next_write"));
    assert!(source.contains("store.datadir()"));
    assert!(source.contains("PERIODIC_WRITE_MIN_SECS"));
}

#[test]
fn coins_flush_ordinary_cycle_and_shutdown_use_existing_durable_owner() {
    use open_bitcoin_node::core::chainstate::{ChainstateSnapshot, FlushMode, FlushPolicyTime};
    use open_bitcoin_node::{DurableSyncRuntime, FjallNodeStore, SyncRuntimeConfig};
    // Arrange
    let temp = crate::tests::temp_store_path("coins-worker-behavior");
    let store = FjallNodeStore::open(&temp).expect("store");
    store
        .seed_coins_from_snapshot(&ChainstateSnapshot::new(
            Vec::new(),
            Default::default(),
            Default::default(),
        ))
        .expect("empty coins");
    let runtime =
        DurableSyncRuntime::open(store.clone(), SyncRuntimeConfig::default()).expect("ready owner");
    let handle = runtime.network_handle();
    handle
        .set_coins_next_write(FlushPolicyTime::new(u64::MAX))
        .expect("future deadline");

    // Act
    let periodic = super::flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("periodic");
    let always = super::flush_cycle(
        &handle,
        FlushMode::Always,
        FlushPolicyTime::new(101),
        u64::MAX,
    )
    .expect("always");
    let mut waits = 0;
    super::coins_flush_worker_loop(handle, store.clone(), |_| {
        waits += 1;
        if waits == 1 {
            super::CheckpointWait::Elapsed
        } else {
            super::CheckpointWait::Shutdown
        }
    })
    .expect("worker settles same owner");

    // Assert
    assert!(!periodic.wrote_coins);
    assert!(always.wrote_coins);
    assert_eq!(waits, 2);
    assert!(!store.load_have_pruned().expect("no invented prune state"));
    drop(runtime);
    drop(store);
    crate::tests::remove_dir_if_exists(&temp);
}
