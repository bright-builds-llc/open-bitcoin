// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    LogStatus, MetricRetentionPolicy, MetricsStatus, RecoveryActionClass, RecoveryCause,
    RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, BlockRelayEvidenceStatus, BuildProvenance,
        ConfigStatus, FieldAvailability, MempoolStatus, NoProgressThresholdEvidence,
        NoProgressThresholdState, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
        PeerContributionEvidence, PeerContributionKind, PeerStatus, ProgressCreditEvidence,
        ProgressCreditKind, ProgressWindowEvidence, RejectedProgressActivity,
        RejectedProgressActivityKind, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
        ResourceBoundUnit, StallDiagnosisConfidence, StallDiagnosisEvidence, StalledSubsystem,
        StayCurrentStatus, SyncProgress, SyncRecoveryCategory, SyncReorgEvidence, SyncStatus,
        SyncStopReasonStatus, TipFreshnessStatus, WalletStatus, inbound_status_unavailable,
        relay_evidence::RelayEvidenceStatus, usage_against_budget,
    },
};

use super::{
    SoakBounds, SoakClock, SoakLoopMode, SoakPeerPolicy, SoakRunId, SoakStatusCollector,
    SoakStopCondition, SoakTestClock,
    ledger::{
        SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerLayout, SoakRunIndex,
        SoakRunIndexEntry,
    },
    outcome::SoakOutcomeLabel,
    run_bounded_soak_loop, runtime, validate_resume_plan, write_operator_stop,
    write_report_projection,
};
use crate::operator::OperatorOutputFormat;

#[path = "runtime/runtime_fixtures.rs"]
mod runtime_fixtures;
use runtime_fixtures::*;
#[path = "runtime/checkpoint_fixtures.rs"]
mod checkpoint_fixtures;
use checkpoint_fixtures::*;
#[path = "runtime/execution_and_checkpoints.rs"]
mod execution_and_checkpoints;
#[path = "runtime/resume_and_stop.rs"]
mod resume_and_stop;
#[path = "runtime/terminal_and_report.rs"]
mod terminal_and_report;
