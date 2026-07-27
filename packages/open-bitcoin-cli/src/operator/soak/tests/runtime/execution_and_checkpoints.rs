use super::*;

#[test]
fn soak_runtime_elapsed_writes_started_checkpoints_stop_and_verdict() {
    // Arrange
    let temp = TestDirectory::new("elapsed");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-0001").expect("run id");
    let bounds = soak_bounds(temp.path(), Some(144), vec![SoakStopCondition::ElapsedTime]);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut collector = ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Start,
    )
    .expect("bounded soak loop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::CleanCompletion);
    assert_eq!(result.latest_sequence, 8);
    assert!(result.report_paths.json_path.exists());
    assert!(result.report_paths.markdown_path.exists());
    assert!(matches!(&events[0].event, SoakLedgerEvent::Started { .. }));
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(&event.event, SoakLedgerEvent::Checkpoint { .. }))
            .count(),
        5
    );
    assert!(matches!(
        &events[6].event,
        SoakLedgerEvent::Stop {
            outcome: SoakOutcomeLabel::CleanCompletion
        }
    ));
    assert!(matches!(
        &events[7].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::CleanCompletion
        }
    ));
}

#[test]
fn soak_start_preflight_refuses_resource_bounds_before_ledger_mutation() {
    // Arrange
    let temp = TestDirectory::new("preflight-resource-stop");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let args = crate::operator::SoakStartArgs {
        elapsed_time_seconds: 30,
        checkpoint_interval_seconds: 15,
        maybe_target_height: None,
        peer_policy: crate::operator::SoakPeerPolicyArg::DaemonConfigured,
        disk_budget_bytes: 1_048_576,
        stop_condition: crate::operator::SoakStopConditionArg::ResourceStop,
        maybe_run_id: Some("soak-1700000000-0001".to_string()),
    };
    let mut collector = ScriptedStatusCollector::repeating(resource_status_snapshot(temp.path()));
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    let result = runtime::execute_soak_start(
        &args,
        OperatorOutputFormat::Json,
        &layout,
        Some(crate::operator::NetworkSelection::Regtest),
        &mut collector,
        &mut clock,
    );

    // Assert
    assert!(result.is_err());
    assert!(!layout.run_index_path().exists());
    assert!(
        !layout
            .paths_for_run(&SoakRunId::try_new("soak-1700000000-0001").expect("run id"))
            .events_path
            .exists()
    );
}

#[test]
fn soak_runtime_target_height_resource_recovery_and_status_verdict_stop_conditions() {
    // Arrange
    let temp = TestDirectory::new("stop-conditions");
    let cases = [
        (
            "target",
            vec![SoakStopCondition::TargetHeight],
            clean_status_snapshot(temp.path(), 144),
            Some(144),
            SoakOutcomeLabel::CleanCompletion,
        ),
        (
            "resource",
            vec![SoakStopCondition::ResourceStop],
            resource_status_snapshot(temp.path()),
            None,
            SoakOutcomeLabel::ResourceStop,
        ),
        (
            "recovery",
            vec![SoakStopCondition::RecoveryStop],
            recovery_status_snapshot(temp.path()),
            None,
            SoakOutcomeLabel::RecoveryStop,
        ),
        (
            "operator",
            vec![SoakStopCondition::OperatorStop],
            operator_stop_status_snapshot(temp.path()),
            None,
            SoakOutcomeLabel::OperatorStop,
        ),
        (
            "verdict",
            vec![SoakStopCondition::StatusVerdict],
            diagnosed_status_snapshot(temp.path()),
            None,
            SoakOutcomeLabel::DiagnosedBlocker,
        ),
    ];

    // Act / Assert
    for (label, stop_conditions, snapshot, maybe_target_height, expected) in cases {
        let run_id = SoakRunId::try_new(format!("soak-1700000000-{label}")).expect("run id");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        let bounds = soak_bounds(temp.path(), maybe_target_height, stop_conditions);
        let mut collector = ScriptedStatusCollector::from_snapshots(vec![snapshot]);
        let mut clock = SoakTestClock::new(1_700_000_000);

        let result = run_bounded_soak_loop(
            &run_id,
            &bounds,
            &layout,
            &mut ledger,
            &mut collector,
            &mut clock,
            SoakLoopMode::Start,
        )
        .expect("bounded stop condition loop");

        assert_eq!(result.final_outcome, expected, "case {label}");
    }
}

#[test]
fn soak_recovery_evidence_checkpoint_available_top_level_evidence_records_labels() {
    // Arrange
    let temp = TestDirectory::new("recovery-evidence-checkpoint");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-recovery-evidence").expect("run id");
    let bounds = soak_bounds(temp.path(), None, vec![SoakStopCondition::StatusVerdict]);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut collector =
        ScriptedStatusCollector::repeating(phase77_recovery_status_snapshot(temp.path()));
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Start,
    )
    .expect("bounded soak loop");
    let checkpoint = latest_checkpoint_status(&layout, &run_id);

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::RecoveryStop);
    assert_eq!(
        checkpoint.maybe_recovery_category_label.as_deref(),
        Some("store_corruption")
    );
    assert_eq!(
        checkpoint.maybe_recovery_action_class_label.as_deref(),
        Some("backup_then_rebuild")
    );
    assert_eq!(
        checkpoint.maybe_recovery_cause_label.as_deref(),
        Some("partial_write")
    );
    assert_eq!(
        checkpoint.maybe_recovery_next_action.as_deref(),
        Some(
            "Back up the selected datadir, then rebuild affected storage before normal operation."
        )
    );
}

#[test]
fn soak_recovery_evidence_checkpoint_unavailable_evidence_leaves_optional_fields_empty() {
    // Arrange
    let temp = TestDirectory::new("recovery-evidence-unavailable");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-recovery-unavailable").expect("run id");
    let bounds = soak_bounds(temp.path(), None, vec![SoakStopCondition::StatusVerdict]);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut collector = ScriptedStatusCollector::repeating(recovery_status_snapshot(temp.path()));
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Start,
    )
    .expect("bounded soak loop");
    let checkpoint = latest_checkpoint_status(&layout, &run_id);

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::RecoveryStop);
    assert_eq!(
        checkpoint.maybe_recovery_category_label.as_deref(),
        Some("store_corruption")
    );
    assert_eq!(checkpoint.maybe_recovery_action_class_label, None);
    assert_eq!(checkpoint.maybe_recovery_cause_label, None);
    assert_eq!(checkpoint.maybe_recovery_next_action, None);
}

#[test]
fn soak_progress_guarantee_checkpoint_available_status_records_shared_fields() {
    // Arrange
    let temp = TestDirectory::new("progress-guarantee-checkpoint");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-progress-guarantee").expect("run id");
    let bounds = soak_bounds(
        temp.path(),
        Some(900_000),
        vec![SoakStopCondition::TargetHeight],
    );
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut collector = ScriptedStatusCollector::repeating(progress_guarantee_status_snapshot(
        temp.path(),
        900_000,
    ));
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Start,
    )
    .expect("bounded soak loop");
    let checkpoint = latest_checkpoint_status(&layout, &run_id);

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::CleanCompletion);
    assert_eq!(
        checkpoint.maybe_progress_credit_kind_label.as_deref(),
        Some("validated_durable_active_chain")
    );
    assert_eq!(checkpoint.maybe_progress_credit_height, Some(900_000));
    assert_eq!(
        checkpoint.maybe_progress_credit_hash.as_deref(),
        Some("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        checkpoint.maybe_progress_credit_work.as_deref(),
        Some("900001")
    );
    assert_eq!(
        checkpoint.maybe_progress_credit_source_unix_seconds,
        Some(1_777_300_060)
    );
    assert_eq!(
        checkpoint.progress_credit_rejected_activity_labels,
        vec![
            "kind=header_download observed_count=2 reason=headers are not durable active-chain progress"
                .to_string(),
        ]
    );
    assert_eq!(checkpoint.maybe_expected_progress_window_seconds, Some(240));
    assert_eq!(
        checkpoint
            .maybe_no_progress_threshold_state_label
            .as_deref(),
        Some("within_window")
    );
    assert_eq!(checkpoint.maybe_no_progress_threshold_seconds, Some(240));
    assert_eq!(
        checkpoint.maybe_last_useful_work_kind_label.as_deref(),
        Some("validated_durable_active_chain")
    );
    assert_eq!(checkpoint.maybe_last_useful_work_height, Some(900_000));
    assert_eq!(
        checkpoint.maybe_last_peer_contribution_label.as_deref(),
        Some(
            "peer=peer-1 kind=headers_and_blocks messages=7 headers=3 blocks=1 failure=unavailable"
        )
    );
    assert_eq!(
        checkpoint.maybe_stalled_subsystem_label.as_deref(),
        Some("slow_or_stalled_peers")
    );
    assert_eq!(
        checkpoint.maybe_stall_confidence_label.as_deref(),
        Some("medium")
    );
    assert_eq!(
        checkpoint.stall_evidence_basis,
        vec!["latest peer stalled before useful work".to_string()]
    );
    assert_eq!(
        checkpoint.maybe_stall_next_action.as_deref(),
        Some("Rotate peers and continue bounded sync.")
    );
}

#[test]
fn soak_progress_guarantee_checkpoint_unavailable_status_leaves_optional_fields_empty() {
    // Arrange
    let temp = TestDirectory::new("progress-guarantee-unavailable");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-progress-unavailable").expect("run id");
    let bounds = soak_bounds(
        temp.path(),
        Some(144),
        vec![SoakStopCondition::TargetHeight],
    );
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut collector = ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Start,
    )
    .expect("bounded soak loop");
    let checkpoint = latest_checkpoint_status(&layout, &run_id);

    // Assert
    assert_eq!(checkpoint.maybe_progress_credit_kind_label, None);
    assert_eq!(checkpoint.maybe_progress_credit_height, None);
    assert_eq!(checkpoint.maybe_progress_credit_hash, None);
    assert_eq!(checkpoint.maybe_progress_credit_work, None);
    assert_eq!(checkpoint.maybe_progress_credit_source_unix_seconds, None);
    assert!(
        checkpoint
            .progress_credit_rejected_activity_labels
            .is_empty()
    );
    assert_eq!(checkpoint.maybe_expected_progress_window_seconds, None);
    assert_eq!(checkpoint.maybe_no_progress_threshold_state_label, None);
    assert_eq!(checkpoint.maybe_no_progress_threshold_seconds, None);
    assert_eq!(checkpoint.maybe_last_useful_work_kind_label, None);
    assert_eq!(checkpoint.maybe_last_useful_work_height, None);
    assert_eq!(checkpoint.maybe_last_peer_contribution_label, None);
    assert_eq!(checkpoint.maybe_stalled_subsystem_label, None);
    assert_eq!(checkpoint.maybe_stall_confidence_label, None);
    assert!(checkpoint.stall_evidence_basis.is_empty());
    assert_eq!(checkpoint.maybe_stall_next_action, None);
}

#[test]
fn soak_recovery_evidence_checkpoint_outcome_prefers_top_level_category_over_legacy_sync() {
    // Arrange
    let temp = TestDirectory::new("recovery-evidence-outcome");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-recovery-outcome").expect("run id");
    let bounds = soak_bounds(temp.path(), None, vec![SoakStopCondition::StatusVerdict]);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut snapshot = phase77_recovery_status_snapshot(temp.path());
    snapshot.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion);
    let mut collector = ScriptedStatusCollector::repeating(snapshot);
    let mut clock = SoakTestClock::new(1_700_000_000);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Start,
    )
    .expect("bounded soak loop");

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::RecoveryStop);
}

#[test]
fn soak_runtime_resume_refuses_clean_completion_and_flags_interrupted_runs() {
    // Arrange
    let temp = TestDirectory::new("resume");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let clean_run_id = SoakRunId::try_new("soak-1700000000-clean").expect("run id");
    let interrupted_run_id = SoakRunId::try_new("soak-1700000000-interrupted").expect("run id");
    let clean_paths = layout.paths_for_run(&clean_run_id);
    let interrupted_paths = layout.paths_for_run(&interrupted_run_id);
    let mut clean_ledger = SoakLedger::create(&layout, clean_run_id.clone());
    clean_ledger
        .append_event(
            1,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("clean started");
    clean_ledger
        .append_event(
            2,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::CleanCompletion,
            },
        )
        .expect("clean stop");
    clean_ledger
        .append_event(
            3,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::CleanCompletion,
            },
        )
        .expect("clean verdict");
    let mut interrupted_ledger = SoakLedger::create(&layout, interrupted_run_id.clone());
    interrupted_ledger
        .append_event(
            1,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("interrupted started");
    let mut index = SoakRunIndex::empty();
    index.record_run(SoakRunIndexEntry {
        run_id: clean_run_id.clone(),
        ledger_path: clean_paths.events_path.clone(),
        started_at_unix_seconds: 1,
        updated_at_unix_seconds: 3,
        maybe_outcome: Some(SoakOutcomeLabel::CleanCompletion),
    });
    index.record_run(SoakRunIndexEntry {
        run_id: interrupted_run_id.clone(),
        ledger_path: interrupted_paths.events_path.clone(),
        started_at_unix_seconds: 1,
        updated_at_unix_seconds: 1,
        maybe_outcome: None,
    });
    index.write_atomic(&layout).expect("write index");

    // Act
    let clean_result = validate_resume_plan(&layout, &clean_run_id, 15);
    let interrupted =
        validate_resume_plan(&layout, &interrupted_run_id, 15).expect("interrupted resume plan");

    // Assert
    assert!(clean_result.is_err());
    assert!(interrupted.interrupted_prior_run);
    assert_eq!(interrupted.next_sequence, 2);
    assert_eq!(interrupted.started_at_unix_seconds, 1);
}
