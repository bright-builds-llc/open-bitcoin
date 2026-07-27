// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus, RecoveryActionClass,
    RecoveryCause, RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, BlockRelayEvidenceStatus,
        BlockServingActivationEvidence, BlockServingEligibilityCounters,
        BlockServingStatusCounters, BuildProvenance, CompactRelayAnnouncementCounters,
        CompactRelayCleanupCounters, CompactRelayFallbackCounters, CompactRelayInFlightCounters,
        CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
        CompactRelayReconstructionCounters, ConfigStatus, FieldAvailability, HealthSignal,
        HealthSignalLevel, MempoolStatus, NoProgressDiagnosis, NoProgressThresholdEvidence,
        NoProgressThresholdState, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
        PeerContributionEvidence, PeerContributionKind, PeerCounts, PeerStatus, PeerTipAgreement,
        PeerTipAgreementStatus, ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence,
        RejectedProgressActivity, RejectedProgressActivityKind, ServiceLifecycleStatus,
        ServicePriorShutdownStatus, ServiceRestartResumeStatus, ServiceResumeProgressStatus,
        ServiceStaleInflightStatus, ServiceStatus, StallDiagnosisConfidence,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncAttemptCounters,
        SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncReconcileProgressStatus, SyncRecoveryCategory, SyncReorgEvidence, SyncResourcePressure,
        SyncStatus, SyncStopReasonStatus, TipFreshnessStatus, WalletFreshness, WalletStatus,
        inbound_status_unavailable,
        relay_evidence::{RelayEvidenceCounters, RelayEvidenceStatus, RelayRecoveryCounters},
    },
};

use super::{
    DASHBOARD_METRIC_KINDS, DashboardState, MAX_DASHBOARD_CHARTS, derive_metric_points,
    metric_label,
};

fn apply_phase78_available_sync_fields(sync: &mut SyncStatus) {
    sync.progress_credit = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_020,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 3,
            reason: "headers do not prove durable active-chain progress".to_string(),
        }],
    });
    sync.expected_progress_window = FieldAvailability::available(ProgressWindowEvidence {
        retry_backoff_seconds: 30,
        max_sync_rounds: 8,
        expected_progress_window_seconds: 300,
        tip_freshness_threshold_seconds: 600,
    });
    sync.no_progress_threshold = FieldAvailability::available(NoProgressThresholdEvidence {
        threshold_seconds: 300,
        elapsed_since_last_useful_work_seconds: 12,
        state: NoProgressThresholdState::WithinWindow,
        evaluated_at_unix_seconds: 1_717_000_032,
    });
    sync.last_useful_work = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::CurrentAtBestKnownTip,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_025,
        rejected_activity: Vec::new(),
    });
    sync.last_peer_contribution = FieldAvailability::available(PeerContributionEvidence {
        peer: "peer-1".to_string(),
        maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 1,
        maybe_last_activity_unix_seconds: Some(1_717_000_028),
        maybe_failure_reason_label: None,
    });
    sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::AtTipWaiting,
        confidence: StallDiagnosisConfidence::High,
        evidence_basis: vec!["stay_current".to_string(), "current_tip".to_string()],
        next_action: "No operator action required.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::CurrentAtBestKnownTip),
        maybe_recovery_category: None,
        maybe_latest_stop_reason_label: Some("best_known_tip_reached".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
}

fn test_snapshot() -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::available("launchd".to_string()),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Running),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
            service_file_path: FieldAvailability::available(
                "/tmp/open-bitcoin-node.service".to_string(),
            ),
            log_path: FieldAvailability::available("/tmp/logs/open-bitcoin.log".to_string()),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::available(ServiceRestartResumeStatus {
                datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
                same_datadir: FieldAvailability::available(true),
                prior_shutdown: FieldAvailability::available(ServicePriorShutdownStatus::Clean),
                durable_progress: FieldAvailability::available(ServiceResumeProgressStatus {
                    downloaded_block_height: 840_006,
                    connected_block_height: 840_004,
                    maybe_downloaded_block_hash: Some("22".repeat(32)),
                    maybe_connected_block_hash: Some("11".repeat(32)),
                }),
                stale_inflight: FieldAvailability::available(ServiceStaleInflightStatus::Cleared),
                recovery_category: FieldAvailability::available(
                    SyncRecoveryCategory::CleanShutdown,
                ),
                next_action: FieldAvailability::available(
                    "Resume service sync review from preserved durable progress.".to_string(),
                ),
            }),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("regtest".to_string()),
            chain_tip: FieldAvailability::unavailable("no tip"),
            sync_progress: FieldAvailability::unavailable("no sync"),
            lifecycle: FieldAvailability::unavailable("no sync lifecycle"),
            phase: FieldAvailability::unavailable("no sync phase"),
            configured_targets: FieldAvailability::<SyncConfiguredTargets>::unavailable(
                "no configured sync targets",
            ),
            attempt_counters: FieldAvailability::<SyncAttemptCounters>::unavailable(
                "no sync attempt counters",
            ),
            progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
            lag: FieldAvailability::unavailable("no sync lag"),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                "no successful sync progress",
            ),
            progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
            expected_progress_window: FieldAvailability::unavailable(
                "expected progress window unavailable",
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                "no-progress threshold evidence unavailable",
            ),
            last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
            last_peer_contribution: FieldAvailability::unavailable(
                "last peer contribution unavailable",
            ),
            stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
            latest_stop_reason: FieldAvailability::<SyncStopReasonStatus>::unavailable(
                "no latest stop reason",
            ),
            last_error: FieldAvailability::unavailable("no sync error"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("no recovery action"),
            resource_pressure: FieldAvailability::unavailable("no sync pressure"),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                "best-known tip evidence unavailable",
            ),
            stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
                "stay-current state unavailable",
            ),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay-current next action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                "no-progress diagnosis unavailable",
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                "no-progress next action unavailable",
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 1,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::unavailable("no peer telemetry"),
            inbound: inbound_status_unavailable(),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(4),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(50_000),
            freshness: FieldAvailability::available(WalletFreshness::Fresh),
            scan_progress: FieldAvailability::unavailable("wallet already fresh"),
        },
        logs: open_bitcoin_node::LogStatus::default(),
        metrics: MetricsStatus::available_with_samples(
            MetricRetentionPolicy::default(),
            vec![MetricSample::new(MetricKind::SyncHeight, 100.0, 10)],
        ),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: vec![HealthSignal {
            level: HealthSignalLevel::Info,
            source: "test".to_string(),
            message: "ok".to_string(),
        }],
        build: BuildProvenance::unavailable(),
    }
}

fn shared_sync_truth_snapshot() -> OpenBitcoinStatusSnapshot {
    let mut snapshot = test_snapshot();
    snapshot.sync = SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
        sync_progress: FieldAvailability::available(SyncProgress {
            header_height: 840_100,
            block_height: 840_004,
            downloaded_block_height: 840_006,
            connected_block_height: 840_004,
            validated_active_chain_height: 840_004,
            maybe_downloaded_block_hash: Some("22".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_work: Some("840005".to_string()),
            progress_ratio: 840_004.0 / 840_100.0,
            messages_processed: 7,
            headers_received: 3,
            blocks_received: 1,
        }),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("block_download".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_200),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 3,
            connected_peers: 2,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::AwaitingBlocks),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 96,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
        progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
        expected_progress_window: FieldAvailability::unavailable(
            "expected progress window unavailable",
        ),
        no_progress_threshold: FieldAvailability::unavailable(
            "no-progress threshold evidence unavailable",
        ),
        last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
        last_peer_contribution: FieldAvailability::unavailable(
            "last peer contribution unavailable",
        ),
        stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
            label: "target_header_reached".to_string(),
            message:
                "sync header target reached: target_header_height=840200 best_header_height=840200"
                    .to_string(),
        }),
        last_error: FieldAvailability::available("peer stalled before block connect".to_string()),
        recovery_category: FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData),
        recovery_action: FieldAvailability::available(
            "Retry sync after peer backoff or choose a different peer.".to_string(),
        ),
        resource_pressure: FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 64,
            max_sync_rounds: 8,
            outbound_peers: 2,
            target_outbound_peers: 4,
        }),
        best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
            "best-known tip evidence unavailable",
        ),
        stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
            "stay-current state unavailable",
        ),
        stay_current_next_action: FieldAvailability::unavailable(
            "stay-current next action unavailable",
        ),
        no_progress_diagnosis: FieldAvailability::unavailable("no-progress diagnosis unavailable"),
        no_progress_next_action: FieldAvailability::unavailable(
            "no-progress next action unavailable",
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    };
    snapshot.peers.peer_counts = FieldAvailability::available(PeerCounts {
        inbound: 0,
        outbound: 2,
    });
    snapshot
}

fn phase77_recovery_evidence() -> RecoveryEvidenceSnapshot {
    RecoveryEvidenceSnapshot {
        category: SyncRecoveryCategory::StorageLockContention,
        action_class: RecoveryActionClass::ReadOnlyInspection,
        cause: RecoveryCause::StaleLockEvidence,
        evidence_basis: vec![RecoveryEvidenceBasis::LockProbe],
        maybe_affected_namespace: None,
        maybe_affected_path: Some("/tmp/open-bitcoin/LOCK".to_string()),
        next_action:
            "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
                .to_string(),
        compatibility_action: FieldAvailability::unavailable(
            "no compatibility recovery action recorded",
        ),
    }
}

mod projection;
mod recovery_unavailable;
mod sync_and_recovery;
