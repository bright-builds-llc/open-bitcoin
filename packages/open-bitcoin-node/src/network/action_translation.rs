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

use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags, block_hash};
use open_bitcoin_network::{
    CompactDownloadCleanupCause, DisconnectReason, InventoryList, MisbehaviorDecision,
    MisbehaviorKind, MisbehaviorPolicy, MisbehaviorResponse, PeerAction, PeerId, TxDownloadAction,
    WireNetworkMessage,
};

use crate::ChainstateStore;

use super::{
    ManagedNetworkError, ManagedPeerNetwork, ManagedResult, ManagedSyncMessageResult,
    block_serving::{
        ManagedCompactBlockTxnServeDecision, serve_managed_compact_block_transactions,
    },
    inventory,
};

pub(super) fn process_transaction_relay_action(
    action: TxDownloadAction,
) -> Option<(PeerId, WireNetworkMessage)> {
    action.maybe_request_inventory().map(|inventory| {
        (
            action.peer_id(),
            WireNetworkMessage::GetData(InventoryList::new(vec![inventory])),
        )
    })
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn disconnect_peer(&mut self, peer_id: PeerId) -> Result<(), ManagedNetworkError> {
        self.disconnect_peer_at(peer_id, 0).map(|_| ())
    }

    pub fn disconnect_peer_at(
        &mut self,
        peer_id: PeerId,
        now_unix_seconds: i64,
    ) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
        let removed_count = self
            .peer_manager
            .compact_download_peer_state(peer_id)
            .map_or(0, |state| state.in_flight.len());
        let actions = self
            .peer_manager
            .remove_peer_with_transaction_cleanup(peer_id, now_unix_seconds)?;
        self.record_compact_cleanup(CompactDownloadCleanupCause::PeerDisconnect, removed_count);
        self.orphanage.cleanup_peer(peer_id);
        self.relay_fanout.cleanup_peer(peer_id);
        self.known_peers.remove(&peer_id);
        Ok(transaction_relay_targeted_messages(actions))
    }

    pub fn disconnect_peer_with_transaction_cleanup(
        &mut self,
        peer_id: PeerId,
        now_unix_seconds: i64,
    ) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
        self.disconnect_peer_at(peer_id, now_unix_seconds)
    }

    pub fn expire_transaction_requests(
        &mut self,
        now_unix_seconds: i64,
    ) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
        Ok(self
            .peer_manager
            .expire_transaction_requests(now_unix_seconds)
            .into_iter()
            .filter_map(|(_peer_id, action)| match action {
                PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
                _ => None,
            })
            .collect())
    }

    /// Expire stale compact-block downloads and return peer-targeted full-block `GetData` fallbacks.
    ///
    /// Caller supplies `now_unix_seconds` (same clock contract as transaction request expiry).
    /// Unlike `expire_transaction_requests`, this keeps `PeerAction::Send` — compact timeout
    /// fallbacks are wire `GetData(Block)` messages, not `TransactionRelay` actions.
    pub fn expire_compact_download_timeouts(
        &mut self,
        now_unix_seconds: i64,
    ) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
        let expired_pairs = self
            .peer_manager
            .expire_compact_download_timeouts(now_unix_seconds);
        let expired_count = expired_pairs.len();
        let outbound = expired_pairs
            .into_iter()
            .filter_map(|(peer_id, action)| match action {
                PeerAction::Send(message) => Some((peer_id, message)),
                _ => None,
            })
            .collect();
        if expired_count > 0 {
            self.record_compact_cleanup(CompactDownloadCleanupCause::Timeout, expired_count);
        }
        Ok(outbound)
    }

    pub(super) fn collect_outbound(
        &mut self,
        actions: Vec<PeerAction>,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        Ok(actions
            .into_iter()
            .filter_map(|action| match action {
                PeerAction::Send(message) => Some(message),
                _ => None,
            })
            .collect())
    }

    pub(super) fn process_actions(
        &mut self,
        peer_id: PeerId,
        actions: Vec<PeerAction>,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        let mut outbound = Vec::new();
        let mut targeted_outbound = Vec::new();
        let mut maybe_block_disposition = None;

        for action in actions {
            match action {
                PeerAction::Send(message) => outbound.push(message),
                PeerAction::ServeInventory(requests) => {
                    let (messages, missing) = self.serve_inventory(peer_id, requests);
                    outbound.extend(messages);
                    if !missing.is_empty() {
                        outbound.push(WireNetworkMessage::NotFound(InventoryList::new(missing)));
                    }
                }
                PeerAction::ServeCompactBlockTransactions(request) => {
                    let inventory = open_bitcoin_core::primitives::InventoryVector {
                        inventory_type: open_bitcoin_core::primitives::InventoryType::Block,
                        object_hash: request.block_hash.into(),
                    };
                    let input = self.managed_block_serve_input(
                        peer_id,
                        &inventory,
                        request.block_hash,
                        false,
                    );
                    let decision =
                        serve_managed_compact_block_transactions(input, &request.indexes, |hash| {
                            self.blocks_by_hash.get(&hash).cloned()
                        });
                    self.record_compact_block_txn_serve_outcome(decision.outcome());
                    match decision {
                        ManagedCompactBlockTxnServeDecision::Served(response) => {
                            outbound.push(WireNetworkMessage::BlockTxn(response));
                        }
                        ManagedCompactBlockTxnServeDecision::Suppressed(_) => {}
                        ManagedCompactBlockTxnServeDecision::Malformed(_) => {
                            let reason = DisconnectReason::CompactBlockMisbehavior;
                            if let Some(policy_decision) =
                                compact_misbehavior_decision(peer_id, &reason)
                            {
                                self.record_peer_policy_misbehavior(policy_decision);
                            }
                            self.disconnect_peer(peer_id)?;
                            return Err(inventory::disconnect_network_error(peer_id, reason).into());
                        }
                    }
                }
                PeerAction::ReceivedTransaction(transaction) => {
                    let bridge = self.process_peer_transaction_admission(
                        peer_id,
                        transaction,
                        timestamp,
                        verify_flags,
                        consensus_params,
                    )?;
                    for (target_peer_id, message) in self.record_relay_fanout_for_outcome(
                        Some(peer_id),
                        &bridge.outcome,
                        timestamp,
                    ) {
                        if target_peer_id == peer_id {
                            outbound.push(message);
                        } else {
                            targeted_outbound.push((target_peer_id, message));
                        }
                    }
                    let _reconsidered = bridge.reconsidered;
                    for (target_peer_id, message) in bridge.targeted_outbound {
                        if target_peer_id == peer_id {
                            outbound.push(message);
                        } else {
                            targeted_outbound.push((target_peer_id, message));
                        }
                    }
                }
                PeerAction::TransactionRelay(action) => {
                    if let Some((target_peer_id, message)) =
                        process_transaction_relay_action(action)
                    {
                        if target_peer_id == peer_id {
                            outbound.push(message);
                        } else {
                            targeted_outbound.push((target_peer_id, message));
                        }
                    }
                }
                PeerAction::ReceivedBlock(block) => {
                    // Clear matching volatile compact in-flight across all peers before
                    // connect so a connect failure cannot leave stale multi-peer slots (D-08).
                    let connected_hash = block_hash(&block.header);
                    let removed_count = self
                        .peer_manager
                        .on_compact_download_block_connected(connected_hash);
                    if removed_count > 0 {
                        self.record_compact_cleanup(
                            CompactDownloadCleanupCause::BlockConnected,
                            removed_count,
                        );
                    }
                    maybe_block_disposition = Some(self.connect_stored_block(
                        &block,
                        self.next_chain_work(),
                        timestamp,
                        verify_flags,
                        consensus_params,
                    )?);
                }
                PeerAction::Disconnect(reason) => {
                    if reason == DisconnectReason::SelfConnection {
                        self.record_runtime_self_connection_rejection(peer_id);
                    }
                    if let Some(decision) = compact_misbehavior_decision(peer_id, &reason) {
                        self.record_peer_policy_misbehavior(decision);
                    }
                    self.disconnect_peer(peer_id)?;
                    return Err(inventory::disconnect_network_error(peer_id, reason).into());
                }
                PeerAction::ResourceGovernanceDisconnect(event) => {
                    return self.disconnect_for_resource_governance(peer_id, event);
                }
            }
        }

        Ok(ManagedSyncMessageResult {
            outbound,
            targeted_outbound,
            maybe_block_disposition,
        })
    }
}

fn transaction_relay_targeted_messages(
    actions: Vec<PeerAction>,
) -> Vec<(PeerId, WireNetworkMessage)> {
    actions
        .into_iter()
        .filter_map(|action| match action {
            PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
            _ => None,
        })
        .collect()
}

fn compact_misbehavior_decision(
    peer_id: PeerId,
    reason: &DisconnectReason,
) -> Option<MisbehaviorDecision> {
    let kind = match reason {
        DisconnectReason::CompactBlockMisbehavior => MisbehaviorKind::MalformedMessage,
        DisconnectReason::CompactBlockHeaderViolation => MisbehaviorKind::HeaderViolation,
        _ => return None,
    };
    let policy = MisbehaviorPolicy::default();
    Some(MisbehaviorDecision {
        peer_label: format!("peer-{peer_id}"),
        kind,
        score: policy.discourage_threshold,
        response: MisbehaviorResponse::Disconnect,
    })
}
