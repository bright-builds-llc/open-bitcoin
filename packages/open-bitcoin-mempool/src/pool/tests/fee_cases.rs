// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use crate::{
    FeeRate, IncrementalRelayFeeRate, RollingMempoolFeeRate, StaticRelayFeeRate,
    effective_admission_fee_rate, evaluate_package_fee_floors,
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
