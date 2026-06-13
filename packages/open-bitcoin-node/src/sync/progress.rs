// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::primitives::NetworkAddress;
use open_bitcoin_network::{LocalPeerConfig, ServiceFlags};

use crate::{
    logging::{StructuredLogError, StructuredLogLevel},
    status::{
        HealthSignal, HealthSignalLevel, NoProgressDiagnosis, StayCurrentStatus,
        SyncProgressSignal, SyncRecoveryCategory,
    },
};

use super::{
    PeerCapabilitySummary, PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState,
    ResolvedSyncPeerAddress, SyncNetwork, SyncRunSummary, SyncRuntimeConfig, SyncRuntimeError,
    types::SyncReconcileProgress,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PeerProgress {
    pub(super) peer: ResolvedSyncPeerAddress,
    pub(super) network: SyncNetwork,
    pub(super) state: PeerSyncState,
    pub(super) attempts: u8,
    pub(super) messages_processed: usize,
    pub(super) headers_received: usize,
    pub(super) blocks_received: usize,
    pub(super) maybe_tip_height: Option<u64>,
    pub(super) maybe_tip_hash: Option<String>,
    pub(super) maybe_tip_work: Option<String>,
    pub(super) maybe_last_activity_unix_seconds: Option<u64>,
    pub(super) maybe_capabilities: Option<PeerCapabilitySummary>,
    pub(super) maybe_failure_reason: Option<PeerFailureReason>,
}

#[derive(Debug)]
pub(super) struct PeerFailure {
    pub(super) peer: ResolvedSyncPeerAddress,
    pub(super) error: super::SyncRuntimeError,
    pub(super) attempts: u8,
    pub(super) reason: PeerFailureReason,
    pub(super) maybe_progress: Option<PeerProgress>,
}

pub(super) struct NoProgressInput<'a> {
    pub(super) stay_current: Option<StayCurrentStatus>,
    pub(super) progress_signal: Option<SyncProgressSignal>,
    pub(super) recovery_category: Option<SyncRecoveryCategory>,
    pub(super) blocks_in_flight: u64,
    pub(super) maybe_reconcile_progress: Option<&'a SyncReconcileProgress>,
    pub(super) peer_outcomes: &'a [PeerSyncOutcome],
}

impl PeerProgress {
    pub(super) fn new(peer: ResolvedSyncPeerAddress, network: SyncNetwork, attempts: u8) -> Self {
        Self {
            peer,
            network,
            state: PeerSyncState::Connected,
            attempts,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
            maybe_tip_height: None,
            maybe_tip_hash: None,
            maybe_tip_work: None,
            maybe_last_activity_unix_seconds: None,
            maybe_capabilities: None,
            maybe_failure_reason: None,
        }
    }

    pub(super) fn record_activity(&mut self, timestamp: i64) {
        self.messages_processed += 1;
        self.maybe_last_activity_unix_seconds = Some(u64::try_from(timestamp).unwrap_or(0));
    }

    pub(super) fn record_validated_headers(&mut self, count: usize) {
        self.headers_received += count;
    }

    pub(super) fn record_tip_observation(
        &mut self,
        height: u64,
        block_hash: String,
        chain_work: String,
    ) {
        self.maybe_tip_height = Some(height);
        self.maybe_tip_hash = Some(block_hash);
        self.maybe_tip_work = Some(chain_work);
    }

    pub(super) fn record_accepted_block(&mut self) {
        self.blocks_received += 1;
    }

    pub(super) fn record_block_notfound(&mut self) {
        self.record_no_credit_block_response(PeerFailureReason::BlockNotFound);
    }

    pub(super) fn record_malformed_block(&mut self) {
        self.record_no_credit_block_response(PeerFailureReason::MalformedBlock);
    }

    pub(super) fn record_invalid_block(&mut self) {
        self.record_no_credit_block_response(PeerFailureReason::InvalidBlock);
    }

    pub(super) fn record_duplicate_block(&mut self) {
        self.record_no_credit_block_response(PeerFailureReason::DuplicateBlock);
    }

    pub(super) fn record_disconnected_block(&mut self) {
        self.record_no_credit_block_response(PeerFailureReason::DisconnectedBlock);
    }

    pub(super) fn record_non_extending_block(&mut self) {
        self.record_no_credit_block_response(PeerFailureReason::NonExtendingBlock);
    }

    fn record_no_credit_block_response(&mut self, reason: PeerFailureReason) {
        self.maybe_failure_reason = Some(reason);
    }

    pub(super) fn is_successful_outbound_slot(&self) -> bool {
        self.state == PeerSyncState::Connected && self.maybe_failure_reason.is_none()
    }

    pub(super) fn should_retry_with_backoff(&self) -> bool {
        self.state == PeerSyncState::Stalled
            || self
                .maybe_failure_reason
                .as_ref()
                .is_some_and(PeerFailureReason::is_no_credit_block_response)
    }

    pub(super) fn into_outcome(self, maybe_error: Option<String>) -> PeerSyncOutcome {
        PeerSyncOutcome {
            peer: self.peer.peer,
            maybe_resolved_endpoint: Some(self.peer.endpoint.to_string()),
            network: self.network,
            state: self.state,
            attempts: self.attempts,
            contribution: PeerContribution {
                messages_processed: self.messages_processed,
                headers_received: self.headers_received,
                blocks_received: self.blocks_received,
            },
            maybe_tip_height: self.maybe_tip_height,
            maybe_tip_hash: self.maybe_tip_hash,
            maybe_tip_work: self.maybe_tip_work,
            maybe_last_activity_unix_seconds: self.maybe_last_activity_unix_seconds,
            maybe_capabilities: self.maybe_capabilities,
            maybe_failure_reason: self.maybe_failure_reason,
            maybe_error,
        }
    }

    pub(super) fn into_failed_outcome(
        mut self,
        reason: PeerFailureReason,
        maybe_error: Option<String>,
    ) -> PeerSyncOutcome {
        self.state = PeerSyncState::Failed;
        self.maybe_failure_reason = Some(reason);
        self.into_outcome(maybe_error)
    }
}

pub(super) fn classify_no_progress(input: &NoProgressInput<'_>) -> NoProgressDiagnosis {
    if input
        .recovery_category
        .is_some_and(is_storage_or_resource_blocker)
    {
        return NoProgressDiagnosis::StorageOrResourceBlocked;
    }

    if input.stay_current == Some(StayCurrentStatus::Recovering)
        && !matches!(
            input.maybe_reconcile_progress,
            Some(SyncReconcileProgress::ReorgPersisted(_))
        )
    {
        return NoProgressDiagnosis::RecoveringFromReorgOrStorage;
    }

    if matches!(
        input.maybe_reconcile_progress,
        Some(SyncReconcileProgress::BranchCompetitionAwaitingBodies { .. })
    ) {
        return NoProgressDiagnosis::BranchCompetitionAwaitingBodies;
    }

    if input.stay_current == Some(StayCurrentStatus::CurrentAtBestKnownTip) {
        return NoProgressDiagnosis::CurrentAtBestKnownTip;
    }

    if input.progress_signal == Some(SyncProgressSignal::AwaitingBlocks) {
        return NoProgressDiagnosis::AwaitingBlockBodies;
    }

    if input.blocks_in_flight > 0 && input.progress_signal == Some(SyncProgressSignal::Steady) {
        return NoProgressDiagnosis::StaleInflightCleanup;
    }

    if input
        .peer_outcomes
        .iter()
        .any(|outcome| outcome.maybe_failure_reason == Some(PeerFailureReason::RetryBackoff))
    {
        return NoProgressDiagnosis::PeerBackoff;
    }

    if input.peer_outcomes.iter().any(|outcome| {
        outcome.state == PeerSyncState::Stalled
            || outcome.maybe_failure_reason == Some(PeerFailureReason::Stall)
    }) {
        return NoProgressDiagnosis::PeerStalled;
    }

    if input.progress_signal == Some(SyncProgressSignal::PeerFailures) {
        return NoProgressDiagnosis::PeerFailuresExhausted;
    }

    NoProgressDiagnosis::BehindAwaitingHeaders
}

pub(super) const fn no_progress_next_action(diagnosis: NoProgressDiagnosis) -> &'static str {
    match diagnosis {
        NoProgressDiagnosis::CurrentAtBestKnownTip => {
            "Confirm current-at-tip evidence; no sync action is required."
        }
        NoProgressDiagnosis::BehindAwaitingHeaders => {
            "Wait for peer headers or try another configured peer."
        }
        NoProgressDiagnosis::AwaitingBlockBodies => "Wait for block bodies from eligible peers.",
        NoProgressDiagnosis::StaleInflightCleanup => {
            "Wait for stale in-flight block cleanup and reassignment."
        }
        NoProgressDiagnosis::PeerBackoff => {
            "Wait for retry backoff or try another configured peer."
        }
        NoProgressDiagnosis::PeerStalled => "Try another peer if stalls repeat after backoff.",
        NoProgressDiagnosis::PeerFailuresExhausted => {
            "Try another configured peer and inspect latest peer failures."
        }
        NoProgressDiagnosis::BranchCompetitionAwaitingBodies => {
            "Wait for replacement branch block bodies before reorg."
        }
        NoProgressDiagnosis::RecoveringFromReorgOrStorage => {
            "Inspect storage health before retrying sync."
        }
        NoProgressDiagnosis::StorageOrResourceBlocked => {
            "Inspect storage health, free disk space for the selected datadir, or increase bounded resource limits."
        }
    }
}

const fn is_storage_or_resource_blocker(category: SyncRecoveryCategory) -> bool {
    matches!(
        category,
        SyncRecoveryCategory::IncompatibleSchema
            | SyncRecoveryCategory::StoreCorruption
            | SyncRecoveryCategory::StorageLockContention
            | SyncRecoveryCategory::StorageBackendFailure
            | SyncRecoveryCategory::ResourceExhaustion
    )
}

pub(super) fn structured_log_level(level: HealthSignalLevel) -> StructuredLogLevel {
    match level {
        HealthSignalLevel::Info => StructuredLogLevel::Info,
        HealthSignalLevel::Warn => StructuredLogLevel::Warn,
        HealthSignalLevel::Error => StructuredLogLevel::Error,
    }
}

pub(super) fn sync_progress_marker(summary: &SyncRunSummary) -> (u64, u64) {
    (summary.best_header_height, summary.best_block_height)
}

pub(super) fn log_write_failed_signal(error: &StructuredLogError) -> HealthSignal {
    let message = match error {
        StructuredLogError::Io { action, source, .. } => {
            format!("structured log write failed: {action}: {source}")
        }
        StructuredLogError::Json { source } => {
            format!("structured log write failed: JSON encoding: {source}")
        }
    };

    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "logging".to_string(),
        message,
    }
}

pub(super) fn stalled_peer_signal() -> HealthSignal {
    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "sync".to_string(),
        message: "peer stalled before sending more sync messages".to_string(),
    }
}

pub(super) fn waiting_peer_signal() -> HealthSignal {
    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "sync".to_string(),
        message: "peer waiting for retry backoff before next attempt".to_string(),
    }
}

pub(super) fn local_peer_config(config: &SyncRuntimeConfig) -> LocalPeerConfig {
    LocalPeerConfig {
        magic: config.network.magic(),
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 0,
        },
        nonce: 0,
        relay: true,
        user_agent: format!("/open-bitcoin:{}/", env!("CARGO_PKG_VERSION")),
    }
}

pub(super) fn retry_backoff_seconds(retry_backoff_ms: u64) -> i64 {
    let seconds = retry_backoff_ms.div_ceil(1_000).max(1);
    i64::try_from(seconds).unwrap_or(i64::MAX)
}

pub(super) fn peer_failure_reason_for_error(error: &SyncRuntimeError) -> PeerFailureReason {
    match error {
        SyncRuntimeError::AddressResolution { .. } => PeerFailureReason::AddressResolution,
        SyncRuntimeError::InvalidData { message } if message.contains("malformed block") => {
            PeerFailureReason::MalformedBlock
        }
        SyncRuntimeError::InvalidData { .. } => PeerFailureReason::InvalidData,
        SyncRuntimeError::InvalidMagic { .. } => PeerFailureReason::InvalidMagic,
        SyncRuntimeError::PeerCompatibility { .. } => PeerFailureReason::Compatibility,
        SyncRuntimeError::Storage(_) => PeerFailureReason::Storage,
        SyncRuntimeError::Io { .. } => PeerFailureReason::Connect,
        SyncRuntimeError::Network { message } if message.contains("malformed block") => {
            PeerFailureReason::MalformedBlock
        }
        SyncRuntimeError::ResourceLimit { .. } => PeerFailureReason::ResourceLimit,
        SyncRuntimeError::Network { .. } | SyncRuntimeError::NoPeersConfigured => {
            PeerFailureReason::Network
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::status::{
        NoProgressDiagnosis, StayCurrentStatus, SyncProgressSignal, SyncRecoveryCategory,
    };

    use super::super::{
        PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState, SyncNetwork,
        SyncPeerAddress, types::SyncReconcileProgress,
    };
    use super::{NoProgressInput, classify_no_progress, no_progress_next_action};

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
}
