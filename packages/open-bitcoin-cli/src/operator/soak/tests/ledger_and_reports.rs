use super::*;

#[test]
fn soak_outcome_label_serializes_phase75_vocabulary() {
    // Arrange
    let cases = [
        (SoakOutcomeLabel::CleanCompletion, "clean_completion"),
        (SoakOutcomeLabel::DiagnosedBlocker, "diagnosed_blocker"),
        (SoakOutcomeLabel::OperatorStop, "operator_stop"),
        (SoakOutcomeLabel::ResourceStop, "resource_stop"),
        (SoakOutcomeLabel::RecoveryStop, "recovery_stop"),
        (
            SoakOutcomeLabel::UnexpectedTermination,
            "unexpected_termination",
        ),
    ];

    // Act / Assert
    for (label, expected) in cases {
        assert_eq!(
            serde_json::to_value(label).expect("outcome label json"),
            Value::String(expected.to_string())
        );
    }
}

#[test]
fn soak_ledger_layout_resolves_datadir_owned_paths() {
    // Arrange
    let layout = SoakLedgerLayout::for_datadir(Path::new("/tmp/node"));
    let run_id = SoakRunId::try_new("soak-1781485562-0001").expect("run id");

    // Act
    let paths = layout.paths_for_run(&run_id);

    // Assert
    assert_eq!(
        layout.run_index_path(),
        Path::new("/tmp/node").join("soak").join("run-index.json")
    );
    assert_eq!(
        paths.events_path,
        Path::new("/tmp/node")
            .join("soak")
            .join("runs")
            .join("soak-1781485562-0001")
            .join("events.jsonl")
    );
    assert_eq!(
        paths.report_json_path,
        Path::new("/tmp/node")
            .join("soak")
            .join("runs")
            .join("soak-1781485562-0001")
            .join("report.json")
    );
    assert_eq!(
        paths.report_markdown_path,
        Path::new("/tmp/node")
            .join("soak")
            .join("runs")
            .join("soak-1781485562-0001")
            .join("report.md")
    );
}

#[test]
fn soak_ledger_append_writes_complete_json_lines_with_increasing_sequences() {
    // Arrange
    let temp = TestDirectory::new("append");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1781485562-0001").expect("run id");
    let mut ledger = SoakLedger::create(&layout, run_id.clone());

    // Act
    let envelopes = [
        ledger
            .append_event(
                10,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path()),
                },
            )
            .expect("append started"),
        ledger
            .append_event(
                11,
                SoakLedgerEvent::Checkpoint {
                    status: Box::new(checkpoint_status()),
                },
            )
            .expect("append checkpoint"),
        ledger
            .append_event(
                12,
                SoakLedgerEvent::Resume {
                    interrupted_prior_run: true,
                },
            )
            .expect("append resume"),
        ledger
            .append_event(
                13,
                SoakLedgerEvent::Stop {
                    outcome: SoakOutcomeLabel::OperatorStop,
                },
            )
            .expect("append stop"),
        ledger
            .append_event(
                14,
                SoakLedgerEvent::Verdict {
                    outcome: SoakOutcomeLabel::OperatorStop,
                },
            )
            .expect("append verdict"),
    ];
    let read = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read ledger events");

    // Assert
    assert_eq!(envelopes.map(|envelope| envelope.sequence), [1, 2, 3, 4, 5]);
    assert_eq!(read.ignored_trailing_bytes, 0);
    assert_eq!(read.events.len(), 5);
    assert_eq!(
        read.events
            .iter()
            .map(|envelope| envelope.sequence)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    let ledger_text =
        fs::read_to_string(layout.paths_for_run(&run_id).events_path).expect("ledger text");
    assert_eq!(ledger_text.lines().count(), 5);
    assert!(ledger_text.ends_with('\n'));
}

#[test]
fn soak_ledger_read_events_ignores_truncated_trailing_line() {
    // Arrange
    let temp = TestDirectory::new("partial-line");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1781485562-0001").expect("run id");
    let paths = layout.paths_for_run(&run_id);
    fs::create_dir_all(paths.run_dir).expect("run directory");
    let envelope = SoakLedgerEventEnvelope::new(
        run_id,
        1,
        10,
        SoakLedgerEvent::Started {
            bounds: soak_bounds(temp.path()),
        },
    );
    let complete_line = serde_json::to_string(&envelope).expect("envelope json");
    fs::write(
        &paths.events_path,
        format!("{complete_line}\n{{\"schema_version\":"),
    )
    .expect("partial ledger");

    // Act
    let read = SoakLedger::read_events(&paths.events_path).expect("read partial ledger");

    // Assert
    assert_eq!(read.events.len(), 1);
    assert!(read.ignored_trailing_bytes > 0);
    assert_eq!(read.events[0].sequence, 1);
}

#[test]
fn soak_ledger_index_write_atomic_uses_tmp_path_and_retention_cap() {
    // Arrange
    let temp = TestDirectory::new("index");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let mut index = SoakRunIndex::empty();
    for sequence in 0..40 {
        let run_id = SoakRunId::try_new(format!("soak-1781485562-{sequence:04}")).expect("run id");
        let paths = layout.paths_for_run(&run_id);
        index.record_run(SoakRunIndexEntry {
            run_id,
            ledger_path: paths.events_path,
            started_at_unix_seconds: 10 + sequence,
            updated_at_unix_seconds: 10 + sequence,
            maybe_outcome: None,
        });
    }

    // Act
    index.write_atomic(&layout).expect("write index");
    let written = fs::read_to_string(layout.run_index_path()).expect("run index");
    let decoded: SoakRunIndex = serde_json::from_str(&written).expect("run index json");

    // Assert
    assert_eq!(
        layout
            .run_index_tmp_path()
            .file_name()
            .and_then(|name| name.to_str()),
        Some("run-index.json.tmp")
    );
    assert!(!layout.run_index_tmp_path().exists());
    assert_eq!(decoded.schema_version, SOAK_LEDGER_SCHEMA_VERSION);
    assert_eq!(decoded.runs.len(), MAX_SOAK_RUNS_IN_INDEX);
    assert_eq!(decoded.runs[0].run_id.as_str(), "soak-1781485562-0039");
}

#[test]
fn soak_ledger_append_rejects_oversized_events() {
    // Arrange
    let temp = TestDirectory::new("oversized");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1781485562-0001").expect("run id");
    let mut ledger = SoakLedger::create(&layout, run_id);
    let oversized_status = SoakCheckpointStatus {
        maybe_network: Some("mainnet".to_string()),
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
        maybe_resource_bound_state_label: None,
        resource_bound_labels: Vec::new(),
        maybe_resource_bound_next_action: None,
        maybe_validated_active_chain_height: Some(1),
        maybe_best_known_tip_height: Some(1),
        maybe_source_status_path: Some(PathBuf::from("x".repeat(20_000))),
    };

    // Act
    let result = ledger.append_event(
        10,
        SoakLedgerEvent::Checkpoint {
            status: Box::new(oversized_status),
        },
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn soak_report_json_includes_projection_source_and_latest_state() {
    // Arrange
    let temp = TestDirectory::new("report-json");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let events = sample_report_events(temp.path());
    let projection =
        SoakReportProjection::from_ledger_events(events, &source_ledger_path).expect("projection");

    // Act
    let rendered = render_soak_report_json(&projection).expect("report json");
    let value: Value = serde_json::from_str(&rendered).expect("report json value");

    // Assert
    assert_eq!(value["is_projection"], Value::Bool(true));
    assert_eq!(
        value["source_ledger_path"],
        Value::String(source_ledger_path.display().to_string())
    );
    assert_eq!(value["latest_sequence"], Value::from(5));
    assert_eq!(
        value["run_id"],
        Value::String("soak-1781485562-0001".to_string())
    );
    assert_eq!(
        value["bounds"]["network"],
        Value::String("mainnet".to_string())
    );
    assert_eq!(value["started_at_unix_seconds"], Value::from(10));
    assert_eq!(value["checkpoint_count"], Value::from(1));
    assert_eq!(value["resume_count"], Value::from(1));
    assert_eq!(value["interrupted_resume_count"], Value::from(1));
    assert_eq!(
        value["latest_checkpoint"]["maybe_network"],
        Value::String("mainnet".to_string())
    );
    assert_eq!(
        value["stop"]["outcome"],
        Value::String("operator_stop".to_string())
    );
    assert_eq!(
        value["verdict"]["outcome"],
        Value::String("operator_stop".to_string())
    );
}

#[test]
fn soak_progress_guarantee_report_json_preserves_checkpoint_field_names() {
    // Arrange
    let temp = TestDirectory::new("progress-report-json");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");

    // Act
    let rendered = render_soak_report_json(&projection).expect("report json");
    let value: Value = serde_json::from_str(&rendered).expect("report json value");
    let latest_checkpoint = value["latest_checkpoint"]
        .as_object()
        .expect("latest checkpoint object");

    // Assert
    for key in [
        "maybe_progress_credit_kind_label",
        "maybe_progress_credit_height",
        "maybe_progress_credit_hash",
        "maybe_progress_credit_work",
        "maybe_progress_credit_source_unix_seconds",
        "progress_credit_rejected_activity_labels",
        "maybe_expected_progress_window_seconds",
        "maybe_no_progress_threshold_state_label",
        "maybe_no_progress_threshold_seconds",
        "maybe_last_useful_work_kind_label",
        "maybe_last_useful_work_height",
        "maybe_last_peer_contribution_label",
        "maybe_stalled_subsystem_label",
        "maybe_stall_confidence_label",
        "stall_evidence_basis",
        "maybe_stall_next_action",
    ] {
        assert!(latest_checkpoint.contains_key(key), "missing {key}");
    }
    assert_eq!(
        value["latest_checkpoint"]["maybe_progress_credit_kind_label"],
        Value::String("validated_durable_active_chain".to_string())
    );
    assert_eq!(
        value["latest_checkpoint"]["progress_credit_rejected_activity_labels"][0],
        Value::String(
            "kind=header_download observed_count=2 reason=headers are not durable active-chain progress"
                .to_string()
        )
    );
}

#[test]
fn soak_recovery_evidence_report_json_preserves_checkpoint_field_names() {
    // Arrange
    let temp = TestDirectory::new("recovery-report-json");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_recovery_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");

    // Act
    let rendered = render_soak_report_json(&projection).expect("report json");
    let value: Value = serde_json::from_str(&rendered).expect("report json value");

    // Assert
    assert_eq!(
        value["latest_checkpoint"]["maybe_recovery_action_class_label"],
        Value::String("backup_then_rebuild".to_string())
    );
    assert_eq!(
        value["latest_checkpoint"]["maybe_recovery_cause_label"],
        Value::String("partial_write".to_string())
    );
    assert_eq!(
        value["latest_checkpoint"]["maybe_recovery_next_action"],
        Value::String(
            "Back up the selected datadir, then rebuild affected storage before normal operation."
                .to_string()
        )
    );
}

#[test]
fn soak_report_markdown_includes_operator_projection_summary() {
    // Arrange
    let temp = TestDirectory::new("report-markdown");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");

    // Act
    let rendered = render_soak_report_markdown(&projection);

    // Assert
    assert!(rendered.contains("# Open Bitcoin Soak Report"));
    assert!(rendered.contains("Source ledger:"));
    assert!(rendered.contains("Latest sequence:"));
    assert!(rendered.contains("Report is a projection: true"));
    assert!(rendered.contains("Final outcome:"));
    assert!(!rendered.contains("raw daemon log line"));
}

#[test]
fn soak_progress_guarantee_report_markdown_renders_credit_and_stall_fields() {
    // Arrange
    let temp = TestDirectory::new("progress-report-markdown");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");

    // Act
    let rendered = render_soak_report_markdown(&projection);

    // Assert
    assert!(
        rendered.contains("Progress credit: kind=validated_durable_active_chain height=900000")
    );
    assert!(rendered.contains(
        "Rejected progress activity: kind=header_download observed_count=2 reason=headers are not durable active-chain progress"
    ));
    assert!(rendered.contains("Expected progress window seconds: 240"));
    assert!(rendered.contains("No-progress threshold: state=within_window seconds=240"));
    assert!(
        rendered.contains("Last useful work: kind=validated_durable_active_chain height=900000")
    );
    assert!(rendered.contains(
        "Last peer contribution: peer=peer-1 kind=headers_and_blocks messages=7 headers=3 blocks=1 failure=unavailable"
    ));
    assert!(rendered.contains("Stalled subsystem: slow_or_stalled_peers"));
    assert!(rendered.contains("Stall confidence: medium"));
    assert!(rendered.contains("Stall evidence basis: latest peer stalled before useful work"));
    assert!(rendered.contains("Stall next action: Rotate peers and continue bounded sync."));
}

#[test]
fn soak_progress_guarantee_report_excludes_forbidden_raw_material() {
    // Arrange
    let temp = TestDirectory::new("progress-report-redaction");
    let source_ledger_path = temp
        .path()
        .join("soak/runs/soak-1781485562-0001/events.jsonl");
    let projection = SoakReportProjection::from_ledger_events(
        sample_report_events(temp.path()),
        &source_ledger_path,
    )
    .expect("projection");
    let forbidden_material = [
        "raw status snapshot",
        "rpcpassword",
        "wallet material",
        "unbounded peer table",
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
