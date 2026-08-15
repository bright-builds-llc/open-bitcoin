// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp

use std::{error::Error, fmt};

const MAX_RETRY_JITTER_SECONDS: u64 = 300;

/// Validated variable delay for one initial-broadcast retry cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RetryJitterSeconds(u64);

impl RetryJitterSeconds {
    /// Validates an injected jitter sample in the inclusive `0..=300` range.
    pub const fn new(seconds: u64) -> Result<Self, RetryJitterRangeError> {
        if seconds > MAX_RETRY_JITTER_SECONDS {
            return Err(RetryJitterRangeError);
        }

        Ok(Self(seconds))
    }

    /// Returns the validated jitter sample in seconds.
    pub const fn seconds(self) -> u64 {
        self.0
    }
}

/// Reports that an injected retry jitter sample exceeded its fixed bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryJitterRangeError;

impl RetryJitterRangeError {
    /// Returns the fixed low-cardinality error label.
    pub const fn as_str(self) -> &'static str {
        "retry_jitter_out_of_range"
    }
}

impl fmt::Display for RetryJitterRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Error for RetryJitterRangeError {}

/// Immutable time and jitter facts supplied to pure retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryDecisionContext {
    pub observed_at_unix_seconds: i64,
    pub jitter: RetryJitterSeconds,
}

impl RetryDecisionContext {
    /// Retains shell-sampled retry facts without acquiring effects.
    pub const fn new(observed_at_unix_seconds: i64, jitter: RetryJitterSeconds) -> Self {
        Self {
            observed_at_unix_seconds,
            jitter,
        }
    }
}

/// Ten-minute base for one process-global initial-broadcast retry cycle.
pub const RETRY_CYCLE_BASE_SECONDS: u64 = 600;

/// Per-tick inspect work cap: at most this many unbroadcast identities are
/// considered on one maintenance tick.
///
/// This is not the `MAX_UNBROADCAST_MEMBERS` / `5000` membership cap, and it
/// is not `PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER` or
/// `PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER`.
pub const MAINTENANCE_INSPECT_BUDGET: usize = 256;

/// Per-tick prepare work cap: at most this many inspected identities are
/// prepared for existing fanout on one maintenance tick.
///
/// This is not the `MAX_UNBROADCAST_MEMBERS` / `5000` membership cap, and it
/// is not `PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER` or
/// `PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER`.
pub const MAINTENANCE_PREPARE_BUDGET: usize = 32;

const MAINTENANCE_BUDGET_MEMBERSHIP_CAP: usize = 5_000;

/// Returns the process-global retry cycle length as ten minutes plus injected jitter.
pub fn retry_cycle_length_seconds(jitter: RetryJitterSeconds) -> u64 {
    RETRY_CYCLE_BASE_SECONDS.saturating_add(jitter.seconds())
}

/// Returns the next process-global due time, or `None` when addition overflows.
pub fn next_retry_due_unix_seconds(context: RetryDecisionContext) -> Option<i64> {
    let cycle_seconds = retry_cycle_length_seconds(context.jitter);
    context
        .observed_at_unix_seconds
        .checked_add(cycle_seconds as i64)
}

/// Reports whether an injected observation has reached a previously computed due time.
pub const fn retry_cycle_is_due(observed_at_unix_seconds: i64, due_at_unix_seconds: i64) -> bool {
    observed_at_unix_seconds >= due_at_unix_seconds
}

/// Validated per-tick inspect budget independent of membership and PHASE104 caps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaintenanceInspectBudget(usize);

impl MaintenanceInspectBudget {
    /// Returns the production inspect work cap.
    pub const fn production() -> Self {
        Self(MAINTENANCE_INSPECT_BUDGET)
    }

    /// Accepts `1..=4999` so tests can use inspect=`3` while rejecting `0` and `>= 5000`.
    pub const fn new(value: usize) -> Result<Self, MaintenanceBudgetRangeError> {
        if !maintenance_budget_in_range(value) {
            return Err(MaintenanceBudgetRangeError);
        }

        Ok(Self(value))
    }

    /// Returns the validated inspect budget.
    pub const fn get(self) -> usize {
        self.0
    }
}

/// Validated per-tick prepare budget independent of membership and PHASE104 caps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaintenancePrepareBudget(usize);

impl MaintenancePrepareBudget {
    /// Returns the production prepare work cap.
    pub const fn production() -> Self {
        Self(MAINTENANCE_PREPARE_BUDGET)
    }

    /// Accepts `1..=4999` so tests can use prepare=`1` while rejecting `0` and `>= 5000`.
    pub const fn new(value: usize) -> Result<Self, MaintenanceBudgetRangeError> {
        if !maintenance_budget_in_range(value) {
            return Err(MaintenanceBudgetRangeError);
        }

        Ok(Self(value))
    }

    /// Returns the validated prepare budget.
    pub const fn get(self) -> usize {
        self.0
    }
}

const fn maintenance_budget_in_range(value: usize) -> bool {
    value != 0 && value < MAINTENANCE_BUDGET_MEMBERSHIP_CAP
}

/// Reports that an inspect or prepare budget was `0` or at least the 5,000-member cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaintenanceBudgetRangeError;

impl MaintenanceBudgetRangeError {
    /// Returns the fixed low-cardinality error label.
    pub const fn as_str(self) -> &'static str {
        "maintenance_budget_out_of_range"
    }
}

impl fmt::Display for MaintenanceBudgetRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Error for MaintenanceBudgetRangeError {}

#[cfg(test)]
mod tests {
    use super::{
        MAINTENANCE_INSPECT_BUDGET, MAINTENANCE_PREPARE_BUDGET, MaintenanceBudgetRangeError,
        MaintenanceInspectBudget, MaintenancePrepareBudget, RetryDecisionContext,
        RetryJitterRangeError, RetryJitterSeconds, next_retry_due_unix_seconds, retry_cycle_is_due,
        retry_cycle_length_seconds,
    };

    #[test]
    fn retry_jitter_accepts_inclusive_bounds() {
        // Arrange
        let minimum_seconds = 0;
        let maximum_seconds = 300;

        // Act
        let minimum = RetryJitterSeconds::new(minimum_seconds);
        let maximum = RetryJitterSeconds::new(maximum_seconds);

        // Assert
        assert_eq!(
            minimum.map(RetryJitterSeconds::seconds),
            Ok(minimum_seconds)
        );
        assert_eq!(
            maximum.map(RetryJitterSeconds::seconds),
            Ok(maximum_seconds)
        );
    }

    #[test]
    fn retry_jitter_rejects_value_above_maximum() {
        // Arrange
        let above_maximum_seconds = 301;

        // Act
        let result = RetryJitterSeconds::new(above_maximum_seconds);

        // Assert
        assert_eq!(result, Err(RetryJitterRangeError));
        assert_eq!(RetryJitterRangeError.as_str(), "retry_jitter_out_of_range");
        assert_eq!(
            RetryJitterRangeError.to_string(),
            "retry_jitter_out_of_range"
        );
    }

    #[test]
    fn retry_context_retains_injected_observation_and_jitter() -> Result<(), RetryJitterRangeError>
    {
        // Arrange
        let observed_at_unix_seconds = -42;
        let jitter = RetryJitterSeconds::new(173)?;

        // Act
        let context = RetryDecisionContext::new(observed_at_unix_seconds, jitter);

        // Assert
        assert_eq!(
            context,
            RetryDecisionContext {
                observed_at_unix_seconds,
                jitter,
            }
        );
        Ok(())
    }

    #[test]
    fn retry_cycle_length_is_ten_minutes_plus_injected_jitter() -> Result<(), RetryJitterRangeError>
    {
        // Arrange
        let zero_jitter = RetryJitterSeconds::new(0)?;
        let max_jitter = RetryJitterSeconds::new(300)?;

        // Act
        let zero_cycle_seconds = retry_cycle_length_seconds(zero_jitter);
        let max_cycle_seconds = retry_cycle_length_seconds(max_jitter);

        // Assert
        assert_eq!(zero_cycle_seconds, 600);
        assert_eq!(max_cycle_seconds, 900);
        Ok(())
    }

    #[test]
    fn next_retry_due_uses_checked_add_and_does_not_sample_time()
    -> Result<(), RetryJitterRangeError> {
        // Arrange
        let due_context = RetryDecisionContext {
            observed_at_unix_seconds: 1_000,
            jitter: RetryJitterSeconds::new(150)?,
        };
        let overflow_context = RetryDecisionContext {
            observed_at_unix_seconds: i64::MAX,
            jitter: RetryJitterSeconds::new(0)?,
        };

        // Act
        let maybe_due = next_retry_due_unix_seconds(due_context);
        let maybe_overflow = next_retry_due_unix_seconds(overflow_context);

        // Assert
        assert_eq!(maybe_due, Some(1_750));
        assert_eq!(maybe_overflow, None);
        Ok(())
    }

    #[test]
    fn retry_cycle_is_due_is_observed_greater_or_equal() {
        // Arrange
        let due_at_unix_seconds = 1_750;

        // Act
        let before_due = retry_cycle_is_due(1_749, due_at_unix_seconds);
        let at_due = retry_cycle_is_due(due_at_unix_seconds, due_at_unix_seconds);
        let after_due = retry_cycle_is_due(1_751, due_at_unix_seconds);

        // Assert
        assert!(!before_due);
        assert!(at_due);
        assert!(after_due);
    }

    #[test]
    fn maintenance_budgets_reject_zero_and_membership_cap() {
        // Arrange
        let inspect_zero = MaintenanceInspectBudget::new(0);
        let inspect_membership_cap = MaintenanceInspectBudget::new(5_000);
        let prepare_zero = MaintenancePrepareBudget::new(0);
        let prepare_membership_cap = MaintenancePrepareBudget::new(5_000);
        let inspect_phase104_drain = MaintenanceInspectBudget::new(16);
        let inspect_cursor_proof = MaintenanceInspectBudget::new(3);
        let prepare_cursor_proof = MaintenancePrepareBudget::new(1);

        // Act
        let production_inspect = MaintenanceInspectBudget::production().get();
        let production_prepare = MaintenancePrepareBudget::production().get();

        // Assert
        assert_eq!(inspect_zero, Err(MaintenanceBudgetRangeError));
        assert_eq!(inspect_membership_cap, Err(MaintenanceBudgetRangeError));
        assert_eq!(prepare_zero, Err(MaintenanceBudgetRangeError));
        assert_eq!(prepare_membership_cap, Err(MaintenanceBudgetRangeError));
        assert_eq!(
            MaintenanceBudgetRangeError.as_str(),
            "maintenance_budget_out_of_range"
        );
        assert_eq!(
            MaintenanceBudgetRangeError.to_string(),
            "maintenance_budget_out_of_range"
        );
        assert_eq!(
            inspect_phase104_drain.map(MaintenanceInspectBudget::get),
            Ok(16)
        );
        assert_eq!(
            inspect_cursor_proof.map(MaintenanceInspectBudget::get),
            Ok(3)
        );
        assert_eq!(
            prepare_cursor_proof.map(MaintenancePrepareBudget::get),
            Ok(1)
        );
        assert_eq!(production_inspect, 256);
        assert_eq!(production_prepare, 32);
    }

    #[test]
    fn production_inspect_and_prepare_are_independent_of_phase104_caps() {
        // Arrange
        let phase104_drain_cap = 16;
        let phase104_queue_cap = 1024;
        let membership_cap = 5_000;

        // Act
        let inspect_budget = MAINTENANCE_INSPECT_BUDGET;
        let prepare_budget = MAINTENANCE_PREPARE_BUDGET;

        // Assert
        assert_ne!(inspect_budget, phase104_drain_cap);
        assert_ne!(prepare_budget, phase104_drain_cap);
        assert_ne!(inspect_budget, phase104_queue_cap);
        assert_ne!(prepare_budget, phase104_queue_cap);
        assert!(inspect_budget < membership_cap);
        assert!(prepare_budget < inspect_budget);
    }
}
