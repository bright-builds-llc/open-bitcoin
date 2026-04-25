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

//! Pure-core domain crate for Open Bitcoin.

pub use open_bitcoin_chainstate as chainstate;
pub use open_bitcoin_codec as codec;
pub use open_bitcoin_consensus as consensus;
pub use open_bitcoin_mempool as mempool;
pub use open_bitcoin_network as network;
pub use open_bitcoin_primitives as primitives;
pub use open_bitcoin_wallet as wallet;

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::crate_ready;

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }
}
