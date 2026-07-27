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
        progress_guarantee: summary_evidence(),
        stall_diagnosis: summary_evidence(),
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
        maybe_recovery_action_class_label: None,
        maybe_recovery_cause_label: None,
        maybe_recovery_next_action: None,
        maybe_no_progress_diagnosis_label: None,
        maybe_progress_credit_kind_label: Some("validated_durable_active_chain".to_string()),
        maybe_progress_credit_height: Some(900_000),
        maybe_progress_credit_hash: Some("11".repeat(32)),
        maybe_progress_credit_work: Some("900001".to_string()),
        maybe_progress_credit_source_unix_seconds: Some(1_777_300_060),
        progress_credit_rejected_activity_labels: vec![
            "kind=header_download observed_count=2 reason=headers are not durable active-chain progress"
                .to_string(),
        ],
        maybe_expected_progress_window_seconds: Some(240),
        maybe_no_progress_threshold_state_label: Some("within_window".to_string()),
        maybe_no_progress_threshold_seconds: Some(240),
        maybe_last_useful_work_kind_label: Some("validated_durable_active_chain".to_string()),
        maybe_last_useful_work_height: Some(900_000),
        maybe_last_peer_contribution_label: Some(
            "peer=peer-1 kind=headers_and_blocks messages=7 headers=3 blocks=1 failure=unavailable"
                .to_string(),
        ),
        maybe_stalled_subsystem_label: Some("slow_or_stalled_peers".to_string()),
        maybe_stall_confidence_label: Some("medium".to_string()),
        stall_evidence_basis: vec!["latest peer stalled before useful work".to_string()],
        maybe_stall_next_action: Some("Rotate peers and continue bounded sync.".to_string()),
        maybe_resource_bound_state_label: Some("normal".to_string()),
        resource_bound_labels: vec!["all_required_bounds=normal".to_string()],
        maybe_resource_bound_next_action: None,
        maybe_validated_active_chain_height: Some(900_000),
        maybe_best_known_tip_height: Some(900_000),
        maybe_source_status_path: Some(PathBuf::from("/tmp/status.json")),
    }
}

fn resource_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_recovery_category_label: Some("resource_exhaustion".to_string()),
        maybe_no_progress_diagnosis_label: Some("storage_or_resource_blocked".to_string()),
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
        maybe_source_status_path: Some(PathBuf::from("/tmp/resource-status.json")),
        ..checkpoint_status()
    }
}

fn recovery_checkpoint_status() -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_recovery_category_label: Some("store_corruption".to_string()),
        maybe_recovery_action_class_label: Some("backup_then_rebuild".to_string()),
        maybe_recovery_cause_label: Some("partial_write".to_string()),
        maybe_recovery_next_action: Some(
            "Back up the selected datadir, then rebuild affected storage before normal operation."
                .to_string(),
        ),
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
                status: Box::new(checkpoint_status()),
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

fn sample_recovery_report_events(datadir: &Path) -> Vec<SoakLedgerEventEnvelope> {
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
                status: Box::new(recovery_checkpoint_status()),
            },
        ),
        SoakLedgerEventEnvelope::new(
            run_id,
            3,
            30,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::RecoveryStop,
            },
        ),
    ]
}

mod ledger_and_reports;
mod recovery_resume_bounds;
