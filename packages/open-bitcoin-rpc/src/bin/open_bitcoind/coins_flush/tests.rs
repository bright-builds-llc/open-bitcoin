// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::{PERIODIC_WRITE_MAX_SECS, PERIODIC_WRITE_MIN_SECS, resample_periodic_next_write};

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
}
