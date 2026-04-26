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
pub mod logging;
pub mod mempool;
pub mod metrics;
pub mod network;
pub mod status;
pub mod storage;
pub mod wallet;

pub use chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};
pub use logging::{LogRetentionPolicy, LogStatus};
pub use mempool::ManagedMempool;
pub use metrics::{MetricRetentionPolicy, MetricsStatus};
pub use network::{ManagedNetworkError, ManagedPeerNetwork};
pub use open_bitcoin_core as core;
pub use status::{
    BuildProvenance, ConfigStatus, FieldAvailability, NodeRuntimeState, OpenBitcoinStatusSnapshot,
    PeerStatus, SyncStatus,
};
pub use storage::{
    PersistMode, SchemaVersion, StorageError, StorageNamespace, StorageRecoveryAction,
};
pub use wallet::{ManagedWallet, MemoryWalletStore, WalletStore};
