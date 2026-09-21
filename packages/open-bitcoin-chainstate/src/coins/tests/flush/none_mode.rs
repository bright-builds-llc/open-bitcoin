// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

#[test]
fn none_mode_classifies_ok_at_small_budget_ninety_percent_threshold() {
    // Arrange
    let input = none_mode_input(45 * MIB, 50 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Ok);
}

#[test]
fn none_mode_classifies_large_one_byte_over_small_budget_ninety_percent() {
    // Arrange
    let input = none_mode_input(45 * MIB + 1, 50 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Large);
}

#[test]
fn none_mode_classifies_critical_when_cache_bytes_exceed_total() {
    // Arrange
    let input = none_mode_input(50 * MIB + 1, 50 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Critical);
}

#[test]
fn none_mode_classifies_ok_at_180_mib_on_200_mib_budget() {
    // Arrange
    let input = none_mode_input(180 * MIB, 200 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Ok);
}

#[test]
fn none_mode_classifies_ok_at_exactly_total_minus_10_mib_on_200_mib_budget() {
    // Arrange
    let input = none_mode_input(190 * MIB, 200 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Ok);
}

#[test]
fn none_mode_classifies_large_one_byte_over_headroom_on_200_mib_budget() {
    // Arrange
    let input = none_mode_input(190 * MIB + 1, 200 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Large);
}

#[test]
fn none_mode_adds_mempool_leftover_to_total_space() {
    // Arrange
    let input = none_mode_input(45 * MIB + 1, 40 * MIB, 10 * MIB);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Large);
}

#[test]
fn none_mode_never_refuses_disk_even_when_free_bytes_are_zero() {
    // Arrange
    let input = none_mode_input(50 * MIB + 1, 50 * MIB, 0);

    // Act
    let decision = decide_flush(input);

    // Assert
    assert_none_decision(decision, CoinsCacheSizeState::Critical);
}

#[test]
fn classify_does_not_panic_when_limit_and_leftover_are_u64_max() {
    // Arrange
    let input = none_mode_input(u64::MAX, u64::MAX, u64::MAX);

    // Act
    let decision = decide_flush(input);

    // Assert
    let _ = decision;
}

#[test]
fn flush_policy_constants_are_9_10_10_mib_and_192() {
    // Arrange / Act / Assert
    assert_eq!(LARGE_CACHE_NUMERATOR, 9);
    assert_eq!(LARGE_CACHE_DENOMINATOR, 10);
    assert_eq!(LARGE_CACHE_HEADROOM_BYTES, 10 * 1024 * 1024);
    assert_eq!(COIN_WRITE_GUARD_BYTES_PER_ENTRY, 48 * 2 * 2);
}
