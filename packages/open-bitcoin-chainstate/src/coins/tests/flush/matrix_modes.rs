// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

#[test]
fn always_mode_returns_flush_with_reason_always() {
    // Arrange
    let input = matrix_input(
        FlushMode::Always,
        MATRIX_OK_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::Flush(_)),
        "expected FlushDecision::Flush, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Always);
    assert_eq!(decision.cache_size(), CoinsCacheSizeState::Ok);
}

#[test]
fn always_mode_refuses_disk_when_free_below_192_times_entries() {
    // Arrange
    let input = matrix_input(
        FlushMode::Always,
        MATRIX_OK_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_GUARD_FAIL,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_eq!(MATRIX_DISK_GUARD_FAIL, 1919);
    assert_eq!(
        COIN_WRITE_GUARD_BYTES_PER_ENTRY.saturating_mul(MATRIX_ENTRY_COUNT),
        1920
    );
    assert!(
        matches!(decision, FlushDecision::RefuseDiskSpace(_)),
        "expected FlushDecision::RefuseDiskSpace, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::FailedDisk);
}

#[test]
fn always_mode_writes_flush_when_free_equals_192_times_entries() {
    // Arrange
    let input = matrix_input(
        FlushMode::Always,
        MATRIX_OK_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_GUARD_PASS,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_eq!(MATRIX_DISK_GUARD_PASS, 1920);
    assert!(
        matches!(decision, FlushDecision::Flush(_)),
        "expected FlushDecision::Flush, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Always);
}

#[test]
fn periodic_large_returns_flush_even_when_not_due() {
    // Arrange
    let input = matrix_input(
        FlushMode::Periodic,
        MATRIX_LARGE_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::Flush(_)),
        "expected FlushDecision::Flush, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Periodic);
    assert_eq!(decision.cache_size(), CoinsCacheSizeState::Large);
}

#[test]
fn periodic_critical_returns_flush_even_when_not_due() {
    // Arrange
    let input = matrix_input(
        FlushMode::Periodic,
        MATRIX_CRITICAL_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::Flush(_)),
        "expected FlushDecision::Flush, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Periodic);
    assert_eq!(decision.cache_size(), CoinsCacheSizeState::Critical);
}

#[test]
fn periodic_ok_due_returns_sync() {
    // Arrange
    let input = matrix_input(
        FlushMode::Periodic,
        MATRIX_OK_CACHE_BYTES,
        false,
        100,
        100,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::Sync(_)),
        "expected FlushDecision::Sync, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Periodic);
    assert_eq!(decision.cache_size(), CoinsCacheSizeState::Ok);
}

#[test]
fn periodic_ok_not_due_returns_none() {
    // Arrange
    let input = matrix_input(
        FlushMode::Periodic,
        MATRIX_OK_CACHE_BYTES,
        false,
        99,
        100,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::None(_)),
        "expected FlushDecision::None, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::None);
}

#[test]
fn periodic_ok_due_refuses_disk_instead_of_sync() {
    // Arrange
    let input = matrix_input(
        FlushMode::Periodic,
        MATRIX_OK_CACHE_BYTES,
        false,
        100,
        100,
        MATRIX_DISK_GUARD_FAIL,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::RefuseDiskSpace(_)),
        "expected FlushDecision::RefuseDiskSpace, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::FailedDisk);
}

#[test]
fn ifneeded_critical_returns_flush_with_reason_needed() {
    // Arrange
    let input = matrix_input(
        FlushMode::IfNeeded,
        MATRIX_CRITICAL_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::Flush(_)),
        "expected FlushDecision::Flush, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Needed);
    assert_eq!(decision.cache_size(), CoinsCacheSizeState::Critical);
}

#[test]
fn ifneeded_ok_with_memory_pressure_returns_flush() {
    // Arrange
    let input = matrix_input(
        FlushMode::IfNeeded,
        MATRIX_OK_CACHE_BYTES,
        true,
        0,
        1,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::Flush(_)),
        "expected FlushDecision::Flush, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::Needed);
}

#[test]
fn ifneeded_large_without_pressure_returns_none() {
    // Arrange
    let input = matrix_input(
        FlushMode::IfNeeded,
        MATRIX_LARGE_CACHE_BYTES,
        false,
        0,
        1,
        MATRIX_DISK_PLENTY,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::None(_)),
        "expected FlushDecision::None, got {decision:?}"
    );
    assert_eq!(decision.reason(), LastFlushReason::None);
}

#[test]
fn ifneeded_large_with_low_disk_still_returns_none() {
    // Arrange
    let input = matrix_input(
        FlushMode::IfNeeded,
        MATRIX_LARGE_CACHE_BYTES,
        false,
        0,
        1,
        0,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert!(
        matches!(decision, FlushDecision::None(_)),
        "expected FlushDecision::None, got {decision:?}"
    );
    assert!(!matches!(decision, FlushDecision::RefuseDiskSpace(_)));
    assert_eq!(decision.reason(), LastFlushReason::None);
}

#[test]
fn none_mode_critical_with_pressure_and_low_disk_still_returns_none() {
    // Arrange
    let input = matrix_input(
        FlushMode::None,
        MATRIX_CRITICAL_CACHE_BYTES,
        true,
        0,
        1,
        0,
        MATRIX_ENTRY_COUNT,
    );

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Critical);
}

#[test]
fn decide_recovery_zero_heads_is_consistent_empty() {
    // Arrange
    let head_marker_count = 0;

    // Act
    let decision = decide_recovery(head_marker_count);

    // Assert
    assert_eq!(decision, RecoveryDecision::ConsistentEmptyHeads);
}

#[test]
fn decide_recovery_one_head_is_first_class() {
    // Arrange
    let head_marker_count = 1;

    // Act
    let decision = decide_recovery(head_marker_count);

    // Assert
    assert_eq!(decision, RecoveryDecision::OneHead);
    assert!(!matches!(
        decision,
        RecoveryDecision::InconsistentOtherCount { count: 1 }
    ));
}

#[test]
fn decide_recovery_two_heads_is_interrupted() {
    // Arrange
    let head_marker_count = 2;

    // Act
    let decision = decide_recovery(head_marker_count);

    // Assert
    assert_eq!(decision, RecoveryDecision::InterruptedTwoHeads);
}

#[test]
fn decide_recovery_three_heads_is_inconsistent_other_count() {
    // Arrange
    let head_marker_count = 3;

    // Act
    let decision = decide_recovery(head_marker_count);

    // Assert
    assert_eq!(
        decision,
        RecoveryDecision::InconsistentOtherCount { count: 3 }
    );
}
