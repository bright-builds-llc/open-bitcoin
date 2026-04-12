#![forbid(unsafe_code)]

//! Shell/runtime crate for Open Bitcoin adapters and orchestration.

pub mod chainstate;

pub use chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};
pub use open_bitcoin_core as core;
