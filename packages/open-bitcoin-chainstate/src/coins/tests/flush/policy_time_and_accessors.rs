// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

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
