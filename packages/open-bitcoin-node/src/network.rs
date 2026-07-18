// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::collections::{BTreeMap, BTreeSet};

mod action_translation;
mod admission_bridge;
mod block_relay_evidence;
mod block_serving;
mod compact_receive_candidates;
mod header_sync;
mod inbound;
mod inventory;
mod mempool_lifecycle;
mod peer_policy;
mod recovery;
mod relay_fanout;
mod relay_serving;
mod types;

pub(crate) use block_relay_evidence::BlockRelayRuntimeEvidenceSnapshot;

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainTransition, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags, block_hash},
    primitives::{Block, BlockHash, Transaction, Txid, Wtxid},
};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, CompactAnnouncementAction, CompactAnnouncementDecision,
    ConnectionRole, HeaderEntry, HeaderStore, HeaderSyncPolicy, HeadersMessage,
    InboundAdmissionPolicy, InboundResourceEvent, LocalAdvertisementDecision, LocalPeerConfig,
    PROTOCOL_VERSION, ParsedNetworkMessage, PeerAction, PeerId, PeerManager, RelayActivationConfig,
    TxOrphanage, WireNetworkMessage,
};

use crate::{ChainstateStore, ManagedChainstate, ManagedMempool};
use header_sync::validate_header_for_sync;
use inbound::is_active_inbound_peer;

pub use inbound::{
    ManagedAddressBoundaryInfo, ManagedInboundAdmissionInfo, ManagedInboundPermissionDecisionInfo,
    ManagedPeerPolicyInfo, ManagedResourceGovernanceInfo,
};
pub use recovery::ManagedMempoolRecoverySummary;
pub use relay_fanout::{
    LocalRelaySubmissionEvidence, LocalRelaySubmissionLabel, ManagedRelayFanoutInfo,
    RebroadcastEvidenceLabel,
};
pub use types::{
    BlockConnectDisposition, ManagedMempoolInfo, ManagedNetworkError, ManagedNetworkInfo,
    ManagedSyncMessageResult,
};

type ManagedResult<T> = Result<T, ManagedNetworkError>;

#[derive(Debug, Clone)]
pub struct ManagedPeerNetwork<S> {
    chainstate: ManagedChainstate<S>,
    mempool: ManagedMempool,
    peer_manager: PeerManager,
    orphanage: TxOrphanage,
    known_peers: BTreeSet<PeerId>,
    inbound_admission_policy: InboundAdmissionPolicy,
    inbound_admission_info: ManagedInboundAdmissionInfo,
    resource_governance_info: ManagedResourceGovernanceInfo,
    relay_activation: RelayActivationConfig,
    block_relay_activation: BlockRelayActivationPolicy,
    inbound_serving_enabled: bool,
    block_relay_evidence: block_relay_evidence::ManagedBlockRelayEvidenceState,
    relay_fanout: relay_fanout::ManagedRelayFanoutState,
    relay_serving: relay_serving::RelayServingCache,
    compact_extra_txn: compact_receive_candidates::CompactExtraTxnBuffer,
    latest_mempool_recovery: Option<ManagedMempoolRecoverySummary>,
    latest_mempool_recovery_storage_error: Option<crate::status::SyncRecoveryCategory>,
    local_config: LocalPeerConfig,
    blocks_by_hash: BTreeMap<BlockHash, Block>,
    transactions_by_txid: BTreeMap<Txid, Transaction>,
    transactions_by_wtxid: BTreeMap<Wtxid, Transaction>,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    #[rustfmt::skip]
    pub fn chainstate(&self) -> &ManagedChainstate<S> { &self.chainstate }

    #[rustfmt::skip]
    pub fn mempool(&self) -> &ManagedMempool { &self.mempool }

    #[rustfmt::skip]
    pub fn peer_manager(&self) -> &PeerManager { &self.peer_manager }

    #[rustfmt::skip]
    pub fn peer_manager_mut(&mut self) -> &mut PeerManager { &mut self.peer_manager }

    pub fn set_local_address_decisions(&mut self, decisions: Vec<LocalAdvertisementDecision>) {
        self.peer_manager.set_local_address_decisions(decisions);
    }

    pub fn address_boundary_info(&self) -> ManagedAddressBoundaryInfo {
        self.peer_manager.address_boundary_evidence().into()
    }

    #[rustfmt::skip]
    pub fn resource_governance_info(&self) -> ManagedResourceGovernanceInfo { self.resource_governance_info.clone() }

    #[rustfmt::skip]
    pub fn record_resource_governance_event(&mut self, event: InboundResourceEvent) { self.resource_governance_info.record_event(event); }

    #[rustfmt::skip]
    pub fn seed_header_store(&mut self, header_store: HeaderStore) { self.peer_manager.seed_header_store(header_store); }

    pub fn header_entries(&self) -> Vec<HeaderEntry> {
        self.peer_manager
            .header_store()
            .entries()
            .cloned()
            .collect()
    }

    #[rustfmt::skip]
    pub fn best_chain_entries(&self) -> Vec<HeaderEntry> { self.peer_manager.header_store().best_chain_entries() }

    pub fn chainstate_snapshot(&self) -> ChainstateSnapshot {
        self.chainstate.chainstate().snapshot()
    }

    #[rustfmt::skip]
    pub fn maybe_chain_tip(&self) -> Option<ChainPosition> { self.chainstate.chainstate().tip().cloned() }

    #[rustfmt::skip]
    pub fn note_local_block_hash(&mut self, block_hash: BlockHash) { self.peer_manager.note_local_block_hash(block_hash); }

    #[rustfmt::skip]
    pub fn peer_requested_blocks(&self, peer_id: PeerId) -> ManagedResult<Vec<BlockHash>> { self.peer_manager.peer_requested_blocks(peer_id).map_err(ManagedNetworkError::from) }

    #[rustfmt::skip]
    pub fn request_missing_blocks(&mut self, peer_id: PeerId, block_hashes: &[BlockHash]) -> ManagedResult<Vec<WireNetworkMessage>> {
        let maybe_message = self.peer_manager.request_missing_blocks(peer_id, block_hashes)?;
        Ok(maybe_message.into_iter().collect())
    }

    pub fn mempool_info(&self) -> ManagedMempoolInfo {
        let entries = self.mempool.mempool().entries();
        let total_fee_sats = entries.values().map(|entry| entry.fee_sats()).sum();
        let pressure = self.mempool.mempool().pressure_summary();

        ManagedMempoolInfo {
            transaction_count: pressure.transaction_count,
            total_virtual_size: pressure.total_virtual_size,
            total_fee_sats,
            min_relay_feerate_sats_per_kvb: pressure.min_relay_feerate_sats_per_kvb,
            incremental_relay_feerate_sats_per_kvb: pressure.incremental_relay_feerate_sats_per_kvb,
            max_mempool_virtual_size: pressure.max_mempool_virtual_size,
            capacity_status: pressure.capacity_status,
            rolling_fee_parity: pressure.rolling_fee_parity,
        }
    }

    pub fn network_info(&self) -> ManagedNetworkInfo {
        let mut connected_peers = 0;
        let mut inbound_peers = 0;
        let mut outbound_peers = 0;
        let mut wtxidrelay_peers = 0;
        let mut header_preferring_peers = 0;

        for peer_id in &self.known_peers {
            let Some(peer) = self.peer_manager.peer_state(*peer_id) else {
                continue;
            };

            match peer.role {
                ConnectionRole::Inbound => {
                    if !is_active_inbound_peer(peer) {
                        continue;
                    }
                    connected_peers += 1;
                    inbound_peers += 1;
                }
                ConnectionRole::Outbound => {
                    connected_peers += 1;
                    outbound_peers += 1;
                }
            }
            if peer.remote_wtxidrelay {
                wtxidrelay_peers += 1;
            }
            if peer.remote_prefers_headers {
                header_preferring_peers += 1;
            }
        }

        ManagedNetworkInfo {
            network_magic: self.local_config.magic,
            protocol_version: PROTOCOL_VERSION,
            user_agent: self.local_config.user_agent.clone(),
            local_services_bits: self.local_config.services.bits(),
            relay: self.relay_activation.enabled,
            connected_peers,
            inbound_peers,
            outbound_peers,
            wtxidrelay_peers,
            header_preferring_peers,
        }
    }

    pub fn connect_outbound_peer(
        &mut self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let actions = self.peer_manager.add_outbound_peer(peer_id, timestamp)?;
        self.known_peers.insert(peer_id);
        self.collect_outbound(actions)
    }

    pub fn receive_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        let observed_block_relay_message = matches!(
            &message,
            WireNetworkMessage::SendCompact(_)
                | WireNetworkMessage::CompactBlock(_)
                | WireNetworkMessage::BlockTxn(_)
        );
        let actions = match message {
            WireNetworkMessage::CompactBlock(payload) => {
                self.handle_compact_block_receive(peer_id, payload, timestamp)?
            }
            other => self
                .peer_manager
                .handle_message(peer_id, other, timestamp)?,
        };
        if observed_block_relay_message {
            self.note_block_relay_observed();
            self.record_compact_download_evidence(&actions);
        }
        let mut result =
            self.process_actions(peer_id, actions, timestamp, verify_flags, consensus_params)?;
        let expired = self.expire_compact_download_timeouts(timestamp)?;
        merge_compact_timeout_outbound(peer_id, expired, &mut result);
        Ok(result)
    }

    pub fn receive_sync_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        let actions = match message {
            WireNetworkMessage::Headers(headers_message) => {
                self.handle_headers_message(peer_id, headers_message, timestamp, consensus_params)?
            }
            WireNetworkMessage::CompactBlock(payload) => {
                let actions = self.handle_compact_block_receive(peer_id, payload, timestamp)?;
                self.note_block_relay_observed();
                self.record_compact_download_evidence(&actions);
                actions
            }
            other => {
                let observed_block_relay_message = matches!(
                    &other,
                    WireNetworkMessage::SendCompact(_) | WireNetworkMessage::BlockTxn(_)
                );
                let actions = self
                    .peer_manager
                    .handle_message(peer_id, other, timestamp)?;
                if observed_block_relay_message {
                    self.note_block_relay_observed();
                    self.record_compact_download_evidence(&actions);
                }
                actions
            }
        };
        let mut result =
            self.process_actions(peer_id, actions, timestamp, verify_flags, consensus_params)?;
        let expired = self.expire_compact_download_timeouts(timestamp)?;
        merge_compact_timeout_outbound(peer_id, expired, &mut result);
        Ok(result)
    }

    pub fn receive_wire_message(
        &mut self,
        peer_id: PeerId,
        bytes: &[u8],
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        let parsed = ParsedNetworkMessage::decode_wire(bytes)?;
        self.receive_message(
            peer_id,
            parsed.message,
            timestamp,
            verify_flags,
            consensus_params,
        )
    }

    pub fn encode_messages(
        &self,
        messages: &[WireNetworkMessage],
    ) -> Result<Vec<Vec<u8>>, ManagedNetworkError> {
        messages
            .iter()
            .map(|message| message.encode_wire(self.local_config.magic))
            .collect::<Result<Vec<_>, _>>()
            .map_err(ManagedNetworkError::from)
    }

    pub fn connect_local_block(
        &mut self,
        block: &Block,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ManagedNetworkError> {
        let position = self.chainstate.connect_block(
            block,
            self.next_chain_work(),
            verify_flags,
            consensus_params,
        )?;
        self.blocks_by_hash
            .insert(position.block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        self.apply_connected_block_mempool_lifecycle(block)?;
        Ok(position)
    }

    pub fn announce_block(
        &mut self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError> {
        let block_hash = block_hash(&block.header);
        let request = open_bitcoin_core::primitives::InventoryVector {
            inventory_type: open_bitcoin_core::primitives::InventoryType::Block,
            object_hash: block_hash.into(),
        };
        let input = self.managed_block_serve_input(peer_id, &request, block_hash, false);
        let status = open_bitcoin_network::classify_block_serving_status(
            &open_bitcoin_network::BlockServingStatusFacts {
                chain_position: input.chain_position,
                validation_state: input.validation_state,
                data_availability: input.data_availability,
                suppressed: input.suppressed,
            },
        );
        let eligibility = open_bitcoin_network::classify_block_serving_eligibility(
            &open_bitcoin_network::BlockServingEligibilityInput {
                activation: input.activation,
                inbound_serving_enabled: input.inbound_serving_enabled,
                connection_class: input.connection_class,
                active_permission_effects: input.active_permission_effects.clone(),
                inactive_permission_effects: input.inactive_permission_effects.clone(),
                status_available: status.may_serve_block,
            },
        );
        let gate = open_bitcoin_network::evaluate_block_serving_resource_gate(
            &open_bitcoin_network::ResourceGovernancePolicy::default(),
            open_bitcoin_network::BlockServingResourceGateInput {
                eligibility,
                status,
                queue_pressure: open_bitcoin_network::QueuePressureInput {
                    active_permission_effects: input.active_permission_effects.clone(),
                    inactive_permission_effects: input.inactive_permission_effects.clone(),
                    ..Default::default()
                },
                request_pressure: open_bitcoin_network::RequestPressureInput {
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
                reconnect: open_bitcoin_network::ReconnectSuppressionInput::default(),
                maybe_cleanup: None,
            },
        );
        let announcement = self.peer_manager.decide_compact_announcement_for_peer(
            peer_id,
            open_bitcoin_network::PeerCompactAnnouncementInput {
                activation: self.block_relay_activation,
                peer_has_previous_header: true,
                peer_has_current_header: false,
                status,
                resource_gate: gate,
            },
        )?;
        self.announce_block_with_nonce(peer_id, block, announcement, || {
            let mut nonce_bytes = [0_u8; 8];
            getrandom::fill(&mut nonce_bytes)?;
            Ok::<u64, getrandom::Error>(u64::from_le_bytes(nonce_bytes))
        })
    }

    fn announce_block_with_nonce<F, E>(
        &mut self,
        peer_id: PeerId,
        block: &Block,
        announcement: CompactAnnouncementDecision,
        compact_nonce: F,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError>
    where
        F: FnOnce() -> Result<u64, E>,
    {
        let maybe_message = match announcement.action {
            CompactAnnouncementAction::AnnounceCompactBlock => match compact_nonce() {
                Ok(nonce) => self.peer_manager.announce_block_with_action(
                    peer_id,
                    block,
                    announcement.action,
                    nonce,
                )?,
                Err(_) => self.peer_manager.announce_block(peer_id, block)?,
            },
            action => self
                .peer_manager
                .announce_block_with_action(peer_id, block, action, 0)?,
        };
        if matches!(maybe_message, Some(WireNetworkMessage::CompactBlock(_))) {
            let block_hash = block_hash(&block.header);
            self.peer_manager
                .record_compact_block_announcement(peer_id, block_hash)?;
        }
        let evidence_reason = block_relay_evidence::compact_announce_evidence_reason(
            announcement,
            maybe_message.as_ref(),
        );
        self.record_compact_announcement_evidence(evidence_reason);
        Ok(maybe_message)
    }

    pub fn announce_transaction(
        &self,
        peer_id: PeerId,
        transaction: &Transaction,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError> {
        self.peer_manager
            .announce_transaction(peer_id, transaction)
            .map_err(ManagedNetworkError::from)
    }

    pub fn connect_stored_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<BlockConnectDisposition> {
        let block_hash = block_hash(&block.header);
        if self
            .chainstate
            .chainstate()
            .snapshot()
            .active_chain
            .iter()
            .any(|position| position.block_hash == block_hash)
        {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::Duplicate(block_hash));
        }

        let maybe_tip = self.chainstate.chainstate().tip().cloned();
        let extends_tip = maybe_tip
            .as_ref()
            .is_none_or(|tip| tip.block_hash == block.header.previous_block_hash);
        let is_genesis = block.header.previous_block_hash.to_byte_array() == [0_u8; 32];
        if maybe_tip.is_some() && !extends_tip {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::NonExtending {
                block_hash,
                previous_block_hash: block.header.previous_block_hash,
            });
        }
        if maybe_tip.is_none() && !is_genesis {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::Disconnected { block_hash });
        }

        let position = self.chainstate.connect_block_with_current_time(
            block,
            chain_work,
            timestamp,
            verify_flags,
            consensus_params,
        )?;
        self.blocks_by_hash.insert(block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        self.apply_connected_block_mempool_lifecycle(block)?;
        Ok(BlockConnectDisposition::Connected(position))
    }

    pub fn reorg_to_branch(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<ChainTransition> {
        let transition = self.chainstate.reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        for anchored_block in replacement_branch {
            let block_hash = block_hash(&anchored_block.block.header);
            self.blocks_by_hash
                .insert(block_hash, anchored_block.block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
        }
        for position in &transition.connected {
            self.peer_manager.note_local_position(position);
        }
        self.apply_reorg_mempool_lifecycle(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        Ok(transition)
    }

    fn handle_headers_message(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
        timestamp: i64,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<PeerAction>, ManagedNetworkError> {
        self.peer_manager
            .handle_headers_with_policy(
                peer_id,
                headers_message,
                HeaderSyncPolicy::HeadersOnly,
                |header_store, header| {
                    validate_header_for_sync(header_store, header, timestamp, consensus_params)?;
                    header_store.insert_header(header.clone())
                },
            )
            .map_err(ManagedNetworkError::from)
    }
}

fn merge_compact_timeout_outbound(
    peer_id: PeerId,
    expired: Vec<(PeerId, WireNetworkMessage)>,
    result: &mut ManagedSyncMessageResult,
) {
    for (expire_peer_id, message) in expired {
        if expire_peer_id == peer_id {
            result.outbound.push(message);
        } else {
            result.targeted_outbound.push((expire_peer_id, message));
        }
    }
}

#[cfg(test)]
mod tests;
