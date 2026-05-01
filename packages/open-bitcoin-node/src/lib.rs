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
pub mod sync;
pub mod wallet;
pub mod wallet_registry;

pub use chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};
pub use logging::{LogRetentionPolicy, LogStatus};
pub use mempool::ManagedMempool;
pub use metrics::{MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus};
pub use network::{ManagedNetworkError, ManagedPeerNetwork};
pub use open_bitcoin_core as core;
pub use status::{
    BuildProvenance, ConfigStatus, FieldAvailability, NodeRuntimeState, OpenBitcoinStatusSnapshot,
    PeerStatus, SyncStatus,
};
pub use storage::{
    FjallNodeStore, MetricsStorageSnapshot, PersistMode, RecoveryMarker, RuntimeMetadata,
    SchemaVersion, StorageError, StorageNamespace, StorageRecoveryAction, StoredHeaderEntries,
};
pub use sync::{
    DurableSyncRuntime, PeerCapabilitySummary, PeerContribution, PeerFailureReason,
    PeerSyncOutcome, PeerSyncState, ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress,
    SyncPeerResolver, SyncPeerSession, SyncPeerSource, SyncRunSummary, SyncRuntimeConfig,
    SyncRuntimeError, SyncTransport, SystemSyncPeerResolver, TcpPeerTransport, WalletRescanRuntime,
};
pub use wallet::{ManagedWallet, MemoryWalletStore, WalletStore};
pub use wallet_registry::{
    SelectedWalletRecord, WalletRegistry, WalletRegistryError, WalletRegistrySnapshot,
    WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};
