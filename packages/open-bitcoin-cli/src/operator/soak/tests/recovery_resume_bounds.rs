use super::*;

#[test]
fn soak_recovery_evidence_report_markdown_renders_recovery_labels() {
    // Arrange
    let temp = TestDirectory::new("recovery-report-markdown");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_recovery_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");

    // Act
    let rendered = render_soak_report_markdown(&projection);

    // Assert
    assert!(rendered.contains("Recovery category: store_corruption"));
    assert!(rendered.contains("Recovery action class: backup_then_rebuild"));
    assert!(rendered.contains("Recovery cause: partial_write"));
    assert!(rendered.contains(
        "Recovery next action: Back up the selected datadir, then rebuild affected storage before normal operation."
    ));
}

#[test]
fn soak_recovery_evidence_report_excludes_forbidden_raw_material() {
    // Arrange
    let temp = TestDirectory::new("recovery-report-redaction");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_recovery_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");
    let forbidden_material = [
        "raw backend",
        "rpcpassword",
        "Authorization",
        "wallet material",
    ];

    // Act
    let json = render_soak_report_json(&projection).expect("report json");
    let markdown = render_soak_report_markdown(&projection);

    // Assert
    for value in forbidden_material {
        assert!(!json.contains(value), "JSON report leaked {value}");
        assert!(!markdown.contains(value), "Markdown report leaked {value}");
    }
}

#[test]
fn soak_report_write_uses_ledger_events_without_updating_run_index() {
    // Arrange
    let temp = TestDirectory::new("report-write");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1781485562-0001").expect("run id");
    let paths = layout.paths_for_run(&run_id);
    let mut ledger = SoakLedger::create(&layout, run_id);
    for envelope in sample_report_events(temp.path()) {
        ledger
            .append_event(envelope.recorded_at_unix_seconds, envelope.event)
            .expect("append report event");
    }
    let read = SoakLedger::read_events(&paths.events_path).expect("read events");

    // Act
    let report_paths =
        write_soak_reports(&read, &paths.events_path, &layout).expect("write reports");

    // Assert
    assert_eq!(report_paths.latest_sequence, 5);
    assert_eq!(report_paths.json_path, paths.report_json_path);
    assert_eq!(report_paths.markdown_path, paths.report_markdown_path);
    assert!(report_paths.json_path.is_file());
    assert!(report_paths.markdown_path.is_file());
    assert!(!layout.run_index_path().exists());
}

#[test]
fn soak_synthetic_interrupted_run_replays_as_unexpected_termination_resume() {
    // Arrange
    let temp = TestDirectory::deterministic("synthetic-interrupted");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1777300000-interrupted").expect("run id");
    let paths = layout.paths_for_run(&run_id);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            SOAK_SYNTHETIC_STARTED_AT,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path()),
            },
        )
        .expect("append started");
    ledger
        .append_event(
            SOAK_SYNTHETIC_CHECKPOINT_AT,
            SoakLedgerEvent::Checkpoint {
                status: Box::new(checkpoint_status()),
            },
        )
        .expect("append checkpoint");
    let mut resume_ledger = SoakLedger::resume(&layout, run_id, 3);

    // Act
    let resume = resume_ledger
        .append_event(
            SOAK_SYNTHETIC_RESUME_OR_STOP_AT,
            SoakLedgerEvent::Resume {
                interrupted_prior_run: true,
            },
        )
        .expect("append interrupted resume");
    let read = SoakLedger::read_events(&paths.events_path).expect("read resumed ledger");
    let projection = SoakReportProjection::from_ledger_events(read.events, &paths.events_path)
        .expect("interrupted resume projection");

    // Assert
    assert_eq!(resume.sequence, 3);
    assert_eq!(
        resume.recorded_at_unix_seconds,
        SOAK_SYNTHETIC_RESUME_OR_STOP_AT
    );
    assert_eq!(projection.latest_sequence, 3);
    assert_eq!(projection.resume_count, 1);
    assert_eq!(projection.interrupted_resume_count, 1);
    assert!(projection.stop.is_none());
    assert!(projection.verdict.is_none());
}

#[test]
fn soak_synthetic_clean_completion_refuses_same_run_resume() {
    // Arrange
    let temp = TestDirectory::deterministic("synthetic-clean-completion");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1777300000-clean").expect("run id");
    let paths = layout.paths_for_run(&run_id);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            SOAK_SYNTHETIC_STARTED_AT,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path()),
            },
        )
        .expect("append started");
    ledger
        .append_event(
            SOAK_SYNTHETIC_CHECKPOINT_AT,
            SoakLedgerEvent::Checkpoint {
                status: Box::new(checkpoint_status()),
            },
        )
        .expect("append checkpoint");
    ledger
        .append_event(
            SOAK_SYNTHETIC_RESUME_OR_STOP_AT,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::CleanCompletion,
            },
        )
        .expect("append clean stop");
    ledger
        .append_event(
            SOAK_SYNTHETIC_RESUME_OR_STOP_AT,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::CleanCompletion,
            },
        )
        .expect("append clean verdict");
    let mut index = SoakRunIndex::empty();
    index.record_run(SoakRunIndexEntry {
        run_id: run_id.clone(),
        ledger_path: paths.events_path,
        started_at_unix_seconds: SOAK_SYNTHETIC_STARTED_AT,
        updated_at_unix_seconds: SOAK_SYNTHETIC_RESUME_OR_STOP_AT,
        maybe_outcome: Some(SoakOutcomeLabel::CleanCompletion),
    });
    index.write_atomic(&layout).expect("write run index");

    // Act
    let error = validate_resume_plan(&layout, &run_id, 60)
        .expect_err("clean_completion cannot resume same run");

    // Assert
    assert!(error.to_string().contains("clean_completion"));
}

#[test]
fn soak_synthetic_resource_stop_report_preserves_final_outcome() {
    // Arrange
    let temp = TestDirectory::deterministic("synthetic-resource-stop");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1777300000-resource").expect("run id");
    let paths = layout.paths_for_run(&run_id);
    let mut ledger = SoakLedger::create(&layout, run_id);
    ledger
        .append_event(
            SOAK_SYNTHETIC_STARTED_AT,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path()),
            },
        )
        .expect("append started");
    ledger
        .append_event(
            SOAK_SYNTHETIC_CHECKPOINT_AT,
            SoakLedgerEvent::Checkpoint {
                status: Box::new(resource_checkpoint_status()),
            },
        )
        .expect("append resource checkpoint");
    ledger
        .append_event(
            SOAK_SYNTHETIC_RESUME_OR_STOP_AT,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::ResourceStop,
            },
        )
        .expect("append resource stop");
    ledger
        .append_event(
            SOAK_SYNTHETIC_RESUME_OR_STOP_AT,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::ResourceStop,
            },
        )
        .expect("append resource verdict");
    let read = SoakLedger::read_events(&paths.events_path).expect("read resource ledger");

    // Act
    let projection =
        SoakReportProjection::from_ledger_events(read.events.clone(), &paths.events_path)
            .expect("resource projection");
    let report_paths =
        write_soak_reports(&read, &paths.events_path, &layout).expect("write resource reports");
    let markdown = fs::read_to_string(report_paths.markdown_path).expect("read markdown report");

    // Assert
    let verdict = projection.verdict.expect("latest resource verdict");
    assert_eq!(verdict.outcome, SoakOutcomeLabel::ResourceStop);
    assert_eq!(
        verdict.recorded_at_unix_seconds,
        SOAK_SYNTHETIC_RESUME_OR_STOP_AT
    );
    assert!(markdown.contains("Final outcome: resource_stop"));
    assert!(markdown.contains("Recovery category: resource_exhaustion"));
    assert!(markdown.contains("No-progress diagnosis: storage_or_resource_blocked"));
}

#[test]
fn soak_bounds_run_id_rejects_empty_and_path_like_values() {
    // Arrange
    let valid = "soak-1781485562-0001";

    // Act / Assert
    assert_eq!(
        SoakRunId::try_new(valid).expect("valid run id").as_str(),
        valid
    );
    assert!(SoakRunId::try_new("").is_err());
    assert!(SoakRunId::try_new("../other-run").is_err());
    assert!(SoakRunId::try_new("nested/run").is_err());
}

#[test]
fn soak_bounds_try_new_rejects_zero_and_missing_boundaries() {
    // Arrange
    let datadir = PathBuf::from("/tmp/open-bitcoin");
    let stop_conditions = vec![SoakStopCondition::ElapsedTime];

    // Act / Assert
    assert!(
        SoakBounds::try_new(
            0,
            60,
            None,
            datadir.clone(),
            "mainnet",
            SoakPeerPolicy::DaemonConfigured,
            1_024,
            stop_conditions.clone(),
        )
        .is_err()
    );
    assert!(
        SoakBounds::try_new(
            86_400,
            0,
            None,
            datadir.clone(),
            "mainnet",
            SoakPeerPolicy::DaemonConfigured,
            1_024,
            stop_conditions.clone(),
        )
        .is_err()
    );
    assert!(
        SoakBounds::try_new(
            86_400,
            60,
            None,
            datadir.clone(),
            "mainnet",
            SoakPeerPolicy::DaemonConfigured,
            0,
            stop_conditions.clone(),
        )
        .is_err()
    );
    assert!(
        SoakBounds::try_new(
            86_400,
            60,
            None,
            datadir,
            "mainnet",
            SoakPeerPolicy::DaemonConfigured,
            1_024,
            Vec::new(),
        )
        .is_err()
    );
}

#[test]
fn soak_bounds_serializes_peer_policy_and_stop_conditions() {
    // Arrange
    let bounds = SoakBounds::try_new(
        86_400,
        60,
        Some(900_000),
        PathBuf::from("/tmp/open-bitcoin"),
        "mainnet",
        SoakPeerPolicy::ManualPeersOnly,
        4_096,
        vec![
            SoakStopCondition::ElapsedTime,
            SoakStopCondition::TargetHeight,
            SoakStopCondition::StatusVerdict,
            SoakStopCondition::OperatorStop,
            SoakStopCondition::ResourceStop,
            SoakStopCondition::RecoveryStop,
        ],
    )
    .expect("valid soak bounds");

    // Act
    let serialized = serde_json::to_value(bounds).expect("bounds json");

    // Assert
    assert_eq!(
        serialized["peer_policy"],
        Value::String("manual_peers_only".to_string())
    );
    assert_eq!(
        serialized["stop_conditions"],
        serde_json::json!([
            "elapsed_time",
            "target_height",
            "status_verdict",
            "operator_stop",
            "resource_stop",
            "recovery_stop"
        ])
    );
    assert_eq!(
        serde_json::to_value(SoakPeerPolicy::DaemonConfigured).expect("peer policy json"),
        Value::String("daemon_configured".to_string())
    );
    assert_eq!(
        serde_json::to_value(SoakPeerPolicy::NoDnsSeeds).expect("peer policy json"),
        Value::String("no_dns_seeds".to_string())
    );
}

#[test]
fn soak_outcome_classifies_recovery_and_resource_evidence() {
    // Arrange
    let category_expectations = [
        (
            SyncRecoveryCategory::StorageLockContention, // StorageLockContention -> RecoveryStop
            SoakOutcomeLabel::RecoveryStop,
        ),
        (
            SyncRecoveryCategory::IncompatibleSchema,
            SoakOutcomeLabel::RecoveryStop,
        ),
        (
            SyncRecoveryCategory::StoreCorruption,
            SoakOutcomeLabel::RecoveryStop,
        ),
        (
            SyncRecoveryCategory::StorageBackendFailure, // StorageBackendFailure -> RecoveryStop
            SoakOutcomeLabel::RecoveryStop,
        ),
        (
            SyncRecoveryCategory::ResourceExhaustion, // ResourceExhaustion -> ResourceStop
            SoakOutcomeLabel::ResourceStop,
        ),
    ];
    let resource_diagnosis = SoakOutcomeEvidence {
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        ..SoakOutcomeEvidence::empty()
    };

    // Act / Assert
    for (recovery_category, expected_outcome) in category_expectations {
        assert_eq!(
            classify_soak_outcome(&SoakOutcomeEvidence {
                maybe_recovery_category: Some(recovery_category),
                ..SoakOutcomeEvidence::empty()
            }),
            expected_outcome,
            "{recovery_category:?} must preserve the Phase 75 {expected_outcome:?} outcome"
        );
    }
    assert_eq!(
        classify_soak_outcome(&resource_diagnosis),
        SoakOutcomeLabel::ResourceStop
    );
}

#[test]
fn soak_outcome_classifies_support_operator_and_process_evidence() {
    // Arrange
    let support_blocker = SoakOutcomeEvidence {
        maybe_full_sync_evidence: Some(full_sync_evidence(
            SupportEvidenceVerdict::DiagnosedBlocker,
        )),
        ..SoakOutcomeEvidence::empty()
    };
    let operator_stop = SoakOutcomeEvidence {
        maybe_sync_stop_reason: Some(SyncStopReasonStatus {
            label: "operator_stop".to_string(),
            message: "operator requested stop".to_string(),
        }),
        maybe_process_exit: Some(SoakProcessExitEvidence::operator_stop()),
        ..SoakOutcomeEvidence::empty()
    };
    let interrupted = SoakOutcomeEvidence {
        maybe_process_exit: Some(SoakProcessExitEvidence::interrupted_process()),
        ..SoakOutcomeEvidence::empty()
    };

    // Act / Assert
    assert_eq!(
        classify_soak_outcome(&support_blocker),
        SoakOutcomeLabel::DiagnosedBlocker
    );
    assert_eq!(
        classify_soak_outcome(&operator_stop),
        SoakOutcomeLabel::OperatorStop
    );
    assert_eq!(
        classify_soak_outcome(&interrupted),
        SoakOutcomeLabel::UnexpectedTermination
    );
}
