// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::path::PathBuf;

use open_bitcoin_node::status::{NoProgressDiagnosis, SyncRecoveryCategory, SyncStopReasonStatus};
use serde_json::Value;

use super::{
    SoakBounds, SoakPeerPolicy, SoakRunId, SoakStopCondition,
    outcome::{
        SoakOutcomeEvidence, SoakOutcomeLabel, SoakProcessExitEvidence, classify_soak_outcome,
    },
};
use crate::operator::support::{
    ActiveChainEvidence, EvidenceState, EvidenceVerdictSummary, FullSyncEvidence, SummaryEvidence,
    SupportEvidenceVerdict, TipEvidence,
};

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
