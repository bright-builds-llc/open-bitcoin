#![forbid(unsafe_code)]

//! Shell/runtime crate for Open Bitcoin adapters and orchestration.

pub mod chainstate;
pub mod mempool;
pub mod network;

pub use chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};
pub use mempool::ManagedMempool;
pub use network::{ManagedNetworkError, ManagedPeerNetwork};
pub use open_bitcoin_core as core;
