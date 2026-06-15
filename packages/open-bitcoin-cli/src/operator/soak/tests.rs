// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::status::{NoProgressDiagnosis, SyncRecoveryCategory, SyncStopReasonStatus};
use serde_json::Value;

use super::{
    SoakBounds, SoakPeerPolicy, SoakRunId, SoakStopCondition,
    ledger::{
        MAX_SOAK_RUNS_IN_INDEX, SOAK_LEDGER_SCHEMA_VERSION, SoakCheckpointStatus, SoakLedger,
        SoakLedgerEvent, SoakLedgerEventEnvelope, SoakLedgerLayout, SoakRunIndex,
        SoakRunIndexEntry,
    },
    outcome::{
        SoakOutcomeEvidence, SoakOutcomeLabel, SoakProcessExitEvidence, classify_soak_outcome,
    },
    report::{
        SoakReportProjection, render_soak_report_json, render_soak_report_markdown,
        write_soak_reports,
    },
    validate_resume_plan,
};
use crate::operator::support::{
    ActiveChainEvidence, EvidenceState, EvidenceVerdictSummary, FullSyncEvidence, SummaryEvidence,
    SupportEvidenceVerdict, TipEvidence,
};

const SOAK_SYNTHETIC_STARTED_AT: u64 = 1_777_300_000;
const SOAK_SYNTHETIC_CHECKPOINT_AT: u64 = 1_777_300_060;
const SOAK_SYNTHETIC_RESUME_OR_STOP_AT: u64 = 1_777_300_120;

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
            "open-bitcoin-soak-{label}-{}-{timestamp}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory");
        Self { path }
    }

    fn deterministic(label: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("open-bitcoin-soak-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
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
                    status: checkpoint_status(),
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
        maybe_no_progress_diagnosis_label: None,
        maybe_validated_active_chain_height: Some(1),
        maybe_best_known_tip_height: Some(1),
        maybe_source_status_path: Some(PathBuf::from("x".repeat(20_000))),
    };

    // Act
    let result = ledger.append_event(
        10,
        SoakLedgerEvent::Checkpoint {
            status: oversized_status,
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
                status: checkpoint_status(),
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
                status: checkpoint_status(),
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
                status: resource_checkpoint_status(),
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
    let resource_recovery = SoakOutcomeEvidence {
        maybe_recovery_category: Some(SyncRecoveryCategory::ResourceExhaustion),
        ..SoakOutcomeEvidence::empty()
    };
    let storage_recovery = SoakOutcomeEvidence {
        maybe_recovery_category: Some(SyncRecoveryCategory::StorageBackendFailure),
        ..SoakOutcomeEvidence::empty()
    };
    let resource_diagnosis = SoakOutcomeEvidence {
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        ..SoakOutcomeEvidence::empty()
    };

    // Act / Assert
    assert_eq!(
        classify_soak_outcome(&resource_recovery),
        SoakOutcomeLabel::ResourceStop
    );
    assert_eq!(
        classify_soak_outcome(&storage_recovery),
        SoakOutcomeLabel::RecoveryStop
    );
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

fn full_sync_evidence(verdict: SupportEvidenceVerdict) -> FullSyncEvidence {
    FullSyncEvidence {
        initial_tip: summary_evidence(),
        final_tip: TipEvidence {
            height: None,
            hash: None,
            work: None,
            freshness: None,
            maybe_unavailable_reason: Some("not needed for outcome test".to_string()),
        },
        connected_active_chain: active_chain_evidence(),
        validated_active_chain: active_chain_evidence(),
        restart_resume_checkpoints: summary_evidence(),
        stay_current_window: summary_evidence(),
        peer_contribution: summary_evidence(),
        no_progress_or_reorg_events: summary_evidence(),
        resource_pressure: summary_evidence(),
        recovery: summary_evidence(),
        verdict: EvidenceVerdictSummary {
            label: verdict,
            justifications: vec!["test verdict".to_string()],
        },
    }
}

fn summary_evidence() -> SummaryEvidence {
    SummaryEvidence {
        state: EvidenceState::Unavailable,
        summary: None,
        maybe_unavailable_reason: Some("not needed for outcome test".to_string()),
    }
}

fn active_chain_evidence() -> ActiveChainEvidence {
    ActiveChainEvidence {
        height: None,
        hash: None,
        work: None,
        maybe_unavailable_reason: Some("not needed for outcome test".to_string()),
    }
}

fn soak_bounds(datadir: &Path) -> SoakBounds {
    SoakBounds::try_new(
        86_400,
        60,
        Some(900_000),
        datadir.to_path_buf(),
        "mainnet",
        SoakPeerPolicy::DaemonConfigured,
        4_096,
        vec![SoakStopCondition::ElapsedTime],
    )
    .expect("valid soak bounds")
}

fn checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("mainnet".to_string()),
        maybe_lifecycle: Some("active".to_string()),
        maybe_latest_stop_reason_label: Some("target_height".to_string()),
        maybe_recovery_category_label: None,
        maybe_no_progress_diagnosis_label: None,
        maybe_validated_active_chain_height: Some(900_000),
        maybe_best_known_tip_height: Some(900_000),
        maybe_source_status_path: Some(PathBuf::from("/tmp/status.json")),
    }
}

fn resource_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_recovery_category_label: Some("resource_exhaustion".to_string()),
        maybe_no_progress_diagnosis_label: Some("storage_or_resource_blocked".to_string()),
        maybe_source_status_path: Some(PathBuf::from("/tmp/resource-status.json")),
        ..checkpoint_status()
    }
}

fn sample_report_events(datadir: &Path) -> Vec<SoakLedgerEventEnvelope> {
    let run_id = SoakRunId::try_new("soak-1781485562-0001").expect("run id");
    vec![
        SoakLedgerEventEnvelope::new(
            run_id.clone(),
            1,
            10,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(datadir),
            },
        ),
        SoakLedgerEventEnvelope::new(
            run_id.clone(),
            2,
            20,
            SoakLedgerEvent::Checkpoint {
                status: checkpoint_status(),
            },
        ),
        SoakLedgerEventEnvelope::new(
            run_id.clone(),
            3,
            30,
            SoakLedgerEvent::Resume {
                interrupted_prior_run: true,
            },
        ),
        SoakLedgerEventEnvelope::new(
            run_id.clone(),
            4,
            40,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        ),
        SoakLedgerEventEnvelope::new(
            run_id,
            5,
            50,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        ),
    ]
}
