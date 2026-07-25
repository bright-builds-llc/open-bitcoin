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

//! Rolling mempool fee state for pressure bumps and block-gated decay.

use crate::context::PolicyTime;
use crate::fee::{FeeRate, IncrementalRelayFeeRate, RollingMempoolFeeRate};
use crate::resource::{AccountedMempoolMemory, MempoolCapacity};

/// Knots `CTxMemPool::ROLLING_FEE_HALFLIFE` — default 12-hour half-life in seconds.
pub const ROLLING_FEE_HALFLIFE_SECONDS: i64 = 60 * 60 * 12;

/// Knots `GetMinFee` minimum elapsed seconds before applying another decay step.
pub const ROLLING_FEE_UPDATE_INTERVAL_SECONDS: i64 = 10;

/// Mutable rolling-fee machine used by pressure trim and block-gated decay.
#[derive(Debug, Clone, PartialEq)]
pub struct RollingFeeState {
    rolling_fee_rate: RollingMempoolFeeRate,
    /// Floating representation retained for occupancy-sensitive decay (Knots `double`).
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

    /// Knots `removeForBlock` rolling-fee side effects: open the decay gate and refresh the clock.
    pub fn open_decay_gate_after_block(&mut self, connected_at: PolicyTime) {
        self.block_since_last_rolling_fee_bump = true;
        self.last_rolling_fee_update = connected_at;
    }

    /// Knots `GetMinFee` decay step with injected time and occupancy.
    ///
    /// Returns the llround-equivalent integer rolling floor. Unlike Knots, this does **not**
    /// raise the return value to the incremental relay fee — Open Bitcoin keeps incremental out
    /// of ordinary admission (`max(static, rolling)` only). Rolling still collapses to zero when
    /// the internal rate falls below `incremental / 2`.
    pub fn decay_toward(
        &mut self,
        now: PolicyTime,
        usage: AccountedMempoolMemory,
        capacity: MempoolCapacity,
        incremental: IncrementalRelayFeeRate,
    ) -> RollingMempoolFeeRate {
        if !self.block_since_last_rolling_fee_bump || self.rolling_minimum_fee_rate_f64 == 0.0 {
            return self.sync_exposed_rolling_rate();
        }

        let now_secs = now.unix_seconds();
        let last_secs = self.last_rolling_fee_update.unix_seconds();
        if now_secs > last_secs + ROLLING_FEE_UPDATE_INTERVAL_SECONDS {
            let mut halflife = ROLLING_FEE_HALFLIFE_SECONDS as f64;
            let usage_bytes = usage.as_usize();
            let capacity_bytes = capacity.as_usize();
            if usage_bytes < capacity_bytes / 4 {
                halflife /= 4.0;
            } else if usage_bytes < capacity_bytes / 2 {
                halflife /= 2.0;
            }

            let dt = (now_secs - last_secs) as f64;
            self.rolling_minimum_fee_rate_f64 /= 2.0_f64.powf(dt / halflife);
            self.last_rolling_fee_update = now;

            let incremental_half = incremental.fee_rate().sats_per_kvb() as f64 / 2.0;
            if self.rolling_minimum_fee_rate_f64 < incremental_half {
                self.rolling_minimum_fee_rate_f64 = 0.0;
            }
        }

        self.sync_exposed_rolling_rate()
    }

    fn sync_exposed_rolling_rate(&mut self) -> RollingMempoolFeeRate {
        let rounded = llround_sats_per_kvb(self.rolling_minimum_fee_rate_f64);
        self.rolling_fee_rate = RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(rounded));
        self.rolling_fee_rate
    }
}

/// Knots `llround` for non-negative fee rates (`f64::round` is half-away-from-zero).
fn llround_sats_per_kvb(value: f64) -> i64 {
    value.round() as i64
}

#[cfg(test)]
mod tests {
    use super::{ROLLING_FEE_HALFLIFE_SECONDS, RollingFeeState, llround_sats_per_kvb};
    use crate::context::PolicyTime;
    use crate::fee::{FeeRate, IncrementalRelayFeeRate, RollingMempoolFeeRate};
    use crate::resource::{AccountedMempoolMemory, MempoolCapacity};

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

    #[test]
    fn open_decay_gate_after_block_sets_gate_and_clock() {
        // Arrange
        let mut state = RollingFeeState::new();
        state.track_package_removed(FeeRate::from_sats_per_kvb(4_000));
        let connected_at = PolicyTime::new(99);

        // Act
        state.open_decay_gate_after_block(connected_at);

        // Assert
        assert!(state.block_since_last_rolling_fee_bump());
        assert_eq!(state.last_rolling_fee_update(), connected_at);
    }

    #[test]
    fn llround_matches_half_away_from_zero_for_positive_rates() {
        assert_eq!(llround_sats_per_kvb(2.5), 3);
        assert_eq!(llround_sats_per_kvb(2.4), 2);
        assert_eq!(llround_sats_per_kvb(10_000.0 / 2.0), 5_000);
        assert_eq!(ROLLING_FEE_HALFLIFE_SECONDS, 43_200);

        let mut state = RollingFeeState::new();
        state.track_package_removed(FeeRate::from_sats_per_kvb(8_000));
        state.open_decay_gate_after_block(PolicyTime::new(0));
        let decayed = state.decay_toward(
            PolicyTime::new(ROLLING_FEE_HALFLIFE_SECONDS),
            AccountedMempoolMemory::new(100),
            MempoolCapacity::new(100),
            IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
        );
        assert_eq!(decayed.fee_rate().sats_per_kvb(), 4_000);
    }
}
