// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::primitives::BlockHash;
use open_bitcoin_network::{HeaderEntry, HeaderStore};

use crate::{
    FieldAvailability,
    status::{
        BestKnownTipSource, BestKnownTipStatus, PeerTipAgreement, PeerTipAgreementStatus,
        StayCurrentStatus, SyncLifecycleState, TipFreshnessStatus,
    },
};

use super::{PeerSyncOutcome, progress::PeerProgress, runtime_state::BlockProgressPoint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BestTipEvidence {
    pub(super) height: u64,
    pub(super) block_hash: String,
    pub(super) work: String,
    pub(super) block_time_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ConnectedTipEvidence {
    pub(super) height: u64,
    pub(super) block_hash: String,
    pub(super) work: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TipEvidenceInput {
    pub(super) maybe_best_tip: Option<BestTipEvidence>,
    pub(super) maybe_connected_tip: Option<ConnectedTipEvidence>,
    pub(super) observed_at_unix_seconds: u64,
    pub(super) tip_freshness_threshold_seconds: u64,
    pub(super) lifecycle: SyncLifecycleState,
    pub(super) made_useful_progress: bool,
    pub(super) peer_agreement: Vec<PeerTipAgreement>,
}

pub(super) fn best_tip_from_header_entry(entry: &HeaderEntry) -> BestTipEvidence {
    BestTipEvidence {
        height: u64::from(entry.height),
        block_hash: block_hash_hex(entry.block_hash),
        work: entry.chain_work.to_string(),
        block_time_unix_seconds: u64::from(entry.header.time),
    }
}

pub(super) fn connected_tip_from_progress(progress: BlockProgressPoint) -> ConnectedTipEvidence {
    ConnectedTipEvidence {
        height: progress.height,
        block_hash: block_hash_hex(progress.block_hash),
        work: progress.chain_work.to_string(),
    }
}

pub(super) fn record_peer_terminal_tip(
    progress: &mut PeerProgress,
    header_store: &HeaderStore,
    maybe_terminal_header_hash: Option<BlockHash>,
) {
    let Some(terminal_header_hash) = maybe_terminal_header_hash else {
        return;
    };
    let Some(entry) = header_store.entry(&terminal_header_hash) else {
        return;
    };
    progress.record_tip_observation(
        u64::from(entry.height),
        block_hash_hex(entry.block_hash),
        entry.chain_work.to_string(),
    );
}

pub(super) fn block_hash_hex(block_hash: BlockHash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = block_hash.as_bytes();
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

pub(super) fn build_best_known_tip_status(
    input: &TipEvidenceInput,
) -> FieldAvailability<BestKnownTipStatus> {
    let Some(best_tip) = &input.maybe_best_tip else {
        return FieldAvailability::unavailable("best-known tip evidence unavailable");
    };

    FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: best_tip.height,
        block_hash: best_tip.block_hash.clone(),
        work: best_tip.work.clone(),
        block_time_unix_seconds: best_tip.block_time_unix_seconds,
        observed_at_unix_seconds: input.observed_at_unix_seconds,
        freshness: freshness_for_tip(input, best_tip),
        peer_agreement: input.peer_agreement.clone(),
    })
}

pub(super) fn classify_stay_current(input: &TipEvidenceInput) -> StayCurrentStatus {
    if input.lifecycle == SyncLifecycleState::Recovering {
        return StayCurrentStatus::Recovering;
    }

    let Some(best_tip) = &input.maybe_best_tip else {
        if input.made_useful_progress {
            return StayCurrentStatus::InitialCatchUp;
        }
        return StayCurrentStatus::NoProgress;
    };

    if freshness_for_tip(input, best_tip) == TipFreshnessStatus::Stale
        || has_stale_peer_evidence(input)
    {
        return StayCurrentStatus::StaleTip;
    }

    if input
        .maybe_connected_tip
        .as_ref()
        .is_some_and(|connected_tip| connected_tip_matches_best(connected_tip, best_tip))
    {
        return StayCurrentStatus::CurrentAtBestKnownTip;
    }

    if input.made_useful_progress {
        return StayCurrentStatus::InitialCatchUp;
    }

    StayCurrentStatus::NoProgress
}

pub(super) fn peer_tip_agreement_for_outcome(
    outcome: &PeerSyncOutcome,
    maybe_best_tip: Option<&BestTipEvidence>,
) -> PeerTipAgreement {
    let status = peer_agreement_status(outcome, maybe_best_tip);
    PeerTipAgreement {
        peer: outcome.peer.label(),
        maybe_resolved_endpoint: outcome.maybe_resolved_endpoint.clone(),
        status,
        maybe_height: outcome.maybe_tip_height,
        maybe_hash: outcome.maybe_tip_hash.clone(),
        maybe_work: outcome.maybe_tip_work.clone(),
        maybe_last_activity_unix_seconds: outcome.maybe_last_activity_unix_seconds,
    }
}

fn peer_agreement_status(
    outcome: &PeerSyncOutcome,
    maybe_best_tip: Option<&BestTipEvidence>,
) -> PeerTipAgreementStatus {
    let Some(best_tip) = maybe_best_tip else {
        return PeerTipAgreementStatus::NoEvidence;
    };
    let (Some(height), Some(block_hash), Some(work)) = (
        outcome.maybe_tip_height,
        outcome.maybe_tip_hash.as_deref(),
        outcome.maybe_tip_work.as_deref(),
    ) else {
        return PeerTipAgreementStatus::NoEvidence;
    };

    if height == best_tip.height && block_hash == best_tip.block_hash && work == best_tip.work {
        return PeerTipAgreementStatus::Agrees;
    }
    if height < best_tip.height {
        return PeerTipAgreementStatus::Behind;
    }
    PeerTipAgreementStatus::Disagrees
}

fn freshness_for_tip(input: &TipEvidenceInput, best_tip: &BestTipEvidence) -> TipFreshnessStatus {
    let age = input
        .observed_at_unix_seconds
        .saturating_sub(best_tip.block_time_unix_seconds);
    if age <= input.tip_freshness_threshold_seconds {
        TipFreshnessStatus::Fresh
    } else {
        TipFreshnessStatus::Stale
    }
}

fn has_stale_peer_evidence(input: &TipEvidenceInput) -> bool {
    input
        .peer_agreement
        .iter()
        .filter(|row| row.status != PeerTipAgreementStatus::NoEvidence)
        .filter_map(|row| row.maybe_last_activity_unix_seconds)
        .any(|last_seen| {
            input.observed_at_unix_seconds.saturating_sub(last_seen)
                > input.tip_freshness_threshold_seconds
        })
}

fn connected_tip_matches_best(
    connected_tip: &ConnectedTipEvidence,
    best_tip: &BestTipEvidence,
) -> bool {
    connected_tip.height == best_tip.height
        && connected_tip.block_hash == best_tip.block_hash
        && connected_tip.work == best_tip.work
}

#[cfg(test)]
mod tests {
    use crate::status::PeerTipAgreement;

    use super::*;

    fn fresh_best_tip() -> BestTipEvidence {
        BestTipEvidence {
            height: 2,
            block_hash: "22".repeat(32),
            work: "3".to_string(),
            block_time_unix_seconds: 1_000,
        }
    }

    fn base_input() -> TipEvidenceInput {
        TipEvidenceInput {
            maybe_best_tip: Some(fresh_best_tip()),
            maybe_connected_tip: None,
            observed_at_unix_seconds: 1_100,
            tip_freshness_threshold_seconds: 1_200,
            lifecycle: SyncLifecycleState::Active,
            made_useful_progress: false,
            peer_agreement: Vec::new(),
        }
    }

    #[test]
    fn phase69_tip_classification_handles_missing_fresh_stale_and_recovering() {
        // Arrange
        let mut input = base_input();

        // Act / Assert
        input.maybe_best_tip = None;
        assert_eq!(classify_stay_current(&input), StayCurrentStatus::NoProgress);

        input.maybe_best_tip = Some(fresh_best_tip());
        input.maybe_connected_tip = Some(ConnectedTipEvidence {
            height: 2,
            block_hash: "22".repeat(32),
            work: "3".to_string(),
        });
        assert_eq!(
            classify_stay_current(&input),
            StayCurrentStatus::CurrentAtBestKnownTip
        );

        input.observed_at_unix_seconds = 2_500;
        assert_eq!(classify_stay_current(&input), StayCurrentStatus::StaleTip);

        input.lifecycle = SyncLifecycleState::Recovering;
        assert_eq!(classify_stay_current(&input), StayCurrentStatus::Recovering);
    }

    #[test]
    fn phase69_tip_status_includes_peer_evidence_and_freshness() {
        // Arrange
        let mut input = base_input();
        input.peer_agreement = vec![PeerTipAgreement {
            peer: "127.0.0.1:18444".to_string(),
            maybe_resolved_endpoint: Some("127.0.0.1:18444".to_string()),
            status: PeerTipAgreementStatus::Agrees,
            maybe_height: Some(2),
            maybe_hash: Some("22".repeat(32)),
            maybe_work: Some("3".to_string()),
            maybe_last_activity_unix_seconds: Some(1_100),
        }];

        // Act
        let status = build_best_known_tip_status(&input);

        // Assert
        let FieldAvailability::Available(status) = status else {
            panic!("best-known tip status should be available");
        };
        assert_eq!(status.source, BestKnownTipSource::HeaderStore);
        assert_eq!(status.height, 2);
        assert_eq!(status.freshness, TipFreshnessStatus::Fresh);
        assert_eq!(status.peer_agreement.len(), 1);
    }
}
