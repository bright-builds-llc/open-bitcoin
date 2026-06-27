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
        InboundAddressDecisionEvent, InboundAddressEvidenceEntry, InboundAdmissionEvent,
        InboundHandshakeStatusCounts, InboundPeerPolicyEvent, InboundPeerServingStatus,
        InboundPermissionDecisionEvent, InboundResourceGovernanceEvent, MempoolStatus,
        NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState,
        NodeRuntimeState, NodeStatus, PeerContributionEvidence, PeerContributionKind, PeerCounts,
        PeerStatus, PeerTipAgreement, PeerTipAgreementStatus, ProgressCreditEvidence,
        ProgressCreditKind, ProgressWindowEvidence, RejectedProgressActivity,
        RejectedProgressActivityKind, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
        ResourceBoundUnit, ServiceLifecycleStatus, ServiceStatus, StallDiagnosisConfidence,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncAttemptCounters,
        SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncRecoveryCategory, SyncResourcePressure, SyncStatus, SyncStopReasonStatus,
        TipFreshnessStatus, WalletStatus, inbound_status_unavailable, usage_against_budget,
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
    support_status_for_bundle,
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
            "inbound peer endpoints bounded/redacted",
            "inbound permission labels bounded to machine classes/effects",
            "inbound address boundary evidence bounded/redacted",
            "inbound peer policy evidence bounded/redacted",
            "inbound resource-governance evidence bounded/redacted",
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
fn phase79_support_forensics_markdown_renders_timeline_chain_and_failure_narrative() {
    // Arrange
    let temp = TestDirectory::new("phase79-markdown");
    seed_phase75_soak_run(
        temp.path(),
        "soak-1781485562-0083",
        SoakOutcomeLabel::CleanCompletion,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Forensic Timeline",
        "## Checkpoint Chain",
        "## Failure Narrative",
        "Verdict: soak_stable",
        "Likely cause: soak completed with validated progress evidence",
        "Evidence basis: outcome=clean_completion",
        "Next action: Archive the support bundle as soak stability evidence.",
        "Confidence: high",
        "Algorithm: sha256-json-v1",
        "Event count: 4",
        "Missing sequence count: 0",
        "Truncated: false",
        "Sequence 1: kind=run_start recorded_at=1781485562",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn phase79_support_forensics_cross_surface_agreement_uses_shared_status_contract() {
    // Arrange
    let temp = TestDirectory::new("phase79-cross-surface");
    let status = phase79_shared_contract_status();
    seed_soak_run_with_checkpoint(
        temp.path(),
        "soak-1781485562-0084",
        SoakOutcomeLabel::RecoveryStop,
        phase79_shared_checkpoint_status(),
    );
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let forensics_text =
        serde_json::to_string(&serialized["support_forensics"]).expect("forensics json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["support_forensics"]["source"]["event_count"],
        json!(4)
    );
    assert_eq!(
        serialized["support_forensics"]["narrative"]["evidence_basis"],
        json!(["recovery=stale_lock_evidence"])
    );
    assert!(serialized["resource_bound_evidence"]["maybe_projected_bundle_size_bytes"].is_number());
    assert!(serialized["support_forensics"]["maybe_projected_bundle_size_bytes"].is_null());
    for topic in [
        "RPC cookie contents",
        "RPC password and RPC auth values",
        "wallet private material and raw wallet files",
        "raw unbounded log contents",
    ] {
        assert!(
            serialized["support_forensics"]["redaction"]["omitted"]
                .as_array()
                .expect("redaction omitted")
                .iter()
                .any(|value| value.as_str() == Some(topic)),
            "missing redaction topic {topic}"
        );
    }
    for expected in [
        "recovery=stale_lock_evidence",
        "resource_bound=warning",
        "resource_bound_label=support_bundle=warning",
        "progress_credit=validated_durable_active_chain",
        "last_peer_contribution=peer=peer-79 kind=headers_and_blocks messages=9 headers=4 blocks=2 failure=unavailable",
        "stall=storage_or_resource_pressure",
        "stall_confidence=medium",
    ] {
        assert!(
            forensics_text.contains(expected),
            "missing JSON label {expected}"
        );
        assert!(
            markdown.contains(expected),
            "missing Markdown label {expected}"
        );
    }
}

#[test]
fn phase79_support_forensics_markdown_and_json_exclude_forbidden_sensitive_material() {
    // Arrange
    let temp = TestDirectory::new("phase79-sensitive-render");
    seed_phase79_sensitive_soak_run(
        temp.path(),
        "soak-1781485562-0085",
        SoakOutcomeLabel::ResourceStop,
    );
    let bundle = phase75_support_bundle_for_test(temp.path());

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in phase79_sensitive_literals() {
            assert_absent(rendered, forbidden);
        }
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

#[test]
fn inbound_support_json_projects_shared_status_evidence_with_redacted_endpoints() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-json");
    let status = phase90_status_with_available_inbound();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let inbound = &serialized["status"]["peers"]["inbound"]["value"];

    // Assert
    assert_eq!(
        serialized["status"]["peers"]["inbound"]["state"],
        json!("available")
    );
    assert_eq!(inbound["listener_state"], json!("listening"));
    assert_eq!(inbound["preflight_reason"], json!("ready"));
    assert_eq!(inbound["admitted_inbound_peers"], json!(3));
    assert_eq!(inbound["rejected_inbound_peers"], json!(4));
    assert_eq!(inbound["duplicate_rejects"], json!(1));
    assert_eq!(inbound["self_connection_rejects"], json!(1));
    assert_eq!(inbound["cap_rejects"], json!(1));
    assert_eq!(inbound["reserved_slot_rejects"], json!(1));
    assert_eq!(
        inbound["bound_endpoints"],
        json!([
            "1 loopback endpoint redacted",
            "2 non-loopback endpoints redacted",
            "1 wildcard endpoint redacted"
        ])
    );
    for forbidden in phase90_raw_inbound_endpoints() {
        assert_absent(&json_text, forbidden);
    }
}

#[test]
fn inbound_support_markdown_renders_bounded_admission_labels_and_next_action() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-markdown");
    let status = phase90_status_with_available_inbound();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Serving",
        "listener_state: listening",
        "preflight_reason: ready",
        "bound_endpoints: 1 loopback endpoint redacted, 2 non-loopback endpoints redacted, 1 wildcard endpoint redacted",
        "admitted_inbound_peers: 3",
        "rejected_inbound_peers: 4",
        "handshake.awaiting_version: 1",
        "handshake.awaiting_verack: 2",
        "handshake.established: 3",
        "handshake.disconnected: 4",
        "duplicate_rejects: 1",
        "self_connection_rejects: 1",
        "cap_rejects: 1",
        "reserved_slot_rejects: 1",
        "latest_admission_event: outcome=rejected reason=cap_reject slot_class=ordinary message=inbound cap reached",
        "Next action: Review configured inbound caps and reserved slots before increasing listener exposure.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    for forbidden in phase90_raw_inbound_endpoints() {
        assert_absent(&markdown, forbidden);
    }
}

#[test]
fn inbound_support_markdown_renders_permission_labels_and_inactive_relay_guidance() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-permissions");
    let status = phase91_status_with_permissioned_inbound();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "permission_class: protected_inbound",
        "permissioned_inbound_peers: 2",
        "protected_inbound_peers: 1",
        "active_permission_effects: admission_protected, download_serving_policy_input",
        "inactive_permission_effects: inactive_relay, inactive_mempool",
        "latest_permission_decision: outcome=admitted reason=admitted permission_class=protected_inbound active_permission_effects=admission_protected inactive_permission_effects=inactive_relay message=inbound permission decision admitted as protected_inbound",
        "Next action: Relay, mempool, bloom, and blockfilter permissions are recorded as inactive Phase 91 evidence; do not treat them as relay support.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_support_markdown_renders_phase92_address_boundary_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-address-boundary");
    let status = phase92_status_with_address_boundary_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Address Boundary Evidence",
        "Local advertisement candidates: 1",
        "source=source_local_listener",
        "routability=publicly_routable",
        "persistence_eligible=true",
        "Suppressed advertisements: 2",
        "not_publicly_routable",
        "local evidence only",
        "Bounded getaddr responses served: 3",
        "Bounded getaddr requests suppressed: 2",
        "Learned address entries: 5",
        "Learned address rejections: 1",
        "Latest address decision: outcome=suppressed reason=already_served label=getaddr_suppressed source=source_inbound_addr message=bounded getaddr already served",
        "Next action: Treat Phase 92 as bounded local advertisement and direct getaddr evidence only; peer discovery, unsolicited address relay, DNS seed discovery, UPnP/NAT-PMP discovery, and public-network readiness remain outside this surface.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    for forbidden in [
        "full address relay support",
        "peer discovery support",
        "addr gossip support",
    ] {
        assert_absent(&markdown, forbidden);
    }
}

#[test]
fn inbound_support_markdown_renders_phase93_peer_policy_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-peer-policy");
    let status = phase93_status_with_peer_policy_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Peer Policy Evidence",
        "Eviction candidates evaluated: 2",
        "Disconnects requested: 1",
        "Discouraged peers: 1",
        "Active bans: 1",
        "Expired bans: 0",
        "Manual unbans: 0",
        "Misbehavior observations: 2",
        "Protected no-actions: 1",
        "Latest peer policy decision: outcome=selected reason=low_activity label=eviction_candidate_selected source=source_eviction_policy message=peer policy decision selected: low_activity",
        "Next action: Treat Phase 93 as bounded eviction, ban, unban, and misbehavior policy evidence only; review labels and counters before changing peer policy.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_support_markdown_renders_phase94_resource_governance_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-resource-governance");
    let status = phase94_status_with_resource_governance_evidence();
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for expected in [
        "## Inbound Resource Governance Evidence",
        "Resource pressure events: 8",
        "Read queue pressure events: 7",
        "Write queue pressure events: 6",
        "Request cap events: 5",
        "Payload rejections: 1",
        "Timeout disconnects: 3",
        "Churn rejections: 2",
        "Reconnect suppressions: 4",
        "Latest resource governance decision: outcome=rejected reason=invalid_checksum label=payload_rejected source=source_inbound_resource_governance message=bounded payload rejected next_action=payload_rejected",
        "Next action: Treat Phase 94 as bounded inbound resource-governance evidence only; inspect resource labels before raising listener exposure, queue caps, request caps, or timeout thresholds.",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
}

#[test]
fn inbound_support_redacts_raw_phase94_resource_governance_material() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-resource-governance-redaction");
    let mut status = phase94_status_with_resource_governance_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.latest_resource_governance_decision =
        FieldAvailability::available(InboundResourceGovernanceEvent {
            outcome: "rejected 127.0.0.1:18444 peer-94".to_string(),
            reason: "invalid_checksum peer_id=94 raw_endpoint=0.0.0.0:8333".to_string(),
            label: "payload_rejected payload_bytes=[00] raw_permission".to_string(),
            source: "source_inbound_resource_governance permission_string=in,noban".to_string(),
            message: "0.0.0.0:8333 ::1 config=operator rpc_password=phase95 credential=phase95 secret=phase95 cookie=phase95"
                .to_string(),
            next_action: "peer-94 payload_bytes raw_endpoint permission_string config=operator"
                .to_string(),
        });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);
    let decision = &serialized["status"]["peers"]["inbound"]["value"]["latest_resource_governance_decision"]
        ["value"];
    let decision_json_text = serde_json::to_string_pretty(decision).expect("decision json");
    let decision_markdown_line = markdown
        .lines()
        .find(|line| line.contains("Latest resource governance decision:"))
        .expect("resource governance decision line");

    // Assert
    for field in [
        "outcome",
        "reason",
        "label",
        "source",
        "message",
        "next_action",
    ] {
        assert_eq!(
            decision[field],
            json!("redacted_resource_governance_evidence"),
            "unexpected {field} redaction"
        );
    }
    for rendered in [&decision_json_text, decision_markdown_line] {
        assert!(rendered.contains("redacted_resource_governance_evidence"));
        for forbidden in [
            "127.0.0.1:",
            "0.0.0.0:",
            "::1",
            "peer_id=",
            "peer-",
            "raw_endpoint",
            "payload_bytes",
            "raw_permission",
            "permission_string",
            "config=",
            "rpc_password",
            "credential",
            "secret",
            "cookie=",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn inbound_support_redacts_raw_phase92_address_boundary_material() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-address-redaction");
    let mut status = phase92_status_with_address_boundary_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.local_advertisement_candidates[0].source =
        "source_local_listener 127.0.0.1:18444 address_bytes=[127,0,0,1]".to_string();
    inbound.suppressed_advertisements[0].message =
        "local evidence only peer_id=92 ::1 operator_loopback raw_permission".to_string();
    inbound.latest_address_decision = FieldAvailability::available(InboundAddressDecisionEvent {
        outcome: "suppressed".to_string(),
        reason: "permission_policy_denied".to_string(),
        label: "getaddr_suppressed".to_string(),
        source: "source_inbound_addr".to_string(),
        message: "bounded getaddr denied 0.0.0.0:8333 peer_id=92 raw_permission config=operator"
            .to_string(),
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "127.0.0.1:",
            "0.0.0.0:",
            "::1",
            "address_bytes",
            "peer_id=",
            "operator_loopback",
            "raw_permission",
            "full address relay support",
            "peer discovery support",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
    assert!(markdown.contains("redacted_address_evidence"));
}

#[test]
fn inbound_support_redacts_raw_phase93_peer_policy_material() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-peer-policy-redaction");
    let mut status = phase93_status_with_peer_policy_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "peer-93-disconnect".to_string(),
        reason: "operator-loopback-secret".to_string(),
        label: "peer_id=93".to_string(),
        source: "127.0.0.1:18444".to_string(),
        message: "peer-93 127.0.0.1:18444 raw_permission config=operator".to_string(),
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        assert!(rendered.contains("redacted_peer_policy_label"));
        for forbidden in [
            "peer-93",
            "peer_id=93",
            "127.0.0.1:",
            "operator-loopback-secret",
            "raw_permission",
            "config=operator",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn inbound_support_json_and_markdown_redact_raw_permission_config_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-permission-redaction");
    let mut status = phase91_status_with_permissioned_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.permission_class = "operator_loopback".to_string();
    inbound.active_permission_effects = vec![
        "admission_protected".to_string(),
        "in,noban,forceinbound".to_string(),
    ];
    inbound.inactive_permission_effects =
        vec!["inactive_relay".to_string(), "peer_id=91".to_string()];
    inbound.latest_permission_decision =
        FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "operator_loopback".to_string(),
            active_permission_effects: vec![
                "admission_protected".to_string(),
                "in,noban,forceinbound".to_string(),
            ],
            inactive_permission_effects: vec![
                "inactive_relay".to_string(),
                "peer_id=91".to_string(),
            ],
            message: "operator_loopback in,noban,forceinbound peer_id=91 127.0.0.1:18444 rpc_password cookie=phase91-secret"
                .to_string(),
        });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);
    let inbound = &serialized["status"]["peers"]["inbound"]["value"];
    let latest_decision = &inbound["latest_permission_decision"]["value"];

    // Assert
    assert_eq!(
        inbound["permission_class"],
        json!("redacted_permission_class")
    );
    assert_eq!(
        inbound["active_permission_effects"],
        json!(["admission_protected", "redacted_permission_effect"])
    );
    assert_eq!(
        inbound["inactive_permission_effects"],
        json!(["inactive_relay", "redacted_permission_effect"])
    );
    assert_eq!(
        latest_decision["permission_class"],
        json!("redacted_permission_class")
    );
    assert_eq!(
        latest_decision["message"],
        json!("inbound permission decision admitted as redacted_permission_class")
    );
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "operator_loopback",
            "in,noban,forceinbound",
            "peer_id=",
            "127.0.0.1:",
            "rpc_password",
            "cookie=phase91-secret",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
    assert!(markdown.contains("permission_class: redacted_permission_class"));
    assert!(
        markdown
            .contains("active_permission_effects: admission_protected, redacted_permission_effect")
    );
    assert!(
        markdown
            .contains("inactive_permission_effects: inactive_relay, redacted_permission_effect")
    );
}

#[test]
fn inbound_support_preserves_unavailable_reason_in_json_and_markdown() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-unavailable");
    let mut status = phase72_status();
    status.peers.inbound = FieldAvailability::unavailable("inbound probe not collected");
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["status"]["peers"]["inbound"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        serialized["status"]["peers"]["inbound"]["value"]["reason"],
        json!("inbound probe not collected")
    );
    assert!(markdown.contains("## Inbound Serving"));
    assert!(markdown.contains("Status: Unavailable: inbound probe not collected"));
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
            inbound: inbound_status_unavailable(),
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
    bundle.status = support_status_for_bundle(status);
    bundle.recovery_evidence =
        RecoverySupportEvidence::from_status(&bundle.status.recovery_evidence);
    bundle.full_sync_evidence = derive_full_sync_evidence(&bundle.status, &bundle.live_smoke);
    bundle.resource_bound_evidence =
        collect_resource_bound_support_evidence(&bundle.status, &data_dir.join("support"));
    bundle
}

fn phase90_status_with_available_inbound() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.peers.inbound = FieldAvailability::available(InboundPeerServingStatus {
        listener_state: "listening".to_string(),
        bound_endpoints: phase90_raw_inbound_endpoints()
            .into_iter()
            .map(str::to_string)
            .collect(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 3,
        rejected_inbound_peers: 4,
        handshake: InboundHandshakeStatusCounts {
            awaiting_version: 1,
            awaiting_verack: 2,
            established: 3,
            disconnected: 4,
        },
        duplicate_rejects: 1,
        self_connection_rejects: 1,
        cap_rejects: 1,
        reserved_slot_rejects: 1,
        latest_admission_event: FieldAvailability::available(InboundAdmissionEvent {
            outcome: "rejected".to_string(),
            reason: "cap_reject".to_string(),
            slot_class: "ordinary".to_string(),
            message: "inbound cap reached".to_string(),
        }),
        permissioned_inbound_peers: 0,
        protected_inbound_peers: 0,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        latest_permission_decision: FieldAvailability::unavailable(
            "inbound permission decision evidence unavailable",
        ),
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 0,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::unavailable(
            "inbound address boundary evidence unavailable",
        ),
        eviction_candidates_evaluated: 0,
        disconnects_requested: 0,
        discouraged_peers: 0,
        active_bans: 0,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 0,
        protected_no_actions: 0,
        latest_peer_policy_decision: FieldAvailability::unavailable(
            "inbound peer policy evidence unavailable",
        ),
        resource_pressure_events: 0,
        read_queue_pressure_events: 0,
        write_queue_pressure_events: 0,
        request_cap_events: 0,
        payload_rejections: 0,
        timeout_disconnects: 0,
        churn_rejections: 0,
        reconnect_suppressions: 0,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "inbound resource governance evidence unavailable",
        ),
    });
    status
}

fn phase91_status_with_permissioned_inbound() -> OpenBitcoinStatusSnapshot {
    let mut status = phase90_status_with_available_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.duplicate_rejects = 0;
    inbound.self_connection_rejects = 0;
    inbound.cap_rejects = 0;
    inbound.reserved_slot_rejects = 0;
    inbound.permissioned_inbound_peers = 2;
    inbound.protected_inbound_peers = 1;
    inbound.permission_class = "protected_inbound".to_string();
    inbound.active_permission_effects = vec![
        "admission_protected".to_string(),
        "download_serving_policy_input".to_string(),
    ];
    inbound.inactive_permission_effects =
        vec!["inactive_relay".to_string(), "inactive_mempool".to_string()];
    inbound.latest_permission_decision =
        FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "protected_inbound".to_string(),
            active_permission_effects: vec!["admission_protected".to_string()],
            inactive_permission_effects: vec!["inactive_relay".to_string()],
            message: "inbound permission decision admitted as protected_inbound".to_string(),
        });
    status
}

fn phase92_status_with_address_boundary_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase90_status_with_available_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.local_advertisement_candidates = vec![InboundAddressEvidenceEntry {
        source: "source_local_listener".to_string(),
        network_kind: "ipv4".to_string(),
        routability: "publicly_routable".to_string(),
        freshness: "fresh".to_string(),
        services_bits: 1,
        port: 8333,
        persistence_eligible: true,
    }];
    inbound.suppressed_advertisements = vec![
        InboundAddressDecisionEvent {
            outcome: "suppressed".to_string(),
            reason: "not_publicly_routable".to_string(),
            label: "not_publicly_routable".to_string(),
            source: "source_local_listener".to_string(),
            message: "local evidence only".to_string(),
        },
        InboundAddressDecisionEvent {
            outcome: "suppressed".to_string(),
            reason: "permission_policy_denied".to_string(),
            label: "permission_policy_denied".to_string(),
            source: "source_inbound_addr".to_string(),
            message: "bounded getaddr permission policy denied".to_string(),
        },
    ];
    inbound.getaddr_responses_served = 3;
    inbound.getaddr_requests_suppressed = 2;
    inbound.learned_address_entries = 5;
    inbound.learned_address_rejections = 1;
    inbound.latest_address_decision = FieldAvailability::available(InboundAddressDecisionEvent {
        outcome: "suppressed".to_string(),
        reason: "already_served".to_string(),
        label: "getaddr_suppressed".to_string(),
        source: "source_inbound_addr".to_string(),
        message: "bounded getaddr already served".to_string(),
    });
    status
}

fn phase93_status_with_peer_policy_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase92_status_with_address_boundary_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.eviction_candidates_evaluated = 2;
    inbound.disconnects_requested = 1;
    inbound.discouraged_peers = 1;
    inbound.active_bans = 1;
    inbound.expired_bans = 0;
    inbound.manual_unbans = 0;
    inbound.misbehavior_observations = 2;
    inbound.protected_no_actions = 1;
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "selected".to_string(),
        reason: "low_activity".to_string(),
        label: "eviction_candidate_selected".to_string(),
        source: "source_eviction_policy".to_string(),
        message: "peer eviction decision eviction_candidate_selected: low_activity".to_string(),
    });
    status
}

fn phase94_status_with_resource_governance_evidence() -> OpenBitcoinStatusSnapshot {
    let mut status = phase93_status_with_peer_policy_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.resource_pressure_events = 8;
    inbound.read_queue_pressure_events = 7;
    inbound.write_queue_pressure_events = 6;
    inbound.request_cap_events = 5;
    inbound.payload_rejections = 1;
    inbound.timeout_disconnects = 3;
    inbound.churn_rejections = 2;
    inbound.reconnect_suppressions = 4;
    inbound.latest_resource_governance_decision =
        FieldAvailability::available(InboundResourceGovernanceEvent {
            outcome: "rejected".to_string(),
            reason: "invalid_checksum".to_string(),
            label: "payload_rejected".to_string(),
            source: "source_inbound_resource_governance".to_string(),
            message: "bounded payload rejected".to_string(),
            next_action: "payload_rejected".to_string(),
        });
    status
}

fn phase90_raw_inbound_endpoints() -> [&'static str; 4] {
    [
        "127.0.0.1:18444",
        "203.0.113.11:8333",
        "198.51.100.21:8333",
        "0.0.0.0:8333",
    ]
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

fn phase79_shared_contract_status() -> OpenBitcoinStatusSnapshot {
    let mut status = phase72_status();
    status.recovery_evidence = FieldAvailability::available(phase77_recovery_evidence());
    status.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::StorageLockContention);
    status.sync.progress_credit = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_020,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 2,
            reason: "headers without durable active-chain update".to_string(),
        }],
    });
    status.sync.last_peer_contribution = FieldAvailability::available(PeerContributionEvidence {
        peer: "peer-79".to_string(),
        maybe_resolved_endpoint: None,
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        maybe_last_activity_unix_seconds: Some(1_717_000_030),
        maybe_failure_reason_label: None,
    });
    status.sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::StorageOrResourcePressure,
        confidence: StallDiagnosisConfidence::Medium,
        evidence_basis: vec![
            "resource_bounds".to_string(),
            "support_bundle_size".to_string(),
        ],
        next_action: "Inspect support bundle resource bounds.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        maybe_recovery_category: Some(SyncRecoveryCategory::StorageLockContention),
        maybe_latest_stop_reason_label: Some("resource_pressure".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
    status.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::StorageOrResourceBlocked);
    status.resource_bounds = FieldAvailability::available(phase79_resource_bound_snapshot());
    status
}

fn phase79_shared_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("mainnet".to_string()),
        maybe_lifecycle: Some("active".to_string()),
        maybe_latest_stop_reason_label: Some("resource_pressure".to_string()),
        maybe_recovery_category_label: Some("storage_lock_contention".to_string()),
        maybe_recovery_action_class_label: Some("read_only_inspection".to_string()),
        maybe_recovery_cause_label: Some("stale_lock_evidence".to_string()),
        maybe_recovery_next_action: Some(
            "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
                .to_string(),
        ),
        maybe_no_progress_diagnosis_label: Some("storage_or_resource_blocked".to_string()),
        maybe_progress_credit_kind_label: Some("validated_durable_active_chain".to_string()),
        maybe_progress_credit_height: Some(840_004),
        maybe_progress_credit_hash: Some("11".repeat(32)),
        maybe_progress_credit_work: Some("840005".to_string()),
        maybe_progress_credit_source_unix_seconds: Some(1_717_000_020),
        progress_credit_rejected_activity_labels: vec![
            "kind=header_download observed_count=2 reason=headers without durable active-chain update"
                .to_string(),
        ],
        maybe_expected_progress_window_seconds: Some(300),
        maybe_no_progress_threshold_state_label: Some("within_window".to_string()),
        maybe_no_progress_threshold_seconds: Some(300),
        maybe_last_useful_work_kind_label: Some("current_at_best_known_tip".to_string()),
        maybe_last_useful_work_height: Some(840_004),
        maybe_last_peer_contribution_label: Some(
            "peer=peer-79 kind=headers_and_blocks messages=9 headers=4 blocks=2 failure=unavailable"
                .to_string(),
        ),
        maybe_stalled_subsystem_label: Some("storage_or_resource_pressure".to_string()),
        maybe_stall_confidence_label: Some("medium".to_string()),
        stall_evidence_basis: vec![
            "resource_bounds".to_string(),
            "support_bundle_size".to_string(),
        ],
        maybe_stall_next_action: Some("Inspect support bundle resource bounds.".to_string()),
        maybe_resource_bound_state_label: Some("warning".to_string()),
        resource_bound_labels: vec!["support_bundle=warning".to_string()],
        maybe_resource_bound_next_action: Some("Archive or rotate large support bundles.".to_string()),
        maybe_validated_active_chain_height: Some(840_004),
        maybe_best_known_tip_height: Some(840_004),
        maybe_source_status_path: Some(PathBuf::from("/tmp/open-bitcoin-mainnet/status-snapshot.json")),
    }
}

fn phase79_resource_bound_snapshot() -> ResourceBoundSnapshot {
    ResourceBoundSnapshot::new(
        ResourceBoundKind::ALL
            .into_iter()
            .map(|kind| {
                let current = if kind == ResourceBoundKind::SupportBundle {
                    85
                } else {
                    10
                };
                ResourceBoundEntry::available(
                    kind,
                    kind.as_str(),
                    usage_against_budget(
                        current,
                        100,
                        phase79_resource_unit(kind),
                        "Review bounded support resource usage.",
                    ),
                )
            })
            .collect(),
    )
}

fn phase79_resource_unit(kind: ResourceBoundKind) -> ResourceBoundUnit {
    match kind {
        ResourceBoundKind::File => ResourceBoundUnit::Files,
        ResourceBoundKind::Queue | ResourceBoundKind::Cache => ResourceBoundUnit::Items,
        ResourceBoundKind::Peer => ResourceBoundUnit::Peers,
        ResourceBoundKind::InFlight => ResourceBoundUnit::Requests,
        ResourceBoundKind::Disk
        | ResourceBoundKind::Log
        | ResourceBoundKind::Metric
        | ResourceBoundKind::SupportBundle => ResourceBoundUnit::Bytes,
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
