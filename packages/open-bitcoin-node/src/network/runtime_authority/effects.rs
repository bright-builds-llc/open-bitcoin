// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Thin authority facades for outside-lock lifecycle effects.

#[cfg(test)]
use std::cell::Cell;

use open_bitcoin_mempool::PolicyTime;
use open_bitcoin_network::PeerId;

use super::{LifecycleCommandResult, ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::network::lifecycle_projection::{
    LifecycleCommand, LifecycleProjectionError, PeerRelayPreparationRequest,
    SnapshotPreparationRequest,
};
use crate::network::recovery::{ManagedMempoolRecoverySummary, PreparedMempoolRecovery};
use crate::network::{
    CheckpointEvidenceSnapshot, PeerEmissionReceipt, PeerEmissionWriteCapability,
    lifecycle_effects::{
        CheckpointTrigger, EffectAbort, EffectCompletion, PeerEffectCapability, PeerEffectReceipt,
        PreparedSnapshotWrite, SnapshotWriteAbort, SnapshotWriteFailure, SnapshotWriteReceipt,
    },
};

#[derive(Debug)]
pub struct CheckpointAbortDispatchError {
    source: ManagedNetworkAuthorityError,
    abort: Box<SnapshotWriteAbort>,
}

impl CheckpointAbortDispatchError {
    pub const fn failure(&self) -> SnapshotWriteFailure {
        SnapshotWriteFailure::AbortDispatch
    }

    pub(crate) fn into_parts(self) -> (ManagedNetworkAuthorityError, SnapshotWriteAbort) {
        (self.source, *self.abort)
    }
}

impl std::fmt::Display for CheckpointAbortDispatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(formatter)
    }
}

impl std::error::Error for CheckpointAbortDispatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
pub struct CheckpointCompletionDispatchError {
    source: ManagedNetworkAuthorityError,
    receipt: Box<SnapshotWriteReceipt>,
}

impl CheckpointCompletionDispatchError {
    pub const fn failure(&self) -> SnapshotWriteFailure {
        SnapshotWriteFailure::AbortDispatch
    }

    pub fn into_receipt(self) -> SnapshotWriteReceipt {
        *self.receipt
    }

    pub(in crate::network) fn into_parts(
        self,
    ) -> (ManagedNetworkAuthorityError, SnapshotWriteReceipt) {
        (self.source, *self.receipt)
    }
}

impl std::fmt::Display for CheckpointCompletionDispatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(formatter)
    }
}

impl std::error::Error for CheckpointCompletionDispatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl From<LifecycleProjectionError> for ManagedNetworkAuthorityError {
    fn from(value: LifecycleProjectionError) -> Self {
        match value {
            LifecycleProjectionError::AuthorityUnavailable => Self::Poisoned,
            error => Self::LifecycleEffect(error.to_string()),
        }
    }
}

impl<S: crate::ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>
    ManagedNetworkHandle<S, V>
{
    #[cfg(test)]
    pub(crate) fn mark_checkpoint_dirty_for_test(
        &self,
    ) -> Result<u64, ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            let next = network
                .lifecycle_generation
                .checked_next()
                .map_err(|error| {
                    ManagedNetworkAuthorityError::LifecycleEffect(error.to_string())
                })?;
            network.lifecycle_generation = next;
            network.dirty_generation = Some(next);
            Ok(next.raw())
        })?
    }

    /// Atomically consumes one authority-bound startup recovery candidate.
    pub fn install_mempool_recovery(
        &self,
        prepared: PreparedMempoolRecovery,
    ) -> Result<ManagedMempoolRecoverySummary, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::InstallRecovery(prepared))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::RecoveryInstalled(summary) => Ok(summary),
            _ => Err(unexpected_result("mempool recovery installation")),
        }
    }

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
        captured_at: PolicyTime,
        trigger: CheckpointTrigger,
    ) -> Result<PreparedSnapshotWrite, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::PrepareSnapshot(
                SnapshotPreparationRequest::new(captured_at, trigger),
            ))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::SnapshotPrepared(prepared) => Ok(prepared),
            _ => Err(unexpected_result("mempool snapshot preparation")),
        }
    }

    /// Releases one exact peer reservation when no external effect was achieved.
    pub fn abort_peer_effect(
        &self,
        capability: PeerEffectCapability,
    ) -> Result<EffectAbort, ManagedNetworkAuthorityError> {
        match self
            .apply_lifecycle_command(LifecycleCommand::AbortPeerEffect(capability))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::PeerEffectAborted(abort) => Ok(abort),
            _ => Err(unexpected_result("peer effect abort")),
        }
    }

    /// Releases one exact prepared peer emission before any write was achieved.
    pub fn abort_peer_emission(
        &self,
        capability: PeerEmissionWriteCapability,
    ) -> Result<EffectAbort, ManagedNetworkAuthorityError> {
        self.abort_peer_effect(capability.into_effect_capability())
    }

    /// Releases one exact typed checkpoint attempt before persistence succeeds.
    pub fn abort_snapshot_write(
        &self,
        abort: SnapshotWriteAbort,
    ) -> Result<EffectAbort, CheckpointAbortDispatchError> {
        if take_injected_checkpoint_abort_dispatch_failure() {
            return Err(CheckpointAbortDispatchError {
                source: ManagedNetworkAuthorityError::Poisoned,
                abort: Box::new(abort),
            });
        }
        self.dispatch_checkpoint_abort(abort)
            .map_err(|(error, abort)| CheckpointAbortDispatchError {
                source: ManagedNetworkAuthorityError::from(error),
                abort,
            })
    }

    #[cfg(test)]
    pub(crate) fn fail_next_checkpoint_abort_dispatch_for_test(&self) {
        INJECT_CHECKPOINT_ABORT_DISPATCH_FAILURE.set(true);
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
        match self
            .apply_lifecycle_command(LifecycleCommand::CompletePeerEmission(receipt))
            .map_err(ManagedNetworkAuthorityError::from)?
        {
            LifecycleCommandResult::PeerEffectCompleted(completion) => Ok(completion),
            _ => Err(unexpected_result("peer emission completion")),
        }
    }

    /// Records one achieved typed checkpoint while retaining its receipt on dispatch failure.
    pub fn complete_snapshot_write(
        &self,
        receipt: SnapshotWriteReceipt,
    ) -> Result<EffectCompletion, CheckpointCompletionDispatchError> {
        self.dispatch_checkpoint_completion(receipt)
            .map_err(|(error, receipt)| CheckpointCompletionDispatchError {
                source: ManagedNetworkAuthorityError::from(error),
                receipt,
            })
    }

    /// Returns bounded durability truth without exposing snapshot contents or member identities.
    pub fn checkpoint_evidence(
        &self,
        now: PolicyTime,
        periodic_interval_seconds: u64,
    ) -> Result<CheckpointEvidenceSnapshot, ManagedNetworkAuthorityError> {
        let network = self
            .authority
            .lock()
            .map_err(|_| ManagedNetworkAuthorityError::Poisoned)?;
        Ok(network.checkpoint_evidence.snapshot(
            network.lifecycle_generation,
            network.dirty_generation,
            now,
            periodic_interval_seconds,
        ))
    }
}

#[cfg(test)]
thread_local! {
    static INJECT_CHECKPOINT_ABORT_DISPATCH_FAILURE: Cell<bool> = const { Cell::new(false) };
}

#[cfg(test)]
fn take_injected_checkpoint_abort_dispatch_failure() -> bool {
    INJECT_CHECKPOINT_ABORT_DISPATCH_FAILURE.replace(false)
}

#[cfg(not(test))]
const fn take_injected_checkpoint_abort_dispatch_failure() -> bool {
    false
}

fn unexpected_result(operation: &str) -> ManagedNetworkAuthorityError {
    ManagedNetworkAuthorityError::LifecycleEffect(format!(
        "lifecycle dispatcher returned an unexpected result for {operation}"
    ))
}
