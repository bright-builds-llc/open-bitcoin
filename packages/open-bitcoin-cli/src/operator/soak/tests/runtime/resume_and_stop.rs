use super::*;

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
                status: Box::new(checkpoint_status(10)),
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
                status: Box::new(checkpoint_status(10)),
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
