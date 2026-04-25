#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure-core mempool and policy domain models for Open Bitcoin.

pub mod error;
pub mod policy;
pub mod pool;
pub mod types;

pub use error::{LimitDirection, LimitKind, MempoolError};
pub use policy::{
    dust_threshold_sats, signals_opt_in_rbf, transaction_sigops_cost,
    transaction_weight_and_virtual_size, validate_standard_transaction,
};
pub use pool::Mempool;
pub use types::{AdmissionResult, AggregateStats, FeeRate, MempoolEntry, PolicyConfig, RbfPolicy};

/// Synthetic height used for in-mempool parents.
pub const MEMPOOL_HEIGHT: u32 = 0x7fff_ffff;

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::{MEMPOOL_HEIGHT, crate_ready};

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }

    #[test]
    fn mempool_height_matches_the_expected_sentinel() {
        assert_eq!(MEMPOOL_HEIGHT, 0x7fff_ffff);
    }
}
