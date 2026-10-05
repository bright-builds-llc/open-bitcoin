// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, CoinsView, FlushMode, FlushPolicyTime, PruneLockInfo, PruneMode,
    PrunePlan,
};
use open_bitcoin_core::primitives::BlockHash;
use std::collections::BTreeMap;

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::ChainstateStore;
use crate::ManagedPeerNetwork;
use crate::chainstate::{FlushExecution, FlushPersistSink};
use crate::storage::StorageError;
use crate::storage::StorageNamespace;

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
    /// Installs the chain-parameter prune threshold before lifecycle workers start.
    pub fn set_prune_network(
        &self,
        network: crate::SyncNetwork,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|_| {
            let mut state = self
                .automatic_prune
                .lock()
                .map_err(|_| ManagedNetworkAuthorityError::Poisoned)?;
            state.set_network(network);
            Ok(())
        })?
    }

    /// Applies `plan` on the same cache-evicting flush `flush_coins` uses.
    ///
    /// Reloads durable locks inside the owner so a stale caller cannot remove protection.
    pub fn flush_applying_prune_plan(
        &self,
        mode: FlushMode,
        now: FlushPolicyTime,
        disk_free_bytes: u64,
        plan: &PrunePlan,
        locks: &[PruneLockInfo],
    ) -> Result<FlushExecution, ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            let mut current = current_prune_locks(network.chainstate().store())?;
            current.extend_from_slice(locks);
            flush_and_evict_pruned_blocks(network, mode, now, disk_free_bytes, plan, &current)
        })?
        .map_err(|error| ManagedNetworkAuthorityError::LifecycleEffect(error.to_string()))
    }

    /// Reads durable protection under the same authority used for deletion.
    pub fn list_prune_locks(&self) -> Result<Vec<PruneLockInfo>, ManagedNetworkAuthorityError> {
        self.read(|network| current_prune_locks(network.chainstate().store()))?
            .map_err(storage_authority_error)
    }

    /// Replaces one named lock atomically with respect to planning and deletion.
    pub fn replace_prune_lock(
        &self,
        record: PruneLockInfo,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        refuse_reserved_name(&record.name)?;
        self.mutate(|network| {
            let store = network.chainstate().store();
            let mut by_name: BTreeMap<_, _> = store
                .load_prune_locks()?
                .into_iter()
                .map(|lock| (lock.name.clone(), lock))
                .collect();
            by_name.insert(record.name.clone(), record);
            store.sync_prune_locks(&by_name.into_values().collect::<Vec<_>>())
        })?
        .map_err(storage_authority_error)
    }

    /// Clears a named lock; absence performs no durable write.
    pub fn clear_prune_lock(&self, name: &str) -> Result<bool, ManagedNetworkAuthorityError> {
        refuse_reserved_name(name)?;
        self.mutate(|network| {
            let store = network.chainstate().store();
            let mut locks = store.load_prune_locks()?;
            let before = locks.len();
            locks.retain(|lock| lock.name != name);
            if before == locks.len() {
                return Ok(false);
            }
            store.sync_prune_locks(&locks)?;
            Ok(true)
        })?
        .map_err(storage_authority_error)
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

pub(super) fn current_prune_locks<S: FlushPersistSink>(
    store: &S,
) -> Result<Vec<PruneLockInfo>, StorageError> {
    match store.load_prune_locks() {
        // Explicitly unsupported transient fixtures have no durable protection map.
        Err(StorageError::UnavailableNamespace {
            namespace: StorageNamespace::BlockIndex,
        }) => Ok(Vec::new()),
        outcome => outcome,
    }
}

fn storage_authority_error(error: StorageError) -> ManagedNetworkAuthorityError {
    ManagedNetworkAuthorityError::LifecycleEffect(error.to_string())
}

fn refuse_reserved_name(name: &str) -> Result<(), ManagedNetworkAuthorityError> {
    if name == BASIC_INDEX_PRUNE_LOCK {
        return Err(ManagedNetworkAuthorityError::LifecycleEffect(
            "reserved BASIC index prune lock is internally owned".to_owned(),
        ));
    }
    Ok(())
}
