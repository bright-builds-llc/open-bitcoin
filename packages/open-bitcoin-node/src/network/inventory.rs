// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_core::{
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{BlockHash, InventoryType, InventoryVector, Transaction, Txid, Wtxid},
};
use open_bitcoin_network::{DisconnectReason, NetworkError, PeerId, WireNetworkMessage};

use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn serve_inventory(
        &self,
        requests: Vec<InventoryVector>,
    ) -> (Vec<WireNetworkMessage>, Vec<InventoryVector>) {
        let mut messages = Vec::new();
        let mut missing = Vec::new();

        for request in requests {
            match request.inventory_type {
                InventoryType::Block | InventoryType::WitnessBlock => {
                    let block_hash = BlockHash::from(request.object_hash);
                    let Some(block) = self.blocks_by_hash.get(&block_hash) else {
                        missing.push(request);
                        continue;
                    };
                    messages.push(WireNetworkMessage::Block(block.clone()));
                }
                InventoryType::Transaction => {
                    let txid = Txid::from(request.object_hash);
                    let Some(transaction) = self.transactions_by_txid.get(&txid) else {
                        missing.push(request);
                        continue;
                    };
                    messages.push(WireNetworkMessage::Tx(transaction.clone()));
                }
                InventoryType::WitnessTransaction => {
                    let wtxid = Wtxid::from(request.object_hash);
                    let Some(transaction) = self.transactions_by_wtxid.get(&wtxid) else {
                        missing.push(request);
                        continue;
                    };
                    messages.push(WireNetworkMessage::Tx(transaction.clone()));
                }
                _ => missing.push(request),
            }
        }

        (messages, missing)
    }

    pub(super) fn store_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(Txid, Wtxid), ManagedNetworkError> {
        let txid = transaction_txid(&transaction)?;
        let wtxid = transaction_wtxid(&transaction)?;
        self.transactions_by_txid.insert(txid, transaction.clone());
        self.transactions_by_wtxid
            .insert(wtxid, transaction.clone());
        self.peer_manager.note_local_transaction(&transaction)?;
        Ok((txid, wtxid))
    }

    pub(super) fn next_chain_work(&self) -> u128 {
        self.chainstate
            .chainstate()
            .tip()
            .map_or(1, |tip| tip.chain_work.saturating_add(1))
    }
}

pub(super) fn disconnect_network_error(peer_id: PeerId, reason: DisconnectReason) -> NetworkError {
    match reason {
        DisconnectReason::DuplicateVersion => NetworkError::DuplicateVersion(peer_id),
        DisconnectReason::SelfConnection => NetworkError::SelfConnection(peer_id),
        DisconnectReason::MissingHeaderAncestor(hash) => NetworkError::MissingHeaderAncestor(hash),
    }
}
