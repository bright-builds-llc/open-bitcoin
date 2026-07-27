// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::relay_evidence::{
    RelayActivationEvidence, RelayCapabilityEvidence, RelayDownloadEligibilityCounters,
    RelayEvidenceCapability, RelayEvidenceCounters, RelayEvidenceField, RelayEvidenceStatus,
    RelayRecoveryCounters,
};
use super::{
    BestKnownTipStatus, BlockRelayEvidenceStatus, BuildProvenance, ChainTipStatus, ConfigStatus,
    FieldAvailability, HealthSignal, HealthSignalLevel, INBOUND_STATUS_UNAVAILABLE_REASON,
    InboundAddressDecisionEvent, InboundAddressEvidenceEntry, InboundHandshakeStatusCounts,
    InboundPeerServingStatus, LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
    LAST_USEFUL_WORK_UNAVAILABLE_REASON, MempoolStatus, NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
    NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON, NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
    NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState, NodeRuntimeState,
    NodeStatus, OpenBitcoinStatusSnapshot, PROGRESS_CREDIT_UNAVAILABLE_REASON,
    PeerContributionEvidence, PeerContributionKind, PeerCounts, PeerStatus, PeerTelemetry,
    ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence,
    RESOURCE_BOUND_STOP_PERCENT, RESOURCE_BOUND_WARNING_PERCENT, RejectedProgressActivity,
    RejectedProgressActivityKind, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
    ResourceBoundUnit, ResourcePressureState, STALL_DIAGNOSIS_UNAVAILABLE_REASON,
    ServiceLifecycleStatus, ServicePriorShutdownStatus, ServiceRestartResumeStatus,
    ServiceStaleInflightStatus, ServiceStatus, StallDiagnosisConfidence, StallDiagnosisEvidence,
    StalledSubsystem, StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus,
    SyncLifecycleState, SyncProgress, SyncProgressSignal, SyncReconcileProgressStatus,
    SyncReorgEvidence, SyncResourcePressure, SyncStatus, SyncStopReasonStatus, WalletFreshness,
    WalletScanProgress, WalletStatus, classify_budget_pressure,
    classify_snapshot_against_disk_budget, usage_against_budget,
};
use crate::recovery::{
    LockEvidence, LockEvidenceKind, RECOVERY_EVIDENCE_UNAVAILABLE_REASON, RecoveryActionClass,
    RecoveryCause, RecoveryEvidenceSnapshot,
};
use crate::{LogStatus, MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus};

fn stopped_snapshot() -> OpenBitcoinStatusSnapshot {
    let unavailable = "node stopped";
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Stopped,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::unavailable("service manager not inspected"),
            lifecycle: FieldAvailability::unavailable("service manager not inspected"),
            installed: FieldAvailability::unavailable("service manager not inspected"),
            enabled: FieldAvailability::unavailable("service manager not inspected"),
            running: FieldAvailability::unavailable("service manager not inspected"),
            service_file_path: FieldAvailability::unavailable("service file path unavailable"),
            log_path: FieldAvailability::unavailable("service log path unavailable"),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: SyncStatus {
            network: FieldAvailability::unavailable(unavailable),
            chain_tip: FieldAvailability::unavailable(unavailable),
            sync_progress: FieldAvailability::unavailable(unavailable),
            lifecycle: FieldAvailability::unavailable(unavailable),
            phase: FieldAvailability::unavailable(unavailable),
            configured_targets: FieldAvailability::unavailable(unavailable),
            attempt_counters: FieldAvailability::unavailable(unavailable),
            progress_signal: FieldAvailability::unavailable(unavailable),
            lag: FieldAvailability::unavailable(unavailable),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(unavailable),
            progress_credit: FieldAvailability::unavailable(PROGRESS_CREDIT_UNAVAILABLE_REASON),
            expected_progress_window: FieldAvailability::unavailable(
                super::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON,
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
            ),
            last_useful_work: FieldAvailability::unavailable(LAST_USEFUL_WORK_UNAVAILABLE_REASON),
            last_peer_contribution: FieldAvailability::unavailable(
                LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
            ),
            stall_diagnosis: FieldAvailability::unavailable(STALL_DIAGNOSIS_UNAVAILABLE_REASON),
            latest_stop_reason: FieldAvailability::unavailable(unavailable),
            last_error: FieldAvailability::unavailable(unavailable),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable(unavailable),
            resource_pressure: FieldAvailability::unavailable(unavailable),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(unavailable),
            stay_current: FieldAvailability::unavailable(unavailable),
            stay_current_next_action: FieldAvailability::unavailable(unavailable),
            no_progress_diagnosis: FieldAvailability::unavailable(
                NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON,
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::unavailable(unavailable),
            recent_peers: FieldAvailability::unavailable(unavailable),
            inbound: FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable(unavailable),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable(unavailable),
            freshness: FieldAvailability::unavailable(unavailable),
            scan_progress: FieldAvailability::unavailable(unavailable),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}

mod availability_and_relay;
mod peer_metrics_projection;
mod progress_guarantee_and_reorg;
mod resource_recovery;
mod service_lifecycle;
mod snapshot_availability;
mod snapshot_projection;
mod sync_truth_and_no_progress;
