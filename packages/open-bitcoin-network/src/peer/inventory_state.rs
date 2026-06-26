// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use open_bitcoin_consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Block, BlockHash, BlockLocator, Hash32, InventoryType, InventoryVector, Transaction, Txid,
    Wtxid,
};

use crate::error::{DisconnectReason, NetworkError, PeerId};
use crate::message::{HeadersMessage, InventoryList, WireNetworkMessage};
use crate::{
    InactivePermissionEffectLabel, PermissionEffectLabel, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy,
};

use super::{PeerAction, PeerManager, PeerState};

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
            peer.requested_txids.len(),
            peer.requested_wtxids.len(),
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
            peer.requested_txids.len(),
            peer.requested_wtxids.len(),
        );
        if let Some(actions) = resource_limit_disconnect_actions(input) {
            return Ok(actions);
        }

        Ok(vec![PeerAction::ServeInventory(inventory.inventory)])
    }

    pub(super) fn handle_inventory(
        &mut self,
        peer_id: PeerId,
        inventory: InventoryList,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let locator = self.headers.locator();
        let mut tx_requests = Vec::new();
        let mut candidate_txids = std::collections::BTreeSet::new();
        let mut candidate_wtxids = std::collections::BTreeSet::new();
        let mut request_headers = false;

        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;

        for item in &inventory.inventory {
            match item.inventory_type {
                InventoryType::Block | InventoryType::WitnessBlock => {
                    let block_hash = BlockHash::from(item.object_hash);
                    if !self.known_blocks.contains(&block_hash) {
                        request_headers = true;
                    }
                }
                InventoryType::Transaction => {
                    let txid = Txid::from(item.object_hash);
                    if !peer.remote_wtxidrelay
                        && !self.known_txids.contains(&txid)
                        && !peer.requested_txids.contains(&txid)
                        && candidate_txids.insert(txid)
                    {
                        tx_requests.push(item.clone());
                    }
                }
                InventoryType::WitnessTransaction => {
                    let wtxid = Wtxid::from(item.object_hash);
                    if peer.remote_wtxidrelay
                        && !self.known_wtxids.contains(&wtxid)
                        && !peer.requested_wtxids.contains(&wtxid)
                        && candidate_wtxids.insert(wtxid)
                    {
                        tx_requests.push(item.clone());
                    }
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
            peer.requested_txids
                .len()
                .saturating_add(candidate_txids.len()),
            peer.requested_wtxids
                .len()
                .saturating_add(candidate_wtxids.len()),
        );
        if let Some(actions) = resource_limit_disconnect_actions(input) {
            return Ok(actions);
        }

        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.requested_txids.extend(candidate_txids);
        peer.requested_wtxids.extend(candidate_wtxids);

        let mut actions = Vec::new();
        if request_headers && !peer.getheaders_in_flight {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            actions.push(PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            }));
        }
        if !tx_requests.is_empty() {
            actions.push(PeerAction::Send(WireNetworkMessage::GetData(
                InventoryList::new(tx_requests),
            )));
        }
        Ok(actions)
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
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;
        self.known_txids.insert(txid);
        self.known_wtxids.insert(wtxid);

        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        forget_requested_inventory(peer, InventoryType::Transaction, txid.into());
        forget_requested_inventory(peer, InventoryType::WitnessTransaction, wtxid.into());

        Ok(vec![PeerAction::ReceivedTransaction(transaction)])
    }

    pub(super) fn handle_block(
        &mut self,
        peer_id: PeerId,
        block: Block,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let hash = block_hash(&block.header);

        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        forget_requested_inventory(peer, InventoryType::Block, hash.into());

        Ok(vec![PeerAction::ReceivedBlock(block)])
    }
}

pub(super) fn forget_requested_inventory(
    peer: &mut PeerState,
    inventory_type: InventoryType,
    object_hash: Hash32,
) {
    match inventory_type {
        InventoryType::Block | InventoryType::WitnessBlock => {
            peer.requested_blocks.remove(&BlockHash::from(object_hash));
        }
        InventoryType::Transaction => {
            peer.requested_txids.remove(&Txid::from(object_hash));
        }
        InventoryType::WitnessTransaction => {
            peer.requested_wtxids.remove(&Wtxid::from(object_hash));
        }
        _ => {}
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
    requested_blocks_in_flight: usize,
    requested_txids_in_flight: usize,
    requested_wtxids_in_flight: usize,
) -> RequestPressureInput {
    let (active_permission_effects, inactive_permission_effects) = permission_effect_vectors(peer);
    RequestPressureInput {
        inventory_items,
        getdata_items,
        header_locator_hashes,
        requested_blocks_in_flight,
        requested_txids_in_flight,
        requested_wtxids_in_flight,
        active_permission_effects,
        inactive_permission_effects,
    }
}

fn resource_limit_disconnect_actions(input: RequestPressureInput) -> Option<Vec<PeerAction>> {
    match ResourceGovernancePolicy::default().decide_request(input) {
        ResourceGovernanceDecision::Accept => None,
        ResourceGovernanceDecision::Backpressure(_)
        | ResourceGovernanceDecision::Disconnect(_)
        | ResourceGovernanceDecision::RecordMisbehavior(_) => Some(vec![PeerAction::Disconnect(
            DisconnectReason::ResourceLimit,
        )]),
    }
}
