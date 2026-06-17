// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::status::{
    BestKnownTipSource, BestKnownTipStatus, FieldAvailability, NoProgressDiagnosis,
    ProgressCreditKind, RejectedProgressActivityKind, StalledSubsystem, StayCurrentStatus,
    SyncProgress, SyncProgressSignal, SyncRecoveryCategory, TipFreshnessStatus,
};

use super::super::{
    PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState, SyncNetwork,
    SyncPeerAddress, types::SyncReconcileProgress,
};
use super::{
    NoProgressInput, ProgressGuaranteeInput, classify_no_progress, classify_progress_credit,
    classify_stall_diagnosis, no_progress_next_action,
};

#[test]
fn phase70_no_progress_classifier_distinguishes_remaining_causes() {
    // Arrange
    let branch_progress = SyncReconcileProgress::BranchCompetitionAwaitingBodies {
        missing_count: 1,
        first_missing_height: 2,
        first_missing_hash: "11".repeat(32),
    };
    let stalled_peer = [peer_outcome(
        PeerSyncState::Stalled,
        Some(PeerFailureReason::Stall),
    )];
    let cases = [
        (
            NoProgressInput {
                stay_current: None,
                progress_signal: Some(SyncProgressSignal::AwaitingBlocks),
                recovery_category: None,
                blocks_in_flight: 0,
                maybe_reconcile_progress: None,
                peer_outcomes: &[],
                maybe_stop_reason: None,
            },
            NoProgressDiagnosis::AwaitingBlockBodies,
        ),
        (
            NoProgressInput {
                stay_current: None,
                progress_signal: Some(SyncProgressSignal::Steady),
                recovery_category: None,
                blocks_in_flight: 0,
                maybe_reconcile_progress: Some(&branch_progress),
                peer_outcomes: &[],
                maybe_stop_reason: None,
            },
            NoProgressDiagnosis::BranchCompetitionAwaitingBodies,
        ),
        (
            NoProgressInput {
                stay_current: None,
                progress_signal: Some(SyncProgressSignal::Steady),
                recovery_category: None,
                blocks_in_flight: 0,
                maybe_reconcile_progress: None,
                peer_outcomes: &stalled_peer,
                maybe_stop_reason: None,
            },
            NoProgressDiagnosis::PeerStalled,
        ),
        (
            NoProgressInput {
                stay_current: None,
                progress_signal: Some(SyncProgressSignal::PeerFailures),
                recovery_category: None,
                blocks_in_flight: 0,
                maybe_reconcile_progress: None,
                peer_outcomes: &[],
                maybe_stop_reason: None,
            },
            NoProgressDiagnosis::PeerFailuresExhausted,
        ),
        (
            NoProgressInput {
                stay_current: Some(StayCurrentStatus::Recovering),
                progress_signal: Some(SyncProgressSignal::Steady),
                recovery_category: None,
                blocks_in_flight: 0,
                maybe_reconcile_progress: None,
                peer_outcomes: &[],
                maybe_stop_reason: None,
            },
            NoProgressDiagnosis::RecoveringFromReorgOrStorage,
        ),
        (
            NoProgressInput {
                stay_current: None,
                progress_signal: Some(SyncProgressSignal::Steady),
                recovery_category: Some(SyncRecoveryCategory::ResourceExhaustion),
                blocks_in_flight: 0,
                maybe_reconcile_progress: None,
                peer_outcomes: &stalled_peer,
                maybe_stop_reason: None,
            },
            NoProgressDiagnosis::StorageOrResourceBlocked,
        ),
    ];

    // Act / Assert
    for (input, expected) in cases {
        assert_eq!(classify_no_progress(&input), expected);
        assert!(!no_progress_next_action(expected).is_empty());
    }
}

#[test]
fn phase78_progress_guarantee_classifier_credits_only_durable_or_at_tip_evidence() {
    // Arrange
    let progress = sync_progress(2, "aa", "3");
    let retry_peer = [peer_outcome(
        PeerSyncState::Waiting,
        Some(PeerFailureReason::RetryBackoff),
    )];
    let input = ProgressGuaranteeInput {
        sync_progress: &progress,
        stay_current: StayCurrentStatus::NoProgress,
        best_known_tip: None,
        progress_signal: Some(SyncProgressSignal::HeaderProgress),
        recovery_category: None,
        blocks_in_flight: 1,
        maybe_reconcile_progress: None,
        peer_outcomes: &retry_peer,
        maybe_stop_reason: None,
        retry_backoff_seconds: 2,
        max_sync_rounds: 4,
        tip_freshness_threshold_seconds: 1_200,
        maybe_previous_credit: None,
        evaluated_at_unix_seconds: 100,
    };

    // Act
    let FieldAvailability::Available(credit) = classify_progress_credit(&input) else {
        panic!("validated active-chain progress should be credited");
    };
    let mut repeated_input = input;
    repeated_input.maybe_previous_credit = Some(&credit);
    let best_tip = best_known_tip(2, "aa", "3");

    // Assert
    assert_eq!(credit.kind, ProgressCreditKind::ValidatedDurableActiveChain);
    assert_eq!(credit.credited_validated_active_chain_height, 2);
    assert!(
        credit
            .rejected_activity
            .iter()
            .any(|activity| activity.kind == RejectedProgressActivityKind::HeaderDownload)
    );
    assert!(matches!(
        classify_progress_credit(&repeated_input),
        FieldAvailability::Unavailable { .. }
    ));
    let mut at_tip_input = repeated_input;
    at_tip_input.stay_current = StayCurrentStatus::CurrentAtBestKnownTip;
    at_tip_input.best_known_tip = Some(&best_tip);
    let FieldAvailability::Available(at_tip_credit) = classify_progress_credit(&at_tip_input)
    else {
        panic!("current-at-best-known-tip evidence should be credited");
    };
    assert_eq!(
        at_tip_credit.kind,
        ProgressCreditKind::CurrentAtBestKnownTip
    );
    let mut storage_input = at_tip_input;
    storage_input.recovery_category = Some(SyncRecoveryCategory::ResourceExhaustion);
    let FieldAvailability::Available(stall) =
        classify_stall_diagnosis(&storage_input, NoProgressDiagnosis::PeerBackoff)
    else {
        panic!("stall diagnosis should be available");
    };
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::StorageOrResourcePressure
    );
}

fn peer_outcome(
    state: PeerSyncState,
    maybe_failure_reason: Option<PeerFailureReason>,
) -> PeerSyncOutcome {
    PeerSyncOutcome {
        peer: SyncPeerAddress::manual("127.0.0.1", 18_444),
        maybe_resolved_endpoint: Some("127.0.0.1:18444".to_string()),
        network: SyncNetwork::Regtest,
        state,
        attempts: 1,
        contribution: PeerContribution {
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        },
        maybe_tip_height: None,
        maybe_tip_hash: None,
        maybe_tip_work: None,
        maybe_last_activity_unix_seconds: None,
        maybe_capabilities: None,
        maybe_failure_reason,
        maybe_error: None,
    }
}

fn sync_progress(height: u64, hash_byte: &str, work: &str) -> SyncProgress {
    SyncProgress {
        header_height: height,
        block_height: height,
        downloaded_block_height: height,
        connected_block_height: height,
        validated_active_chain_height: height,
        maybe_downloaded_block_hash: Some(hash_byte.repeat(32)),
        maybe_connected_block_hash: Some(hash_byte.repeat(32)),
        maybe_validated_active_chain_hash: Some(hash_byte.repeat(32)),
        maybe_validated_active_chain_work: Some(work.to_string()),
        progress_ratio: 1.0,
        messages_processed: 1,
        headers_received: 1,
        blocks_received: 1,
    }
}

fn best_known_tip(height: u64, hash_byte: &str, work: &str) -> BestKnownTipStatus {
    BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height,
        block_hash: hash_byte.repeat(32),
        work: work.to_string(),
        block_time_unix_seconds: 90,
        observed_at_unix_seconds: 100,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: Vec::new(),
    }
}
