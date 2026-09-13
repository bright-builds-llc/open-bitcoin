// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::chainstate::CoinsView;
use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{ConsensusParams, ScriptVerifyFlags},
};
use open_bitcoin_mempool::{PolicyConfig, PolicyTime};

use crate::ChainstateStore;
use crate::status::{SyncRecoveryCategory, relay_evidence::RelayRecoveryCounters};
use crate::storage::{MempoolRecoveryRecord, MempoolRecoveryStatus, MempoolSnapshot, StorageError};

use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::network::lifecycle_projection::AuthorityEpoch;

pub(crate) mod staging;
pub(crate) mod topology;

pub use staging::PreparedMempoolRecovery;

pub(in crate::network) fn prepare_mempool_recovery_from_inputs(
    snapshot: &MempoolSnapshot,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    config: PolicyConfig,
    startup_at: PolicyTime,
    authority_epoch: AuthorityEpoch,
) -> Result<PreparedMempoolRecovery, ManagedNetworkError> {
    staging::prepare_mempool_recovery(
        snapshot,
        chainstate,
        verify_flags,
        consensus_params,
        config,
        startup_at,
        authority_epoch,
    )
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManagedMempoolRecoverySummary {
    pub recovered_count: u64,
    pub dropped_confirmed_count: u64,
    pub dropped_duplicate_count: u64,
    pub dropped_missing_parent_count: u64,
    pub dropped_policy_incompatible_count: u64,
    pub dropped_expired_count: u64,
    pub dropped_evicted_count: u64,
    pub records: Vec<MempoolRecoveryRecord>,
}

impl ManagedMempoolRecoverySummary {
    pub fn from_records(records: Vec<MempoolRecoveryRecord>) -> Self {
        let mut summary = Self {
            records,
            ..Self::default()
        };

        for record in &summary.records {
            match record.status {
                MempoolRecoveryStatus::Recovered => summary.recovered_count += 1,
                MempoolRecoveryStatus::DroppedConfirmed => {
                    summary.dropped_confirmed_count += 1;
                }
                MempoolRecoveryStatus::DroppedDuplicate => {
                    summary.dropped_duplicate_count += 1;
                }
                MempoolRecoveryStatus::DroppedMissingParent => {
                    summary.dropped_missing_parent_count += 1;
                }
                MempoolRecoveryStatus::DroppedPolicyIncompatible => {
                    summary.dropped_policy_incompatible_count += 1;
                }
                MempoolRecoveryStatus::DroppedExpired => summary.dropped_expired_count += 1,
                MempoolRecoveryStatus::DroppedEvicted => summary.dropped_evicted_count += 1,
            }
        }

        summary
    }
}

impl From<&ManagedMempoolRecoverySummary> for RelayRecoveryCounters {
    fn from(summary: &ManagedMempoolRecoverySummary) -> Self {
        Self {
            recovered_count: summary.recovered_count,
            dropped_confirmed_count: summary.dropped_confirmed_count,
            dropped_duplicate_count: summary.dropped_duplicate_count,
            dropped_missing_parent_count: summary.dropped_missing_parent_count,
            dropped_policy_incompatible_count: summary.dropped_policy_incompatible_count,
            dropped_expired_count: summary.dropped_expired_count,
            dropped_evicted_count: summary.dropped_evicted_count,
        }
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    #[allow(dead_code)] // Installed by the atomic startup cutover in Plan 135-03.
    pub(crate) fn prepare_mempool_recovery_at(
        &self,
        snapshot: &MempoolSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        startup_at: open_bitcoin_mempool::PolicyTime,
    ) -> Result<PreparedMempoolRecovery, ManagedNetworkError> {
        prepare_mempool_recovery_from_inputs(
            snapshot,
            &self.chainstate.export_chainstate_snapshot()?,
            verify_flags,
            consensus_params,
            self.mempool.mempool().config().clone(),
            startup_at,
            self.authority_epoch(),
        )
    }

    pub fn latest_mempool_recovery_summary(&self) -> Option<ManagedMempoolRecoverySummary> {
        self.latest_mempool_recovery.clone()
    }

    pub fn record_mempool_recovery_storage_error(&mut self, error: &StorageError) {
        self.record_mempool_recovery_unavailable(error.recovery_category());
    }

    pub fn record_mempool_recovery_unavailable(&mut self, category: SyncRecoveryCategory) {
        self.latest_mempool_recovery = None;
        self.latest_mempool_recovery_storage_error = Some(category);
    }

    pub fn latest_mempool_recovery_storage_error(&self) -> Option<SyncRecoveryCategory> {
        self.latest_mempool_recovery_storage_error
    }
}
