// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    BuildProvenance, LogStatus, MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus,
    OpenBitcoinStatusSnapshot, RecoveryActionClass, RecoveryCause, RecoveryEvidenceBasis,
    RecoveryEvidenceSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, BlockRelayEvidenceStatus, ChainTipStatus,
        ConfigStatus, FieldAvailability, InboundAddressDecisionEvent, InboundAddressEvidenceEntry,
        InboundAdmissionEvent, InboundHandshakeStatusCounts, InboundPeerPolicyEvent,
        InboundPeerServingStatus, InboundPermissionDecisionEvent, InboundResourceGovernanceEvent,
        MempoolStatus, NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState,
        NodeRuntimeState, NodeStatus, PeerContributionEvidence, PeerContributionKind, PeerCounts,
        PeerStatus, PeerTipAgreement, PeerTipAgreementStatus, ProgressCreditEvidence,
        ProgressCreditKind, ProgressWindowEvidence, RejectedProgressActivity,
        RejectedProgressActivityKind, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
        ResourceBoundUnit, ServiceLifecycleStatus, ServiceStatus, StallDiagnosisConfidence,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncAttemptCounters,
        SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncRecoveryCategory, SyncResourcePressure, SyncStatus, SyncStopReasonStatus,
        TipFreshnessStatus, WalletStatus, inbound_status_unavailable,
        relay_evidence::{
            RelayActivationEvidence, RelayCapabilityEvidence, RelayDownloadEligibilityCounters,
            RelayEvidenceCapability, RelayEvidenceCounters, RelayEvidenceField,
            RelayEvidenceStatus, RelayRecoveryCounters,
        },
        usage_against_budget,
    },
};
use open_bitcoin_rpc::method::{
    GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse, GetNetworkInfoResponse,
    GetWalletInfoResponse, OpenBitcoinNetworkStatusResponse,
};
use serde_json::json;

use crate::operator::{
    OperatorOutputFormat, SupportArgs, SupportBundleArgs, SupportCommand,
    config::OperatorConfigResolution,
    soak::{
        SoakBounds, SoakPeerPolicy, SoakRunId, SoakStopCondition,
        ledger::{
            SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerEventEnvelope,
            SoakLedgerLayout, SoakRunIndex, SoakRunIndexEntry,
        },
        outcome::SoakOutcomeLabel,
        report::write_soak_reports,
    },
    status::{
        StatusCollectorInput, StatusDetectionEvidence, StatusRenderMode, StatusRequest,
        StatusRpcClient, StatusRpcError, StatusWalletRpcAccess, collect_status_snapshot,
    },
};

use super::{
    ConfigEvidence, EvidenceAvailability, EvidenceState, LiveSmokeEvidence, MetricsHistoryEvidence,
    RecoverySupportEvidence, RuntimeMetadataEvidence, StoreHealthEvidence, SupportEvidenceBundle,
    SupportEvidenceOutput, collect_resource_bound_support_evidence, collect_soak_support_evidence,
    collect_store_health, derive_full_sync_evidence, evidence::SupportEvidenceVerdict,
    execute_support_command, forensics::SupportForensicsEvidence, redaction_summary, render,
    soak_outcome_label, support_status_for_bundle,
};

mod sync_fixtures;
use sync_fixtures::*;
mod inbound_status_fixtures;
use inbound_status_fixtures::*;
mod soak_forensics_fixtures;
use soak_forensics_fixtures::*;
mod forensics_recovery_relay;
mod inbound;
mod recovery_progress_inbound;
mod sync_soak_forensics;
