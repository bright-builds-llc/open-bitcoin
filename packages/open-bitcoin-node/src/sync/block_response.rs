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
