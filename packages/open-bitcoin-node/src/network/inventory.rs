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
    BlockServingChainPosition, BlockServingDataAvailability, BlockServingValidationState,
    ConnectionRole, DisconnectReason, InactivePermissionEffectLabel, InboundResourceEvent,
    NetworkError, PeerConnectionClass, PeerId, PermissionEffectLabel, TxServeOutcomeLabel,
    TxServingRecordStatus, WireNetworkMessage,
};

use super::block_serving::{
    ManagedBlockServeGateDecision, ManagedBlockServeInput, gate_managed_block_request,
    serve_managed_block_request,
};
use super::{
    ManagedInboundResponsePlanItem, ManagedNetworkError, ManagedPeerNetwork,
    ManagedSyncMessageResult,
};
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
                    let input =
                        self.managed_block_serve_input(peer_id, &request, block_hash, false, false);
                    let decision = serve_managed_block_request(input, |hash| {
                        self.blocks_by_hash.get(&hash).cloned()
                    });
                    self.record_block_serving_evidence(request.inventory_type, &decision);
                    let Some(block) = decision.maybe_block else {
                        missing.push(request);
                        continue;
                    };
                    if decision.missing_inventory {
                        missing.push(request);
                        continue;
                    }
                    messages.push(WireNetworkMessage::Block(block.clone()));
                }
                InventoryType::CompactBlock => {
                    let block_hash = BlockHash::from(request.object_hash);
                    let input =
                        self.managed_block_serve_input(peer_id, &request, block_hash, true, false);
                    let decision = serve_managed_block_request(input, |hash| {
                        self.blocks_by_hash.get(&hash).cloned()
                    });
                    self.record_block_serving_evidence(request.inventory_type, &decision);
                    missing.push(request);
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

    pub(super) fn gate_inventory_for_durable_serving(
        &mut self,
        peer_id: PeerId,
        requests: Vec<InventoryVector>,
    ) -> Vec<ManagedInboundResponsePlanItem> {
        let mut response_plan = Vec::new();
        let mut requests = requests.into_iter().peekable();
        let (peer_mode, relay_eligibility) = self.relay_serving_context_for_peer(peer_id);
        self.relay_serving.clear_latest_outcomes();

        while requests.peek().is_some() {
            let mut missing_transactions = Vec::new();
            while requests.peek().is_some_and(|request| {
                matches!(
                    request.inventory_type,
                    InventoryType::Transaction | InventoryType::WitnessTransaction
                )
            }) {
                let Some(request) = requests.next() else {
                    break;
                };
                let decision =
                    self.relay_serving
                        .classify_request(&request, peer_mode, &relay_eligibility);
                let Some(transaction) = decision.maybe_transaction else {
                    missing_transactions.push(request);
                    continue;
                };
                if decision.label != TxServeOutcomeLabel::Served {
                    missing_transactions.push(request);
                    continue;
                }
                response_plan.push(ManagedInboundResponsePlanItem::Immediate(
                    WireNetworkMessage::Tx(transaction.clone()),
                ));
            }

            if let Some(request) = requests.next() {
                match request.inventory_type {
                    InventoryType::Block
                    | InventoryType::WitnessBlock
                    | InventoryType::CompactBlock => {
                        let block_hash = BlockHash::from(request.object_hash);
                        let input = self
                            .managed_block_serve_input(peer_id, &request, block_hash, false, true);
                        match gate_managed_block_request(input) {
                            ManagedBlockServeGateDecision::Serve(intent) => {
                                response_plan
                                    .push(ManagedInboundResponsePlanItem::DurableBlock(intent));
                            }
                            ManagedBlockServeGateDecision::Deny(decision) => {
                                self.record_block_serving_evidence(
                                    request.inventory_type,
                                    &decision,
                                );
                                response_plan.push(ManagedInboundResponsePlanItem::Immediate(
                                    WireNetworkMessage::NotFound(
                                        open_bitcoin_network::InventoryList::new(vec![request]),
                                    ),
                                ));
                            }
                        }
                    }
                    _ => response_plan.push(ManagedInboundResponsePlanItem::Immediate(
                        WireNetworkMessage::NotFound(open_bitcoin_network::InventoryList::new(
                            vec![request],
                        )),
                    )),
                }
            }

            if !missing_transactions.is_empty() {
                response_plan.push(ManagedInboundResponsePlanItem::Immediate(
                    WireNetworkMessage::NotFound(open_bitcoin_network::InventoryList::new(
                        missing_transactions,
                    )),
                ));
            }
        }

        response_plan
    }

    pub(super) fn managed_block_serve_input(
        &self,
        peer_id: PeerId,
        request: &InventoryVector,
        block_hash: BlockHash,
        suppressed: bool,
        durable_availability: bool,
    ) -> ManagedBlockServeInput {
        let snapshot = self.chainstate.chainstate().snapshot();
        let maybe_active_index = snapshot
            .active_chain
            .iter()
            .position(|position| position.block_hash == block_hash);
        let has_local_data = self.blocks_by_hash.contains_key(&block_hash);
        let is_active = maybe_active_index.is_some();
        let is_tip =
            maybe_active_index.is_some_and(|index| index + 1 == snapshot.active_chain.len());
        let chain_position = if is_active {
            BlockServingChainPosition::Active
        } else if has_local_data {
            BlockServingChainPosition::SideChain
        } else {
            BlockServingChainPosition::Unknown
        };
        let validation_state = if is_active {
            BlockServingValidationState::Validated
        } else {
            BlockServingValidationState::Unknown
        };
        let data_availability = match (is_active, is_tip, has_local_data || durable_availability) {
            (true, _, true) => BlockServingDataAvailability::Available,
            (true, false, false) => BlockServingDataAvailability::Pruned,
            _ => BlockServingDataAvailability::Unavailable,
        };
        let (connection_class, active_permission_effects, inactive_permission_effects) =
            self.block_serving_context_for_peer(peer_id);
        let request_snapshot = self.peer_manager.transaction_request_snapshot(peer_id);
        let requested_blocks_in_flight = self
            .peer_manager
            .peer_requested_blocks(peer_id)
            .unwrap_or_default()
            .len();

        ManagedBlockServeInput {
            inventory_type: request.inventory_type,
            block_hash,
            activation: self.block_relay_activation,
            inbound_serving_enabled: self.inbound_serving_enabled,
            connection_class,
            active_permission_effects,
            inactive_permission_effects,
            requested_blocks_in_flight,
            requested_txids_in_flight: request_snapshot.in_flight_count,
            requested_wtxids_in_flight: 0,
            chain_position,
            validation_state,
            data_availability,
            suppressed,
        }
    }

    fn block_serving_context_for_peer(
        &self,
        peer_id: PeerId,
    ) -> (
        PeerConnectionClass,
        Vec<PermissionEffectLabel>,
        Vec<InactivePermissionEffectLabel>,
    ) {
        let Some(peer) = self.peer_manager.peer_state(peer_id) else {
            return (PeerConnectionClass::OrdinaryInbound, Vec::new(), Vec::new());
        };

        match peer.role {
            ConnectionRole::Outbound => (PeerConnectionClass::Outbound, Vec::new(), Vec::new()),
            ConnectionRole::Inbound => {
                let Some(record) = peer.maybe_inbound_record.as_ref() else {
                    return (PeerConnectionClass::OrdinaryInbound, Vec::new(), Vec::new());
                };
                (
                    record.connection_class,
                    record.permission_decision.active_effects().to_vec(),
                    record.permission_decision.inactive_effects().to_vec(),
                )
            }
        }
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
        DisconnectReason::CompactBlockMisbehavior => NetworkError::CompactBlockMisbehavior(peer_id),
        DisconnectReason::CompactBlockHeaderViolation => {
            NetworkError::CompactBlockHeaderViolation(peer_id)
        }
    }
}
