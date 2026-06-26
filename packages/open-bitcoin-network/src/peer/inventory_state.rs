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
    Block, BlockHash, Hash32, InventoryType, InventoryVector, Transaction, Txid, Wtxid,
};

use crate::error::{NetworkError, PeerId};
use crate::message::{InventoryList, WireNetworkMessage};

use super::{PeerAction, PeerManager, PeerState};

impl PeerManager {
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
