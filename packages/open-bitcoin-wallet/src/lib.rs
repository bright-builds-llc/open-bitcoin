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

//! Pure-core wallet models for Open Bitcoin.

pub mod address;
pub mod descriptor;
pub mod error;
pub mod wallet;

pub use address::{Address, AddressNetwork, AddressType, PrivateKey};
pub use descriptor::{
    DescriptorKind, DescriptorRecord, DescriptorRole, KeySource, SingleKeyDescriptor,
    TaprootKeySource,
};
pub use error::WalletError;
pub use wallet::{
    BuildRequest, BuiltTransaction, Recipient, Wallet, WalletBalance, WalletSnapshot, WalletUtxo,
};

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
