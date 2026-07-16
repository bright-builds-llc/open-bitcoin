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
pub mod recovery;
pub mod status;
pub mod storage;
pub mod sync;
pub mod wallet;
pub mod wallet_registry;

pub use chainstate::{ChainstateStore, ManagedChainstate, MemoryChainstateStore};
pub use logging::{LogRetentionPolicy, LogStatus};
pub use mempool::ManagedMempool;
pub use metrics::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus, block_relay_metric_samples,
    inbound_metric_samples, relay_metric_samples,
};
pub use network::{
    ManagedAddressBoundaryInfo, ManagedInboundAdmissionInfo, ManagedInboundPermissionDecisionInfo,
    ManagedNetworkError, ManagedNetworkInfo, ManagedPeerNetwork, ManagedPeerPolicyInfo,
};
pub use open_bitcoin_core as core;
pub use recovery::{
    LockEvidence, LockEvidenceKind, RECOVERY_EVIDENCE_UNAVAILABLE_REASON, RecoveryActionClass,
    RecoveryCause, RecoveryClassifierInput, RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
    classify_recovery,
};
pub use status::{
    BuildProvenance, ConfigStatus, DurableSyncState, FieldAvailability,
    INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON, INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON,
    INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON, INBOUND_STATUS_UNAVAILABLE_REASON,
    InboundAddressDecisionEvent, InboundAddressEvidenceEntry, InboundAdmissionEvent,
    InboundHandshakeStatusCounts, InboundPeerPolicyEvent, InboundPeerServingStatus,
    InboundPermissionDecisionEvent, InboundPermissionEvidence, NoProgressThresholdEvidence,
    NoProgressThresholdState, NodeRuntimeState, OpenBitcoinStatusSnapshot,
    PeerContributionEvidence, PeerContributionKind, PeerStatus, PeerTelemetry,
    ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence, RejectedProgressActivity,
    RejectedProgressActivityKind, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
    ResourceBoundUnit, ResourceBoundUsage, ResourcePressureLevel, ResourcePressureState,
    ServiceLifecycleStatus, StallDiagnosisConfidence, StallDiagnosisEvidence, StalledSubsystem,
    SyncControlState, SyncLagStatus, SyncLifecycleState, SyncProgressSignal, SyncRecoveryCategory,
    SyncResourcePressure, SyncStatus,
};
pub use storage::{
    FjallNodeStore, MetricsStorageSnapshot, PersistMode, RecoveryMarker, RuntimeMetadata,
    SchemaVersion, StorageError, StorageNamespace, StorageRecoveryAction, StoredHeaderEntries,
};
pub use sync::{
    DurableSyncRuntime, PeerCapabilitySummary, PeerContribution, PeerFailureReason,
    PeerSyncOutcome, PeerSyncState, ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress,
    SyncPeerReceiveOutcome, SyncPeerResolver, SyncPeerSession, SyncPeerSource, SyncRunSummary,
    SyncRuntimeConfig, SyncRuntimeError, SyncStopReason, SyncTransport, SystemSyncPeerResolver,
    TcpPeerTransport, WalletRescanRuntime,
};
pub use wallet::{ManagedWallet, MemoryWalletStore, WalletStore};
pub use wallet_registry::{
    SelectedWalletRecord, WalletRegistry, WalletRegistryError, WalletRegistrySnapshot,
    WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};
