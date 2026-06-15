// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    LogStatus, MetricRetentionPolicy, MetricsStatus, RecoveryActionClass, RecoveryCause,
    RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, BuildProvenance, ConfigStatus, FieldAvailability,
        MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot, PeerStatus,
        ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot, ResourceBoundUnit,
        StayCurrentStatus, SyncProgress, SyncRecoveryCategory, SyncReorgEvidence, SyncStatus,
        SyncStopReasonStatus, TipFreshnessStatus, WalletStatus, usage_against_budget,
    },
};

use super::{
    SoakBounds, SoakClock, SoakLoopMode, SoakPeerPolicy, SoakRunId, SoakStatusCollector,
    SoakStopCondition, SoakTestClock,
    ledger::{
        SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerLayout, SoakRunIndex,
        SoakRunIndexEntry,
    },
    outcome::SoakOutcomeLabel,
    run_bounded_soak_loop, runtime, validate_resume_plan, write_operator_stop,
    write_report_projection,
};
use crate::operator::OperatorOutputFormat;

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
            "open-bitcoin-soak-runtime-{label}-{}-{timestamp}",
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

#[derive(Debug)]
struct ScriptedStatusCollector {
    snapshots: Vec<OpenBitcoinStatusSnapshot>,
    index: usize,
}

impl ScriptedStatusCollector {
    fn repeating(snapshot: OpenBitcoinStatusSnapshot) -> Self {
        Self {
            snapshots: vec![snapshot],
            index: 0,
        }
    }

    fn from_snapshots(snapshots: Vec<OpenBitcoinStatusSnapshot>) -> Self {
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

struct StopDuringSleepClock {
    now_unix_seconds: u64,
    layout: SoakLedgerLayout,
    run_id: SoakRunId,
    stop_written: bool,
}

impl StopDuringSleepClock {
    const fn new(now_unix_seconds: u64, layout: SoakLedgerLayout, run_id: SoakRunId) -> Self {
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

struct StopDuringCollectCollector {
    snapshot: OpenBitcoinStatusSnapshot,
    layout: SoakLedgerLayout,
    run_id: SoakRunId,
    stop_written: bool,
}

impl StopDuringCollectCollector {
    const fn new(
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
    fn snapshot_time(&self) -> u64 {
        1_700_000_000
    }
}

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

#[test]
fn soak_runtime_resume_preserves_original_elapsed_time_budget() {
    // Arrange
    let temp = TestDirectory::new("resume-elapsed-budget");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-resume-budget").expect("run id");
    let started_at = 1_700_000_000;
    let deadline = started_at + 60;
    let bounds = soak_bounds(temp.path(), Some(144), vec![SoakStopCondition::ElapsedTime]);
    let mut initial_ledger = SoakLedger::create(&layout, run_id.clone());
    initial_ledger
        .append_event(
            started_at,
            SoakLedgerEvent::Started {
                bounds: bounds.clone(),
            },
        )
        .expect("started");
    let mut resume_ledger = SoakLedger::resume(&layout, run_id.clone(), 2);
    let mut collector = ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
    let mut clock = SoakTestClock::new(started_at + 45);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut resume_ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Resume {
            interrupted_prior_run: true,
            run_started_at_unix_seconds: started_at,
        },
    )
    .expect("resume soak loop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.updated_at_unix_seconds, deadline);
    assert_eq!(result.latest_sequence, 6);
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(&event.event, SoakLedgerEvent::Checkpoint { .. }))
            .map(|event| event.recorded_at_unix_seconds)
            .collect::<Vec<_>>(),
        vec![started_at + 45, deadline]
    );
}

#[test]
fn soak_runtime_resume_finishes_immediately_after_original_elapsed_deadline() {
    // Arrange
    let temp = TestDirectory::new("resume-elapsed-expired");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-resume-expired").expect("run id");
    let started_at = 1_700_000_000;
    let resumed_at = started_at + 120;
    let bounds = soak_bounds(temp.path(), Some(144), vec![SoakStopCondition::ElapsedTime]);
    let mut initial_ledger = SoakLedger::create(&layout, run_id.clone());
    initial_ledger
        .append_event(
            started_at,
            SoakLedgerEvent::Started {
                bounds: bounds.clone(),
            },
        )
        .expect("started");
    let mut resume_ledger = SoakLedger::resume(&layout, run_id.clone(), 2);
    let mut collector = ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
    let mut clock = SoakTestClock::new(resumed_at);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut resume_ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Resume {
            interrupted_prior_run: true,
            run_started_at_unix_seconds: started_at,
        },
    )
    .expect("resume soak loop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.updated_at_unix_seconds, resumed_at);
    assert_eq!(result.latest_sequence, 5);
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(&event.event, SoakLedgerEvent::Checkpoint { .. }))
            .map(|event| event.recorded_at_unix_seconds)
            .collect::<Vec<_>>(),
        vec![resumed_at]
    );
}

#[test]
fn soak_runtime_resume_continues_after_historical_operator_stop_verdict() {
    // Arrange
    let temp = TestDirectory::new("resume-after-stop");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-resume-after-stop").expect("run id");
    let started_at = 1_700_000_000;
    let stopped_at = started_at + 15;
    let resumed_at = started_at + 30;
    let bounds = soak_bounds(
        temp.path(),
        Some(144),
        vec![SoakStopCondition::TargetHeight],
    );
    let mut initial_ledger = SoakLedger::create(&layout, run_id.clone());
    initial_ledger
        .append_event(
            started_at,
            SoakLedgerEvent::Started {
                bounds: bounds.clone(),
            },
        )
        .expect("started");
    initial_ledger
        .append_event(
            stopped_at,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .expect("operator stop");
    initial_ledger
        .append_event(
            stopped_at,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .expect("operator verdict");
    let mut resume_ledger = SoakLedger::resume(&layout, run_id.clone(), 4);
    let mut collector = ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
    let mut clock = SoakTestClock::new(resumed_at);

    // Act
    let result = run_bounded_soak_loop(
        &run_id,
        &bounds,
        &layout,
        &mut resume_ledger,
        &mut collector,
        &mut clock,
        SoakLoopMode::Resume {
            interrupted_prior_run: false,
            run_started_at_unix_seconds: started_at,
        },
    )
    .expect("resume soak loop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::CleanCompletion);
    assert_eq!(result.latest_sequence, 7);
    assert!(matches!(
        &events[3].event,
        SoakLedgerEvent::Resume {
            interrupted_prior_run: false
        }
    ));
    assert!(matches!(
        &events[4].event,
        SoakLedgerEvent::Checkpoint { .. }
    ));
    assert!(matches!(
        &events[6].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::CleanCompletion
        }
    ));
}

#[test]
fn soak_runtime_runner_returns_external_stop_written_during_collect() {
    // Arrange
    let temp = TestDirectory::new("stop-during-collect");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-stop-collect").expect("run id");
    let bounds = soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let snapshot = base_status_snapshot(temp.path());
    let mut collector = StopDuringCollectCollector::new(snapshot, layout.clone(), run_id.clone());
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
    .expect("bounded soak loop with stop during collect");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::OperatorStop);
    assert_eq!(result.latest_sequence, 3);
    assert_eq!(events.len(), 3);
    assert!(matches!(&events[0].event, SoakLedgerEvent::Started { .. }));
    assert!(matches!(
        &events[1].event,
        SoakLedgerEvent::Stop {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
    assert!(matches!(
        &events[2].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
}

#[test]
fn soak_runtime_resume_plan_treats_latest_unterminated_invocation_as_interrupted() {
    // Arrange
    let temp = TestDirectory::new("resume-after-interrupted-resume");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-interrupted-resume").expect("run id");
    let paths = layout.paths_for_run(&run_id);
    let started_at = 1_700_000_000;
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            started_at,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("started");
    ledger
        .append_event(
            started_at + 15,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .expect("operator stop");
    ledger
        .append_event(
            started_at + 15,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .expect("operator verdict");
    ledger
        .append_event(
            started_at + 30,
            SoakLedgerEvent::Resume {
                interrupted_prior_run: false,
            },
        )
        .expect("resume");
    ledger
        .append_event(
            started_at + 30,
            SoakLedgerEvent::Checkpoint {
                status: checkpoint_status(10),
            },
        )
        .expect("checkpoint");
    let mut index = SoakRunIndex::empty();
    index.record_run(SoakRunIndexEntry {
        run_id: run_id.clone(),
        ledger_path: paths.events_path.clone(),
        started_at_unix_seconds: started_at,
        updated_at_unix_seconds: started_at + 30,
        maybe_outcome: Some(SoakOutcomeLabel::OperatorStop),
    });
    index.write_atomic(&layout).expect("write index");

    // Act
    let resume = validate_resume_plan(&layout, &run_id, 15).expect("resume plan");

    // Assert
    assert!(resume.interrupted_prior_run);
    assert_eq!(resume.next_sequence, 6);
    assert_eq!(resume.started_at_unix_seconds, started_at);
}

#[test]
fn soak_runtime_stop_accepts_active_resume_after_historical_terminal_verdict() {
    // Arrange
    let temp = TestDirectory::new("stop-active-resume");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-stop-active-resume").expect("run id");
    let started_at = 1_700_000_000;
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            started_at,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("started");
    ledger
        .append_event(
            started_at + 15,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .expect("historical operator stop");
    ledger
        .append_event(
            started_at + 15,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop,
            },
        )
        .expect("historical operator verdict");
    ledger
        .append_event(
            started_at + 30,
            SoakLedgerEvent::Resume {
                interrupted_prior_run: false,
            },
        )
        .expect("resume");
    ledger
        .append_event(
            started_at + 30,
            SoakLedgerEvent::Checkpoint {
                status: checkpoint_status(10),
            },
        )
        .expect("checkpoint");

    // Act
    let result = write_operator_stop(&layout, &run_id, started_at + 45).expect("operator stop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::OperatorStop);
    assert_eq!(result.latest_sequence, 7);
    assert_eq!(events.len(), 7);
    assert!(matches!(
        &events[5].event,
        SoakLedgerEvent::Stop {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
    assert!(matches!(
        &events[6].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
}

#[test]
fn soak_runtime_stop_records_operator_stop_verdict() {
    // Arrange
    let temp = TestDirectory::new("stop");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-stop").expect("run id");
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            1,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("started");

    // Act
    let result = write_operator_stop(&layout, &run_id, 2).expect("operator stop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::OperatorStop);
    assert!(matches!(
        &events[1].event,
        SoakLedgerEvent::Stop {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
    assert!(matches!(
        &events[2].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
}

#[test]
fn soak_runtime_stop_rejects_terminal_verdict() {
    // Arrange
    let temp = TestDirectory::new("stop-terminal");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-stop-terminal").expect("run id");
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            1,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("started");
    ledger
        .append_event(
            2,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::CleanCompletion,
            },
        )
        .expect("stop");
    ledger
        .append_event(
            3,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::CleanCompletion,
            },
        )
        .expect("verdict");

    // Act
    let error = write_operator_stop(&layout, &run_id, 4).expect_err("terminal stop rejection");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert!(error.to_string().contains("already has a terminal verdict"));
    assert_eq!(events.len(), 3);
    assert!(matches!(
        &events[2].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::CleanCompletion
        }
    ));
}

#[test]
fn soak_runtime_runner_returns_existing_terminal_verdict_after_external_stop() {
    // Arrange
    let temp = TestDirectory::new("stop-race");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-stop-race").expect("run id");
    let bounds = soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]);
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    let mut collector = ScriptedStatusCollector::repeating(base_status_snapshot(temp.path()));
    let mut clock = StopDuringSleepClock::new(1_700_000_000, layout.clone(), run_id.clone());

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
    .expect("bounded soak loop with external stop");
    let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
        .expect("read events")
        .events;

    // Assert
    assert_eq!(result.final_outcome, SoakOutcomeLabel::OperatorStop);
    assert_eq!(result.latest_sequence, 4);
    assert_eq!(events.len(), 4);
    assert!(matches!(
        &events[1].event,
        SoakLedgerEvent::Checkpoint { .. }
    ));
    assert!(matches!(
        &events[2].event,
        SoakLedgerEvent::Stop {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
    assert!(matches!(
        &events[3].event,
        SoakLedgerEvent::Verdict {
            outcome: SoakOutcomeLabel::OperatorStop
        }
    ));
}

#[test]
fn soak_runtime_report_rewrites_projection_without_ledger_append() {
    // Arrange
    let temp = TestDirectory::new("report");
    let layout = SoakLedgerLayout::for_datadir(temp.path());
    let run_id = SoakRunId::try_new("soak-1700000000-report").expect("run id");
    let mut ledger = SoakLedger::create(&layout, run_id.clone());
    ledger
        .append_event(
            1,
            SoakLedgerEvent::Started {
                bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
            },
        )
        .expect("started");
    ledger
        .append_event(
            2,
            SoakLedgerEvent::Checkpoint {
                status: checkpoint_status(144),
            },
        )
        .expect("checkpoint");
    let events_path = layout.paths_for_run(&run_id).events_path;
    let before_lines = fs::read_to_string(&events_path)
        .expect("ledger before")
        .lines()
        .count();

    // Act
    let result = write_report_projection(&layout, &run_id).expect("report projection");
    let after_lines = fs::read_to_string(&events_path)
        .expect("ledger after")
        .lines()
        .count();

    // Assert
    assert_eq!(before_lines, after_lines);
    assert_eq!(result.latest_sequence, 2);
    assert!(result.report_paths.json_path.exists());
}

fn soak_bounds(
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

fn checkpoint_status(height: u64) -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_network: Some("regtest".to_string()),
        maybe_lifecycle: Some("active".to_string()),
        maybe_latest_stop_reason_label: None,
        maybe_recovery_category_label: None,
        maybe_recovery_action_class_label: None,
        maybe_recovery_cause_label: None,
        maybe_recovery_next_action: None,
        maybe_no_progress_diagnosis_label: None,
        maybe_resource_bound_state_label: Some("normal".to_string()),
        resource_bound_labels: vec!["all_required_bounds=normal".to_string()],
        maybe_resource_bound_next_action: None,
        maybe_validated_active_chain_height: Some(height),
        maybe_best_known_tip_height: Some(height),
        maybe_source_status_path: None,
    }
}

fn clean_status_snapshot(datadir: &Path, height: u64) -> OpenBitcoinStatusSnapshot {
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

fn resource_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion);
    snapshot.resource_bounds = FieldAvailability::available(resource_stop_bounds());
    snapshot
}

fn recovery_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.recovery_category =
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption);
    snapshot
}

fn operator_stop_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
    let mut snapshot = base_status_snapshot(datadir);
    snapshot.sync.latest_stop_reason = FieldAvailability::available(SyncStopReasonStatus {
        label: "operator_stop".to_string(),
        message: "operator requested stop".to_string(),
    });
    snapshot
}

fn diagnosed_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
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

fn phase77_recovery_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
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

fn latest_checkpoint_status(layout: &SoakLedgerLayout, run_id: &SoakRunId) -> SoakCheckpointStatus {
    let events = SoakLedger::read_events(&layout.paths_for_run(run_id).events_path)
        .expect("read soak ledger")
        .events;
    events
        .into_iter()
        .rev()
        .find_map(|event| match event.event {
            SoakLedgerEvent::Checkpoint { status } => Some(status),
            SoakLedgerEvent::Started { .. }
            | SoakLedgerEvent::Resume { .. }
            | SoakLedgerEvent::Stop { .. }
            | SoakLedgerEvent::Verdict { .. } => None,
        })
        .expect("latest checkpoint status")
}

fn base_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
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
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable("mempool unavailable"),
        },
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

fn normal_resource_bounds() -> ResourceBoundSnapshot {
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

fn resource_stop_bounds() -> ResourceBoundSnapshot {
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

const fn resource_unit(kind: ResourceBoundKind) -> ResourceBoundUnit {
    match kind {
        ResourceBoundKind::Disk
        | ResourceBoundKind::Cache
        | ResourceBoundKind::Log
        | ResourceBoundKind::SupportBundle => ResourceBoundUnit::Bytes,
        ResourceBoundKind::File => ResourceBoundUnit::Files,
        ResourceBoundKind::Metric => ResourceBoundUnit::Items,
        ResourceBoundKind::Peer => ResourceBoundUnit::Peers,
        ResourceBoundKind::Queue | ResourceBoundKind::InFlight => ResourceBoundUnit::Requests,
    }
}
