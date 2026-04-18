#![forbid(unsafe_code)]

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
