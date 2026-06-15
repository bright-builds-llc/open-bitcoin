// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

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

use crate::operator::{
    config::OperatorConfigResolution,
    soak::{
        SoakBounds, SoakPeerPolicy, SoakRunId, SoakStopCondition,
        ledger::{
            SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerLayout, SoakRunIndex,
            SoakRunIndexEntry,
        },
        outcome::SoakOutcomeLabel,
        report::write_soak_reports,
    },
};

use super::{
    EvidenceAvailability, EvidenceState, LiveSmokeEvidence, MetricsHistoryEvidence,
    RuntimeMetadataEvidence, StoreHealthEvidence, SupportEvidenceBundle, SupportEvidenceOutput,
    collect_resource_bound_support_evidence, collect_soak_support_evidence,
    derive_full_sync_evidence, evidence::SupportEvidenceVerdict, redaction_summary, render,
    soak_outcome_label,
};

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-support-{label}-{}-{timestamp}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

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
            "resource bounds are recorded as compact status summaries only",
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

#[test]
fn phase75_soak_support_evidence_available_projects_latest_ledger_summary() {
    // Arrange
    let temp = TestDirectory::new("soak-available");
    let (run_id, paths) = seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0001",
        SoakOutcomeLabel::OperatorStop,
    );
    let resolution = phase75_config_resolution(temp.path());

    // Act
    let evidence = collect_soak_support_evidence(&resolution);
    let serialized = serde_json::to_value(&evidence).expect("soak evidence json");

    // Assert
    assert_eq!(serialized["state"], json!("available"));
    assert_eq!(serialized["maybe_run_id"], json!(run_id.as_str()));
    assert_eq!(
        serialized["maybe_final_outcome"],
        json!(soak_outcome_label(SoakOutcomeLabel::OperatorStop))
    );
    assert_eq!(serialized["maybe_latest_sequence"], json!(4));
    assert_eq!(
        serialized["maybe_source_ledger_path"],
        json!(paths.events_path.display().to_string())
    );
    assert_eq!(
        serialized["maybe_json_report_path"],
        json!(paths.report_json_path.display().to_string())
    );
    assert_eq!(
        serialized["maybe_markdown_report_path"],
        json!(paths.report_markdown_path.display().to_string())
    );
    assert_eq!(serialized["maybe_unavailable_reason"], json!(null));
}

#[test]
fn phase75_soak_support_markdown_renders_compact_section() {
    // Arrange
    let temp = TestDirectory::new("soak-markdown");
    let (_run_id, paths) = seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0002",
        SoakOutcomeLabel::CleanCompletion,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Soak Evidence",
        "State: available",
        "Run: soak-1781485562-0002",
        "Final outcome: clean_completion",
        "Source ledger:",
        "JSON report:",
        "Markdown report:",
        "Latest sequence: 4",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    assert!(markdown.contains(paths.events_path.to_str().expect("ledger path")));
}

#[test]
fn phase75_soak_support_evidence_unavailable_without_ledger() {
    // Arrange
    let temp = TestDirectory::new("soak-unavailable");
    let resolution = phase75_config_resolution(temp.path());
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let evidence = collect_soak_support_evidence(&resolution);
    let serialized = serde_json::to_value(&evidence).expect("soak evidence json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(serialized["state"], json!("unavailable"));
    assert_eq!(
        serialized["maybe_unavailable_reason"],
        json!("soak ledger unavailable")
    );
    assert_eq!(serialized["maybe_run_id"], json!(null));
    assert!(markdown.contains("## Soak Evidence"));
    assert!(markdown.contains("State: unavailable"));
    assert!(markdown.contains("Reason: soak ledger unavailable"));
}

#[test]
fn phase75_soak_support_summary_excludes_raw_local_evidence() {
    // Arrange
    let temp = TestDirectory::new("soak-redaction");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0003",
        SoakOutcomeLabel::ResourceStop,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw ledger line phase75-secret",
            "raw daemon logs phase75-secret",
            "raw reports phase75-secret",
            "wallet material phase75-secret",
            "RPC credentials phase75-secret",
            "unbounded peer tables phase75-secret",
            "\"kind\":\"started\"",
            "\"kind\":\"checkpoint\"",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
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
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
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

fn phase75_config_resolution(data_dir: &Path) -> OperatorConfigResolution {
    OperatorConfigResolution {
        maybe_data_dir: Some(data_dir.to_path_buf()),
        ..OperatorConfigResolution::default()
    }
}

fn phase75_support_bundle_for_test(data_dir: &Path) -> SupportEvidenceBundle {
    let resolution = phase75_config_resolution(data_dir);
    let status = phase72_status();
    let live_smoke = missing_live_smoke();
    let full_sync_evidence = derive_full_sync_evidence(&status, &live_smoke);
    let output_dir = data_dir.join("support");
    SupportEvidenceBundle {
        generated_at_unix_seconds: 1_781_485_562,
        generated_by: "phase75 test".to_string(),
        output: SupportEvidenceOutput {
            directory: output_dir.display().to_string(),
            json_path: output_dir
                .join("support-evidence.json")
                .display()
                .to_string(),
            markdown_path: output_dir.join("support-evidence.md").display().to_string(),
        },
        redaction: redaction_summary(),
        config: super::ConfigEvidence::from_resolution(&resolution),
        status: status.clone(),
        store_health: unavailable_store_health(),
        live_smoke,
        full_sync_evidence,
        soak_evidence: collect_soak_support_evidence(&resolution),
        resource_bound_evidence: collect_resource_bound_support_evidence(&status, &output_dir),
    }
}

fn unavailable_store_health() -> StoreHealthEvidence {
    StoreHealthEvidence {
        state: EvidenceState::Unavailable,
        durable_store: EvidenceAvailability::unavailable("durable store unavailable"),
        runtime_metadata: RuntimeMetadataEvidence {
            availability: EvidenceAvailability::unavailable("runtime metadata unavailable"),
            metadata: None,
        },
        metrics_history: MetricsHistoryEvidence {
            availability: EvidenceAvailability::unavailable("metrics history unavailable"),
            samples: 0,
            status: None,
        },
    }
}

fn seed_phase75_soak_run(
    data_dir: &Path,
    run_id_text: &str,
    outcome: SoakOutcomeLabel,
) -> (SoakRunId, crate::operator::soak::ledger::SoakRunPaths) {
    let layout = SoakLedgerLayout::for_datadir(data_dir);
    let run_id = SoakRunId::try_new(run_id_text).expect("run id");
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            1_781_485_562,
            SoakLedgerEvent::Started {
                bounds: phase75_soak_bounds(data_dir),
            },
        )
        .expect("append started");
    ledger
        .append_event(
            1_781_485_622,
            SoakLedgerEvent::Checkpoint {
                status: phase75_checkpoint_status(),
            },
        )
        .expect("append checkpoint");
    ledger
        .append_event(1_781_485_682, SoakLedgerEvent::Stop { outcome })
        .expect("append stop");
    ledger
        .append_event(1_781_485_682, SoakLedgerEvent::Verdict { outcome })
        .expect("append verdict");

    let paths = layout.paths_for_run(&run_id);
    let read = SoakLedger::read_events(&paths.events_path).expect("read soak ledger");
    write_soak_reports(&read, &paths.events_path, &layout).expect("write soak reports");

    let mut index = SoakRunIndex::empty();
    index.record_run(SoakRunIndexEntry {
        run_id: run_id.clone(),
        ledger_path: paths.events_path.clone(),
        started_at_unix_seconds: 1_781_485_562,
        updated_at_unix_seconds: 1_781_485_682,
        maybe_outcome: Some(outcome),
    });
    index.write_atomic(&layout).expect("write soak run index");

    (run_id, paths)
}

fn phase75_soak_bounds(data_dir: &Path) -> SoakBounds {
    SoakBounds::try_new(
        86_400,
        60,
        Some(900_000),
        data_dir.to_path_buf(),
        "raw ledger line phase75-secret",
        SoakPeerPolicy::DaemonConfigured,
        4_096,
        vec![SoakStopCondition::ElapsedTime],
    )
    .expect("valid soak bounds")
}

fn phase75_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("mainnet".to_string()),
        maybe_lifecycle: Some("raw daemon logs phase75-secret".to_string()),
        maybe_latest_stop_reason_label: Some("raw reports phase75-secret".to_string()),
        maybe_recovery_category_label: Some("wallet material phase75-secret".to_string()),
        maybe_no_progress_diagnosis_label: Some("RPC credentials phase75-secret".to_string()),
        maybe_resource_bound_state_label: Some("normal".to_string()),
        resource_bound_labels: vec!["all_required_bounds=normal".to_string()],
        maybe_resource_bound_next_action: None,
        maybe_validated_active_chain_height: Some(900_000),
        maybe_best_known_tip_height: Some(900_000),
        maybe_source_status_path: Some(PathBuf::from("unbounded peer tables phase75-secret")),
    }
}

fn assert_absent(text: &str, value: &str) {
    assert!(
        !text.contains(value),
        "unexpected sensitive value in {text}"
    );
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
