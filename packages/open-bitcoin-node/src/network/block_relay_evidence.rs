// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_core::primitives::InventoryType;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingEligibilityReason, BlockServingStatusLabel,
    CompactAnnouncementReason, CompactDownloadCleanupCause, PeerAction, PeerManager,
    WireNetworkMessage,
};

use crate::{
    ChainstateStore,
    status::{
        BlockRelayEvidenceStatus, BlockServingActivationEvidence, BlockServingEligibilityCounters,
        BlockServingEvidenceStatus, BlockServingStatusCounters, CompactRelayAnnouncementCounters,
        CompactRelayCleanupCounters, CompactRelayFallbackCounters, CompactRelayInFlightCounters,
        CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
        CompactRelayReconstructionCounters,
    },
};

use super::{ManagedPeerNetwork, block_serving::ManagedBlockServeDecision};

#[derive(Debug, Clone, Default)]
pub(super) struct ManagedBlockRelayEvidenceState {
    observed: bool,
    block_serving_eligibility: BlockServingEligibilityCounters,
    block_serving_status: BlockServingStatusCounters,
    announcement: CompactRelayAnnouncementCounters,
    reconstruction: CompactRelayReconstructionCounters,
    missing_transaction: CompactRelayMissingTransactionCounters,
    fallback: CompactRelayFallbackCounters,
    cleanup: CompactRelayCleanupCounters,
}

impl ManagedBlockRelayEvidenceState {
    fn observed_status(
        &self,
        peer_manager: &PeerManager,
        block_relay_activation: BlockRelayActivationPolicy,
    ) -> BlockRelayEvidenceStatus {
        if !self.observed && !has_live_compact_state(peer_manager) {
            return BlockRelayEvidenceStatus::default_unavailable();
        }

        let block_serving = BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            BlockServingActivationEvidence {
                block_serving_enabled: block_relay_activation.block_serving.enabled,
                compact_relay_enabled: block_relay_activation.compact_relay.enabled,
            },
            self.block_serving_eligibility,
            self.block_serving_status,
        );

        BlockRelayEvidenceStatus::with_components(
            block_serving,
            negotiation_counters(peer_manager),
            self.announcement,
            self.reconstruction,
            self.missing_transaction,
            fallback_counters(peer_manager, self.fallback),
            in_flight_counters(peer_manager),
            self.cleanup,
        )
    }

    fn note_observed(&mut self) {
        self.observed = true;
    }

    fn record_block_serving(
        &mut self,
        inventory_type: InventoryType,
        decision: &ManagedBlockServeDecision,
    ) {
        self.note_observed();

        match decision.eligibility_reason {
            BlockServingEligibilityReason::Eligible => {
                self.block_serving_eligibility.eligible_peer_count += 1;
            }
            BlockServingEligibilityReason::Disabled => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility.disabled_count += 1;
            }
            BlockServingEligibilityReason::ActivationRequired => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility.activation_required_count += 1;
            }
            BlockServingEligibilityReason::InboundServingRequired => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility
                    .inbound_serving_required_count += 1;
            }
            BlockServingEligibilityReason::PermissionRequired => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility.permission_required_count += 1;
            }
            BlockServingEligibilityReason::ProtectedNotServing => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility.protected_not_serving_count += 1;
            }
            BlockServingEligibilityReason::StatusUnavailable => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility.status_unavailable_count += 1;
            }
            BlockServingEligibilityReason::PermissionEffectInactive => {
                self.block_serving_eligibility.ineligible_peer_count += 1;
                self.block_serving_eligibility
                    .permission_effect_inactive_count += 1;
            }
        }

        match decision.status_label {
            BlockServingStatusLabel::Validated => self.block_serving_status.validated_count += 1,
            BlockServingStatusLabel::Available => self.block_serving_status.available_count += 1,
            BlockServingStatusLabel::Stale => self.block_serving_status.stale_count += 1,
            BlockServingStatusLabel::SideChain => self.block_serving_status.side_chain_count += 1,
            BlockServingStatusLabel::Pruned => self.block_serving_status.pruned_count += 1,
            BlockServingStatusLabel::Unavailable => {
                self.block_serving_status.unavailable_count += 1
            }
            BlockServingStatusLabel::Unvalidated => {
                self.block_serving_status.unvalidated_count += 1
            }
            BlockServingStatusLabel::Unknown => self.block_serving_status.unknown_count += 1,
            BlockServingStatusLabel::Suppressed => self.block_serving_status.suppressed_count += 1,
        }

        if inventory_type == InventoryType::CompactBlock {
            self.announcement.compact_suppressed_count += 1;
        }
    }

    fn record_announcement(&mut self, reason: CompactAnnouncementReason) {
        self.note_observed();
        match reason {
            CompactAnnouncementReason::CompactAnnounced => {
                self.announcement.compact_announced_count += 1;
            }
            CompactAnnouncementReason::CompactHeadersFallback => {
                self.announcement.compact_headers_fallback_count += 1;
            }
            CompactAnnouncementReason::CompactInventoryFallback => {
                self.announcement.compact_inventory_fallback_count += 1;
            }
            CompactAnnouncementReason::CompactRelayDisabled
            | CompactAnnouncementReason::CompactPeerNotNegotiated
            | CompactAnnouncementReason::CompactUnsupportedVersion
            | CompactAnnouncementReason::CompactHighBandwidthNotRequested
            | CompactAnnouncementReason::CompactHeaderContinuityMissing
            | CompactAnnouncementReason::CompactPeerAlreadyHasHeader
            | CompactAnnouncementReason::CompactBlockUnavailable
            | CompactAnnouncementReason::CompactResourceLimited => {
                self.announcement.compact_suppressed_count += 1;
            }
        }
    }

    fn record_compact_download_actions(&mut self, actions: &[PeerAction]) {
        if actions.is_empty() {
            return;
        }
        self.note_observed();
        for action in actions {
            match action {
                PeerAction::Send(WireNetworkMessage::GetBlockTxn(_)) => {
                    self.missing_transaction.compact_missing_tx_requested_count += 1;
                }
                PeerAction::Send(WireNetworkMessage::GetData(inventory))
                    if inventory
                        .inventory
                        .iter()
                        .any(|item| item.inventory_type == InventoryType::Block) =>
                {
                    self.fallback.compact_fallback_count += 1;
                }
                PeerAction::ReceivedBlock(_) => {
                    self.reconstruction.compact_reconstructed_count += 1;
                }
                _ => {}
            }
        }
    }

    fn record_cleanup(&mut self, cause: CompactDownloadCleanupCause, removed_count: usize) {
        if removed_count == 0 {
            return;
        }

        self.note_observed();
        self.cleanup.compact_cleanup_count += removed_count as u64;
        match cause {
            CompactDownloadCleanupCause::PeerDisconnect => {
                self.cleanup.compact_download_peer_disconnect_count += removed_count as u64;
            }
            CompactDownloadCleanupCause::Timeout => {
                self.cleanup.compact_download_timeout_count += removed_count as u64;
                self.fallback.compact_timeout_count += removed_count as u64;
                self.fallback.compact_fallback_count += removed_count as u64;
            }
            CompactDownloadCleanupCause::Reorg => {
                self.cleanup.compact_download_reorg_count += removed_count as u64;
            }
            CompactDownloadCleanupCause::RuntimeRestart => {
                self.cleanup.compact_download_restart_count += removed_count as u64;
            }
            CompactDownloadCleanupCause::BlockConnected => {
                self.cleanup.compact_download_block_connected_count += removed_count as u64;
            }
        }
    }
}

fn has_live_compact_state(peer_manager: &PeerManager) -> bool {
    peer_manager.peer_ids().into_iter().any(|peer_id| {
        let Some(peer) = peer_manager.peer_state(peer_id) else {
            return false;
        };
        !matches!(
            peer.compact_relay.capability,
            open_bitcoin_network::CompactRelayCapability::Unknown
        ) || !matches!(
            peer.compact_relay.high_bandwidth_preference,
            open_bitcoin_network::CompactRelayPreference::Unknown
        ) || !matches!(
            peer.compact_relay.low_bandwidth_preference,
            open_bitcoin_network::CompactRelayPreference::Unknown
        ) || peer_manager
            .compact_download_peer_state(peer_id)
            .is_some_and(|state| !state.in_flight.is_empty())
    })
}

fn negotiation_counters(peer_manager: &PeerManager) -> CompactRelayNegotiationCounters {
    let mut counters = CompactRelayNegotiationCounters::default();
    for peer_id in peer_manager.peer_ids() {
        let Some(peer) = peer_manager.peer_state(peer_id) else {
            continue;
        };
        match peer.compact_relay.capability {
            open_bitcoin_network::CompactRelayCapability::Supported { version: 2 }
                if matches!(
                    peer.compact_relay.high_bandwidth_preference,
                    open_bitcoin_network::CompactRelayPreference::Requested
                ) =>
            {
                counters.version2_high_bandwidth_count += 1;
            }
            open_bitcoin_network::CompactRelayCapability::Supported { version: 2 }
                if matches!(
                    peer.compact_relay.low_bandwidth_preference,
                    open_bitcoin_network::CompactRelayPreference::Requested
                ) =>
            {
                counters.version2_low_bandwidth_count += 1;
            }
            open_bitcoin_network::CompactRelayCapability::Unsupported { .. } => {
                counters.unsupported_version_count += 1;
            }
            _ => {}
        }
    }
    counters
}

fn in_flight_counters(peer_manager: &PeerManager) -> CompactRelayInFlightCounters {
    let mut counters = CompactRelayInFlightCounters::default();
    for peer_id in peer_manager.peer_ids() {
        let Some(state) = peer_manager.compact_download_peer_state(peer_id) else {
            continue;
        };
        let in_flight_count = state.in_flight.len() as u64;
        if in_flight_count == 0 {
            continue;
        }
        counters.in_flight_count += in_flight_count;
        counters.peers_with_in_flight_count += 1;
        counters.getblocktxn_in_flight_count += state
            .in_flight
            .values()
            .filter(|entry| entry.getblocktxn_in_flight)
            .count() as u64;
    }
    counters
}

fn fallback_counters(
    peer_manager: &PeerManager,
    mut counters: CompactRelayFallbackCounters,
) -> CompactRelayFallbackCounters {
    let timed_out = peer_manager
        .peer_ids()
        .into_iter()
        .filter_map(|peer_id| peer_manager.compact_download_peer_state(peer_id))
        .flat_map(|state| state.in_flight.values())
        .filter(|entry| entry.getblocktxn_in_flight)
        .count() as u64;
    counters.compact_timeout_count += timed_out;
    counters
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn block_relay_evidence_status(&self) -> BlockRelayEvidenceStatus {
        self.block_relay_evidence
            .observed_status(&self.peer_manager, self.block_relay_activation)
    }

    pub(super) fn record_block_serving_evidence(
        &mut self,
        inventory_type: InventoryType,
        decision: &ManagedBlockServeDecision,
    ) {
        self.block_relay_evidence
            .record_block_serving(inventory_type, decision);
    }

    pub(super) fn record_compact_announcement_evidence(
        &mut self,
        reason: CompactAnnouncementReason,
    ) {
        self.block_relay_evidence.record_announcement(reason);
    }

    pub(super) fn record_compact_download_evidence(&mut self, actions: &[PeerAction]) {
        self.block_relay_evidence
            .record_compact_download_actions(actions);
    }

    pub(super) fn note_block_relay_observed(&mut self) {
        self.block_relay_evidence.note_observed();
    }

    pub(super) fn record_compact_cleanup(
        &mut self,
        cause: CompactDownloadCleanupCause,
        removed_count: usize,
    ) {
        self.block_relay_evidence
            .record_cleanup(cause, removed_count);
    }
}
