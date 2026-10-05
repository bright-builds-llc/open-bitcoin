// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Explicit trusted Rust host transitions; no product activation route or worker.

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::storage::{StorageError, filter_index::index_corruption};
use crate::{FjallChainstateStore, storage::FjallCoinsView};
use open_bitcoin_core::chainstate::{CoinsView, VerifiedChainstateFence};

impl ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView> {
    /// Stop BASIC work durably before releasing its owned prune protection.
    /// This trusted host call shares the chainstate/deletion authority.
    pub fn disable_basic_filter_index(&self) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            network
                .chainstate()
                .store()
                .inner()
                .disable_basic_filter_index()
        })?
        .map_err(lifecycle_error)
    }

    /// Acquire BASIC protection after validating all required retained history.
    /// The host supplies no name, range, checkpoint, generation or chainstate fence.
    pub fn enable_basic_filter_index(&self) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            let store = network.chainstate().store().inner();
            let (positions, _) = store.load_chain_meta_for_open()?;
            let fence = VerifiedChainstateFence::new(
                store.coins_view().best_block().map_err(index_corruption)?,
                Some(&positions),
            )
            .map_err(index_corruption)?;
            store.enable_basic_filter_index(&fence)
        })?
        .map_err(lifecycle_error)
    }
}

fn lifecycle_error(error: StorageError) -> ManagedNetworkAuthorityError {
    ManagedNetworkAuthorityError::LifecycleEffect(error.to_string())
}
