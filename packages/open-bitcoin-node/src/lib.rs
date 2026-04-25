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

//! Shell/runtime crate for Open Bitcoin adapters and orchestration.

pub mod chainstate;
pub mod mempool;
pub mod network;
pub mod wallet;

pub use chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};
pub use mempool::ManagedMempool;
pub use network::{ManagedNetworkError, ManagedPeerNetwork};
pub use open_bitcoin_core as core;
pub use wallet::{ManagedWallet, MemoryWalletStore, WalletStore};
