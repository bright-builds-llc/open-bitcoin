// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use crate::coins::{
    COIN_WRITE_GUARD_BYTES_PER_ENTRY, CoinsCacheSizeState, FlushDecision, FlushDecisionFacts,
    FlushMode, FlushPolicyInput, FlushPolicyTime, LARGE_CACHE_DENOMINATOR,
    LARGE_CACHE_HEADROOM_BYTES, LARGE_CACHE_NUMERATOR, LastFlushReason, decide_flush,
};

const MIB: u64 = 1024 * 1024;

fn none_mode_input(
    cache_bytes: u64,
    cache_byte_limit: u64,
    mempool_leftover_bytes: u64,
) -> FlushPolicyInput {
    FlushPolicyInput {
        mode: FlushMode::None,
        cache_bytes,
        cache_byte_limit,
        mempool_leftover_bytes,
        now: FlushPolicyTime::from_unix_seconds(1_700_000_000),
        next_write: FlushPolicyTime::from_unix_seconds(1_700_000_100),
        memory_pressure: false,
        disk_free_bytes: 0,
        cache_entry_count: 10,
    }
}

fn assert_none_decision(decision: FlushDecision, cache_size: CoinsCacheSizeState) {
    assert!(
        matches!(decision, FlushDecision::None(_)),
        "expected FlushDecision::None, got {decision:?}"
    );
    assert!(!matches!(decision, FlushDecision::RefuseDiskSpace(_)));
    assert_eq!(decision.cache_size(), cache_size);
    assert_eq!(decision.reason(), LastFlushReason::None);
}

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

#[test]
fn flush_policy_module_source_has_no_clock_fs_or_fjall() {
    // Arrange
    let source = include_str!("../flush.rs");
    let forbidden = [
        "Instant",
        "SystemTime",
        concat!("std::", "fs"),
        "fjall",
        concat!("tok", "io"),
    ];

    // Act
    let maybe_found = forbidden.into_iter().find(|needle| source.contains(needle));

    // Assert
    assert_eq!(maybe_found, None);
}

#[test]
fn flush_policy_time_exposes_injected_unix_seconds() {
    // Arrange
    let now = FlushPolicyTime::from_unix_seconds(1_700_000_000);

    // Act
    let seconds = now.unix_seconds();

    // Assert
    assert_eq!(seconds, 1_700_000_000);
}

#[test]
fn flush_policy_input_periodic_due_when_now_equals_next_write() {
    // Arrange
    let input = FlushPolicyInput {
        now: FlushPolicyTime::from_unix_seconds(1_700_000_100),
        next_write: FlushPolicyTime::from_unix_seconds(1_700_000_100),
        ..none_mode_input(0, 50 * MIB, 0)
    };

    // Act
    let is_due = input.periodic_due();

    // Assert
    assert!(is_due);
}

#[test]
fn flush_policy_input_periodic_due_when_now_is_before_next_write() {
    // Arrange
    let input = FlushPolicyInput {
        now: FlushPolicyTime::from_unix_seconds(1_700_000_099),
        next_write: FlushPolicyTime::from_unix_seconds(1_700_000_100),
        ..none_mode_input(0, 50 * MIB, 0)
    };

    // Act
    let is_due = input.periodic_due();

    // Assert
    assert!(!is_due);
}

#[test]
fn flush_decision_accessors_read_shared_facts_on_every_variant() {
    // Arrange
    let facts = FlushDecisionFacts {
        cache_size: CoinsCacheSizeState::Large,
        reason: LastFlushReason::Needed,
    };
    let decisions = [
        FlushDecision::None(facts),
        FlushDecision::Flush(facts),
        FlushDecision::Sync(facts),
        FlushDecision::RefuseDiskSpace(facts),
    ];

    // Act
    let observed: Vec<_> = decisions
        .into_iter()
        .map(|decision| (decision.cache_size(), decision.reason()))
        .collect();

    // Assert
    assert_eq!(
        observed,
        vec![(CoinsCacheSizeState::Large, LastFlushReason::Needed); 4]
    );
}
