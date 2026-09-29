// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use open_bitcoin_core::chainstate::{
    CoinsView, FlushMode, FlushPolicyTime, PruneLockInfo, PruneMode, PrunePlan,
};
use open_bitcoin_core::primitives::BlockHash;

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
    let outcome =
        network
            .chainstate_mut()
            .flush_applying_plan(mode, now, disk_free_bytes, plan, locks);
    let deleted = match &outcome {
        Ok(execution) => execution.deleted_block_hashes.clone(),
        Err(failure) => failure.deleted_block_hashes.clone(),
    };
    for hash in &deleted {
        network.blocks_by_hash.remove(hash);
    }
    outcome.map_err(|failure| failure.error)
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

    /// Stores prune mode so version-message services follow the mode.
    pub fn set_prune_mode(&mut self, mode: PruneMode) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.set_prune_mode(mode))
    }

    /// Reads the prune mode Plan 02 stored on this handle.
    pub fn prune_mode(&self) -> Result<PruneMode, ManagedNetworkAuthorityError> {
        self.read(|network| network.prune_mode())
    }

    /// Whether the in-memory block cache still holds `block_hash`.
    pub fn cached_block_present(
        &self,
        block_hash: BlockHash,
    ) -> Result<bool, ManagedNetworkAuthorityError> {
        self.read(|network| network.blocks_by_hash.contains_key(&block_hash))
    }

    /// Drops one connected block's payload and undo while leaving the chain position.
    ///
    /// RPC tests call this. A non-test build keeps it so the dependent crate can
    /// see the method; production callers do not use it.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn forget_block_payload_and_undo_for_test(
        &self,
        block_hash: BlockHash,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            network.blocks_by_hash.remove(&block_hash);
            network.chainstate_mut().forget_undo_for_test(block_hash);
        })
    }
}
