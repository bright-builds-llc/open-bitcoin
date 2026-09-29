// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h

//! Durable prune-lock reads and writes for RPC.

use std::collections::BTreeMap;

use open_bitcoin_node::core::chainstate::{PRUNE_LOCK_BUFFER, PruneLockInfo};

use super::ManagedRpcContext;
use crate::RpcFailure;

const PRUNE_LOCKS_UNAVAILABLE: &str = "prune locks are unavailable";

impl<S, V> ManagedRpcContext<S, V>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    pub(crate) fn list_prune_locks(&self) -> Result<Vec<PruneLockInfo>, RpcFailure> {
        let Some(store) = self.maybe_metrics_store.as_ref() else {
            return Ok(Vec::new());
        };
        store
            .load_prune_locks()
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))
    }

    /// Replaces one name, then SyncAll-writes the whole map.
    ///
    /// `height_last > u32::MAX - PRUNE_LOCK_BUFFER` is refused before any write
    /// so `height_last + PRUNE_LOCK_BUFFER` in `height_forbidden_by_lock` cannot wrap.
    pub(crate) fn replace_prune_lock(
        &self,
        name: String,
        height_first: u32,
        height_last: u32,
    ) -> Result<PruneLockInfo, RpcFailure> {
        if name.is_empty() {
            return Err(RpcFailure::invalid_parameter("empty prune lock name"));
        }
        if height_last < height_first {
            return Err(RpcFailure::invalid_parameter(
                "height_last is below height_first",
            ));
        }
        if height_last > u32::MAX - PRUNE_LOCK_BUFFER {
            return Err(RpcFailure::invalid_parameter(
                "height_last would overflow the prune lock buffer",
            ));
        }

        let Some(store) = self.maybe_metrics_store.as_ref() else {
            return Err(RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE));
        };
        let loaded = store
            .load_prune_locks()
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))?;
        let mut by_name = BTreeMap::new();
        for lock in loaded {
            by_name.insert(lock.name.clone(), lock);
        }
        let record = PruneLockInfo {
            name,
            height_first,
            height_last,
        };
        by_name.insert(record.name.clone(), record.clone());
        let stored: Vec<PruneLockInfo> = by_name.into_values().collect();
        store
            .sync_prune_locks(&stored)
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))?;
        Ok(record)
    }

    /// Removes one name. A missing name returns false and does not write.
    pub(crate) fn clear_named_prune_lock(&self, name: &str) -> Result<bool, RpcFailure> {
        let Some(store) = self.maybe_metrics_store.as_ref() else {
            return Err(RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE));
        };
        let loaded = store
            .load_prune_locks()
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))?;
        if !loaded.iter().any(|lock| lock.name == name) {
            return Ok(false);
        }
        let mut by_name = BTreeMap::new();
        for lock in loaded {
            if lock.name != name {
                by_name.insert(lock.name.clone(), lock);
            }
        }
        let stored: Vec<PruneLockInfo> = by_name.into_values().collect();
        store
            .sync_prune_locks(&stored)
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))?;
        Ok(true)
    }
}
