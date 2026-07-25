// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

use crate::{
    AccountedMempoolMemory, BlockLifecycleContext, FeeRate, IncrementalRelayFeeRate, Mempool,
    MempoolCapacity, PolicyConfig, PolicyTime, ROLLING_FEE_HALFLIFE_SECONDS,
    ROLLING_FEE_UPDATE_INTERVAL_SECONDS, RollingFeeState, StaticRelayFeeRate,
    effective_admission_fee_rate,
};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, MerkleRoot};

fn empty_block() -> Block {
    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 0,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions: Vec::new(),
    }
}

fn high_occupancy() -> (AccountedMempoolMemory, MempoolCapacity) {
    (AccountedMempoolMemory::new(100), MempoolCapacity::new(100))
}

fn below_half_capacity() -> (AccountedMempoolMemory, MempoolCapacity) {
    (AccountedMempoolMemory::new(40), MempoolCapacity::new(100))
}

fn below_quarter_capacity() -> (AccountedMempoolMemory, MempoolCapacity) {
    (AccountedMempoolMemory::new(20), MempoolCapacity::new(100))
}

fn incremental_1000() -> IncrementalRelayFeeRate {
    IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000))
}

#[test]
fn rolling_fee_decay_does_not_run_before_block_after_bump() {
    // Arrange
    let mut state = RollingFeeState::new();
    let bumped = FeeRate::from_sats_per_kvb(10_000);
    state.track_package_removed(bumped);
    let (usage, capacity) = high_occupancy();
    let later = PolicyTime::new(ROLLING_FEE_HALFLIFE_SECONDS * 10);

    // Act
    let materialized = state.decay_toward(later, usage, capacity, incremental_1000());

    // Assert
    assert_eq!(materialized.fee_rate(), bumped);
    assert_eq!(state.rolling_fee_rate().fee_rate(), bumped);
    assert!(!state.block_since_last_rolling_fee_bump());
}

#[test]
fn rolling_fee_decay_twelve_hour_halflife_at_high_occupancy() {
    // Arrange
    let mut state = RollingFeeState::new();
    let bumped = FeeRate::from_sats_per_kvb(10_000);
    state.track_package_removed(bumped);
    let connected_at = PolicyTime::new(42);
    state.open_decay_gate_after_block(connected_at);
    let (usage, capacity) = high_occupancy();
    let after_halflife = PolicyTime::new(42 + ROLLING_FEE_HALFLIFE_SECONDS);

    // Act
    let materialized = state.decay_toward(after_halflife, usage, capacity, incremental_1000());

    // Assert — Knots: llround(rate / 2) after one default half-life at high occupancy
    assert_eq!(
        materialized.fee_rate().sats_per_kvb(),
        (bumped.sats_per_kvb() as f64 / 2.0).round() as i64
    );
}

#[test]
fn rolling_fee_decay_six_hour_halflife_below_half_capacity() {
    // Arrange
    let mut state = RollingFeeState::new();
    let bumped = FeeRate::from_sats_per_kvb(10_000);
    state.track_package_removed(bumped);
    let connected_at = PolicyTime::new(42);
    state.open_decay_gate_after_block(connected_at);
    let (usage, capacity) = below_half_capacity();
    let after_six_hours = PolicyTime::new(42 + ROLLING_FEE_HALFLIFE_SECONDS / 2);

    // Act
    let materialized = state.decay_toward(after_six_hours, usage, capacity, incremental_1000());

    // Assert — occupancy < capacity/2 shortens half-life to 6h
    assert_eq!(
        materialized.fee_rate().sats_per_kvb(),
        (bumped.sats_per_kvb() as f64 / 2.0).round() as i64
    );
}

#[test]
fn rolling_fee_decay_three_hour_halflife_below_quarter_capacity() {
    // Arrange
    let mut state = RollingFeeState::new();
    let bumped = FeeRate::from_sats_per_kvb(10_000);
    state.track_package_removed(bumped);
    let connected_at = PolicyTime::new(42);
    state.open_decay_gate_after_block(connected_at);
    let (usage, capacity) = below_quarter_capacity();
    let after_three_hours = PolicyTime::new(42 + ROLLING_FEE_HALFLIFE_SECONDS / 4);

    // Act
    let materialized = state.decay_toward(after_three_hours, usage, capacity, incremental_1000());

    // Assert — occupancy < capacity/4 shortens half-life to 3h
    assert_eq!(
        materialized.fee_rate().sats_per_kvb(),
        (bumped.sats_per_kvb() as f64 / 2.0).round() as i64
    );
}

#[test]
fn rolling_fee_decay_skips_updates_within_ten_seconds() {
    // Arrange
    let mut state = RollingFeeState::new();
    let bumped = FeeRate::from_sats_per_kvb(10_000);
    state.track_package_removed(bumped);
    let connected_at = PolicyTime::new(100);
    state.open_decay_gate_after_block(connected_at);
    let (usage, capacity) = high_occupancy();
    let within_gate = PolicyTime::new(100 + ROLLING_FEE_UPDATE_INTERVAL_SECONDS);

    // Act
    let materialized = state.decay_toward(within_gate, usage, capacity, incremental_1000());

    // Assert — Knots uses `time > last + 10`, so dt == 10 does not decay
    assert_eq!(materialized.fee_rate(), bumped);
    assert_eq!(state.last_rolling_fee_update(), connected_at);
}

#[test]
fn rolling_fee_decay_zeros_below_incremental_half() {
    // Arrange
    let mut state = RollingFeeState::new();
    let incremental = IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000));
    // Start just above incremental/2 so one long decay collapses through the threshold.
    state.track_package_removed(FeeRate::from_sats_per_kvb(600));
    let connected_at = PolicyTime::new(0);
    state.open_decay_gate_after_block(connected_at);
    let (usage, capacity) = high_occupancy();
    let far_future = PolicyTime::new(ROLLING_FEE_HALFLIFE_SECONDS * 8);
    // Static above incremental so effective≠incremental when rolling collapses to zero.
    let static_floor = StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(2_000));

    // Act
    let materialized = state.decay_toward(far_future, usage, capacity, incremental);
    let effective = effective_admission_fee_rate(static_floor, materialized);

    // Assert
    assert_eq!(materialized.fee_rate().sats_per_kvb(), 0);
    assert_eq!(effective.fee_rate(), static_floor.fee_rate());
    assert_ne!(
        effective.fee_rate(),
        incremental.fee_rate(),
        "effective admission must stay max(static, rolling), never incremental mid-decay"
    );
}

#[test]
fn rolling_fee_decay_opens_gate_on_connected_block_lifecycle() {
    // Arrange — capacity 0 keeps empty-pool occupancy off the /4 and /2 shorteners.
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    });
    mempool.track_package_removed(FeeRate::from_sats_per_kvb(10_000));
    let connected_at = PolicyTime::new(1_700_000_000);
    let context = BlockLifecycleContext::new(connected_at, 3);

    // Act
    mempool
        .remove_for_connected_block_transition(&empty_block(), context)
        .expect("empty connect opens decay gate");
    let after_halflife = PolicyTime::new(1_700_000_000 + ROLLING_FEE_HALFLIFE_SECONDS);
    let materialized = mempool.materialize_rolling_fee_rate(after_halflife);

    // Assert
    assert_eq!(materialized.fee_rate().sats_per_kvb(), 5_000);
}
