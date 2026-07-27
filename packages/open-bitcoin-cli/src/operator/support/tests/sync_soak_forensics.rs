use super::*;

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
            "raw transaction hex, txids, wtxids, cmpctblock/blocktxn payloads, peer endpoints, permission strings, credentials, and dynamic relay labels",
        ]
    );
    assert_eq!(
        safeguards,
        [
            "credential sources are represented as metadata only",
            "live smoke reports are summarized from allowlisted fields only",
            "logs are limited to existing structured status signals",
            "resource bounds are recorded as compact status summaries only",
            "relay and mempool evidence bounded/redacted",
            "block relay evidence bounded/redacted",
            "inbound peer endpoints bounded/redacted",
            "inbound permission labels bounded to machine classes/effects",
            "inbound address boundary evidence bounded/redacted",
            "inbound peer policy evidence bounded/redacted",
            "inbound resource-governance evidence bounded/redacted",
        ]
    );
    assert!(omitted.contains(
        &"raw transaction hex, txids, wtxids, cmpctblock/blocktxn payloads, peer endpoints, permission strings, credentials, and dynamic relay labels"
            .to_string()
    ));
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
