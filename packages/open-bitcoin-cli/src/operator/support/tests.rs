// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    BuildProvenance, LogStatus, MetricsStatus, OpenBitcoinStatusSnapshot, RecoveryActionClass,
    RecoveryCause, RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, ChainTipStatus, ConfigStatus, FieldAvailability,
        MempoolStatus, NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState,
        NodeRuntimeState, NodeStatus, PeerCounts, PeerStatus, PeerTipAgreement,
        PeerTipAgreementStatus, ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence,
        RejectedProgressActivity, RejectedProgressActivityKind, ServiceLifecycleStatus,
        ServiceStatus, StallDiagnosisConfidence, StallDiagnosisEvidence, StalledSubsystem,
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
            SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerEventEnvelope,
            SoakLedgerLayout, SoakRunIndex, SoakRunIndexEntry,
        },
        outcome::SoakOutcomeLabel,
        report::write_soak_reports,
    },
};

use super::{
    EvidenceAvailability, EvidenceState, LiveSmokeEvidence, MetricsHistoryEvidence,
    RecoverySupportEvidence, RuntimeMetadataEvidence, StoreHealthEvidence, SupportEvidenceBundle,
    SupportEvidenceOutput, collect_resource_bound_support_evidence, collect_soak_support_evidence,
    collect_store_health, derive_full_sync_evidence, evidence::SupportEvidenceVerdict,
    forensics::SupportForensicsEvidence, redaction_summary, render, soak_outcome_label,
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
    let evidence = collect_soak_support_evidence(&resolution, &redaction_summary()).soak_evidence;
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
    let evidence = collect_soak_support_evidence(&resolution, &redaction_summary()).soak_evidence;
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
fn phase79_support_forensics_projection_builds_timeline_chain_and_narrative() {
    // Arrange
    let temp = TestDirectory::new("phase79-forensics-projection");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0079",
        SoakOutcomeLabel::CleanCompletion,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");

    // Assert
    assert_eq!(serialized["support_forensics"]["state"], json!("available"));
    assert_eq!(
        serialized["support_forensics"]["checkpoint_chain"]["algorithm"],
        json!("sha256-json-v1")
    );
    assert_eq!(
        serialized["support_forensics"]["checkpoint_chain"]["ordered"],
        json!(true)
    );
    assert_eq!(
        serialized["support_forensics"]["checkpoint_chain"]["missing_sequence_count"],
        json!(0)
    );
    assert_eq!(
        serialized["support_forensics"]["checkpoint_chain"]["truncated"],
        json!(false)
    );
    assert_eq!(
        serialized["support_forensics"]["narrative"]["verdict"],
        json!("soak_stable")
    );
    for field in [
        "likely_cause",
        "evidence_basis",
        "next_action",
        "confidence",
    ] {
        assert!(
            !serialized["support_forensics"]["narrative"][field].is_null(),
            "missing {field}"
        );
    }
    assert_eq!(
        serialized["support_forensics"]["source"]["event_count"],
        json!(4)
    );
    assert!(
        serialized["support_forensics"]["timeline"]
            .as_array()
            .expect("timeline array")
            .len()
            >= 4
    );
}

#[test]
fn phase79_support_forensics_projection_detects_sequence_gaps_and_truncation() {
    // Arrange
    let temp = TestDirectory::new("phase79-gap-truncation");
    let run_id = SoakRunId::try_new("soak-1781485562-0080").expect("run id");
    let source_ledger_path = temp.path().join("events.jsonl");
    let events = vec![
        soak_event(
            run_id.clone(),
            1,
            SoakLedgerEvent::Started {
                bounds: phase75_soak_bounds(temp.path()),
            },
        ),
        soak_event(
            run_id.clone(),
            2,
            SoakLedgerEvent::Checkpoint {
                status: Box::new(phase75_checkpoint_status()),
            },
        ),
        soak_event(
            run_id,
            4,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::UnexpectedTermination,
            },
        ),
    ];
    let projection = crate::operator::soak::report::SoakReportProjection::from_ledger_events(
        events.clone(),
        &source_ledger_path,
    )
    .expect("projection");
    let read = crate::operator::soak::ledger::SoakLedgerReadResult {
        events,
        ignored_trailing_bytes: 11,
    };

    // Act
    let evidence = SupportForensicsEvidence::available(
        &read,
        &projection,
        &source_ledger_path,
        &temp.path().join("report.json"),
        &temp.path().join("report.md"),
        &redaction_summary(),
    );
    let serialized = serde_json::to_value(&evidence).expect("forensics json");

    // Assert
    assert_eq!(serialized["checkpoint_chain"]["ordered"], json!(false));
    assert_eq!(
        serialized["checkpoint_chain"]["missing_sequence_count"],
        json!(1)
    );
    assert_eq!(serialized["checkpoint_chain"]["truncated"], json!(true));
}

#[test]
fn phase79_support_forensics_projection_keeps_unavailable_evidence_conservative() {
    // Arrange
    let temp = TestDirectory::new("phase79-unavailable");
    let resolution = phase75_config_resolution(temp.path());

    // Act
    let collection = collect_soak_support_evidence(&resolution, &redaction_summary());
    let serialized = serde_json::to_value(&collection.support_forensics).expect("forensics json");

    // Assert
    assert_eq!(serialized["state"], json!("unavailable"));
    assert_eq!(
        serialized["narrative"]["verdict"],
        json!("collection_failed")
    );
    assert_eq!(
        serialized["narrative"]["likely_cause"],
        json!("soak ledger unavailable")
    );
    assert_eq!(serialized["narrative"]["confidence"], json!("low"));
    assert_eq!(serialized["timeline"], json!([]));
}

#[test]
fn phase79_support_forensics_json_includes_sidecar_contract() {
    // Arrange
    let temp = TestDirectory::new("phase79-json-contract");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0081",
        SoakOutcomeLabel::CleanCompletion,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");

    // Assert
    for field in [
        "timeline",
        "checkpoint_chain",
        "narrative",
        "source",
        "redaction",
    ] {
        assert!(
            !serialized["support_forensics"][field].is_null(),
            "missing {field}"
        );
    }
    assert!(
        !serialized["support_forensics"]["narrative"]["evidence_basis"]
            .as_array()
            .expect("evidence basis")
            .is_empty()
    );
}

#[test]
fn phase79_support_forensics_json_excludes_sensitive_seed_material() {
    // Arrange
    let temp = TestDirectory::new("phase79-sensitive-json");
    seed_phase79_sensitive_soak_run(
        temp.path(),
        "soak-1781485562-0082",
        SoakOutcomeLabel::ResourceStop,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");

    // Assert
    for forbidden in phase79_sensitive_literals() {
        assert_absent(&json_text, forbidden);
    }
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

#[test]
fn support_recovery_evidence_json_projects_shared_status_evidence() {
    // Arrange
    let temp = TestDirectory::new("recovery-json");
    let status = phase77_status_with_available_recovery();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");

    // Assert
    assert_eq!(serialized["recovery_evidence"]["state"], json!("available"));
    assert_eq!(
        serialized["recovery_evidence"]["category"],
        json!("storage_lock_contention")
    );
    assert_eq!(
        serialized["recovery_evidence"]["cause"],
        json!("stale_lock_evidence")
    );
    assert_eq!(
        serialized["recovery_evidence"]["action_class"],
        json!("read_only_inspection")
    );
    assert_eq!(
        serialized["recovery_evidence"]["evidence_basis"],
        json!(["lock_probe"])
    );
    assert_eq!(
        serialized["recovery_evidence"]["next_action"],
        json!("Inspect the datadir read-only and avoid deleting lock artifacts automatically.")
    );
    assert_eq!(
        serialized["recovery_evidence"]["maybe_unavailable_reason"],
        json!(null)
    );
    assert_eq!(
        serialized["recovery_evidence"]["source"],
        json!("status.recovery_evidence")
    );
}

#[test]
fn support_recovery_evidence_markdown_renders_operator_fields() {
    // Arrange
    let temp = TestDirectory::new("recovery-markdown");
    let status = phase77_status_with_available_recovery();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Recovery Evidence",
        "Category: storage_lock_contention",
        "Cause: stale_lock_evidence",
        "Action class: read_only_inspection",
        "Next action: Inspect the datadir read-only and avoid deleting lock artifacts automatically.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn support_recovery_evidence_collection_preserves_probe_only_store_health() {
    // Arrange
    let status = phase72_status();

    // Act
    let health = collect_store_health(&status);

    // Assert
    assert_eq!(health.state, EvidenceState::Unavailable);
    assert_eq!(
        health.runtime_metadata.availability.reason,
        Some(
            "runtime metadata unavailable: probe-only support bundle does not open Fjall stores"
                .to_string()
        )
    );
    assert_eq!(
        health.metrics_history.availability.reason,
        Some(
            "metrics history unavailable: probe-only support bundle does not open Fjall stores"
                .to_string()
        )
    );
}

#[test]
fn support_recovery_evidence_unavailable_status_preserves_reason() {
    // Arrange
    let temp = TestDirectory::new("recovery-unavailable");
    let mut status = phase72_status();
    status.recovery_evidence =
        FieldAvailability::unavailable("status recovery evidence probe disabled");
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["recovery_evidence"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        serialized["recovery_evidence"]["maybe_unavailable_reason"],
        json!("status recovery evidence probe disabled")
    );
    assert!(markdown.contains("Status: Unavailable: status recovery evidence probe disabled"));
}

#[test]
fn support_recovery_evidence_full_sync_prefers_top_level_status_evidence() {
    // Arrange
    let status = phase77_status_with_available_recovery();

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());

    // Assert
    assert_eq!(evidence.recovery.state, EvidenceState::Available);
    assert_eq!(
        evidence.recovery.summary.as_deref(),
        Some(
            "category=storage_lock_contention cause=stale_lock_evidence action_class=read_only_inspection next_action=Inspect the datadir read-only and avoid deleting lock artifacts automatically."
        )
    );
}

#[test]
fn support_phase78_progress_guarantee_json_projects_shared_status() {
    // Arrange
    let mut status = phase72_status();
    apply_phase78_available_sync_fields(&mut status.sync);

    // Act
    let evidence = derive_full_sync_evidence(&status, &missing_live_smoke());
    let serialized = serde_json::to_value(&evidence).expect("evidence json");

    // Assert
    assert_eq!(
        serialized["progress_guarantee"]["summary"],
        json!(
            "credit=kind=validated_durable_active_chain height=840004 source_unix_seconds=1717000020 rejected_activity_count=1 last_useful_work=kind=current_at_best_known_tip height=840004 source_unix_seconds=1717000025 rejected_activity_count=0 expected_window=seconds=300 retry_backoff_seconds=30 max_sync_rounds=8 threshold=state=within_window seconds=300 elapsed_seconds=12"
        )
    );
    assert_eq!(
        serialized["stall_diagnosis"]["summary"],
        json!(
            "stalled_subsystem=at_tip_waiting confidence=high basis=stay_current,current_tip next_action=No operator action required."
        )
    );
}

#[test]
fn support_phase78_progress_guarantee_markdown_renders_operator_fields() {
    // Arrange
    let temp = TestDirectory::new("phase78-progress-markdown");
    let mut status = phase72_status();
    apply_phase78_available_sync_fields(&mut status.sync);
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "- Progress guarantee: credit=kind=validated_durable_active_chain",
        "last_useful_work=kind=current_at_best_known_tip",
        "expected_window=seconds=300",
        "threshold=state=within_window",
        "- Stall diagnosis: stalled_subsystem=at_tip_waiting confidence=high",
        "next_action=No operator action required.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn support_phase78_progress_guarantee_excludes_raw_status_body() {
    // Arrange
    let temp = TestDirectory::new("phase78-progress-redaction");
    let mut status = phase72_status();
    apply_phase78_available_sync_fields(&mut status.sync);
    status.sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::StorageOrResourcePressure,
        confidence: StallDiagnosisConfidence::High,
        evidence_basis: vec!["compact evidence only".to_string()],
        next_action: "Inspect bounded resource evidence.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        maybe_recovery_category: Some(SyncRecoveryCategory::ResourceExhaustion),
        maybe_latest_stop_reason_label: Some("resource_stop".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw status snapshot phase78-secret",
            "raw live-smoke input phase78-secret",
            "raw daemon log phase78-secret",
            "credential phase78-secret",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

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
    let redaction = redaction_summary();
    let soak_collection = collect_soak_support_evidence(&resolution, &redaction);
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
        redaction,
        config: super::ConfigEvidence::from_resolution(&resolution),
        status: status.clone(),
        recovery_evidence: RecoverySupportEvidence::from_status(&status.recovery_evidence),
        store_health: unavailable_store_health(),
        live_smoke,
        full_sync_evidence,
        soak_evidence: soak_collection.soak_evidence,
        support_forensics: soak_collection.support_forensics,
        resource_bound_evidence: collect_resource_bound_support_evidence(&status, &output_dir),
    }
}

fn phase77_support_bundle_with_status(
    data_dir: &Path,
    status: OpenBitcoinStatusSnapshot,
) -> SupportEvidenceBundle {
    let mut bundle = phase75_support_bundle_for_test(data_dir);
    bundle.status = status;
    bundle.recovery_evidence =
        RecoverySupportEvidence::from_status(&bundle.status.recovery_evidence);
    bundle.full_sync_evidence = derive_full_sync_evidence(&bundle.status, &bundle.live_smoke);
    bundle.resource_bound_evidence =
        collect_resource_bound_support_evidence(&bundle.status, &data_dir.join("support"));
    bundle
}

fn phase77_status_with_available_recovery() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.sync.recovery_category =
        FieldAvailability::unavailable("legacy recovery category unavailable");
    status.sync.recovery_action =
        FieldAvailability::unavailable("legacy recovery action unavailable");
    status.recovery_evidence = FieldAvailability::available(phase77_recovery_evidence());
    status
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
    seed_soak_run_with_checkpoint(data_dir, run_id_text, outcome, phase75_checkpoint_status())
}

fn seed_phase79_sensitive_soak_run(
    data_dir: &Path,
    run_id_text: &str,
    outcome: SoakOutcomeLabel,
) -> (SoakRunId, crate::operator::soak::ledger::SoakRunPaths) {
    seed_soak_run_with_checkpoint(
        data_dir,
        run_id_text,
        outcome,
        phase79_sensitive_checkpoint_status(),
    )
}

fn seed_soak_run_with_checkpoint(
    data_dir: &Path,
    run_id_text: &str,
    outcome: SoakOutcomeLabel,
    checkpoint: SoakCheckpointStatus,
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
                status: Box::new(checkpoint),
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

fn soak_event(run_id: SoakRunId, sequence: u64, event: SoakLedgerEvent) -> SoakLedgerEventEnvelope {
    SoakLedgerEventEnvelope::new(run_id, sequence, 1_781_485_562 + sequence, event)
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
        maybe_recovery_action_class_label: None,
        maybe_recovery_cause_label: None,
        maybe_recovery_next_action: None,
        maybe_no_progress_diagnosis_label: Some("RPC credentials phase75-secret".to_string()),
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
        maybe_validated_active_chain_height: Some(900_000),
        maybe_best_known_tip_height: Some(900_000),
        maybe_source_status_path: Some(PathBuf::from("unbounded peer tables phase75-secret")),
    }
}

fn phase79_sensitive_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("mainnet".to_string()),
        maybe_lifecycle: Some("raw daemon stdout phase79-secret".to_string()),
        maybe_latest_stop_reason_label: Some("raw daemon stderr phase79-secret".to_string()),
        maybe_recovery_category_label: Some("phase79 wallet seed phrase".to_string()),
        maybe_recovery_action_class_label: Some("raw options phase79-secret".to_string()),
        maybe_recovery_cause_label: Some("rpcpassword=phase79-secret".to_string()),
        maybe_recovery_next_action: Some("rpcauth=phase79-secret".to_string()),
        maybe_no_progress_diagnosis_label: Some("raw live-smoke input phase79-secret".to_string()),
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
        maybe_last_peer_contribution_label: Some("endpoint table phase79-secret".to_string()),
        maybe_stalled_subsystem_label: None,
        maybe_stall_confidence_label: None,
        stall_evidence_basis: Vec::new(),
        maybe_stall_next_action: None,
        maybe_resource_bound_state_label: Some("normal".to_string()),
        resource_bound_labels: vec!["all_required_bounds=normal".to_string()],
        maybe_resource_bound_next_action: None,
        maybe_validated_active_chain_height: Some(900_000),
        maybe_best_known_tip_height: Some(900_000),
        maybe_source_status_path: None,
    }
}

fn phase79_sensitive_literals() -> [&'static str; 8] {
    [
        "rpcpassword=phase79-secret",
        "rpcauth=phase79-secret",
        "phase79 wallet seed phrase",
        "raw daemon stdout phase79-secret",
        "raw daemon stderr phase79-secret",
        "raw live-smoke input phase79-secret",
        "raw options phase79-secret",
        "endpoint table phase79-secret",
    ]
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
