// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_mempool::PolicyTime;

use crate::{StorageError, status::SyncRecoveryCategory, storage::MempoolSnapshot};

use super::{
    super::{ManagedMempoolRecoverySummary, PreparedMempoolRecovery},
    ManagedNetworkAuthorityError, ManagedNetworkHandle, ManagedPeerNetwork,
};

impl ManagedNetworkHandle {
    /// Stages startup recovery after releasing the authority guard.
    pub fn prepare_mempool_recovery_at(
        &self,
        snapshot: &MempoolSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        startup_at: PolicyTime,
    ) -> Result<PreparedMempoolRecovery, ManagedNetworkAuthorityError> {
        let (chainstate, config, authority_epoch) = self.read(|network| {
            (
                network.chainstate_snapshot(),
                network.mempool().mempool().config().clone(),
                network.authority_epoch(),
            )
        })?;
        crate::network::recovery::prepare_mempool_recovery_from_inputs(
            snapshot,
            &chainstate,
            verify_flags,
            consensus_params,
            config,
            startup_at,
            authority_epoch,
        )
        .map_err(ManagedNetworkAuthorityError::from)
    }

    pub fn record_mempool_recovery_storage_error(
        &self,
        error: &StorageError,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_mempool_recovery_storage_error(error))
    }

    pub fn record_mempool_recovery_unavailable(
        &self,
        category: SyncRecoveryCategory,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_mempool_recovery_unavailable(category))
    }

    pub fn latest_mempool_recovery_summary(
        &self,
    ) -> Result<Option<ManagedMempoolRecoverySummary>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::latest_mempool_recovery_summary)
    }

    pub fn latest_mempool_recovery_storage_error(
        &self,
    ) -> Result<Option<SyncRecoveryCategory>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::latest_mempool_recovery_storage_error)
    }
}
