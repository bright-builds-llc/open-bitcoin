// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Thin authority facades for outside-lock lifecycle effects.

use open_bitcoin_network::PeerId;

use super::{LifecycleCommandResult, ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::network::lifecycle_projection::{
    LifecycleCommand, LifecycleProjectionError, PeerRelayPreparationRequest,
    SnapshotPreparationRequest,
};
use crate::network::{
    PeerEmissionReceipt,
    lifecycle_effects::{
        EffectCompletion, PeerEffectCapability, PeerEffectReceipt, PreparedSnapshotWrite,
        SnapshotWriteReceipt,
    },
};

impl From<LifecycleProjectionError> for ManagedNetworkAuthorityError {
    fn from(value: LifecycleProjectionError) -> Self {
        match value {
            LifecycleProjectionError::AuthorityUnavailable => Self::Poisoned,
            error => Self::LifecycleEffect(error.to_string()),
        }
    }
}

impl ManagedNetworkHandle {
    /// Reserves one peer-bound success capability through the lifecycle dispatcher.
    pub fn prepare_peer_relay_effect(
        &self,
        peer_id: PeerId,
    ) -> Result<PeerEffectCapability, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::PrepareRelay(
                PeerRelayPreparationRequest::new(peer_id),
            ))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::RelayPrepared(capability) => Ok(capability),
            _ => Err(unexpected_result("peer relay preparation")),
        }
    }

    /// Captures one owned current-schema snapshot through the lifecycle dispatcher.
    pub fn prepare_mempool_snapshot_write(
        &self,
    ) -> Result<PreparedSnapshotWrite, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::PrepareSnapshot(
                SnapshotPreparationRequest::new(),
            ))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::SnapshotPrepared(prepared) => Ok(prepared),
            _ => Err(unexpected_result("mempool snapshot preparation")),
        }
    }

    /// Classifies one achieved peer write through the lifecycle dispatcher.
    ///
    /// ```compile_fail
    /// # use open_bitcoin_node::network::{ManagedNetworkHandle, SnapshotWriteReceipt};
    /// fn wrong_family(
    ///     handle: &ManagedNetworkHandle,
    ///     receipt: SnapshotWriteReceipt,
    /// ) {
    ///     let _ = handle.complete_peer_effect(receipt);
    /// }
    /// ```
    pub fn complete_peer_effect(
        &self,
        receipt: PeerEffectReceipt,
    ) -> Result<EffectCompletion, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::CompletePeerEffect(receipt))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::PeerEffectCompleted(completion) => Ok(completion),
            _ => Err(unexpected_result("peer effect completion")),
        }
    }

    /// Classifies one achieved emission and records relay evidence only when current.
    pub fn complete_peer_emission(
        &self,
        receipt: PeerEmissionReceipt,
    ) -> Result<EffectCompletion, ManagedNetworkAuthorityError> {
        let peer_id = receipt.peer_id();
        let (effect_receipt, evidence) = receipt.into_parts();
        let completion = self.complete_peer_effect(effect_receipt)?;
        if completion == EffectCompletion::Applied {
            self.try_mutate(|network| network.record_peer_emission(peer_id, evidence))?;
        }
        Ok(completion)
    }

    /// Classifies one achieved snapshot write through the lifecycle dispatcher.
    ///
    /// ```compile_fail
    /// # use open_bitcoin_node::network::{ManagedNetworkHandle, PeerEffectReceipt};
    /// fn wrong_family(handle: &ManagedNetworkHandle, receipt: PeerEffectReceipt) {
    ///     let _ = handle.complete_snapshot_write(receipt);
    /// }
    /// ```
    pub fn complete_snapshot_write(
        &self,
        receipt: SnapshotWriteReceipt,
    ) -> Result<EffectCompletion, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::CompleteSnapshotEffect(receipt))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::SnapshotEffectCompleted(completion) => Ok(completion),
            _ => Err(unexpected_result("snapshot effect completion")),
        }
    }
}

fn unexpected_result(operation: &str) -> ManagedNetworkAuthorityError {
    ManagedNetworkAuthorityError::LifecycleEffect(format!(
        "lifecycle dispatcher returned an unexpected result for {operation}"
    ))
}
