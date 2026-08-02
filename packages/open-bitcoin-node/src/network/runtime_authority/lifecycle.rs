// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sole mutex dispatcher for typed lifecycle commands.

#[cfg(test)]
use std::cell::Cell;

use open_bitcoin_mempool::MempoolLifecycleDelta;

use super::ManagedNetworkHandle;
use crate::network::announcement_transport::PeerEmissionEvidence;
use crate::network::lifecycle_effects::{
    EffectAbort, EffectCompletion, ExactEffectLedgerCompletion, PeerEffectCapability,
    PeerEffectReceipt, PreparedSnapshotWrite, SnapshotWriteReceipt,
};
use crate::network::lifecycle_projection::{LifecycleCommand, LifecycleProjectionError};
use crate::storage::mempool_snapshot::CapturedMempoolGeneration;
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{ChainstateStore, ManagedPeerNetwork};

pub(in crate::network) enum LifecycleCommandResult {
    Lifecycle(MempoolLifecycleDelta),
    SnapshotPrepared(PreparedSnapshotWrite),
    RelayPrepared(PeerEffectCapability),
    PeerEffectAborted(EffectAbort),
    SnapshotEffectAborted(EffectAbort),
    PeerEffectCompleted(EffectCompletion),
    SnapshotEffectCompleted(EffectCompletion),
}

impl ManagedNetworkHandle {
    pub(super) fn apply_lifecycle_command(
        &self,
        command: LifecycleCommand,
    ) -> Result<LifecycleCommandResult, LifecycleProjectionError> {
        let mut network = self
            .authority
            .lock()
            .map_err(|_| LifecycleProjectionError::AuthorityUnavailable)?;
        apply_lifecycle_command(&mut network, command)
    }

    pub(super) fn dispatch_checkpoint_completion(
        &self,
        receipt: SnapshotWriteReceipt,
    ) -> Result<EffectCompletion, (LifecycleProjectionError, Box<SnapshotWriteReceipt>)> {
        if take_injected_checkpoint_completion_dispatch_failure() {
            return Err((
                LifecycleProjectionError::AuthorityUnavailable,
                Box::new(receipt),
            ));
        }
        let mut network = match self.authority.lock() {
            Ok(network) => network,
            Err(_) => {
                return Err((
                    LifecycleProjectionError::AuthorityUnavailable,
                    Box::new(receipt),
                ));
            }
        };
        match complete_checkpoint_snapshot_effect(&mut network, &receipt) {
            Ok(completion) => Ok(completion),
            Err(error) => Err((error, Box::new(receipt))),
        }
    }

    #[cfg(test)]
    pub(in crate::network) fn fail_next_checkpoint_completion_dispatch_for_test(&self) {
        INJECT_CHECKPOINT_COMPLETION_DISPATCH_FAILURE.set(true);
    }
}

#[cfg(test)]
thread_local! {
    static INJECT_CHECKPOINT_COMPLETION_DISPATCH_FAILURE: Cell<bool> = const { Cell::new(false) };
}

#[cfg(test)]
fn take_injected_checkpoint_completion_dispatch_failure() -> bool {
    INJECT_CHECKPOINT_COMPLETION_DISPATCH_FAILURE.replace(false)
}

#[cfg(not(test))]
const fn take_injected_checkpoint_completion_dispatch_failure() -> bool {
    false
}

/// Dispatches one typed lifecycle command while the caller holds the sole authority guard.
pub(in crate::network) fn apply_lifecycle_command<S: ChainstateStore>(
    network: &mut ManagedPeerNetwork<S>,
    command: LifecycleCommand,
) -> Result<LifecycleCommandResult, LifecycleProjectionError> {
    match command {
        LifecycleCommand::SingletonAdmission(plan)
        | LifecycleCommand::PackageAdmission(plan)
        | LifecycleCommand::Pressure(plan)
        | LifecycleCommand::Expiry(plan)
        | LifecycleCommand::ConnectedBlock(plan)
        | LifecycleCommand::ReorgStep(plan)
        | LifecycleCommand::Maintenance(plan) => {
            let sealed = network.validate_prepared_lifecycle(plan)?;
            let delta = network.commit_sealed_lifecycle(sealed)?;
            Ok(LifecycleCommandResult::Lifecycle(delta))
        }
        LifecycleCommand::PrepareSnapshot(request) => {
            let records = network
                .mempool()
                .mempool()
                .entries()
                .values()
                .map(|entry| {
                    MempoolSnapshotRecord::try_from_canonical(
                        entry.transaction.clone(),
                        entry.metadata.accepted_at,
                    )
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(LifecycleProjectionError::MempoolSnapshot)?;
            let snapshot = MempoolSnapshot::try_new_current(
                CapturedMempoolGeneration::new(network.lifecycle_generation.raw()),
                request.captured_at,
                records,
                network.unbroadcast_members().clone(),
            )
            .map_err(LifecycleProjectionError::MempoolSnapshot)?;
            let prepared = network.snapshot_effect_ledger.reserve_next(
                network.authority_epoch,
                network.lifecycle_generation,
                request.captured_at,
                request.trigger,
                snapshot,
            )?;
            network.checkpoint_evidence.note_prepared(
                network.lifecycle_generation,
                request.captured_at,
                request.trigger,
            );
            Ok(LifecycleCommandResult::SnapshotPrepared(prepared))
        }
        LifecycleCommand::PrepareRelay(request) => Ok(LifecycleCommandResult::RelayPrepared(
            network.peer_effect_ledger.reserve_next(
                network.authority_epoch,
                network.lifecycle_generation,
                request.peer_id,
                network.peer_session_generation(request.peer_id),
            )?,
        )),
        LifecycleCommand::AbortPeerEffect(capability) => {
            if capability.authority_epoch() != network.authority_epoch {
                return Ok(LifecycleCommandResult::PeerEffectAborted(
                    EffectAbort::NotPending,
                ));
            }
            let peer_id = capability.peer_id();
            let abort = network.peer_effect_ledger.abort_exact(&capability);
            if abort == EffectAbort::Aborted {
                network.maybe_forget_peer_session_generation(peer_id);
            }
            Ok(LifecycleCommandResult::PeerEffectAborted(abort))
        }
        LifecycleCommand::AbortSnapshotEffect(capability) => {
            let generation = capability.persistence_generation();
            let abort = network.snapshot_effect_ledger.abort_exact(&capability);
            if abort == EffectAbort::Aborted {
                network
                    .checkpoint_evidence
                    .clear_compatibility_binding(generation);
            }
            Ok(LifecycleCommandResult::SnapshotEffectAborted(abort))
        }
        LifecycleCommand::AbortCheckpointSnapshotEffect(abort_request) => {
            let generation = abort_request.capability().persistence_generation();
            let (capability, failed_at, failure) = abort_request.into_parts();
            let abort = network.snapshot_effect_ledger.abort_exact(&capability);
            if abort == EffectAbort::Aborted {
                network
                    .checkpoint_evidence
                    .note_aborted(generation, failed_at, failure);
            }
            Ok(LifecycleCommandResult::SnapshotEffectAborted(abort))
        }
        LifecycleCommand::CompletePeerEffect(receipt) => {
            complete_peer_effect(network, receipt, None)
                .map(LifecycleCommandResult::PeerEffectCompleted)
        }
        LifecycleCommand::CompletePeerEmission(receipt) => {
            let (effect_receipt, evidence) = receipt.into_parts();
            complete_peer_effect(network, effect_receipt, Some(evidence))
                .map(LifecycleCommandResult::PeerEffectCompleted)
        }
        LifecycleCommand::CompleteSnapshotEffect(receipt) => {
            let effect_id = receipt.exact_key();
            if network.snapshot_effect_ledger.is_completed(effect_id) {
                return Ok(LifecycleCommandResult::SnapshotEffectCompleted(
                    EffectCompletion::AlreadyApplied,
                ));
            }
            let exact_completion = network.snapshot_effect_ledger.complete_exact(&receipt);
            if exact_completion == ExactEffectLedgerCompletion::NotPending {
                return Err(LifecycleProjectionError::InvalidEffectReceipt("snapshot"));
            }
            let is_fresh = receipt.authority_epoch() == network.authority_epoch
                && receipt.persistence_generation() == network.lifecycle_generation;
            network
                .checkpoint_evidence
                .clear_compatibility_binding(receipt.persistence_generation());
            let completion = if is_fresh {
                if network.dirty_generation == Some(receipt.persistence_generation()) {
                    network.dirty_generation = None;
                }
                EffectCompletion::Applied
            } else {
                EffectCompletion::AchievedButStale
            };
            Ok(LifecycleCommandResult::SnapshotEffectCompleted(completion))
        }
        LifecycleCommand::CompleteCheckpointSnapshotEffect(receipt) => {
            complete_checkpoint_snapshot_effect(network, &receipt)
                .map(LifecycleCommandResult::SnapshotEffectCompleted)
        }
    }
}

fn complete_checkpoint_snapshot_effect<S: ChainstateStore>(
    network: &mut ManagedPeerNetwork<S>,
    receipt: &SnapshotWriteReceipt,
) -> Result<EffectCompletion, LifecycleProjectionError> {
    if receipt.completed_at().is_none() || receipt.persistence_strength().is_none() {
        return Err(LifecycleProjectionError::InvalidEffectReceipt(
            "typed snapshot",
        ));
    }
    let effect_id = receipt.exact_key();
    if network.snapshot_effect_ledger.is_completed(effect_id) {
        return Ok(EffectCompletion::AlreadyApplied);
    }
    let exact_completion = network.snapshot_effect_ledger.complete_exact(receipt);
    if exact_completion != ExactEffectLedgerCompletion::Recorded {
        return Err(LifecycleProjectionError::InvalidEffectReceipt(
            "typed snapshot",
        ));
    }
    network.checkpoint_evidence.note_completed(receipt);
    let is_fresh = receipt.authority_epoch() == network.authority_epoch
        && receipt.persistence_generation() == network.lifecycle_generation;
    if is_fresh && network.dirty_generation == Some(receipt.persistence_generation()) {
        network.dirty_generation = None;
    }
    Ok(if is_fresh {
        EffectCompletion::Applied
    } else {
        EffectCompletion::AchievedButStale
    })
}

fn complete_peer_effect<S: ChainstateStore>(
    network: &mut ManagedPeerNetwork<S>,
    receipt: PeerEffectReceipt,
    maybe_evidence: Option<PeerEmissionEvidence>,
) -> Result<EffectCompletion, LifecycleProjectionError> {
    let effect_id = receipt.exact_key();
    if network.peer_effect_ledger.is_completed(effect_id) {
        return Ok(EffectCompletion::AlreadyApplied);
    }
    if !network.peer_effect_ledger.is_pending(&receipt) {
        return Err(LifecycleProjectionError::InvalidEffectReceipt("peer"));
    }

    let peer_id = receipt.peer_id();
    let is_fresh = receipt.authority_epoch() == network.authority_epoch
        && receipt.lifecycle_generation() == network.lifecycle_generation
        && receipt.peer_session_generation() == network.peer_session_generation(peer_id);
    if is_fresh && let Some(evidence) = maybe_evidence {
        network
            .record_peer_emission(peer_id, evidence)
            .map_err(LifecycleProjectionError::PeerEvidence)?;
    }

    let exact_completion = network.peer_effect_ledger.complete_exact(&receipt);
    if exact_completion != ExactEffectLedgerCompletion::Recorded {
        return Err(LifecycleProjectionError::InvalidEffectReceipt("peer"));
    }
    network.maybe_forget_peer_session_generation(peer_id);
    Ok(if is_fresh {
        EffectCompletion::Applied
    } else {
        EffectCompletion::AchievedButStale
    })
}
