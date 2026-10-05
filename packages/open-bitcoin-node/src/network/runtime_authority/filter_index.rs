// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Trusted lifecycle transitions and the ordinary bounded BASIC turn.

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::storage::{StorageError, filter_index::index_corruption};
use crate::{FjallChainstateStore, storage::FjallCoinsView};
use open_bitcoin_core::chainstate::{CoinsView, VerifiedChainstateFence};

mod catch_up;
pub use catch_up::BasicFilterTurnOutcome;

#[cfg(test)]
pub(crate) struct AcceptedBasicFactsTestEvidence {
    pub(crate) spent_scripts: Vec<Vec<u8>>,
    pub(crate) work: open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork,
}

impl ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView> {
    pub(crate) fn note_basic_index_failure(
        &self,
        failure: crate::chainstate::filter_index::AcceptedBasicIndexFailure,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.chainstate_mut().note_basic_index_failure(failure))
    }

    #[cfg(test)]
    pub(crate) fn maybe_basic_index_progress(
        &self,
    ) -> Result<
        Option<open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress>,
        ManagedNetworkAuthorityError,
    > {
        self.read(|network| network.chainstate().maybe_basic_index_progress())
    }

    #[cfg(test)]
    pub(crate) fn install_basic_index_owner_for_test(
        &self,
        progress: open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            if network.chainstate().maybe_basic_index_progress() == Some(progress) {
                return Ok(());
            }
            network
                .chainstate_mut()
                .replace_basic_index_owner_for_test(progress)
        })?
        .map_err(|error| ManagedNetworkAuthorityError::LifecycleEffect(error.to_string()))
    }

    #[cfg(test)]
    pub(crate) fn force_basic_index_flush_for_test(
        &self,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            network.chainstate_mut().flush_lifecycle =
                crate::chainstate::FlushLifecycle::ready_for_test(
                    0,
                    0,
                    open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
                    false,
                )
        })
    }

    #[cfg(test)]
    pub(crate) fn accepted_basic_facts_for_test(
        &self,
    ) -> Result<Option<AcceptedBasicFactsTestEvidence>, ManagedNetworkAuthorityError> {
        self.read(|network| {
            network
                .chainstate()
                .maybe_accepted_basic_facts()
                .map(|facts| AcceptedBasicFactsTestEvidence {
                    spent_scripts: facts
                        .inputs()
                        .expect("captured inputs")
                        .spent_scripts()
                        .map(<[u8]>::to_vec)
                        .collect(),
                    work: facts.work(),
                })
        })
    }

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
