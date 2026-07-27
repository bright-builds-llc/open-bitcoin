use super::*;

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
                status: Box::new(checkpoint_status(144)),
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
