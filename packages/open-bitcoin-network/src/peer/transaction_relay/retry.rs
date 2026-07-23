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

#[cfg(test)]
mod tests {
    use super::{RetryDecisionContext, RetryJitterRangeError, RetryJitterSeconds};

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
}
