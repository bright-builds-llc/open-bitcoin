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

mod matrix_modes;
mod none_mode;
mod policy_time_and_accessors;

const MATRIX_CACHE_BYTE_LIMIT: u64 = 50 * 1024 * 1024;
const MATRIX_OK_CACHE_BYTES: u64 = 0;
const MATRIX_LARGE_CACHE_BYTES: u64 = 45 * 1024 * 1024 + 1;
const MATRIX_CRITICAL_CACHE_BYTES: u64 = 50 * 1024 * 1024 + 1;
const MATRIX_DISK_PLENTY: u64 = u64::MAX;
const MATRIX_ENTRY_COUNT: u64 = 10;
const MATRIX_DISK_GUARD_FAIL: u64 = 192 * 10 - 1;
const MATRIX_DISK_GUARD_PASS: u64 = 1920;

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
fn managed_chainstate_persist_calls_decide_flush_and_does_not_write_snapshot() {
    // Arrange
    let persist_src = include_str!("../../../../open-bitcoin-node/src/chainstate.rs");

    // Act
    let calls_execute = persist_src.contains("self.flush_lifecycle.execute_flush")
        && persist_src.contains("FlushMode::IfNeeded");
    let leftover_snapshot = persist_src.contains("save_snapshot(self.chainstate.snapshot())");

    // Assert
    assert!(
        calls_execute,
        "ManagedChainstate::persist must call execute_flush with IfNeeded"
    );
    assert!(
        !leftover_snapshot,
        "ManagedChainstate::persist must not write leftover snapshots"
    );
}

#[test]
fn durable_sync_persist_progress_does_not_write_snapshot() {
    // Arrange
    let persist_src = include_str!("../../../../open-bitcoin-node/src/sync/runtime_state.rs");

    // Act
    let has_persist = persist_src.contains("fn persist_progress");
    let leftover_write = persist_src.contains("save_chainstate_snapshot")
        || persist_src.contains("seed_coins_from_snapshot");

    // Assert
    assert!(has_persist, "DurableSyncRuntime must keep persist_progress");
    assert!(
        !leftover_write,
        "DurableSyncRuntime::persist_progress must not write leftover snapshots"
    );
}
