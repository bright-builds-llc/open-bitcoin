// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::{
    BuildProvenanceInputs, StatusCollectorInput, StatusDetectionEvidence,
    StatusLiveRpcAdapterInput, StatusRenderMode, StatusRequest, StatusRpcAuthSource,
    StatusRpcClient, StatusRpcError, StatusWalletRpcAccess, build_provenance_from_inputs,
    collect_status_snapshot, render_status, resolve_status_wallet_rpc_access,
    service_status::service_lifecycle_from_snapshot,
};
use crate::operator::{
    NetworkSelection,
    config::{
        OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigResolution,
        OperatorConfigSource,
    },
    detect::{
        DetectedInstallation, DetectionConfidence, DetectionSourcePath, DetectionSourcePathKind,
        DetectionUncertainty, ProductFamily, ServiceCandidate, ServiceManager, WalletCandidate,
        WalletCandidateKind,
    },
    service::{
        ServiceError, ServiceLifecycleState, ServiceStateSnapshot, fake::FakeServiceManager,
    },
};
use open_bitcoin_node::status::{
    BestKnownTipStatus, BlockRelayEvidenceStatus, BuildProvenance, ConfigStatus, FieldAvailability,
    INBOUND_STATUS_UNAVAILABLE_REASON, InboundAddressDecisionEvent, InboundAddressEvidenceEntry,
    InboundAdmissionEvent, InboundHandshakeStatusCounts, InboundPeerPolicyEvent,
    InboundPeerServingStatus, InboundPermissionDecisionEvent, MempoolStatus, NodeRuntimeState,
    NodeStatus, OpenBitcoinStatusSnapshot, PeerCounts, PeerStatus, ServiceLifecycleStatus,
    ServiceStatus, StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets,
    SyncProgressSignal, SyncStatus, SyncStopReasonStatus, WalletFreshness, WalletScanProgress,
    WalletStatus,
    relay_evidence::{
        RelayActivationEvidence, RelayCapabilityEvidence, RelayDownloadEligibilityCounters,
        RelayEvidenceCapability, RelayEvidenceCounters, RelayEvidenceField, RelayEvidenceStatus,
        RelayRecoveryCounters,
    },
};
use open_bitcoin_node::storage::FJALL_LOCK_FILE_NAME;
use open_bitcoin_node::{MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus};
use open_bitcoin_rpc::{
    RpcErrorCode, RpcErrorDetail,
    method::{
        GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse,
        GetNetworkInfoResponse, GetWalletInfoResponse, OpenBitcoinNetworkStatusResponse,
        WalletBalanceDetails,
    },
};

mod status_inputs;
use status_inputs::*;
mod relay_and_filesystem_fixtures;
use relay_and_filesystem_fixtures::*;
mod inbound_recovery_wallet;
mod rendering_and_service;
mod restart_resume;
mod service_manager;
mod snapshot;
