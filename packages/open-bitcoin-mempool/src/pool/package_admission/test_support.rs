// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Objective package-policy stage and trim probes for deterministic tests.

use std::cell::Cell;

use super::{
    FORCE_RESIDUAL_FEE_GROUP_ERROR, FORCE_RESIDUAL_FEE_GROUP_HARD, FORCE_RESIDUAL_ROLLING_ERROR,
    FORCE_RESIDUAL_ROLLING_HARD, FORCE_RESIDUAL_ZERO_ROLLING, FORCE_SINGLETON_ROLLING_ERROR,
    FORCE_SINGLETON_ROLLING_HARD, FORCE_SINGLETON_STATIC_ERROR, FORCE_SINGLETON_ZERO_ROLLING,
    Mempool, PACKAGE_TRIM_COUNT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::pool) enum PackagePolicyStage {
    Static,
    Truc,
    Rolling,
    Limits,
    Replacement,
    Ephemeral,
    Scripts,
    Trim,
}

pub(in crate::pool) fn reset_package_trim_count_for_test() {
    PACKAGE_TRIM_COUNT.with(|count| count.set(0));
}

pub(in crate::pool) fn package_trim_count_for_test() -> usize {
    PACKAGE_TRIM_COUNT.with(Cell::get)
}

pub(in crate::pool) fn set_mempool_capacity_for_test(
    mempool: &mut Mempool,
    capacity: crate::MempoolCapacity,
) {
    mempool.config.mempool_capacity = capacity;
}

pub(in crate::pool) fn force_residual_fee_group_error_for_test(force: bool) {
    FORCE_RESIDUAL_FEE_GROUP_ERROR.with(|value| value.set(force));
}

pub(in crate::pool) fn force_residual_fee_group_hard_for_test(force: bool) {
    FORCE_RESIDUAL_FEE_GROUP_HARD.with(|value| value.set(force));
}

pub(in crate::pool) fn force_staged_fee_branches_for_test(
    singleton_zero: bool,
    residual_zero: bool,
    singleton_rolling_hard: bool,
    residual_rolling_hard: bool,
) {
    FORCE_SINGLETON_ZERO_ROLLING.with(|value| value.set(singleton_zero));
    FORCE_RESIDUAL_ZERO_ROLLING.with(|value| value.set(residual_zero));
    FORCE_SINGLETON_ROLLING_HARD.with(|value| value.set(singleton_rolling_hard));
    FORCE_RESIDUAL_ROLLING_HARD.with(|value| value.set(residual_rolling_hard));
}

pub(in crate::pool) fn force_staged_fee_errors_for_test(
    singleton_static: bool,
    singleton_rolling: bool,
    residual_rolling: bool,
) {
    FORCE_SINGLETON_STATIC_ERROR.with(|value| value.set(singleton_static));
    FORCE_SINGLETON_ROLLING_ERROR.with(|value| value.set(singleton_rolling));
    FORCE_RESIDUAL_ROLLING_ERROR.with(|value| value.set(residual_rolling));
}

pub(in crate::pool) fn package_policy_probe_for_test(
    maybe_failure: Option<PackagePolicyStage>,
) -> (Vec<PackagePolicyStage>, usize, usize) {
    let mut trace = Vec::new();
    let mut scripts = 0;
    let mut trims = 0;
    for stage in [
        PackagePolicyStage::Static,
        PackagePolicyStage::Truc,
        PackagePolicyStage::Rolling,
        PackagePolicyStage::Limits,
        PackagePolicyStage::Replacement,
        PackagePolicyStage::Ephemeral,
        PackagePolicyStage::Scripts,
        PackagePolicyStage::Trim,
    ] {
        trace.push(stage);
        if stage == PackagePolicyStage::Scripts {
            scripts += 1;
        }
        if maybe_failure == Some(stage) {
            break;
        }
        if stage == PackagePolicyStage::Trim {
            trims += 1;
        }
    }
    (trace, scripts, trims)
}
