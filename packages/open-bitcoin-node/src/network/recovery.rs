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

use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::OutPoint,
};
use open_bitcoin_mempool::{
    AdmissionContext, MempoolEntryMetadata, MempoolOrigin, PolicyConfig, PolicyTime, RelayIntent,
};
use open_bitcoin_network::TxServingRecordStatus;

use crate::ChainstateStore;
use crate::status::{SyncRecoveryCategory, relay_evidence::RelayRecoveryCounters};
use crate::storage::mempool_snapshot::recovery_status_from_outcome;
use crate::storage::{MempoolRecoveryRecord, MempoolRecoveryStatus, MempoolSnapshot, StorageError};

use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::network::lifecycle_projection::AuthorityEpoch;
use topology::{RecoveryTopologyLimits, prepare_recovery_topology};

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
            dropped_evicted_count: summary.dropped_evicted_count,
        }
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
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
            &self.chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
            self.mempool.mempool().config().clone(),
            startup_at,
            self.authority_epoch(),
        )
    }

    pub fn recover_mempool_snapshot(
        &mut self,
        snapshot: &MempoolSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedMempoolRecoverySummary, ManagedNetworkError> {
        let topology =
            prepare_recovery_topology(&snapshot.records, RecoveryTopologyLimits::standard())
                .map_err(|_| {
                    ManagedNetworkError::LifecycleEffect(
                        "mempool recovery topology preparation failed",
                    )
                })?;
        let (records, mut recovery_records) = topology.into_parts();
        let chainstate = self.chainstate.chainstate().snapshot();
        recovery_records.reserve(records.len());

        for topology_record in records {
            let snapshot_record = topology_record.record;
            let member = topology_record.identity;
            let txid = member.txid;
            let wtxid = member.wtxid;
            let restored_unbroadcast = snapshot.unbroadcast_members().contains(&member);
            let metadata = MempoolEntryMetadata::new(
                snapshot_record.acceptance_time,
                if restored_unbroadcast {
                    MempoolOrigin::Local
                } else {
                    MempoolOrigin::RecoveryUnknown
                },
                if restored_unbroadcast {
                    RelayIntent::Requested
                } else {
                    RelayIntent::NotRequested
                },
            );
            let confirmed = (0..snapshot_record.transaction.outputs.len()).any(|index| {
                let Ok(vout) = u32::try_from(index) else {
                    return false;
                };
                chainstate.utxos.contains_key(&OutPoint { txid, vout })
            });
            let status = if confirmed {
                MempoolRecoveryStatus::DroppedConfirmed
            } else {
                let transition_result = self
                    .mempool
                    .mempool_mut()
                    .accept_transaction_transition_with_context(
                        snapshot_record.transaction.clone(),
                        &chainstate,
                        verify_flags,
                        consensus_params,
                        AdmissionContext::recovery(metadata),
                    );
                match transition_result {
                    Ok(transition) => {
                        let status = recovery_status_from_outcome(Ok(transition.outcome.clone()));
                        self.apply_admitted_transition(
                            &transition,
                            snapshot_record.transaction.clone(),
                        )?;
                        if status == MempoolRecoveryStatus::Recovered {
                            self.relay_fanout.seed_recovered_transaction(txid, wtxid);
                        }
                        status
                    }
                    Err(error) => recovery_status_from_outcome(Err(error)),
                }
            };

            match status {
                MempoolRecoveryStatus::Recovered | MempoolRecoveryStatus::DroppedDuplicate => {}
                MempoolRecoveryStatus::DroppedConfirmed => {
                    self.relay_serving.record_status(
                        txid,
                        Some(wtxid),
                        TxServingRecordStatus::Confirmed,
                    );
                }
                MempoolRecoveryStatus::DroppedMissingParent
                | MempoolRecoveryStatus::DroppedPolicyIncompatible => {
                    self.relay_serving.record_status(
                        txid,
                        Some(wtxid),
                        TxServingRecordStatus::Rejected,
                    );
                }
                MempoolRecoveryStatus::DroppedExpired | MempoolRecoveryStatus::DroppedEvicted => {
                    self.relay_serving.record_status(
                        txid,
                        Some(wtxid),
                        TxServingRecordStatus::Evicted,
                    );
                }
            }

            recovery_records.push(MempoolRecoveryRecord { txid, status });
        }

        let summary = ManagedMempoolRecoverySummary::from_records(recovery_records);
        self.latest_mempool_recovery = Some(summary.clone());
        self.latest_mempool_recovery_storage_error = None;
        Ok(summary)
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
