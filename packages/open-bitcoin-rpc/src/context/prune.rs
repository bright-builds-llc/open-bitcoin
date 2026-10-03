// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h

//! Durable prune-lock reads and writes for RPC.

use std::time::{SystemTime, UNIX_EPOCH};

use open_bitcoin_node::chainstate::probe_disk_free_bytes;
use open_bitcoin_node::core::chainstate::{
    FlushMode, FlushPolicyTime, PRUNE_LOCK_BUFFER, PruneLockInfo, PrunePlan,
};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::status::ManualPruneSurface;

use super::ManagedRpcContext;
use crate::RpcFailure;

const PRUNE_LOCKS_UNAVAILABLE: &str = "prune locks are unavailable";

impl<S, V> ManagedRpcContext<S, V>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    pub(crate) fn list_prune_locks(&self) -> Result<Vec<PruneLockInfo>, RpcFailure> {
        self.load_prune_locks()
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))
    }

    /// Operator status and the explicit lock RPC read the same current owner map.
    pub(crate) fn load_prune_locks(
        &self,
    ) -> Result<Vec<PruneLockInfo>, open_bitcoin_node::ManagedNetworkAuthorityError> {
        self.network.list_prune_locks()
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

        let record = PruneLockInfo {
            name,
            height_first,
            height_last,
        };
        self.network
            .replace_prune_lock(record.clone())
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))?;
        Ok(record)
    }

    /// Removes one name. A missing name returns false and does not write.
    pub(crate) fn clear_named_prune_lock(&self, name: &str) -> Result<bool, RpcFailure> {
        self.network
            .clear_prune_lock(name)
            .map_err(|_| RpcFailure::internal_error(PRUNE_LOCKS_UNAVAILABLE))
    }

    pub(crate) fn manual_prune_surface(&self) -> ManualPruneSurface {
        self.manual_prune.clone()
    }

    pub(crate) fn record_manual_prune(&mut self, surface: ManualPruneSurface) {
        self.manual_prune = surface;
    }

    /// Knots `PruneAfterHeight`: mainnet 100000, every other chain 1000.
    pub(crate) fn prune_after_height(&self) -> u32 {
        let network = match self.chain {
            AddressNetwork::Mainnet => open_bitcoin_node::SyncNetwork::Mainnet,
            AddressNetwork::Testnet => open_bitcoin_node::SyncNetwork::Testnet,
            AddressNetwork::Signet => open_bitcoin_node::SyncNetwork::Signet,
            AddressNetwork::Regtest => open_bitcoin_node::SyncNetwork::Regtest,
        };
        network.prune_after_height()
    }

    /// Flushes a legal manual plan. Callers refuse before this when the plan errs.
    pub(crate) fn flush_applying_prune_plan(
        &self,
        plan: &PrunePlan,
        locks: &[PruneLockInfo],
    ) -> Result<(), open_bitcoin_node::ManagedNetworkAuthorityError> {
        let disk_free_bytes = match self.maybe_metrics_store.as_ref() {
            Some(store) => probe_disk_free_bytes(store.datadir()),
            None => u64::MAX,
        };
        self.network
            .flush_applying_prune_plan(
                FlushMode::Always,
                flush_policy_now(),
                disk_free_bytes,
                plan,
                locks,
            )
            .map(|_| ())
    }
}

fn flush_policy_now() -> FlushPolicyTime {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(u64::MAX);
    FlushPolicyTime::from_unix_seconds(seconds)
}
