// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::status::{
    FieldAvailability, NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState,
    PeerContributionEvidence, PeerContributionKind, ProgressCreditEvidence, ProgressCreditKind,
    ProgressWindowEvidence, RejectedProgressActivity, RejectedProgressActivityKind,
    StallDiagnosisConfidence, StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus,
    SyncProgress, SyncProgressSignal,
};

use super::super::{
    PeerFailureReason, PeerSyncOutcome, SyncStopReason, types::SyncReconcileProgress,
};
use super::{ProgressGuaranteeInput, is_storage_or_resource_blocker, no_progress_next_action};

struct ActiveChainCredit<'a> {
    height: u64,
    hash: &'a str,
    work: &'a str,
}

pub(super) fn classify_progress_credit(
    input: &ProgressGuaranteeInput<'_>,
) -> FieldAvailability<ProgressCreditEvidence> {
    let maybe_current = active_chain_credit(input.sync_progress);
    if let Some(current) = maybe_current.as_ref()
        && input
            .maybe_previous_credit
            .is_none_or(|previous| !same_credit_tip(previous, current))
    {
        return FieldAvailability::available(progress_credit(
            ProgressCreditKind::ValidatedDurableActiveChain,
            current,
            input,
        ));
    }
    if input.stay_current == StayCurrentStatus::CurrentAtBestKnownTip
        && let (Some(best_tip), Some(current)) = (input.best_known_tip, maybe_current.as_ref())
        && current.height == best_tip.height
        && current.hash == best_tip.block_hash
        && current.work == best_tip.work
    {
        return FieldAvailability::available(progress_credit(
            ProgressCreditKind::CurrentAtBestKnownTip,
            current,
            input,
        ));
    }
    FieldAvailability::unavailable("no validated durable active-chain or at-tip credit available")
}

pub(super) fn derive_last_useful_work(
    input: &ProgressGuaranteeInput<'_>,
    maybe_current_credit: Option<&ProgressCreditEvidence>,
) -> FieldAvailability<ProgressCreditEvidence> {
    if let Some(credit) = maybe_current_credit.or(input.maybe_previous_credit) {
        return FieldAvailability::available(credit.clone());
    }
    FieldAvailability::unavailable("no useful active-chain work recorded")
}

pub(super) fn derive_progress_window(
    input: &ProgressGuaranteeInput<'_>,
) -> FieldAvailability<ProgressWindowEvidence> {
    FieldAvailability::available(ProgressWindowEvidence {
        retry_backoff_seconds: input.retry_backoff_seconds,
        max_sync_rounds: input.max_sync_rounds,
        expected_progress_window_seconds: input
            .retry_backoff_seconds
            .saturating_mul(input.max_sync_rounds),
        tip_freshness_threshold_seconds: input.tip_freshness_threshold_seconds,
    })
}

pub(super) fn derive_no_progress_threshold(
    input: &ProgressGuaranteeInput<'_>,
    maybe_last_useful_work: Option<&ProgressCreditEvidence>,
) -> FieldAvailability<NoProgressThresholdEvidence> {
    let threshold_seconds = input
        .retry_backoff_seconds
        .saturating_mul(input.max_sync_rounds);
    let elapsed = maybe_last_useful_work.map_or(threshold_seconds.saturating_add(1), |credit| {
        input
            .evaluated_at_unix_seconds
            .saturating_sub(credit.source_unix_seconds)
    });
    FieldAvailability::available(NoProgressThresholdEvidence {
        threshold_seconds,
        elapsed_since_last_useful_work_seconds: elapsed,
        state: if elapsed > threshold_seconds {
            NoProgressThresholdState::Exceeded
        } else {
            NoProgressThresholdState::WithinWindow
        },
        evaluated_at_unix_seconds: input.evaluated_at_unix_seconds,
    })
}

pub(super) fn derive_last_peer_contribution(
    input: &ProgressGuaranteeInput<'_>,
) -> FieldAvailability<PeerContributionEvidence> {
    let Some(outcome) = input.peer_outcomes.iter().next_back() else {
        return FieldAvailability::unavailable("no peer contribution recorded");
    };
    let contribution = &outcome.contribution;
    FieldAvailability::available(PeerContributionEvidence {
        peer: outcome.peer.label(),
        maybe_resolved_endpoint: outcome.maybe_resolved_endpoint.clone(),
        kind: peer_contribution_kind(outcome),
        messages_processed: contribution.messages_processed as u64,
        headers_received: contribution.headers_received as u64,
        blocks_received: contribution.blocks_received as u64,
        maybe_last_activity_unix_seconds: outcome.maybe_last_activity_unix_seconds,
        maybe_failure_reason_label: outcome
            .maybe_failure_reason
            .as_ref()
            .map(ToString::to_string),
    })
}

pub(super) fn classify_stall_diagnosis(
    input: &ProgressGuaranteeInput<'_>,
    diagnosis: NoProgressDiagnosis,
) -> FieldAvailability<StallDiagnosisEvidence> {
    let (stalled_subsystem, confidence) = stalled_subsystem(input, diagnosis);
    FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem,
        confidence,
        evidence_basis: stall_evidence_basis(input, diagnosis),
        next_action: no_progress_next_action(diagnosis).to_string(),
        maybe_no_progress_diagnosis: Some(diagnosis),
        maybe_recovery_category: input.recovery_category,
        maybe_latest_stop_reason_label: input
            .maybe_stop_reason
            .map(|reason| reason.label().to_string()),
        source_unix_seconds: input.evaluated_at_unix_seconds,
    })
}

pub(super) fn made_validated_durable_progress(
    progress: &SyncProgress,
    maybe_previous_credit: Option<&ProgressCreditEvidence>,
) -> bool {
    active_chain_credit(progress).is_some_and(|current| {
        maybe_previous_credit.is_none_or(|previous| !same_credit_tip(previous, &current))
    })
}

pub(super) fn progress_credit_log_label(
    field: &FieldAvailability<ProgressCreditEvidence>,
) -> String {
    match field {
        FieldAvailability::Available(credit) => format!(
            "{}:{}",
            progress_credit_kind_label(credit.kind),
            credit.credited_validated_active_chain_height
        ),
        FieldAvailability::Unavailable { .. } => "unavailable".to_string(),
    }
}

pub(super) fn peer_contribution_log_label(
    field: &FieldAvailability<PeerContributionEvidence>,
) -> String {
    match field {
        FieldAvailability::Available(contribution) => {
            peer_contribution_kind_label(contribution.kind).to_string()
        }
        FieldAvailability::Unavailable { .. } => "unavailable".to_string(),
    }
}

pub(super) fn no_progress_threshold_log_label(
    field: &FieldAvailability<NoProgressThresholdEvidence>,
) -> String {
    match field {
        FieldAvailability::Available(threshold) => {
            no_progress_threshold_state_label(threshold.state).to_string()
        }
        FieldAvailability::Unavailable { .. } => "unavailable".to_string(),
    }
}

pub(super) fn stalled_subsystem_log_label(
    field: &FieldAvailability<StallDiagnosisEvidence>,
) -> String {
    match field {
        FieldAvailability::Available(diagnosis) => {
            stalled_subsystem_label(diagnosis.stalled_subsystem).to_string()
        }
        FieldAvailability::Unavailable { .. } => "unavailable".to_string(),
    }
}

pub(super) fn stall_confidence_log_label(
    field: &FieldAvailability<StallDiagnosisEvidence>,
) -> String {
    match field {
        FieldAvailability::Available(diagnosis) => {
            stall_confidence_label(diagnosis.confidence).to_string()
        }
        FieldAvailability::Unavailable { .. } => "unavailable".to_string(),
    }
}

fn active_chain_credit(progress: &SyncProgress) -> Option<ActiveChainCredit<'_>> {
    let (Some(hash), Some(work)) = (
        progress.maybe_validated_active_chain_hash.as_deref(),
        progress.maybe_validated_active_chain_work.as_deref(),
    ) else {
        return None;
    };
    Some(ActiveChainCredit {
        height: progress.validated_active_chain_height,
        hash,
        work,
    })
}

const fn progress_credit_kind_label(kind: ProgressCreditKind) -> &'static str {
    match kind {
        ProgressCreditKind::ValidatedDurableActiveChain => "validated_durable_active_chain",
        ProgressCreditKind::CurrentAtBestKnownTip => "current_at_best_known_tip",
    }
}

const fn no_progress_threshold_state_label(state: NoProgressThresholdState) -> &'static str {
    match state {
        NoProgressThresholdState::WithinWindow => "within_window",
        NoProgressThresholdState::Exceeded => "exceeded",
    }
}

const fn peer_contribution_kind_label(kind: PeerContributionKind) -> &'static str {
    match kind {
        PeerContributionKind::HeadersOnly => "headers_only",
        PeerContributionKind::BlocksOnly => "blocks_only",
        PeerContributionKind::HeadersAndBlocks => "headers_and_blocks",
        PeerContributionKind::MessagesOnly => "messages_only",
        PeerContributionKind::NoUsefulContribution => "no_useful_contribution",
        PeerContributionKind::Failure => "failure",
    }
}

const fn stalled_subsystem_label(subsystem: StalledSubsystem) -> &'static str {
    match subsystem {
        StalledSubsystem::PublicNetworkReachability => "public_network_reachability",
        StalledSubsystem::IncompatiblePeers => "incompatible_peers",
        StalledSubsystem::SlowOrStalledPeers => "slow_or_stalled_peers",
        StalledSubsystem::PeerFailuresExhausted => "peer_failures_exhausted",
        StalledSubsystem::StaleInflightCleanup => "stale_inflight_cleanup",
        StalledSubsystem::BranchCompetitionAwaitingBodies => "branch_competition_awaiting_bodies",
        StalledSubsystem::Validation => "validation",
        StalledSubsystem::StorageOrResourcePressure => "storage_or_resource_pressure",
        StalledSubsystem::AtTipWaiting => "at_tip_waiting",
        StalledSubsystem::OperatorStop => "operator_stop",
        StalledSubsystem::LocalShutdown => "local_shutdown",
        StalledSubsystem::Unknown => "unknown",
    }
}

const fn stall_confidence_label(confidence: StallDiagnosisConfidence) -> &'static str {
    match confidence {
        StallDiagnosisConfidence::High => "high",
        StallDiagnosisConfidence::Medium => "medium",
        StallDiagnosisConfidence::Low => "low",
    }
}

fn same_credit_tip(previous: &ProgressCreditEvidence, current: &ActiveChainCredit<'_>) -> bool {
    previous.credited_validated_active_chain_height == current.height
        && previous.credited_validated_active_chain_hash == current.hash
        && previous.credited_validated_active_chain_work == current.work
}

fn progress_credit(
    kind: ProgressCreditKind,
    current: &ActiveChainCredit<'_>,
    input: &ProgressGuaranteeInput<'_>,
) -> ProgressCreditEvidence {
    ProgressCreditEvidence {
        kind,
        credited_validated_active_chain_height: current.height,
        credited_validated_active_chain_hash: current.hash.to_string(),
        credited_validated_active_chain_work: current.work.to_string(),
        source_unix_seconds: input.evaluated_at_unix_seconds,
        rejected_activity: rejected_activity(input),
    }
}

fn rejected_activity(input: &ProgressGuaranteeInput<'_>) -> Vec<RejectedProgressActivity> {
    let progress = input.sync_progress;
    let mut rejected = Vec::new();
    push_rejected(
        &mut rejected,
        RejectedProgressActivityKind::HeaderDownload,
        progress.headers_received,
        "headers do not prove durable active-chain progress",
    );
    push_rejected(
        &mut rejected,
        RejectedProgressActivityKind::BlockDownload,
        progress.blocks_received,
        "block responses do not prove durable active-chain progress",
    );
    push_rejected(
        &mut rejected,
        RejectedProgressActivityKind::PeerMessage,
        progress.messages_processed,
        "peer messages are liveness evidence only",
    );
    push_rejected(
        &mut rejected,
        RejectedProgressActivityKind::InFlightRequest,
        input.blocks_in_flight,
        "in-flight requests are outstanding work, not progress",
    );
    push_rejected(
        &mut rejected,
        RejectedProgressActivityKind::Retry,
        retry_count(input),
        "retry and backoff activity is not useful work",
    );
    push_rejected(
        &mut rejected,
        RejectedProgressActivityKind::ReportProjection,
        report_projection_count(input),
        "report projection is diagnosis evidence only",
    );
    rejected
}

fn push_rejected(
    rejected: &mut Vec<RejectedProgressActivity>,
    kind: RejectedProgressActivityKind,
    observed_count: u64,
    reason: &'static str,
) {
    if observed_count > 0 {
        rejected.push(RejectedProgressActivity {
            kind,
            observed_count,
            reason: reason.to_string(),
        });
    }
}

fn retry_count(input: &ProgressGuaranteeInput<'_>) -> u64 {
    let waiting_signal =
        u64::from(input.progress_signal == Some(SyncProgressSignal::WaitingForPeers));
    waiting_signal
        + input
            .peer_outcomes
            .iter()
            .filter(|outcome| outcome.maybe_failure_reason == Some(PeerFailureReason::RetryBackoff))
            .count() as u64
}

fn report_projection_count(input: &ProgressGuaranteeInput<'_>) -> u64 {
    match input.maybe_reconcile_progress {
        Some(SyncReconcileProgress::ExtendedActiveChain { .. })
        | Some(SyncReconcileProgress::ReorgPersisted(_))
        | None => 0,
        Some(
            SyncReconcileProgress::NoChange
            | SyncReconcileProgress::BranchCompetitionAwaitingBodies { .. }
            | SyncReconcileProgress::SideBranchPreserved,
        ) => 1,
    }
}

fn peer_contribution_kind(outcome: &PeerSyncOutcome) -> PeerContributionKind {
    let contribution = &outcome.contribution;
    if outcome.maybe_failure_reason.is_some() {
        return PeerContributionKind::Failure;
    }
    match (
        contribution.headers_received > 0,
        contribution.blocks_received > 0,
        contribution.messages_processed > 0,
    ) {
        (true, true, _) => PeerContributionKind::HeadersAndBlocks,
        (true, false, _) => PeerContributionKind::HeadersOnly,
        (false, true, _) => PeerContributionKind::BlocksOnly,
        (false, false, true) => PeerContributionKind::MessagesOnly,
        (false, false, false) => PeerContributionKind::NoUsefulContribution,
    }
}

fn stalled_subsystem(
    input: &ProgressGuaranteeInput<'_>,
    diagnosis: NoProgressDiagnosis,
) -> (StalledSubsystem, StallDiagnosisConfidence) {
    if input
        .recovery_category
        .is_some_and(is_storage_or_resource_blocker)
    {
        return (
            StalledSubsystem::StorageOrResourcePressure,
            StallDiagnosisConfidence::High,
        );
    }
    if input.maybe_stop_reason == Some(SyncStopReason::OperatorPaused) {
        return (
            StalledSubsystem::OperatorStop,
            StallDiagnosisConfidence::High,
        );
    }
    if input.maybe_stop_reason == Some(SyncStopReason::ShutdownRequested) {
        return (
            StalledSubsystem::LocalShutdown,
            StallDiagnosisConfidence::High,
        );
    }
    if let Some(reason) = latest_failure_reason(input) {
        return stalled_subsystem_for_peer_failure(reason);
    }
    stalled_subsystem_for_no_progress(diagnosis)
}

fn latest_failure_reason<'a>(input: &ProgressGuaranteeInput<'a>) -> Option<&'a PeerFailureReason> {
    input
        .peer_outcomes
        .iter()
        .rev()
        .filter_map(|outcome| outcome.maybe_failure_reason.as_ref())
        .next()
}

fn stalled_subsystem_for_peer_failure(
    reason: &PeerFailureReason,
) -> (StalledSubsystem, StallDiagnosisConfidence) {
    match reason {
        PeerFailureReason::AddressResolution
        | PeerFailureReason::Connect
        | PeerFailureReason::Network => (
            StalledSubsystem::PublicNetworkReachability,
            StallDiagnosisConfidence::High,
        ),
        PeerFailureReason::Compatibility | PeerFailureReason::InvalidMagic => (
            StalledSubsystem::IncompatiblePeers,
            StallDiagnosisConfidence::High,
        ),
        PeerFailureReason::Stall
        | PeerFailureReason::RetryBackoff
        | PeerFailureReason::BlockNotFound => (
            StalledSubsystem::SlowOrStalledPeers,
            StallDiagnosisConfidence::High,
        ),
        PeerFailureReason::InvalidBlock
        | PeerFailureReason::MalformedBlock
        | PeerFailureReason::InvalidData
        | PeerFailureReason::DisconnectedBlock
        | PeerFailureReason::NonExtendingBlock => {
            (StalledSubsystem::Validation, StallDiagnosisConfidence::High)
        }
        PeerFailureReason::ResourceLimit | PeerFailureReason::Storage => (
            StalledSubsystem::StorageOrResourcePressure,
            StallDiagnosisConfidence::High,
        ),
        PeerFailureReason::DuplicateBlock => (
            StalledSubsystem::SlowOrStalledPeers,
            StallDiagnosisConfidence::Medium,
        ),
    }
}

fn stalled_subsystem_for_no_progress(
    diagnosis: NoProgressDiagnosis,
) -> (StalledSubsystem, StallDiagnosisConfidence) {
    match diagnosis {
        NoProgressDiagnosis::CurrentAtBestKnownTip => (
            StalledSubsystem::AtTipWaiting,
            StallDiagnosisConfidence::High,
        ),
        NoProgressDiagnosis::BehindAwaitingHeaders => (
            StalledSubsystem::PublicNetworkReachability,
            StallDiagnosisConfidence::Low,
        ),
        NoProgressDiagnosis::AwaitingBlockBodies
        | NoProgressDiagnosis::PeerBackoff
        | NoProgressDiagnosis::PeerStalled => (
            StalledSubsystem::SlowOrStalledPeers,
            StallDiagnosisConfidence::Medium,
        ),
        NoProgressDiagnosis::StaleInflightCleanup => (
            StalledSubsystem::StaleInflightCleanup,
            StallDiagnosisConfidence::High,
        ),
        NoProgressDiagnosis::PeerFailuresExhausted => (
            StalledSubsystem::PeerFailuresExhausted,
            StallDiagnosisConfidence::Medium,
        ),
        NoProgressDiagnosis::BranchCompetitionAwaitingBodies => (
            StalledSubsystem::BranchCompetitionAwaitingBodies,
            StallDiagnosisConfidence::High,
        ),
        NoProgressDiagnosis::RecoveringFromReorgOrStorage
        | NoProgressDiagnosis::StorageOrResourceBlocked => (
            StalledSubsystem::StorageOrResourcePressure,
            StallDiagnosisConfidence::High,
        ),
    }
}

fn stall_evidence_basis(
    input: &ProgressGuaranteeInput<'_>,
    diagnosis: NoProgressDiagnosis,
) -> Vec<String> {
    let mut basis = vec![format!("no_progress_diagnosis={diagnosis:?}")];
    if let Some(category) = input.recovery_category {
        basis.push(format!("recovery_category={}", category.as_str()));
    }
    if let Some(reason) = input.maybe_stop_reason {
        basis.push(format!("stop_reason={}", reason.label()));
    }
    if let Some(reason) = latest_failure_reason(input) {
        basis.push(format!("peer_failure_reason={reason}"));
    }
    basis
}
