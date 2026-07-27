// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

pub(super) fn phase79_resource_bound_snapshot() -> ResourceBoundSnapshot {
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

pub(super) fn phase79_resource_unit(kind: ResourceBoundKind) -> ResourceBoundUnit {
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

pub(super) fn unavailable_store_health() -> StoreHealthEvidence {
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

pub(super) fn seed_phase75_soak_run(
    data_dir: &Path,
    run_id_text: &str,
    outcome: SoakOutcomeLabel,
) -> (SoakRunId, crate::operator::soak::ledger::SoakRunPaths) {
    seed_soak_run_with_checkpoint(data_dir, run_id_text, outcome, phase75_checkpoint_status())
}

pub(super) fn seed_phase79_sensitive_soak_run(
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

pub(super) fn seed_soak_run_with_checkpoint(
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

pub(super) fn soak_event(
    run_id: SoakRunId,
    sequence: u64,
    event: SoakLedgerEvent,
) -> SoakLedgerEventEnvelope {
    SoakLedgerEventEnvelope::new(run_id, sequence, 1_781_485_562 + sequence, event)
}

pub(super) fn phase75_soak_bounds(data_dir: &Path) -> SoakBounds {
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

pub(super) fn phase75_checkpoint_status() -> SoakCheckpointStatus {
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

pub(super) fn phase79_sensitive_checkpoint_status() -> SoakCheckpointStatus {
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

pub(super) fn phase79_sensitive_literals() -> [&'static str; 8] {
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

pub(super) fn assert_absent(text: &str, value: &str) {
    assert!(
        !text.contains(value),
        "unexpected sensitive value in {text}"
    );
}

pub(super) trait FieldAvailabilityTestExt<T> {
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
