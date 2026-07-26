// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Objective package-policy stage and trim probes for deterministic tests.

use std::cell::Cell;

use super::{Mempool, PACKAGE_TRIM_COUNT};

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
