// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use std::collections::BTreeMap;

use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{Block, BlockHash, InventoryType, InventoryVector},
};
use open_bitcoin_network::{
    BlockServingEligibilityInput, BlockServingResourceGateInput, BlockServingStatusFacts,
    CompactAnnouncementAction, CompactAnnouncementReason, PHASE94_MAX_PEER_QUEUED_MESSAGES,
    PeerCompactAnnouncementInput, PeerId, QueuePressureInput, ReconnectSuppressionInput,
    RequestPressureInput, ResourceGovernancePolicy, WireNetworkMessage,
    classify_block_serving_eligibility, classify_block_serving_status,
    evaluate_block_serving_resource_gate,
};

use crate::ChainstateStore;

use super::{ManagedPeerNetwork, block_relay_evidence};

/// A live snapshot of one session's bounded announcement outbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerOutboxSnapshot {
    peer_id: PeerId,
    queued_messages: usize,
    capacity: usize,
}

impl PeerOutboxSnapshot {
    pub const fn new(peer_id: PeerId, queued_messages: usize, capacity: usize) -> Self {
        Self {
            peer_id,
            queued_messages,
            capacity,
        }
    }

    pub const fn peer_id(self) -> PeerId {
        self.peer_id
    }

    pub const fn queued_messages(self) -> usize {
        self.queued_messages
    }

    pub const fn capacity(self) -> usize {
        self.capacity
    }

    pub const fn is_full(self) -> bool {
        let bounded_capacity = if self.capacity < PHASE94_MAX_PEER_QUEUED_MESSAGES {
            self.capacity
        } else {
            PHASE94_MAX_PEER_QUEUED_MESSAGES
        };
        self.queued_messages >= bounded_capacity
    }
}

/// A peer-targeted announcement that owns everything needed by transport.
///
/// The carrier deliberately does not implement `Clone`; transport must either
/// consume it into a success receipt or discard it after a failed write.
#[derive(Debug, PartialEq, Eq)]
pub struct PeerEmission {
    peer_id: PeerId,
    message: WireNetworkMessage,
    block_hash: BlockHash,
    evidence_reason: CompactAnnouncementReason,
}

impl PeerEmission {
    pub(crate) const fn new(
        peer_id: PeerId,
        message: WireNetworkMessage,
        block_hash: BlockHash,
        evidence_reason: CompactAnnouncementReason,
    ) -> Self {
        Self {
            peer_id,
            message,
            block_hash,
            evidence_reason,
        }
    }

    pub const fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub const fn message(&self) -> &WireNetworkMessage {
        &self.message
    }

    pub fn into_parts(self) -> (PeerId, WireNetworkMessage, PeerEmissionReceipt) {
        let Self {
            peer_id,
            message,
            block_hash,
            evidence_reason,
        } = self;
        (
            peer_id,
            message,
            PeerEmissionReceipt {
                peer_id,
                block_hash,
                evidence_reason,
            },
        )
    }
}

/// A non-replayable acknowledgement capability bound to one prepared emission.
#[derive(Debug, PartialEq, Eq)]
pub struct PeerEmissionReceipt {
    peer_id: PeerId,
    block_hash: BlockHash,
    evidence_reason: CompactAnnouncementReason,
}

impl PeerEmissionReceipt {
    pub const fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub const fn block_hash(&self) -> BlockHash {
        self.block_hash
    }

    pub const fn evidence_reason(&self) -> CompactAnnouncementReason {
        self.evidence_reason
    }
}

/// Stable preparation outcomes for every active or requested peer session.
#[derive(Debug, PartialEq, Eq)]
pub enum AnnouncementPreparationOutcome {
    Ready(PeerEmission),
    QueueFull {
        peer_id: PeerId,
    },
    Disconnected {
        peer_id: PeerId,
    },
    OutboxUnavailable {
        peer_id: PeerId,
    },
    Ineligible {
        peer_id: PeerId,
    },
    Suppressed {
        peer_id: PeerId,
        reason: CompactAnnouncementReason,
    },
    ConstructionFailed {
        peer_id: PeerId,
    },
}

impl AnnouncementPreparationOutcome {
    pub const fn peer_id(&self) -> PeerId {
        match self {
            Self::Ready(emission) => emission.peer_id(),
            Self::QueueFull { peer_id }
            | Self::Disconnected { peer_id }
            | Self::OutboxUnavailable { peer_id }
            | Self::Ineligible { peer_id }
            | Self::Suppressed { peer_id, .. }
            | Self::ConstructionFailed { peer_id } => *peer_id,
        }
    }
}

pub(super) fn compact_nonces(outboxes: &[PeerOutboxSnapshot]) -> BTreeMap<PeerId, Option<u64>> {
    outboxes
        .iter()
        .map(|outbox| {
            let mut nonce_bytes = [0_u8; 8];
            let maybe_nonce = getrandom::fill(&mut nonce_bytes)
                .ok()
                .map(|()| u64::from_le_bytes(nonce_bytes));
            (outbox.peer_id(), maybe_nonce)
        })
        .collect()
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(crate) fn prepare_block_announcements(
        &mut self,
        block: &Block,
        outboxes: &[PeerOutboxSnapshot],
        compact_nonces: &BTreeMap<PeerId, Option<u64>>,
    ) -> Vec<AnnouncementPreparationOutcome> {
        let active_peer_ids = self.peer_manager.peer_ids();
        let outboxes_by_peer: BTreeMap<_, _> = outboxes
            .iter()
            .map(|snapshot| (snapshot.peer_id(), *snapshot))
            .collect();
        let aggregate_queued_messages = outboxes
            .iter()
            .map(|snapshot| snapshot.queued_messages())
            .sum();
        let mut outcomes = Vec::with_capacity(active_peer_ids.len() + outboxes.len());

        for peer_id in &active_peer_ids {
            outcomes.push(self.prepare_peer_announcement(
                *peer_id,
                block,
                outboxes_by_peer.get(peer_id).copied(),
                aggregate_queued_messages,
                compact_nonces.get(peer_id).copied().flatten(),
            ));
        }
        for outbox in outboxes {
            if !active_peer_ids.contains(&outbox.peer_id()) {
                outcomes.push(AnnouncementPreparationOutcome::Disconnected {
                    peer_id: outbox.peer_id(),
                });
            }
        }

        outcomes
    }

    fn prepare_peer_announcement(
        &mut self,
        peer_id: PeerId,
        block: &Block,
        maybe_outbox: Option<PeerOutboxSnapshot>,
        aggregate_queued_messages: usize,
        maybe_compact_nonce: Option<u64>,
    ) -> AnnouncementPreparationOutcome {
        let Some(outbox) = maybe_outbox else {
            return AnnouncementPreparationOutcome::OutboxUnavailable { peer_id };
        };
        if outbox.is_full() {
            return AnnouncementPreparationOutcome::QueueFull { peer_id };
        }

        let Some(peer) = self.peer_manager.peer_state(peer_id) else {
            return AnnouncementPreparationOutcome::Disconnected { peer_id };
        };
        let handshake_established =
            peer.remote_version_received && peer.local_verack_sent && peer.remote_verack_received;
        if !handshake_established {
            return AnnouncementPreparationOutcome::Ineligible { peer_id };
        }
        let block_hash = block_hash(&block.header);
        let peer_has_previous_header = peer
            .compact_announcements
            .contains(&block.header.previous_block_hash);
        let peer_has_current_header = peer.compact_announcements.contains(&block_hash);
        let (status, gate) = self.announcement_status_and_gate(
            peer_id,
            block_hash,
            outbox,
            aggregate_queued_messages,
        );
        let decision = match self.peer_manager.decide_compact_announcement_for_peer(
            peer_id,
            PeerCompactAnnouncementInput {
                activation: self.block_relay_activation,
                peer_has_previous_header,
                peer_has_current_header,
                status,
                resource_gate: gate,
            },
        ) {
            Ok(decision) => decision,
            Err(_) => return AnnouncementPreparationOutcome::Disconnected { peer_id },
        };
        if decision.action == CompactAnnouncementAction::Suppress {
            return AnnouncementPreparationOutcome::Suppressed {
                peer_id,
                reason: decision.reason,
            };
        }

        let maybe_message = match (decision.action, maybe_compact_nonce) {
            (CompactAnnouncementAction::AnnounceCompactBlock, Some(nonce)) => self
                .peer_manager
                .announce_block_with_action(peer_id, block, decision.action, nonce),
            (CompactAnnouncementAction::AnnounceCompactBlock, None) => {
                self.peer_manager.announce_block(peer_id, block)
            }
            (action, _) => self
                .peer_manager
                .announce_block_with_action(peer_id, block, action, 0),
        };
        let Ok(Some(message)) = maybe_message else {
            return AnnouncementPreparationOutcome::ConstructionFailed { peer_id };
        };
        let evidence_reason =
            block_relay_evidence::compact_announce_evidence_reason(decision, Some(&message));
        AnnouncementPreparationOutcome::Ready(PeerEmission::new(
            peer_id,
            message,
            block_hash,
            evidence_reason,
        ))
    }

    fn announcement_status_and_gate(
        &self,
        peer_id: PeerId,
        block_hash: BlockHash,
        outbox: PeerOutboxSnapshot,
        aggregate_queued_messages: usize,
    ) -> (
        open_bitcoin_network::BlockServingStatusDecision,
        open_bitcoin_network::BlockServingResourceGateDecision,
    ) {
        let request = InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash.into(),
        };
        let input = self.managed_block_serve_input(peer_id, &request, block_hash, false, false);
        let status = classify_block_serving_status(&BlockServingStatusFacts {
            chain_position: input.chain_position,
            validation_state: input.validation_state,
            data_availability: input.data_availability,
            suppressed: input.suppressed,
        });
        let eligibility = classify_block_serving_eligibility(&BlockServingEligibilityInput {
            activation: input.activation,
            inbound_serving_enabled: input.inbound_serving_enabled,
            connection_class: input.connection_class,
            active_permission_effects: input.active_permission_effects.clone(),
            inactive_permission_effects: input.inactive_permission_effects.clone(),
            status_available: status.may_serve_block,
        });
        let gate = evaluate_block_serving_resource_gate(
            &ResourceGovernancePolicy::default(),
            BlockServingResourceGateInput {
                eligibility,
                status,
                queue_pressure: QueuePressureInput {
                    peer_queued_messages: outbox.queued_messages(),
                    aggregate_queued_messages,
                    active_permission_effects: input.active_permission_effects.clone(),
                    inactive_permission_effects: input.inactive_permission_effects.clone(),
                    ..Default::default()
                },
                request_pressure: RequestPressureInput {
                    requested_blocks_in_flight: input.requested_blocks_in_flight,
                    requested_txids_in_flight: input.requested_txids_in_flight,
                    requested_wtxids_in_flight: input.requested_wtxids_in_flight,
                    active_permission_effects: input.active_permission_effects,
                    inactive_permission_effects: input.inactive_permission_effects,
                    ..Default::default()
                },
                maybe_timeout: None,
                maybe_churn: None,
                maybe_repeated_failure: None,
                reconnect: ReconnectSuppressionInput::default(),
                maybe_cleanup: None,
            },
        );
        (status, gate)
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::primitives::BlockHash;
    use open_bitcoin_network::{
        CompactAnnouncementReason, PHASE94_MAX_PEER_QUEUED_MESSAGES, PeerId, WireNetworkMessage,
    };

    use super::{PeerEmission, PeerOutboxSnapshot};

    #[test]
    fn announcement_transport_emission_binds_peer_message_block_and_evidence() {
        // Arrange
        let peer_id: PeerId = 128_201;
        let block_hash = BlockHash::from_byte_array([0x21; 32]);
        let emission = PeerEmission::new(
            peer_id,
            WireNetworkMessage::Verack,
            block_hash,
            CompactAnnouncementReason::CompactInventoryFallback,
        );

        // Act
        let (actual_peer_id, message, receipt) = emission.into_parts();

        // Assert
        assert_eq!(actual_peer_id, peer_id);
        assert_eq!(message, WireNetworkMessage::Verack);
        assert_eq!(receipt.block_hash(), block_hash);
    }

    #[test]
    fn announcement_transport_outbox_snapshot_fails_closed_at_the_cap() {
        // Arrange
        let snapshot = PeerOutboxSnapshot::new(
            128_202,
            PHASE94_MAX_PEER_QUEUED_MESSAGES,
            PHASE94_MAX_PEER_QUEUED_MESSAGES,
        );

        // Act
        let is_full = snapshot.is_full();

        // Assert
        assert!(is_full);
    }
}
