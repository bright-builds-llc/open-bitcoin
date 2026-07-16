// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{Block, BlockHash, InventoryType},
};
use open_bitcoin_network::{PeerId, WireNetworkMessage};

use super::{DurableSyncRuntime, SyncRuntimeError, progress::PeerProgress};
use crate::network::BlockConnectDisposition;

impl DurableSyncRuntime {
    pub(super) fn message_reports_requested_block_notfound(
        &self,
        peer_id: PeerId,
        message: &WireNetworkMessage,
    ) -> bool {
        let WireNetworkMessage::NotFound(inventory) = message else {
            return false;
        };
        inventory.inventory.iter().any(|item| {
            matches!(
                item.inventory_type,
                InventoryType::Block | InventoryType::WitnessBlock
            ) && self.peer_requested_block(peer_id, BlockHash::from(item.object_hash))
        })
    }

    pub(super) fn peer_requested_block(&self, peer_id: PeerId, block_hash: BlockHash) -> bool {
        if !self.inflight_blocks.contains(&block_hash) {
            return false;
        }
        self.network
            .peer_requested_blocks(peer_id)
            .is_ok_and(|blocks| blocks.contains(&block_hash))
    }

    pub(super) fn block_has_best_chain_header(&self, block_hash: BlockHash) -> bool {
        self.network
            .best_chain_entries()
            .iter()
            .any(|entry| entry.block_hash == block_hash)
    }

    pub(super) fn block_extends_active_tip(&self, block: &Block) -> bool {
        self.network
            .maybe_chain_tip()
            .is_some_and(|tip| tip.block_hash == block.header.previous_block_hash)
    }

    pub(super) fn classify_unrequested_block(
        &self,
        block_hash: BlockHash,
        previous_block_hash: BlockHash,
    ) -> BlockConnectDisposition {
        let active_chain = self.network.chainstate_snapshot().active_chain;
        if active_chain
            .iter()
            .any(|position| position.block_hash == block_hash)
        {
            return BlockConnectDisposition::Duplicate(block_hash);
        }

        let Some(tip) = active_chain.last() else {
            return BlockConnectDisposition::Disconnected { block_hash };
        };
        if tip.block_hash == previous_block_hash {
            return BlockConnectDisposition::Disconnected { block_hash };
        }

        BlockConnectDisposition::NonExtending {
            block_hash,
            previous_block_hash,
        }
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
        );
        self.record_block_disposition(progress, Some(block), disposition, false, is_best_chain)
    }

    pub(super) fn record_block_disposition(
        &mut self,
        progress: &mut PeerProgress,
        maybe_block: Option<&Block>,
        disposition: BlockConnectDisposition,
        was_requested: bool,
        is_best_chain: bool,
    ) -> Result<(), SyncRuntimeError> {
        match disposition {
            BlockConnectDisposition::Connected(_) => {
                if was_requested && is_best_chain {
                    let Some(block) = maybe_block else {
                        return Ok(());
                    };
                    self.store.save_block(block, self.config.persist_mode)?;
                    self.network
                        .note_local_block_hash(block_hash(&block.header));
                    progress.record_accepted_block();
                }
            }
            BlockConnectDisposition::Duplicate(_) => progress.record_duplicate_block(),
            BlockConnectDisposition::Disconnected { .. } => progress.record_disconnected_block(),
            BlockConnectDisposition::NonExtending { .. } => progress.record_non_extending_block(),
        }
        Ok(())
    }
}
