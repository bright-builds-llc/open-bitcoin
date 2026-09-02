// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use crate::coins::{
    COIN_WRITE_GUARD_BYTES_PER_ENTRY, CoinsCacheSizeState, FlushDecision, FlushDecisionFacts,
    FlushMode, FlushPolicyInput, FlushPolicyTime, LARGE_CACHE_DENOMINATOR,
    LARGE_CACHE_HEADROOM_BYTES, LARGE_CACHE_NUMERATOR, LastFlushReason, RecoveryDecision,
    decide_flush, decide_recovery,
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

const MATRIX_CACHE_BYTE_LIMIT: u64 = 50 * 1024 * 1024;
const MATRIX_OK_CACHE_BYTES: u64 = 0;
const MATRIX_LARGE_CACHE_BYTES: u64 = 45 * 1024 * 1024 + 1;
const MATRIX_CRITICAL_CACHE_BYTES: u64 = 50 * 1024 * 1024 + 1;
const MATRIX_DISK_PLENTY: u64 = u64::MAX;
const MATRIX_ENTRY_COUNT: u64 = 10;
const MATRIX_DISK_GUARD_FAIL: u64 = 192 * 10 - 1;
const MATRIX_DISK_GUARD_PASS: u64 = 1920;

fn matrix_input(
    mode: FlushMode,
    cache_bytes: u64,
    memory_pressure: bool,
    now_seconds: u64,
    next_write_seconds: u64,
    disk_free_bytes: u64,
    cache_entry_count: u64,
) -> FlushPolicyInput {
    FlushPolicyInput {
        mode,
        cache_bytes,
        cache_byte_limit: MATRIX_CACHE_BYTE_LIMIT,
        mempool_leftover_bytes: 0,
        now: FlushPolicyTime::from_unix_seconds(now_seconds),
        next_write: FlushPolicyTime::from_unix_seconds(next_write_seconds),
        memory_pressure,
        disk_free_bytes,
        cache_entry_count,
    }
}

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

#[test]
fn decide_recovery_does_not_mention_block_hash_or_replay_in_source() {
    // Arrange
    let flush_src = include_str!("../flush.rs");

    // Act
    let mentions_forbidden = flush_src.contains("ReplayBlocks")
        || flush_src.contains("BlockHash")
        || flush_src.contains("PruneAndFlush")
        || flush_src.contains("FlushForPrune");

    // Assert
    assert!(
        !mentions_forbidden,
        "flush.rs must stay count-only and prune-flush-free"
    );
}

#[test]
fn crate_root_reexports_flush_and_recovery_types() {
    // Arrange
    let now = crate::FlushPolicyTime::from_unix_seconds(0);
    let input = crate::FlushPolicyInput {
        mode: crate::FlushMode::None,
        cache_bytes: 0,
        cache_byte_limit: 1,
        mempool_leftover_bytes: 0,
        now,
        next_write: now,
        memory_pressure: false,
        disk_free_bytes: 0,
        cache_entry_count: 0,
    };

    // Act
    let flush_decision = crate::decide_flush(input);
    let recovery_decision = crate::decide_recovery(0);

    // Assert
    assert!(matches!(flush_decision, crate::FlushDecision::None(_)));
    assert_eq!(
        recovery_decision,
        crate::RecoveryDecision::ConsistentEmptyHeads
    );
}

#[test]
fn managed_chainstate_persist_still_writes_snapshot_and_does_not_call_decide_flush() {
    // Arrange
    let persist_src = include_str!("../../../../open-bitcoin-node/src/chainstate.rs");

    // Act
    let writes_snapshot =
        persist_src.contains("fn persist(") && persist_src.contains("save_snapshot");
    let retargeted =
        persist_src.contains("decide_flush") || persist_src.contains("RefuseDiskSpace");

    // Assert
    assert!(
        writes_snapshot,
        "ManagedChainstate::persist must still write a snapshot"
    );
    assert!(
        !retargeted,
        "ManagedChainstate::persist must not call decide_flush or mention RefuseDiskSpace"
    );
}

#[test]
fn durable_sync_persist_progress_still_writes_snapshot_and_does_not_call_decide_flush() {
    // Arrange
    let persist_src = include_str!("../../../../open-bitcoin-node/src/sync/runtime_state.rs");

    // Act
    let writes_snapshot = persist_src.contains("fn persist_progress")
        && persist_src.contains("save_chainstate_snapshot");
    let retargeted = persist_src.contains("decide_flush") || persist_src.contains("FlushMode");

    // Assert
    assert!(
        writes_snapshot,
        "DurableSyncRuntime::persist_progress must still write a chainstate snapshot"
    );
    assert!(
        !retargeted,
        "DurableSyncRuntime::persist_progress must not call decide_flush or mention FlushMode"
    );
}
