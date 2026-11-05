// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{Block, BlockHash},
};

use super::{DurableSyncRuntime, DurableTipAdvanced, SyncRuntimeError, progress::PeerProgress};
use crate::network::BlockConnectDisposition;

/// Identify the body being answered without expanding receipt-cleanup semantics.
pub(super) fn maybe_block_response_hash(
    message: &open_bitcoin_network::WireNetworkMessage,
) -> Option<BlockHash> {
    use open_bitcoin_network::WireNetworkMessage;
    match message {
        WireNetworkMessage::Block(block) => Some(block_hash(&block.header)),
        WireNetworkMessage::CompactBlock(payload) => Some(block_hash(&payload.header)),
        WireNetworkMessage::BlockTxn(payload) => Some(payload.block_hash),
        _ => None,
    }
}

/// Runtime handoff and current original-peer membership are independent facts.
pub(super) fn runtime_requested_block(
    inflight: &std::collections::BTreeSet<BlockHash>,
    current_peer_requests: &[BlockHash],
    hash: BlockHash,
) -> bool {
    inflight.contains(&hash) && current_peer_requests.contains(&hash)
}

/// Only the incoming bounded receipt inventory is inspected; no global flight scan.
pub(super) fn inflight_receipt_hashes(
    message: &open_bitcoin_network::WireNetworkMessage,
    inflight: &std::collections::BTreeSet<BlockHash>,
) -> Vec<BlockHash> {
    use open_bitcoin_core::primitives::InventoryType;
    use open_bitcoin_network::WireNetworkMessage;
    let receipt_hashes = match message {
        WireNetworkMessage::Block(block) => vec![block_hash(&block.header)],
        WireNetworkMessage::NotFound(inventory) => inventory
            .inventory
            .iter()
            .filter(|item| {
                matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                )
            })
            .map(|item| BlockHash::from(item.object_hash))
            .collect(),
        _ => Vec::new(),
    };
    receipt_hashes
        .into_iter()
        .filter(|hash| inflight.contains(hash))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

impl DurableSyncRuntime {
    pub(super) fn block_has_best_chain_header(
        &self,
        block_hash: BlockHash,
    ) -> Result<bool, SyncRuntimeError> {
        Ok(self
            .network
            .best_chain_entries()?
            .iter()
            .any(|entry| entry.block_hash == block_hash))
    }

    pub(super) fn block_extends_active_tip(&self, block: &Block) -> Result<bool, SyncRuntimeError> {
        Ok(self
            .network
            .maybe_chain_tip()?
            .is_some_and(|tip| tip.block_hash == block.header.previous_block_hash))
    }

    pub(super) fn classify_unrequested_block(
        &self,
        block_hash: BlockHash,
        previous_block_hash: BlockHash,
    ) -> Result<BlockConnectDisposition, SyncRuntimeError> {
        let active_chain = self.network.chainstate_snapshot()?.active_chain;
        if active_chain
            .iter()
            .any(|position| position.block_hash == block_hash)
        {
            return Ok(BlockConnectDisposition::Duplicate(block_hash));
        }

        let Some(tip) = active_chain.last() else {
            return Ok(BlockConnectDisposition::Disconnected { block_hash });
        };
        if tip.block_hash == previous_block_hash {
            return Ok(BlockConnectDisposition::Disconnected { block_hash });
        }

        Ok(BlockConnectDisposition::NonExtending {
            block_hash,
            previous_block_hash,
        })
    }

    pub(super) fn record_unrequested_block_response(
        &mut self,
        progress: &mut PeerProgress,
        block: &Block,
        is_best_chain: bool,
    ) -> Result<(), SyncRuntimeError> {
        let disposition = self.classify_unrequested_block(
            block_hash(&block.header),
            block.header.previous_block_hash,
        )?;
        self.record_block_disposition(progress, Some(block), disposition, false, is_best_chain)
    }

    pub(super) fn record_block_disposition(
        &mut self,
        progress: &mut PeerProgress,
        maybe_block: Option<&Block>,
        disposition: BlockConnectDisposition,
        was_requested: bool,
        _is_best_chain: bool,
    ) -> Result<(), SyncRuntimeError> {
        // Requested side and pending bodies must survive for later branch reconciliation.
        if was_requested && let Some(block) = maybe_block {
            if let Err(error) = self.store.save_block(block, self.config.persist_mode) {
                if let Err(notification) = self.network.note_basic_index_failure(
                    crate::chainstate::filter_index::AcceptedBasicIndexFailure::RequiredBody,
                ) {
                    return Err(crate::storage::StorageError::BackendFailure {
                        namespace: crate::storage::StorageNamespace::BlockIndex,
                        message: format!("{error}; BASIC failure notification: {notification}"),
                        action: crate::storage::StorageRecoveryAction::Restart,
                    }
                    .into());
                }
                return Err(error.into());
            }
            self.network
                .note_local_block_hash(block_hash(&block.header))?;
        }
        match disposition {
            BlockConnectDisposition::Connected(_) => {
                if was_requested {
                    let Some(block) = maybe_block else {
                        return Ok(());
                    };
                    progress.record_accepted_block();
                    if self.block_is_current_active_tip(block)? {
                        self.queue_durable_tip_advanced(block.clone());
                    }
                }
            }
            BlockConnectDisposition::Duplicate(_) => progress.record_duplicate_block(),
            BlockConnectDisposition::Disconnected { .. } => progress.record_disconnected_block(),
            BlockConnectDisposition::NonExtending { .. } => progress.record_non_extending_block(),
        }
        Ok(())
    }

    fn block_is_current_active_tip(&self, block: &Block) -> Result<bool, SyncRuntimeError> {
        let block_hash = block_hash(&block.header);
        let maybe_active_tip = self.network.maybe_chain_tip()?;
        Ok(maybe_active_tip.is_some_and(|tip| tip.block_hash == block_hash))
    }

    pub(super) fn queue_durable_tip_advanced(&mut self, block: Block) {
        self.maybe_pending_durable_tip = Some(DurableTipAdvanced::new(block));
    }

    pub(super) fn clear_pending_durable_tip(&mut self) {
        self.maybe_pending_durable_tip = None;
    }

    pub(super) fn dispatch_pending_durable_tip(&mut self) -> Result<(), SyncRuntimeError> {
        let Some(event) = self.maybe_pending_durable_tip.take() else {
            return Ok(());
        };
        let Some(sink) = self.maybe_durable_tip_announcement_sink.as_ref() else {
            return Ok(());
        };
        sink(event)
    }

    pub(super) fn persist_progress_and_dispatch_tip(&mut self) -> Result<(), SyncRuntimeError> {
        if let Err(error) = self.persist_progress() {
            self.clear_pending_durable_tip();
            return Err(error);
        }
        self.dispatch_pending_durable_tip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        FjallNodeStore,
        sync::{
            SyncNetwork, SyncRuntimeConfig,
            block_reconcile::{self, PeerRequests},
            progress::PeerProgress,
        },
    };
    use open_bitcoin_core::primitives::{InventoryType, InventoryVector};
    use open_bitcoin_network::{InventoryList, PeerId, VersionMessage, WireNetworkMessage};
    use std::path::PathBuf;

    fn runtime() -> (PathBuf, DurableSyncRuntime, Block) {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned nonce");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical temp parent")
            .join(format!(
                "phase167-fresh-classifier-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let config = SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        };
        let runtime = DurableSyncRuntime::open(
            FjallNodeStore::open(&path).expect("new empty store"),
            config,
        )
        .expect("empty protocol owner");
        (
            path,
            runtime,
            open_bitcoin_core::consensus::CanonicalChain::Regtest
                .spec()
                .genesis_block()
                .clone(),
        )
    }
    fn handshake(runtime: &DurableSyncRuntime, peer: PeerId) {
        for message in [
            WireNetworkMessage::Version(VersionMessage::default()),
            WireNetworkMessage::Verack,
        ] {
            runtime
                .network
                .receive_sync_message(
                    peer,
                    message,
                    0,
                    runtime.verify_flags,
                    runtime.consensus_params,
                )
                .expect("real handshake");
        }
    }
    fn notfound(hash: BlockHash) -> WireNetworkMessage {
        WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: hash.into(),
        }]))
    }

    #[test]
    fn phase167_peer_session_body_response_identity_keeps_receipt_cleanup_bounded() {
        // Arrange
        let block = open_bitcoin_core::consensus::CanonicalChain::Regtest
            .spec()
            .genesis_block()
            .clone();
        let hash = block_hash(&block.header);
        let compact = open_bitcoin_codec::CompactBlockPayload {
            header: block.header.clone(),
            nonce: 1,
            short_ids: Vec::new(),
            prefilled_transactions: Vec::new(),
        };
        let inflight = [hash].into_iter().collect();
        for (message, maybe_expected_identity, cleanup) in [
            (WireNetworkMessage::Block(block), Some(hash), vec![hash]),
            (
                WireNetworkMessage::CompactBlock(compact),
                Some(hash),
                vec![],
            ),
            (
                WireNetworkMessage::BlockTxn(open_bitcoin_codec::BlockTransactions {
                    block_hash: hash,
                    transactions: Vec::new(),
                }),
                Some(hash),
                vec![],
            ),
            (notfound(hash), None, vec![hash]),
            (WireNetworkMessage::Ping { nonce: 1 }, None, vec![]),
        ] {
            // Act
            let maybe_identity = maybe_block_response_hash(&message);
            let actual_cleanup = inflight_receipt_hashes(&message, &inflight);
            // Assert
            assert_eq!(maybe_identity, maybe_expected_identity);
            assert_eq!(actual_cleanup, cleanup);
        }
    }

    fn grant(runtime: &mut DurableSyncRuntime, peer: PeerId, hash: BlockHash) {
        let messages = runtime
            .network
            .request_missing_blocks(peer, &[hash])
            .expect("actual network registration");
        assert!(!messages.is_empty());
        block_reconcile::handoff_requested_messages(
            runtime,
            peer,
            messages
                .into_iter()
                .map(|message| (peer, message))
                .collect(),
        )
        .expect("actual runtime handoff");
    }

    #[test]
    fn phase167_peer_session_canceled_after_renewal_uses_fresh_manager_and_preserves_shared_slot() {
        for is_notfound in [false, true] {
            // Arrange
            let (path, mut runtime, block) = runtime();
            let hash = open_bitcoin_core::consensus::block_hash(&block.header);
            let (token, outbound) = runtime.network.connect_sync_peer(1, 0).expect("real A");
            outbound.expect("version");
            handshake(&runtime, 1);
            runtime.network.connect_outbound_peer(2, 0).expect("real B");
            handshake(&runtime, 2);
            grant(&mut runtime, 1, hash);
            grant(&mut runtime, 2, hash);
            let mut requests = PeerRequests::new(token);
            requests
                .renew(&mut runtime, &[])
                .expect("earned A observation");
            assert!(
                requests
                    .block_was_requested(&runtime, hash)
                    .expect("real positive handoff")
            );
            runtime
                .network
                .clone()
                .receive_sync_message(
                    1,
                    notfound(hash),
                    1,
                    runtime.verify_flags,
                    runtime.consensus_params,
                )
                .expect("actual A cancellation after renewal");
            let before = runtime.network.chainstate_snapshot().expect("before");
            let mut progress = PeerProgress::new(
                crate::sync::ResolvedSyncPeerAddress::new(
                    crate::sync::SyncPeerAddress::manual("127.0.0.1", 18444),
                    "127.0.0.1:18444".parse().expect("endpoint"),
                ),
                runtime.config.network,
                1,
            );
            // Act
            let requested = if is_notfound {
                requests.notfound_was_requested(&runtime, &notfound(hash))
            } else {
                requests.block_was_requested(&runtime, hash)
            }
            .expect("fresh actual membership");
            // Assert
            assert!(!requested);
            if is_notfound {
                requests
                    .dispatch(&mut runtime, notfound(hash), 2, &[], &mut progress)
                    .expect("authorized")
                    .expect("real unrequested NotFound");
            } else {
                requests
                    .record_ignored(&mut runtime, 2, &mut progress, &[])
                    .expect("actual guarded ignored policy");
                runtime
                    .record_unrequested_block_response(&mut progress, &block, false)
                    .expect("actual ignored body outcome");
            }
            requests.finish(&mut runtime).expect("A teardown");
            assert_eq!(
                runtime.network.peer_requested_blocks(2).expect("B exact"),
                vec![hash]
            );
            assert_eq!(runtime.inflight_blocks, [hash].into_iter().collect());
            assert_eq!(
                runtime.network.chainstate_snapshot().expect("unchanged"),
                before
            );
            assert_eq!(progress.blocks_received, 0);
            assert!(
                runtime
                    .store
                    .load_block(hash)
                    .expect("body unavailable")
                    .is_none()
            );
            drop(runtime);
            std::fs::remove_dir_all(path).expect("owned cleanup");
        }
    }
}
