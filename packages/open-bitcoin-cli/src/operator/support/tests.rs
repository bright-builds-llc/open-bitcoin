// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    BuildProvenance, LogStatus, MetricsStatus, OpenBitcoinStatusSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, ChainTipStatus, ConfigStatus, FieldAvailability,
        MempoolStatus, NoProgressDiagnosis, NodeRuntimeState, NodeStatus, PeerCounts, PeerStatus,
        PeerTipAgreement, PeerTipAgreementStatus, ServiceLifecycleStatus, ServiceStatus,
        StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus,
        SyncLifecycleState, SyncProgress, SyncProgressSignal, SyncRecoveryCategory,
        SyncResourcePressure, SyncStatus, SyncStopReasonStatus, TipFreshnessStatus, WalletStatus,
    },
};
use serde_json::json;

use super::{
    LiveSmokeEvidence, derive_full_sync_evidence, evidence::SupportEvidenceVerdict,
    redaction_summary,
};

#[test]
fn phase71_support_redaction_names_compact_evidence_bounds() {
    // Arrange
    let summary = redaction_summary();

    // Act
    let omitted = summary.omitted;
    let safeguards = summary.safeguards;

    // Assert
    assert_eq!(
        omitted,
        [
            "RPC cookie contents",
            "RPC password and RPC auth values",
            "wallet private material and raw wallet files",
            "raw unbounded log contents",
        ]
    );
    assert_eq!(
        safeguards,
        [
            "credential sources are represented as metadata only",
            "live smoke reports are summarized from allowlisted fields only",
            "logs are limited to existing structured status signals",
        ]
    );
}

#[test]
fn phase72_support_verdict_sync_to_tip_requires_validated_tip_match() {
    // Arrange
    let status = phase72_status();
    let live_smoke = missing_live_smoke();

    // Act
    let evidence = derive_full_sync_evidence(&status, &live_smoke);
    let serialized = serde_json::to_value(&evidence).expect("evidence json");

    // Assert
    assert_eq!(
        evidence.verdict.label,
        SupportEvidenceVerdict::SyncToTipProven
    );
    assert!(
        evidence
            .verdict
            .justifications
            .contains(&"validated_active_chain_matches_best_known_tip".to_string())
    );
    assert_eq!(
        serialized["connected_active_chain"]["height"],
        json!(840_004)
    );
    assert_eq!(
        serialized["connected_active_chain"]["hash"],
        json!("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        serialized["connected_active_chain"]["work"],
        json!("840005")
    );
    assert_eq!(
        serialized["validated_active_chain"]["height"],
        json!(840_004)
    );
    assert_eq!(
        serialized["validated_active_chain"]["hash"],
        json!("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        serialized["validated_active_chain"]["work"],
        json!("840005")
    );
}

#[test]
fn phase72_support_verdict_stay_current_requires_current_at_best_known_tip() {
    // Arrange
    let mut status = phase72_status();
    status.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    status.sync.stay_current_next_action =
        FieldAvailability::available("Continue monitoring best-known tip freshness.".to_string());

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(
        evidence.verdict.label,
        SupportEvidenceVerdict::StayCurrentProven
    );
    assert!(
        evidence
            .verdict
            .justifications
            .contains(&"stay_current_current_at_best_known_tip".to_string())
    );
}

#[test]
fn phase72_support_verdict_diagnosed_blocker_uses_shared_diagnosis() {
    // Arrange
    let mut status = phase72_status_missing_tip_match();
    status.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::StorageOrResourceBlocked);
    status.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion);

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(
        evidence.verdict.label,
        SupportEvidenceVerdict::DiagnosedBlocker
    );
    assert!(
        evidence
            .verdict
            .justifications
            .contains(&"blocking_diagnosis_available".to_string())
    );
}

#[test]
fn phase72_support_verdict_normal_resource_pressure_alone_is_inconclusive() {
    // Arrange
    let mut status = phase72_status_missing_tip_match();
    status.sync.resource_pressure = FieldAvailability::available(normal_resource_pressure());

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(evidence.verdict.label, SupportEvidenceVerdict::Inconclusive);
    assert_eq!(
        evidence.verdict.justifications,
        ["missing_required_sync_to_tip_evidence"]
    );
}

#[test]
fn phase72_support_verdict_peer_shortfall_without_blocking_signal_is_inconclusive() {
    // Arrange
    let mut status = phase72_status_missing_tip_match();
    let mut pressure = normal_resource_pressure();
    pressure.outbound_peers = 3;
    pressure.target_outbound_peers = 4;
    status.sync.resource_pressure = FieldAvailability::available(pressure);

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(evidence.verdict.label, SupportEvidenceVerdict::Inconclusive);
    assert_eq!(
        evidence.verdict.justifications,
        ["missing_required_sync_to_tip_evidence"]
    );
}

#[test]
fn phase72_support_verdict_inconclusive_lists_missing_evidence() {
    // Arrange
    let mut status = phase72_status();
    status.sync.best_known_tip =
        FieldAvailability::unavailable("best-known tip evidence unavailable");
    status
        .sync
        .sync_progress
        .as_available_mut()
        .expect("sync progress")
        .maybe_connected_block_hash = None;
    status
        .sync
        .sync_progress
        .as_available_mut()
        .expect("sync progress")
        .maybe_validated_active_chain_hash = None;
    status
        .sync
        .sync_progress
        .as_available_mut()
        .expect("sync progress")
        .maybe_validated_active_chain_work = None;

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(evidence.verdict.label, SupportEvidenceVerdict::Inconclusive);
    assert!(
        evidence
            .verdict
            .justifications
            .contains(&"missing_required_sync_to_tip_evidence".to_string())
    );
    assert_eq!(
        evidence.connected_active_chain.maybe_unavailable_reason,
        Some("connected active-chain hash unavailable".to_string())
    );
    assert_eq!(
        evidence.validated_active_chain.maybe_unavailable_reason,
        Some("validated active-chain hash unavailable".to_string())
    );
}

fn phase72_status_missing_tip_match() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.sync.best_known_tip =
        FieldAvailability::unavailable("best-known tip evidence unavailable");
    status
}

fn phase72_status() -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin-mainnet".to_string()),
            config_paths: vec![],
        },
        service: ServiceStatus {
            manager: FieldAvailability::unavailable("service manager unavailable"),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Unmanaged),
            installed: FieldAvailability::unavailable("service install state unavailable"),
            enabled: FieldAvailability::unavailable("service enablement unavailable"),
            running: FieldAvailability::unavailable("service runtime unavailable"),
            service_file_path: FieldAvailability::unavailable("service file path unavailable"),
            log_path: FieldAvailability::unavailable("service log path unavailable"),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: phase72_sync_status(),
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 3,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable("mempool unavailable"),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable("wallet unavailable"),
            freshness: FieldAvailability::unavailable("wallet unavailable"),
            scan_progress: FieldAvailability::unavailable("wallet unavailable"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}

fn phase72_sync_status() -> SyncStatus {
    SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::available(ChainTipStatus {
            height: 840_004,
            block_hash: "1111111111111111111111111111111111111111111111111111111111111111"
                .to_string(),
        }),
        sync_progress: FieldAvailability::available(SyncProgress {
            header_height: 840_004,
            block_height: 840_004,
            downloaded_block_height: 840_004,
            connected_block_height: 840_004,
            validated_active_chain_height: 840_004,
            maybe_downloaded_block_hash: Some("11".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_work: Some("840005".to_string()),
            progress_ratio: 1.0,
            messages_processed: 128,
            headers_received: 4,
            blocks_received: 4,
        }),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("blocks".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_004),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 4,
            connected_peers: 3,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 0,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_020),
        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
            label: "best_known_tip_reached".to_string(),
            message: "best known tip reached".to_string(),
        }),
        last_error: FieldAvailability::unavailable("sync error unavailable"),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable(
            "daemon sync recovery guidance unavailable",
        ),
        resource_pressure: FieldAvailability::available(normal_resource_pressure()),
        best_known_tip: FieldAvailability::available(BestKnownTipStatus {
            source: BestKnownTipSource::HeaderStore,
            height: 840_004,
            block_hash: "11".repeat(32),
            work: "840005".to_string(),
            block_time_unix_seconds: 1_717_000_010,
            observed_at_unix_seconds: 1_717_000_020,
            freshness: TipFreshnessStatus::Fresh,
            peer_agreement: vec![PeerTipAgreement {
                peer: "peer-1".to_string(),
                maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
                status: PeerTipAgreementStatus::Agrees,
                maybe_height: Some(840_004),
                maybe_hash: Some("11".repeat(32)),
                maybe_work: Some("840005".to_string()),
                maybe_last_activity_unix_seconds: Some(1_717_000_020),
            }],
        }),
        stay_current: FieldAvailability::available(StayCurrentStatus::InitialCatchUp),
        stay_current_next_action: FieldAvailability::available(
            "Wait for best-known tip catch-up evidence.".to_string(),
        ),
        no_progress_diagnosis: FieldAvailability::available(
            NoProgressDiagnosis::CurrentAtBestKnownTip,
        ),
        no_progress_next_action: FieldAvailability::available(
            "No operator action required.".to_string(),
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    }
}

fn normal_resource_pressure() -> SyncResourcePressure {
    SyncResourcePressure {
        blocks_in_flight: 1,
        max_header_requests_in_flight_per_peer: 1,
        max_headers_per_message: 2_000,
        max_blocks_in_flight_per_peer: 16,
        max_blocks_in_flight_total: 64,
        max_messages_per_peer: 64,
        max_sync_rounds: 8,
        outbound_peers: 4,
        target_outbound_peers: 4,
    }
}

fn missing_live_smoke() -> LiveSmokeEvidence {
    LiveSmokeEvidence {
        state: super::EvidenceState::Unavailable,
        report_path: None,
        summary: None,
        reason: Some("live smoke report not provided".to_string()),
    }
}

trait FieldAvailabilityTestExt<T> {
    fn as_available_mut(&mut self) -> Option<&mut T>;
}

impl<T> FieldAvailabilityTestExt<T> for FieldAvailability<T> {
    fn as_available_mut(&mut self) -> Option<&mut T> {
        match self {
            FieldAvailability::Available(value) => Some(value),
            FieldAvailability::Unavailable { .. } => None,
        }
    }
}
