// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/doc/policy/packages.md

//! Rolling mempool fee state for pressure bumps and block-gated decay.

use crate::context::PolicyTime;
use crate::fee::{FeeRate, RollingMempoolFeeRate};

/// Mutable rolling-fee machine used by pressure trim and (later) block-gated decay.
#[derive(Debug, Clone, PartialEq)]
pub struct RollingFeeState {
    rolling_fee_rate: RollingMempoolFeeRate,
    /// Floating representation retained for Plan 02 occupancy-sensitive decay.
    rolling_minimum_fee_rate_f64: f64,
    block_since_last_rolling_fee_bump: bool,
    last_rolling_fee_update: PolicyTime,
}

impl Default for RollingFeeState {
    fn default() -> Self {
        Self::new()
    }
}

impl RollingFeeState {
    /// Creates the restart baseline with a zero rolling floor.
    pub const fn new() -> Self {
        Self {
            rolling_fee_rate: RollingMempoolFeeRate::ZERO,
            rolling_minimum_fee_rate_f64: 0.0,
            block_since_last_rolling_fee_bump: true,
            last_rolling_fee_update: PolicyTime::new(0),
        }
    }

    /// Returns the exposed integer rolling floor.
    pub const fn rolling_fee_rate(&self) -> RollingMempoolFeeRate {
        self.rolling_fee_rate
    }

    /// Returns whether a connected block has opened the decay gate since the last bump.
    pub const fn block_since_last_rolling_fee_bump(&self) -> bool {
        self.block_since_last_rolling_fee_bump
    }

    /// Returns the last injected update time used by decay.
    pub const fn last_rolling_fee_update(&self) -> PolicyTime {
        self.last_rolling_fee_update
    }

    /// Test/inject seam that installs a rolling floor without claiming production ownership.
    pub fn set_rolling_fee_rate(&mut self, rate: RollingMempoolFeeRate) {
        self.rolling_fee_rate = rate;
        self.rolling_minimum_fee_rate_f64 = rate.fee_rate().sats_per_kvb() as f64;
    }

    /// Knots `trackPackageRemoved`: bump only when strictly greater, then clear the block gate.
    pub fn track_package_removed(&mut self, package_plus_incremental: FeeRate) {
        if package_plus_incremental <= self.rolling_fee_rate.fee_rate() {
            return;
        }

        self.rolling_fee_rate = RollingMempoolFeeRate::new(package_plus_incremental);
        self.rolling_minimum_fee_rate_f64 = package_plus_incremental.sats_per_kvb() as f64;
        self.block_since_last_rolling_fee_bump = false;
    }
}

#[cfg(test)]
mod tests {
    use super::RollingFeeState;
    use crate::context::PolicyTime;
    use crate::fee::{FeeRate, RollingMempoolFeeRate};

    #[test]
    fn default_matches_new_baseline() {
        // Arrange / Act
        let default_state = RollingFeeState::default();
        let new_state = RollingFeeState::new();

        // Assert
        assert_eq!(default_state, new_state);
        assert_eq!(
            default_state.rolling_fee_rate(),
            RollingMempoolFeeRate::ZERO
        );
        assert!(default_state.block_since_last_rolling_fee_bump());
        assert_eq!(default_state.last_rolling_fee_update(), PolicyTime::new(0));
    }

    #[test]
    fn track_package_removed_bumps_only_when_strictly_greater() {
        // Arrange
        let mut state = RollingFeeState::new();
        let below_or_equal = FeeRate::from_sats_per_kvb(0);
        let higher = FeeRate::from_sats_per_kvb(2_500);

        // Act
        state.track_package_removed(below_or_equal);
        let unchanged = state.clone();
        state.track_package_removed(higher);

        // Assert
        assert_eq!(unchanged.rolling_fee_rate(), RollingMempoolFeeRate::ZERO);
        assert!(unchanged.block_since_last_rolling_fee_bump());
        assert_eq!(state.rolling_fee_rate(), RollingMempoolFeeRate::new(higher));
        assert!(!state.block_since_last_rolling_fee_bump());
        assert_eq!(state.last_rolling_fee_update(), PolicyTime::new(0));
    }

    #[test]
    fn set_rolling_fee_rate_updates_inject_seam() {
        // Arrange
        let mut state = RollingFeeState::default();
        let rate = RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(7_000));

        // Act
        state.set_rolling_fee_rate(rate);

        // Assert
        assert_eq!(state.rolling_fee_rate(), rate);
    }
}
