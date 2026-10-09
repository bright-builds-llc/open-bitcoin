// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Native undo envelopes are admitted before serde can allocate nested containers.

use super::*;
use open_bitcoin_core::chainstate::BlockUndo;

impl FjallNodeStore {
    pub(crate) fn basic_filter_required_undo(
        &self,
        hash: BlockHash,
        expected: &BlockUndo,
        work: &mut TurnWork,
        maximum: TurnWork,
    ) -> Result<BlockUndo, StorageError> {
        super::super::append::charge_append_work(
            work,
            TurnWork {
                record_operations: 1,
                ..Default::default()
            },
            Some(maximum),
        )?;
        let bytes = self
            .chainstate
            .get(super::super::super::coins::undo_key(hash))
            .map_err(|error| backend_failure(StorageNamespace::Chainstate, error))?
            .ok_or_else(|| index_corruption("missing BASIC required undo"))?;
        let len = bytes.len() as u64;
        // Every decoded container item consumes at least one JSON byte. Reserve
        // that worst-case count and both DTO/domain containers before decoding.
        let element_bytes = (std::mem::size_of::<open_bitcoin_core::chainstate::Coin>()
            + std::mem::size_of::<BlockUndo>()
            + std::mem::size_of::<Vec<u8>>()) as u64;
        let clones = len
            .checked_mul(element_bytes)
            .and_then(|n| n.checked_mul(2))
            .ok_or_else(|| index_corruption("BASIC undo allocation overflow"))?;
        super::super::append::charge_append_work(
            work,
            TurnWork {
                undo_bytes: len,
                cloned_bytes: clones,
                checkpoint_operations: len,
                script_items: len,
                script_bytes: len,
                ..Default::default()
            },
            Some(maximum),
        )?;
        let undo = crate::storage::snapshot_codec::decode_block_undo(bytes.as_ref())
            .map_err(|error| index_corruption(format!("invalid BASIC required undo: {error}")))?;
        if &undo != expected {
            return Err(index_corruption(
                "BASIC required undo differs from genuine historical undo",
            ));
        }
        Ok(undo)
    }
}
