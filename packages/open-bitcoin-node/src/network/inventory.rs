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

use open_bitcoin_core::{
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{BlockHash, InventoryType, InventoryVector, Transaction, Txid, Wtxid},
};
use open_bitcoin_network::{
    DisconnectReason, InboundResourceEvent, NetworkError, PeerId, TxServeOutcomeLabel,
    TxServingRecordStatus, WireNetworkMessage,
};

use super::{ManagedNetworkError, ManagedPeerNetwork, ManagedSyncMessageResult};
use crate::ChainstateStore;

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn serve_inventory(
        &mut self,
        peer_id: PeerId,
        requests: Vec<InventoryVector>,
    ) -> (Vec<WireNetworkMessage>, Vec<InventoryVector>) {
        let mut messages = Vec::new();
        let mut missing = Vec::new();
        let (peer_mode, relay_eligibility) = self.relay_serving_context_for_peer(peer_id);
        self.relay_serving.clear_latest_outcomes();

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
                InventoryType::Transaction | InventoryType::WitnessTransaction => {
                    let decision = self.relay_serving.classify_request(
                        &request,
                        peer_mode,
                        &relay_eligibility,
                    );
                    let Some(transaction) = decision.maybe_transaction else {
                        missing.push(request);
                        continue;
                    };
                    if decision.label != TxServeOutcomeLabel::Served {
                        missing.push(request);
                        continue;
                    }
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
        self.relay_serving.record_accepted(transaction.clone())?;
        self.peer_manager.note_local_transaction(&transaction)?;
        Ok((txid, wtxid))
    }

    pub(super) fn remove_stored_transactions_with_status(
        &mut self,
        txids: &[Txid],
        status: TxServingRecordStatus,
    ) -> Result<(), ManagedNetworkError> {
        for txid in txids {
            let Some(transaction) = self.transactions_by_txid.remove(txid) else {
                continue;
            };
            let wtxid = transaction_wtxid(&transaction)?;
            self.transactions_by_wtxid.remove(&wtxid);
        }
        if let Some(reason) = super::relay_fanout::cleanup_reason_for_serving_status(status) {
            self.relay_fanout.cleanup_transactions(txids, reason);
        }
        self.relay_serving.remove_transactions(txids, status)?;
        Ok(())
    }

    pub(super) fn next_chain_work(&self) -> u128 {
        self.chainstate
            .chainstate()
            .tip()
            .map_or(1, |tip| tip.chain_work.saturating_add(1))
    }

    pub(super) fn disconnect_for_resource_governance(
        &mut self,
        peer_id: PeerId,
        event: InboundResourceEvent,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        self.record_resource_governance_event(event);
        self.disconnect_peer(peer_id)?;
        Err(ManagedNetworkError::Network(NetworkError::ResourceLimit(
            peer_id,
        )))
    }
}

pub(super) fn disconnect_network_error(peer_id: PeerId, reason: DisconnectReason) -> NetworkError {
    match reason {
        DisconnectReason::DuplicateVersion => NetworkError::DuplicateVersion(peer_id),
        DisconnectReason::SelfConnection => NetworkError::SelfConnection(peer_id),
        DisconnectReason::ResourceLimit => NetworkError::ResourceLimit(peer_id),
        DisconnectReason::MissingHeaderAncestor(hash) => NetworkError::MissingHeaderAncestor(hash),
    }
}
