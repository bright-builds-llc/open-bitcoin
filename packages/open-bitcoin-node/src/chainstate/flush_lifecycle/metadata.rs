// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;

pub(super) enum CoinsWriteKind {
    Flush,
    Sync,
}

pub(super) const fn coins_write_kind(decision: FlushDecision) -> Option<CoinsWriteKind> {
    match decision {
        FlushDecision::Flush(_) => Some(CoinsWriteKind::Flush),
        FlushDecision::Sync(_) => Some(CoinsWriteKind::Sync),
        FlushDecision::None(_) | FlushDecision::RefuseDiskSpace(_) => None,
    }
}

pub(super) fn complete_coins_write<S: FlushPersistSink>(
    sink: &mut S,
    cache: &mut impl CoinsFlush,
    decision: FlushDecision,
    active_chain: &[ChainPosition],
    deleted_block_hashes: Vec<BlockHash>,
) -> Result<FlushExecution, StorageError> {
    let Some(write_kind) = coins_write_kind(decision) else {
        if matches!(decision, FlushDecision::RefuseDiskSpace(_)) {
            return Err(refuse_disk_space());
        }
        return Ok(FlushExecution {
            decision,
            wrote_coins: false,
            deleted_block_hashes,
        });
    };
    // An opened or previously flushed cache has no staged tip. Carry current
    // coins authority into the write without changing a newer overlay tip.
    cache.prepare_current_best_block().map_err(map_chainstate)?;
    match write_kind {
        CoinsWriteKind::Flush => cache.flush().map_err(map_chainstate)?,
        CoinsWriteKind::Sync => cache.sync().map_err(map_chainstate)?,
    }
    sink.persist_chain_meta_with_counts(
        active_chain,
        cache
            .maybe_canonical_confirmed_txid_counts()
            .map_err(map_chainstate)?,
    )?;
    Ok(FlushExecution {
        decision,
        wrote_coins: true,
        deleted_block_hashes,
    })
}
