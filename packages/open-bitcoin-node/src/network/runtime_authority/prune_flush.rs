// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use open_bitcoin_core::chainstate::{
    CoinsView, FlushMode, FlushPolicyTime, PruneLockInfo, PrunePlan,
};

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::ChainstateStore;
use crate::ManagedPeerNetwork;
use crate::chainstate::{FlushExecution, FlushPersistSink};
use crate::storage::StorageError;

/// Flushes with `plan`, then removes only the hashes that flush reported as deleted.
pub(super) fn flush_and_evict_pruned_blocks<S, V>(
    network: &mut ManagedPeerNetwork<S, V>,
    mode: FlushMode,
    now: FlushPolicyTime,
    disk_free_bytes: u64,
    plan: &PrunePlan,
    locks: &[PruneLockInfo],
) -> Result<FlushExecution, StorageError>
where
    S: ChainstateStore + FlushPersistSink,
    V: CoinsView,
{
    let execution =
        network
            .chainstate_mut()
            .flush_applying_plan(mode, now, disk_free_bytes, plan, locks)?;
    for hash in &execution.deleted_block_hashes {
        network.blocks_by_hash.remove(hash);
    }
    Ok(execution)
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    /// Applies `plan` on the same cache-evicting flush `flush_coins` uses.
    ///
    /// Production callers still pass an empty plan through `flush_coins`.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn flush_applying_prune_plan(
        &self,
        mode: FlushMode,
        now: FlushPolicyTime,
        disk_free_bytes: u64,
        plan: &PrunePlan,
        locks: &[PruneLockInfo],
    ) -> Result<FlushExecution, ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            flush_and_evict_pruned_blocks(network, mode, now, disk_free_bytes, plan, locks)
        })?
        .map_err(|error| ManagedNetworkAuthorityError::LifecycleEffect(error.to_string()))
    }
}
