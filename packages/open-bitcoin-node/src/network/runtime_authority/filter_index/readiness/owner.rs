// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Notifications follow the existing authority; no request owns scheduled work.

use super::super::query::{BasicFilterQuery, BasicFilterQueryError, BasicFilterReadFrontier};
use super::{BasicFilterReadCompletion, BasicFilterReadFailure, wake_all};
use crate::{
    ChainstateStore, ManagedNetworkAuthorityError, ManagedNetworkHandle, ManagedPeerNetwork,
};
use open_bitcoin_core::{
    chainstate::{CoinsView, filter_index::catch_up::BasicIndexState},
    primitives::BlockHash,
};

pub(in crate::network::runtime_authority::filter_index) fn maybe_frontier_result<
    S: ChainstateStore,
    V: CoinsView,
>(
    network: &ManagedPeerNetwork<S, V>,
    frontier: BasicFilterReadFrontier,
) -> Option<Result<(), BasicFilterReadFailure>> {
    let manager = network.chainstate();
    let Some(progress) = manager.maybe_basic_index_progress() else {
        return Some(Err(BasicFilterReadFailure::OwnerStopped));
    };
    if network.authority_epoch.raw() != frontier.authority_incarnation()
        || progress.generation() != frontier.generation()
        || progress.branch_identity() != frontier.branch_identity()
        || manager
            .chainstate()
            .active_chain()
            .get(frontier.accepted_height() as usize)
            .is_none_or(|position| position.block_hash != frontier.accepted_hash())
    {
        return Some(Err(BasicFilterReadFailure::Invalidated));
    }
    if manager.has_pending_validation_history()
        || manager.maybe_basic_index_failure().is_some()
        || progress.state() != BasicIndexState::Active
    {
        return Some(Err(BasicFilterReadFailure::OwnerFailed));
    }
    progress
        .maybe_processed_endpoint()
        .filter(|record| record.height() >= frontier.accepted_height())
        .map(|_| Ok(()))
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    #[cfg(test)]
    pub(crate) fn basic_filter_waiter_count_for_test(&self) -> usize {
        self.basic_filter_readiness
            .registry
            .lock()
            .expect("registry")
            .waiters
            .len()
    }

    #[cfg(test)]
    pub(crate) fn set_basic_filter_waiter_id_for_test(&self, id: u64) {
        self.basic_filter_readiness
            .registry
            .lock()
            .expect("registry")
            .next_id = id;
    }

    #[cfg(test)]
    pub(crate) fn basic_filter_read_locks_available_for_test(&self) -> bool {
        let authority_available = match self.authority.try_lock() {
            Ok(guard) => {
                drop(guard);
                true
            }
            Err(std::sync::TryLockError::Poisoned(poison)) => {
                drop(poison.into_inner());
                true
            }
            Err(std::sync::TryLockError::WouldBlock) => false,
        };
        authority_available && self.basic_filter_readiness.registry.try_lock().is_ok()
    }

    pub(in crate::network::runtime_authority) fn basic_filter_authority_unavailable(
        &self,
    ) -> ManagedNetworkAuthorityError {
        wake_all(
            self.basic_filter_readiness
                .collect(|_| Some(Err(BasicFilterReadFailure::AuthorityUnavailable))),
        );
        ManagedNetworkAuthorityError::Poisoned
    }

    pub(in crate::network::runtime_authority) fn lock_authority(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, ManagedPeerNetwork<S, V>>, ManagedNetworkAuthorityError>
    {
        self.authority.lock().map_err(|poison| {
            drop(poison.into_inner());
            self.basic_filter_authority_unavailable()
        })
    }

    pub(in crate::network::runtime_authority) fn read<T>(
        &self,
        snapshot: impl FnOnce(&ManagedPeerNetwork<S, V>) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let network = self.lock_authority()?;
        Ok(snapshot(&network))
    }

    pub(in crate::network::runtime_authority) fn mutate<T>(
        &self,
        command: impl FnOnce(&mut ManagedPeerNetwork<S, V>) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let mut network = self.lock_authority()?;
        let outcome = command(&mut network);
        let wakes = self.collect_basic_filter_wakes(&network);
        drop(network);
        wake_all(wakes);
        Ok(outcome)
    }

    /// Collect notifications under the authority; caller releases it before waking.
    pub(in crate::network) fn collect_basic_filter_wakes(
        &self,
        network: &ManagedPeerNetwork<S, V>,
    ) -> Vec<std::task::Waker> {
        self.basic_filter_readiness.collect(|frontier| {
            let Some(store) = network
                .chainstate()
                .store()
                .maybe_basic_filter_query_store()
            else {
                return Some(Err(BasicFilterReadFailure::OwnerStopped));
            };
            let status = store.with_basic_filter_query(|reader| {
                Ok(reader.enabled && reader.matches_generation(frontier.generation()))
            });
            match status {
                Err(_) => Some(Err(BasicFilterReadFailure::OwnerFailed)),
                Ok(false) => Some(Err(BasicFilterReadFailure::Invalidated)),
                Ok(true) => maybe_frontier_result(network, frontier),
            }
        })
    }

    /// Settle all captured waits when the existing scheduled maintenance owner stops.
    /// Stopping is terminal for this authority; reopening creates a fresh owner.
    pub fn stop_basic_filter_readiness(&self) -> Result<(), ManagedNetworkAuthorityError> {
        let network = self.lock_authority()?;
        let wakes = self.basic_filter_readiness.stop();
        drop(network);
        wake_all(wakes);
        Ok(())
    }

    /// Read the original requested hash after earned completion. New accepts never
    /// extend this target; captured provenance still governs a missing row.
    pub fn complete_basic_filter_read(
        &self,
        hash: BlockHash,
        completion: BasicFilterReadCompletion,
    ) -> Result<BasicFilterQuery, BasicFilterQueryError> {
        self.read(|network| {
            if self.basic_filter_readiness.is_stopped()
                || hash != completion.request.hash
                || maybe_frontier_result(network, completion.frontier) != Some(Ok(()))
            {
                return Err(BasicFilterQueryError::Readiness(
                    BasicFilterReadFailure::Invalidated,
                ));
            }
            let Some(store) = network
                .chainstate()
                .store()
                .maybe_basic_filter_query_store()
            else {
                return Err(BasicFilterQueryError::Readiness(
                    BasicFilterReadFailure::OwnerStopped,
                ));
            };
            let result = store
                .with_basic_filter_query(|reader| {
                    if !reader.enabled
                        || !reader.matches_generation(completion.frontier.generation())
                    {
                        return Ok(None);
                    }
                    let query = match reader.maybe_record(hash)? {
                        Some(record) => {
                            if record.identity().height() != completion.request.height
                                || record.identity().parent_hash() != completion.request.parent
                            {
                                return Err(crate::storage::filter_index::index_corruption(
                                    "BASIC completed read differs from captured request",
                                ));
                            }
                            BasicFilterQuery::Found(record)
                        }
                        None => BasicFilterQuery::Missing {
                            provenance: completion.request.provenance,
                            initially_synchronized: completion.request.initially_synchronized,
                        },
                    };
                    Ok(Some(query))
                })
                .map_err(BasicFilterQueryError::Storage)?;
            result.ok_or(BasicFilterQueryError::Readiness(
                BasicFilterReadFailure::Invalidated,
            ))
        })
        .map_err(BasicFilterQueryError::Authority)?
    }
}
