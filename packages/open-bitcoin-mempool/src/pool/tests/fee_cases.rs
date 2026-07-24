// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/doc/policy/packages.md

use open_bitcoin_primitives::{Amount, TransactionInput};

use super::{sample_chainstate_snapshot, spend_transaction, submit};
use crate::{
    FeeRate, IncrementalRelayFeeRate, Mempool, MempoolError, PolicyConfig, RollingMempoolFeeRate,
    StaticRelayFeeRate, effective_admission_fee_rate, evaluate_package_fee_floors,
    transaction_weight_and_virtual_size,
};

fn fee_rate(sats_per_kvb: i64) -> FeeRate {
    FeeRate::from_sats_per_kvb(sats_per_kvb)
}

#[test]
fn rolling_below_static_derives_static_floor() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(500));

    // Act
    let effective_floor = effective_admission_fee_rate(static_floor, rolling_floor);

    // Assert
    assert_eq!(effective_floor.fee_rate(), fee_rate(1_000));
}

#[test]
fn rolling_equal_to_static_derives_static_floor() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(1_000));

    // Act
    let effective_floor = effective_admission_fee_rate(static_floor, rolling_floor);

    // Assert
    assert_eq!(effective_floor.fee_rate(), fee_rate(1_000));
}

#[test]
fn rolling_above_static_derives_rolling_floor() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(2_000));

    // Act
    let effective_floor = effective_admission_fee_rate(static_floor, rolling_floor);

    // Assert
    assert_eq!(effective_floor.fee_rate(), fee_rate(2_000));
}

#[test]
fn zero_rolling_baseline_derives_static_floor() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));

    // Act
    let effective_floor = effective_admission_fee_rate(static_floor, RollingMempoolFeeRate::ZERO);

    // Assert
    assert_eq!(effective_floor.fee_rate(), fee_rate(1_000));
}

#[test]
fn incremental_relay_fee_is_not_an_admission_floor() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(2_000));
    let incremental_fee = IncrementalRelayFeeRate::new(fee_rate(10_000));

    // Act
    let effective_floor = effective_admission_fee_rate(static_floor, rolling_floor);

    // Assert
    assert_eq!(effective_floor.fee_rate(), fee_rate(2_000));
    assert!(incremental_fee.fee_rate() > effective_floor.fee_rate());
}

#[test]
fn package_member_below_static_fails_even_when_aggregate_exceeds_rolling() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(5_000));

    // Act
    let assessment =
        evaluate_package_fee_floors(fee_rate(500), fee_rate(6_000), static_floor, rolling_floor);

    // Assert
    assert!(!assessment.member_meets_static_floor());
    assert!(assessment.aggregate_meets_rolling_floor());
}

#[test]
fn package_aggregate_can_satisfy_rolling_independently() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(5_000));

    // Act
    let assessment = evaluate_package_fee_floors(
        fee_rate(1_000),
        fee_rate(5_000),
        static_floor,
        rolling_floor,
    );

    // Assert
    assert!(assessment.member_meets_static_floor());
    assert!(assessment.aggregate_meets_rolling_floor());
}

#[test]
fn ordinary_admission_rejects_below_effective_floor() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(10_000));
    let mut transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let (_weight, virtual_size) =
        transaction_weight_and_virtual_size(&transaction).expect("transaction size");
    let required_fee = rolling_floor
        .fee_rate()
        .fee_for_virtual_size(crate::TransactionVirtualSize::new(virtual_size));
    transaction.outputs[0].value =
        Amount::from_sats(500_000_000 - required_fee + 1).expect("valid output value");
    let mut mempool = Mempool::new(PolicyConfig::default());
    mempool.set_rolling_mempool_fee_rate(rolling_floor);

    // Act
    let error = submit(&mut mempool, &snapshot, transaction)
        .expect_err("fee below effective floor should fail");

    // Assert
    assert!(matches!(error, MempoolError::RelayFeeTooLow { .. }));
}

#[test]
fn ordinary_admission_accepts_at_effective_floor() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(10_000));
    let mut transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let (_weight, virtual_size) =
        transaction_weight_and_virtual_size(&transaction).expect("transaction size");
    let required_fee = rolling_floor
        .fee_rate()
        .fee_for_virtual_size(crate::TransactionVirtualSize::new(virtual_size));
    transaction.outputs[0].value =
        Amount::from_sats(500_000_000 - required_fee).expect("valid output value");
    let mut mempool = Mempool::new(PolicyConfig::default());
    mempool.set_rolling_mempool_fee_rate(rolling_floor);

    // Act
    let result = submit(&mut mempool, &snapshot, transaction);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn replacement_bump_uses_incremental_relay_fee() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::ZERO),
        incremental_relay_fee_rate: IncrementalRelayFeeRate::new(fee_rate(10_000)),
        ..PolicyConfig::default()
    });
    submit(&mut mempool, &snapshot, original).expect("original admission");

    // Act
    let error = submit(&mut mempool, &snapshot, replacement)
        .expect_err("incremental replacement bump should fail");

    // Assert
    assert!(matches!(
        error,
        MempoolError::ReplacementRejected { ref reason }
            if reason.contains("replacement fee bump")
    ));
}

#[test]
fn pressure_summary_exposes_typed_fee_roles() {
    // Arrange
    let static_floor = StaticRelayFeeRate::new(fee_rate(1_000));
    let incremental_rate = IncrementalRelayFeeRate::new(fee_rate(2_000));
    let rolling_floor = RollingMempoolFeeRate::new(fee_rate(3_000));
    let mut mempool = Mempool::new(PolicyConfig {
        static_relay_fee_rate: static_floor,
        incremental_relay_fee_rate: incremental_rate,
        ..PolicyConfig::default()
    });
    mempool.set_rolling_mempool_fee_rate(rolling_floor);

    // Act
    let summary = mempool.pressure_summary();

    // Assert
    assert_eq!(mempool.rolling_mempool_fee_rate(), rolling_floor);
    assert_eq!(summary.static_relay_fee_rate, static_floor);
    assert_eq!(summary.incremental_relay_fee_rate, incremental_rate);
    assert_eq!(summary.rolling_mempool_fee_rate, rolling_floor);
    assert_eq!(
        summary.effective_admission_fee_rate,
        effective_admission_fee_rate(static_floor, rolling_floor)
    );
}
