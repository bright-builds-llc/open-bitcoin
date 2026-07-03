// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::collections::BTreeSet;

use open_bitcoin_consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Block, BlockHash, BlockLocator, InventoryType, InventoryVector, Transaction, Txid, Wtxid,
};

use crate::error::{NetworkError, PeerId};
use crate::message::{HeadersMessage, InventoryList, WireNetworkMessage};
use crate::{
    InactivePermissionEffectLabel, PermissionEffectLabel, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy,
};

use super::relay_download::relay_download_eligibility;
use super::{
    PeerAction, PeerManager, PeerState, TxAnnouncementInput, TxDownloadAction,
    TxDownloadLocalFacts, TxParentRequestInput, TxPeerRequestSnapshot, TxRelayId, TxRelayPeerMode,
};

impl PeerManager {
    pub(super) fn handle_getheaders(
        &self,
        peer_id: PeerId,
        locator: BlockLocator,
        stop_hash: BlockHash,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let input = request_pressure_input(
            peer,
            0,
            0,
            locator.block_hashes.len(),
            peer.requested_blocks.len(),
            self.tx_download.peer_snapshot(peer_id).in_flight_count,
            0,
        );
        if let Some(actions) = resource_limit_disconnect_actions(input) {
            return Ok(actions);
        }

        let headers =
            self.headers
                .headers_after_locator(&locator, stop_hash, crate::MAX_HEADERS_RESULTS);
        Ok(vec![PeerAction::Send(WireNetworkMessage::Headers(
            HeadersMessage { headers },
        ))])
    }

    pub(super) fn handle_getdata(
        &self,
        peer_id: PeerId,
        inventory: InventoryList,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let input = request_pressure_input(
            peer,
            0,
            inventory.inventory.len(),
            0,
            peer.requested_blocks.len(),
            self.tx_download.peer_snapshot(peer_id).in_flight_count,
            0,
        );
        if let Some(actions) = resource_limit_disconnect_actions(input) {
            return Ok(actions);
        }

        Ok(vec![PeerAction::ServeInventory(
            inventory
                .inventory
                .into_iter()
                .map(typed_serve_inventory_vector)
                .collect(),
        )])
    }

    pub(super) fn handle_inventory(
        &mut self,
        peer_id: PeerId,
        inventory: InventoryList,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let locator = self.headers.locator();
        let mut transaction_inputs = Vec::new();
        let mut transaction_candidate_count = 0;
        let mut request_headers = false;

        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let peer_mode = TxRelayPeerMode::from_remote_wtxidrelay(peer.remote_wtxidrelay);
        let relay_eligibility = relay_download_eligibility(peer, self.relay_download_policy);
        let local_facts = self.transaction_download_local_facts();

        for item in &inventory.inventory {
            match item.inventory_type {
                InventoryType::Block | InventoryType::WitnessBlock => {
                    let block_hash = BlockHash::from(item.object_hash);
                    if !self.known_blocks.contains(&block_hash) {
                        request_headers = true;
                    }
                }
                InventoryType::Transaction | InventoryType::WitnessTransaction => {
                    transaction_candidate_count += 1;
                    transaction_inputs.push(TxAnnouncementInput {
                        peer_id,
                        inventory: item.clone(),
                        peer_mode,
                        now_unix_seconds: timestamp,
                        local_facts: local_facts.clone(),
                        relay_eligibility: relay_eligibility.clone(),
                        preferred_peer: true,
                        peer_overloaded: false,
                    });
                }
                _ => {}
            }
        }

        let input = request_pressure_input(
            peer,
            inventory.inventory.len(),
            0,
            0,
            peer.requested_blocks.len(),
            self.tx_download
                .peer_snapshot(peer_id)
                .in_flight_count
                .saturating_add(transaction_candidate_count),
            0,
        );
        if let Some(actions) = resource_limit_disconnect_actions(input) {
            return Ok(actions);
        }

        let mut transaction_actions = Vec::new();
        for input in transaction_inputs {
            transaction_actions.extend(self.tx_download.record_announcement(input));
        }

        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        let mut actions = Vec::new();
        if request_headers && !peer.getheaders_in_flight {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            actions.push(PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            }));
        }
        actions.extend(handle_transaction_relay_actions(transaction_actions));
        Ok(actions)
    }

    pub(super) fn handle_notfound(
        &mut self,
        peer_id: PeerId,
        inventory: InventoryList,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if !self.peers.contains_key(&peer_id) {
            return Err(NetworkError::UnknownPeer(peer_id));
        }

        let mut transaction_actions = Vec::new();
        for item in inventory.inventory {
            match item.inventory_type {
                InventoryType::Block | InventoryType::WitnessBlock => {
                    let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                    peer.requested_blocks
                        .remove(&BlockHash::from(item.object_hash));
                }
                InventoryType::Transaction => {
                    transaction_actions.extend(self.tx_download.record_notfound(
                        peer_id,
                        TxRelayId::Txid(Txid::from(item.object_hash)),
                        timestamp,
                    ));
                }
                InventoryType::WitnessTransaction => {
                    transaction_actions.extend(self.tx_download.record_notfound(
                        peer_id,
                        TxRelayId::Wtxid(Wtxid::from(item.object_hash)),
                        timestamp,
                    ));
                }
                _ => {}
            }
        }

        Ok(handle_transaction_relay_actions(transaction_actions))
    }

    pub fn request_missing_blocks(
        &mut self,
        peer_id: PeerId,
        block_hashes: &[BlockHash],
    ) -> Result<Option<WireNetworkMessage>, NetworkError> {
        let known_blocks = self.known_blocks.clone();
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        if !peer.remote_verack_received || !peer.local_verack_sent {
            return Ok(None);
        }

        let available_slots = self
            .max_blocks_in_flight_per_peer
            .saturating_sub(peer.requested_blocks.len());
        if available_slots == 0 {
            return Ok(None);
        }

        let mut inventory = Vec::new();
        for block_hash in block_hashes.iter().copied() {
            if inventory.len() >= available_slots {
                break;
            }
            if known_blocks.contains(&block_hash) || peer.requested_blocks.contains(&block_hash) {
                continue;
            }
            peer.requested_blocks.insert(block_hash);
            inventory.push(InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: block_hash.into(),
            });
        }

        if inventory.is_empty() {
            return Ok(None);
        }

        Ok(Some(WireNetworkMessage::GetData(InventoryList::new(
            inventory,
        ))))
    }

    pub(super) fn handle_transaction(
        &mut self,
        peer_id: PeerId,
        transaction: Transaction,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if !self.peers.contains_key(&peer_id) {
            return Err(NetworkError::UnknownPeer(peer_id));
        }
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;

        let transaction_actions = self
            .tx_download
            .record_received_transaction(peer_id, txid, wtxid);
        let should_suppress_received_transaction = transaction_actions
            .iter()
            .any(|action| matches!(action, TxDownloadAction::SuppressIdentityMismatch { .. }));
        let mut actions = handle_transaction_relay_actions(transaction_actions);
        if !should_suppress_received_transaction {
            actions.push(PeerAction::ReceivedTransaction(transaction));
        }

        Ok(actions)
    }

    pub(super) fn handle_block(
        &mut self,
        peer_id: PeerId,
        block: Block,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let hash = block_hash(&block.header);

        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.requested_blocks.remove(&hash);

        Ok(vec![PeerAction::ReceivedBlock(block)])
    }
}

pub(super) fn handle_transaction_relay_actions(actions: Vec<TxDownloadAction>) -> Vec<PeerAction> {
    actions
        .into_iter()
        .map(PeerAction::TransactionRelay)
        .collect()
}

fn typed_serve_inventory_vector(item: InventoryVector) -> InventoryVector {
    match item.inventory_type {
        InventoryType::Transaction => {
            TxRelayId::Txid(Txid::from(item.object_hash)).to_inventory_vector()
        }
        InventoryType::WitnessTransaction => {
            TxRelayId::Wtxid(Wtxid::from(item.object_hash)).to_inventory_vector()
        }
        _ => item,
    }
}

impl PeerManager {
    pub fn expire_transaction_requests(
        &mut self,
        now_unix_seconds: i64,
    ) -> Vec<(PeerId, PeerAction)> {
        self.tx_download
            .expire_and_schedule(now_unix_seconds)
            .into_iter()
            .map(|action| (action.peer_id(), PeerAction::TransactionRelay(action)))
            .collect()
    }

    pub fn request_orphan_parent(
        &mut self,
        peer_id: PeerId,
        parent_txid: Txid,
        now_unix_seconds: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        self.request_orphan_parent_relay(peer_id, TxRelayId::Txid(parent_txid), now_unix_seconds)
    }

    pub fn remove_peer_with_transaction_cleanup(
        &mut self,
        peer_id: PeerId,
        now_unix_seconds: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let Some(_) = self.peers.remove(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        Ok(handle_transaction_relay_actions(
            self.tx_download.cleanup_peer(peer_id, now_unix_seconds),
        ))
    }

    pub fn remove_peer(&mut self, peer_id: PeerId) -> Result<(), NetworkError> {
        self.remove_peer_with_transaction_cleanup(peer_id, 0)?;
        Ok(())
    }

    pub fn transaction_request_snapshot(&self, peer_id: PeerId) -> TxPeerRequestSnapshot {
        self.tx_download.peer_snapshot(peer_id)
    }

    pub fn note_mempool_known(&mut self, relay_id: TxRelayId) {
        self.mempool_known.insert(relay_id);
    }

    pub(super) fn request_orphan_parent_relay(
        &mut self,
        peer_id: PeerId,
        relay_id: TxRelayId,
        now_unix_seconds: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let relay_eligibility = relay_download_eligibility(peer, self.relay_download_policy);

        Ok(handle_transaction_relay_actions(
            self.tx_download.request_parent(TxParentRequestInput {
                peer_id,
                relay_id,
                now_unix_seconds,
                local_facts: self.transaction_download_local_facts(),
                relay_eligibility,
            }),
        ))
    }

    pub(super) fn transaction_download_local_facts(&self) -> TxDownloadLocalFacts {
        let mut already_have = BTreeSet::new();
        already_have.extend(self.known_txids.iter().copied().map(TxRelayId::Txid));
        already_have.extend(self.known_wtxids.iter().copied().map(TxRelayId::Wtxid));

        TxDownloadLocalFacts {
            already_have,
            recent_rejects: self.recent_rejects.clone(),
            mempool_known: self.mempool_known.clone(),
        }
    }
}

pub(super) fn permission_effect_vectors(
    peer: &PeerState,
) -> (
    Vec<PermissionEffectLabel>,
    Vec<InactivePermissionEffectLabel>,
) {
    let Some(record) = peer.maybe_inbound_record.as_ref() else {
        return (Vec::new(), Vec::new());
    };
    (
        record.permission_decision.active_effects().to_vec(),
        record.permission_decision.inactive_effects().to_vec(),
    )
}

fn request_pressure_input(
    peer: &PeerState,
    inventory_items: usize,
    getdata_items: usize,
    header_locator_hashes: usize,
    block_in_flight_count: usize,
    txid_in_flight_count: usize,
    wtxid_in_flight_count: usize,
) -> RequestPressureInput {
    let (active_permission_effects, inactive_permission_effects) = permission_effect_vectors(peer);
    RequestPressureInput {
        inventory_items,
        getdata_items,
        header_locator_hashes,
        requested_blocks_in_flight: block_in_flight_count,
        requested_txids_in_flight: txid_in_flight_count,
        requested_wtxids_in_flight: wtxid_in_flight_count,
        active_permission_effects,
        inactive_permission_effects,
    }
}

fn resource_limit_disconnect_actions(input: RequestPressureInput) -> Option<Vec<PeerAction>> {
    resource_limit_disconnect_actions_from_decision(
        ResourceGovernancePolicy::default().decide_request(input),
    )
}

pub(super) fn resource_limit_disconnect_actions_from_decision(
    decision: ResourceGovernanceDecision,
) -> Option<Vec<PeerAction>> {
    match decision {
        ResourceGovernanceDecision::Accept => None,
        ResourceGovernanceDecision::Backpressure(event)
        | ResourceGovernanceDecision::Disconnect(event)
        | ResourceGovernanceDecision::RecordMisbehavior(event) => {
            Some(vec![PeerAction::ResourceGovernanceDisconnect(event)])
        }
    }
}
