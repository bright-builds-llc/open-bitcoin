use super::*;

#[derive(Debug)]
pub(super) struct TestDirectory {
    pub(super) path: PathBuf,
}

impl TestDirectory {
    pub(super) fn new(label: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-soak-runtime-{label}-{}-{timestamp}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory");
        Self { path }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[derive(Debug)]
pub(super) struct ScriptedStatusCollector {
    pub(super) snapshots: Vec<OpenBitcoinStatusSnapshot>,
    pub(super) index: usize,
}

impl ScriptedStatusCollector {
    pub(super) fn repeating(snapshot: OpenBitcoinStatusSnapshot) -> Self {
        Self {
            snapshots: vec![snapshot],
            index: 0,
        }
    }

    pub(super) fn from_snapshots(snapshots: Vec<OpenBitcoinStatusSnapshot>) -> Self {
        Self {
            snapshots,
            index: 0,
        }
    }
}

impl SoakStatusCollector for ScriptedStatusCollector {
    fn collect(&mut self) -> OpenBitcoinStatusSnapshot {
        let snapshot = self
            .snapshots
            .get(self.index)
            .or_else(|| self.snapshots.last())
            .expect("scripted status snapshot")
            .clone();
        self.index += 1;
        snapshot
    }
}

pub(super) struct StopDuringSleepClock {
    pub(super) now_unix_seconds: u64,
    pub(super) layout: SoakLedgerLayout,
    pub(super) run_id: SoakRunId,
    pub(super) stop_written: bool,
}

impl StopDuringSleepClock {
    pub(super) const fn new(
        now_unix_seconds: u64,
        layout: SoakLedgerLayout,
        run_id: SoakRunId,
    ) -> Self {
        Self {
            now_unix_seconds,
            layout,
            run_id,
            stop_written: false,
        }
    }
}

impl SoakClock for StopDuringSleepClock {
    fn now_unix_seconds(&mut self) -> u64 {
        self.now_unix_seconds
    }

    fn sleep_until(&mut self, scheduled_unix_seconds: u64) {
        let should_write_stop =
            scheduled_unix_seconds > self.now_unix_seconds && !self.stop_written;
        self.now_unix_seconds = scheduled_unix_seconds;
        if should_write_stop {
            write_operator_stop(&self.layout, &self.run_id, scheduled_unix_seconds)
                .expect("external operator stop");
            self.stop_written = true;
        }
    }
}

pub(super) struct StopDuringCollectCollector {
    pub(super) snapshot: OpenBitcoinStatusSnapshot,
    pub(super) layout: SoakLedgerLayout,
    pub(super) run_id: SoakRunId,
    pub(super) stop_written: bool,
}

impl StopDuringCollectCollector {
    pub(super) const fn new(
        snapshot: OpenBitcoinStatusSnapshot,
        layout: SoakLedgerLayout,
        run_id: SoakRunId,
    ) -> Self {
        Self {
            snapshot,
            layout,
            run_id,
            stop_written: false,
        }
    }
}

impl SoakStatusCollector for StopDuringCollectCollector {
    fn collect(&mut self) -> OpenBitcoinStatusSnapshot {
        if !self.stop_written {
            write_operator_stop(&self.layout, &self.run_id, self.snapshot_time())
                .expect("external operator stop during collect");
            self.stop_written = true;
        }
        self.snapshot.clone()
    }
}

impl StopDuringCollectCollector {
    pub(super) fn snapshot_time(&self) -> u64 {
        1_700_000_000
    }
}

pub(super) fn soak_bounds(
    datadir: &Path,
    maybe_target_height: Option<u64>,
    stop_conditions: Vec<SoakStopCondition>,
) -> SoakBounds {
    SoakBounds::try_new(
        60,
        15,
        maybe_target_height,
        datadir.to_path_buf(),
        "regtest",
        SoakPeerPolicy::DaemonConfigured,
        1_048_576,
        stop_conditions,
    )
    .expect("soak bounds")
}

pub(super) fn checkpoint_status(height: u64) -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("regtest".to_string()),
        maybe_lifecycle: Some("active".to_string()),
        maybe_latest_stop_reason_label: None,
        maybe_recovery_category_label: None,
        maybe_recovery_action_class_label: None,
        maybe_recovery_cause_label: None,
        maybe_recovery_next_action: None,
        maybe_no_progress_diagnosis_label: None,
        maybe_progress_credit_kind_label: None,
        maybe_progress_credit_height: None,
        maybe_progress_credit_hash: None,
        maybe_progress_credit_work: None,
        maybe_progress_credit_source_unix_seconds: None,
        progress_credit_rejected_activity_labels: Vec::new(),
        maybe_expected_progress_window_seconds: None,
        maybe_no_progress_threshold_state_label: None,
        maybe_no_progress_threshold_seconds: None,
        maybe_last_useful_work_kind_label: None,
        maybe_last_useful_work_height: None,
        maybe_last_peer_contribution_label: None,
        maybe_stalled_subsystem_label: None,
        maybe_stall_confidence_label: None,
        stall_evidence_basis: Vec::new(),
        maybe_stall_next_action: None,
        maybe_resource_bound_state_label: Some("normal".to_string()),
        resource_bound_labels: vec!["all_required_bounds=normal".to_string()],
        maybe_resource_bound_next_action: None,
        maybe_validated_active_chain_height: Some(height),
        maybe_best_known_tip_height: Some(height),
        maybe_source_status_path: None,
    }
}

pub(super) fn clean_status_snapshot(datadir: &Path, height: u64) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.sync_progress = FieldAvailability::available(SyncProgress {
        header_height: height,
        block_height: height,
        downloaded_block_height: height,
        connected_block_height: height,
        validated_active_chain_height: height,
        maybe_downloaded_block_hash: Some("11".repeat(32)),
        maybe_connected_block_hash: Some("11".repeat(32)),
        maybe_validated_active_chain_hash: Some("11".repeat(32)),
        maybe_validated_active_chain_work: Some("work".to_string()),
        progress_ratio: 1.0,
        messages_processed: 1,
        headers_received: 1,
        blocks_received: 1,
    });
    snapshot.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height,
        block_hash: "11".repeat(32),
        work: "work".to_string(),
        block_time_unix_seconds: 1_700_000_000,
        observed_at_unix_seconds: 1_700_000_000,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: Vec::new(),
    });
    snapshot.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    snapshot
}

pub(super) fn progress_guarantee_status_snapshot(
    datadir: &Path,
    height: u64,
) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = clean_status_snapshot(datadir, height);
    let progress_credit = ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: height,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "900001".to_string(),
        source_unix_seconds: 1_777_300_060,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 2,
            reason: "headers are not durable active-chain progress".to_string(),
        }],
    };
    snapshot.sync.progress_credit = FieldAvailability::available(progress_credit.clone());
    snapshot.sync.expected_progress_window = FieldAvailability::available(ProgressWindowEvidence {
        retry_backoff_seconds: 30,
        max_sync_rounds: 8,
        expected_progress_window_seconds: 240,
        tip_freshness_threshold_seconds: 600,
    });
    snapshot.sync.no_progress_threshold =
        FieldAvailability::available(NoProgressThresholdEvidence {
            threshold_seconds: 240,
            elapsed_since_last_useful_work_seconds: 30,
            state: NoProgressThresholdState::WithinWindow,
            evaluated_at_unix_seconds: 1_777_300_090,
        });
    snapshot.sync.last_useful_work = FieldAvailability::available(progress_credit);
    snapshot.sync.last_peer_contribution = FieldAvailability::available(PeerContributionEvidence {
        peer: "peer-1".to_string(),
        maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 1,
        maybe_last_activity_unix_seconds: Some(1_777_300_060),
        maybe_failure_reason_label: None,
    });
    snapshot.sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::SlowOrStalledPeers,
        confidence: StallDiagnosisConfidence::Medium,
        evidence_basis: vec!["latest peer stalled before useful work".to_string()],
        next_action: "Rotate peers and continue bounded sync.".to_string(),
        maybe_no_progress_diagnosis: None,
        maybe_recovery_category: None,
        maybe_latest_stop_reason_label: None,
        source_unix_seconds: 1_777_300_090,
    });
    snapshot
}

pub(super) fn resource_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion);
    snapshot.resource_bounds = FieldAvailability::available(resource_stop_bounds());
    snapshot
}

pub(super) fn recovery_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption);
    snapshot
}

pub(super) fn operator_stop_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.latest_stop_reason = FieldAvailability::available(SyncStopReasonStatus {
        label: "operator_stop".to_string(),
        message: "operator requested stop".to_string(),
    });
    snapshot
}

pub(super) fn diagnosed_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
        common_ancestor_height: 1,
        common_ancestor_hash: "00".repeat(32),
        disconnected_count: 1,
        connected_count: 0,
        final_active_height: 1,
        final_active_hash: "11".repeat(32),
        fully_persisted: false,
    });
    snapshot
}

pub(super) fn phase77_recovery_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.recovery_evidence = FieldAvailability::available(RecoveryEvidenceSnapshot {
        category: SyncRecoveryCategory::StoreCorruption,
        action_class: RecoveryActionClass::BackupThenRebuild,
        cause: RecoveryCause::PartialWrite,
        evidence_basis: vec![RecoveryEvidenceBasis::RecoveryMarker],
        maybe_affected_namespace: Some("runtime".to_string()),
        maybe_affected_path: None,
        next_action:
            "Back up the selected datadir, then rebuild affected storage before normal operation."
                .to_string(),
        compatibility_action: FieldAvailability::unavailable(
            "no compatibility recovery action recorded",
        ),
    });
    snapshot
}

pub(super) fn latest_checkpoint_status(
    layout: &SoakLedgerLayout,
    run_id: &SoakRunId,
) -> SoakCheckpointStatus {
    let events = SoakLedger::read_events(&layout.paths_for_run(run_id).events_path)
        .expect("read soak ledger")
        .events;
    events
        .into_iter()
        .rev()
        .find_map(|event| match event.event {
            SoakLedgerEvent::Checkpoint { status } => Some(*status),
            SoakLedgerEvent::Started { .. }
            | SoakLedgerEvent::Resume { .. }
            | SoakLedgerEvent::Stop { .. }
            | SoakLedgerEvent::Verdict { .. } => None,
        })
        .expect("latest checkpoint status")
}

pub(super) fn base_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "test".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available(datadir.display().to_string()),
            config_paths: Vec::new(),
        },
        service: open_bitcoin_node::status::ServiceStatus {
            manager: FieldAvailability::unavailable("no service manager"),
            lifecycle: FieldAvailability::unavailable("service lifecycle unavailable"),
            installed: FieldAvailability::unavailable("service installed unavailable"),
            enabled: FieldAvailability::unavailable("service enabled unavailable"),
            running: FieldAvailability::unavailable("service running unavailable"),
            service_file_path: FieldAvailability::unavailable("service file unavailable"),
            log_path: FieldAvailability::unavailable("service log unavailable"),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable("service restart/resume unavailable"),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("regtest".to_string()),
            chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
            sync_progress: FieldAvailability::unavailable("sync progress unavailable"),
            lifecycle: FieldAvailability::available(
                open_bitcoin_node::status::SyncLifecycleState::Active,
            ),
            phase: FieldAvailability::available("test".to_string()),
            configured_targets: FieldAvailability::unavailable("targets unavailable"),
            attempt_counters: FieldAvailability::unavailable("attempts unavailable"),
            progress_signal: FieldAvailability::unavailable("signal unavailable"),
            lag: FieldAvailability::unavailable("lag unavailable"),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                "progress unavailable",
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
            latest_stop_reason: FieldAvailability::unavailable("stop reason unavailable"),
            last_error: FieldAvailability::unavailable("last error unavailable"),
            recovery_category: FieldAvailability::unavailable("recovery unavailable"),
            recovery_action: FieldAvailability::unavailable("recovery action unavailable"),
            resource_pressure: FieldAvailability::unavailable("pressure unavailable"),
            best_known_tip: FieldAvailability::unavailable("best tip unavailable"),
            stay_current: FieldAvailability::unavailable("stay current unavailable"),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay current action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                "no-progress diagnosis unavailable",
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                "no-progress action unavailable",
            ),
            latest_reorg: FieldAvailability::unavailable("reorg unavailable"),
            reconcile_progress: FieldAvailability::unavailable("reconcile unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::unavailable("peer counts unavailable"),
            recent_peers: FieldAvailability::unavailable("recent peers unavailable"),
            inbound: inbound_status_unavailable(),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable("mempool unavailable"),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable("wallet unavailable"),
            freshness: FieldAvailability::unavailable("wallet freshness unavailable"),
            scan_progress: FieldAvailability::unavailable("wallet scan unavailable"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::unavailable(
            MetricRetentionPolicy::default(),
            "metrics unavailable",
        ),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::available(normal_resource_bounds()),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}

pub(super) fn normal_resource_bounds() -> ResourceBoundSnapshot {
    ResourceBoundSnapshot::new(
        ResourceBoundKind::ALL
            .into_iter()
            .map(|kind| {
                ResourceBoundEntry::available(
                    kind,
                    kind.as_str(),
                    usage_against_budget(
                        1,
                        1_048_576,
                        resource_unit(kind),
                        "No resource-bound action required.",
                    ),
                )
            })
            .collect(),
    )
}

pub(super) fn resource_stop_bounds() -> ResourceBoundSnapshot {
    ResourceBoundSnapshot::new(
        ResourceBoundKind::ALL
            .into_iter()
            .map(|kind| {
                let current = if kind == ResourceBoundKind::Disk {
                    1_048_576
                } else {
                    1
                };
                ResourceBoundEntry::available(
                    kind,
                    kind.as_str(),
                    usage_against_budget(
                        current,
                        1_048_576,
                        resource_unit(kind),
                        "Free disk space before continuing.",
                    ),
                )
            })
            .collect(),
    )
}
