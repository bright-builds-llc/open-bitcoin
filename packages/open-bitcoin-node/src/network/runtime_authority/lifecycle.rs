// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sole mutex dispatcher for typed lifecycle commands.

#[cfg(test)]
use std::cell::Cell;

use open_bitcoin_mempool::{
    MempoolCapacityBounds, MempoolLifecycleDelta, MempoolRetryClear, MempoolRetryClearCause,
};

use super::ManagedNetworkHandle;
use crate::network::announcement_transport::PeerEmissionEvidence;
use crate::network::lifecycle_effects::{
    EffectAbort, EffectCompletion, ExactEffectLedgerCompletion, PeerEffectCapability,
    PeerEffectReceipt, PreparedSnapshotWrite, SnapshotWriteAbort, SnapshotWriteReceipt,
};
use crate::network::lifecycle_projection::{LifecycleCommand, LifecycleProjectionError};
use crate::network::recovery::ManagedMempoolRecoverySummary;
use crate::storage::mempool_snapshot::CapturedMempoolGeneration;
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{ChainstateStore, ManagedPeerNetwork};

pub(in crate::network) enum LifecycleCommandResult {
    Lifecycle(MempoolLifecycleDelta),
    RecoveryInstalled(ManagedMempoolRecoverySummary),
    SnapshotPrepared(PreparedSnapshotWrite),
    RelayPrepared(PeerEffectCapability),
    PeerEffectAborted(EffectAbort),
    #[cfg_attr(not(test), allow(dead_code))]
    SnapshotEffectAborted(EffectAbort),
    PeerEffectCompleted(EffectCompletion),
    #[cfg(test)]
    SnapshotEffectCompleted(EffectCompletion),
}

impl<S: crate::ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>
    ManagedNetworkHandle<S, V>
{
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

    pub(super) fn dispatch_checkpoint_abort(
        &self,
        abort: SnapshotWriteAbort,
    ) -> Result<EffectAbort, (LifecycleProjectionError, Box<SnapshotWriteAbort>)> {
        let mut network = match self.authority.lock() {
            Ok(network) => network,
            Err(_) => {
                return Err((
                    LifecycleProjectionError::AuthorityUnavailable,
                    Box::new(abort),
                ));
            }
        };
        match abort_checkpoint_snapshot_effect(&mut network, &abort) {
            Ok(classification) => Ok(classification),
            Err(error) => Err((error, Box::new(abort))),
        }
    }

    #[cfg(test)]
    pub(crate) fn fail_next_checkpoint_completion_dispatch_for_test(&self) {
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
pub(in crate::network) fn apply_lifecycle_command<
    S: ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
>(
    network: &mut ManagedPeerNetwork<S, V>,
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
        LifecycleCommand::InstallRecovery(prepared) => network
            .install_prepared_recovery(prepared)
            .map(LifecycleCommandResult::RecoveryInstalled),
        LifecycleCommand::PrepareSnapshot(request) => {
            let mempool = network.mempool().mempool();
            let max_records =
                MempoolCapacityBounds::from_capacity(mempool.config().mempool_capacity)
                    .max_live_entries();
            if mempool.entries().len() > max_records {
                return Err(LifecycleProjectionError::MempoolSnapshot(
                    crate::storage::mempool_snapshot::MempoolSnapshotError::ResourceBoundExceeded,
                ));
            }
            let records = mempool
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
                CapturedMempoolGeneration::try_new(network.lifecycle_generation.raw())
                    .map_err(LifecycleProjectionError::MempoolSnapshot)?,
                request.captured_at,
                records,
                network.unbroadcast_members().clone(),
            )
            .map_err(LifecycleProjectionError::MempoolSnapshot)?;
            crate::storage::snapshot_codec::assert_mempool_snapshot_representable(&snapshot)
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
        LifecycleCommand::AbortSnapshotEffect(abort_request) => {
            abort_checkpoint_snapshot_effect(network, &abort_request)
                .map(LifecycleCommandResult::SnapshotEffectAborted)
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
        #[cfg(test)]
        LifecycleCommand::CompleteSnapshotEffect(receipt) => {
            complete_checkpoint_snapshot_effect(network, &receipt)
                .map(LifecycleCommandResult::SnapshotEffectCompleted)
        }
    }
}

fn abort_checkpoint_snapshot_effect<
    S: ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
>(
    network: &mut ManagedPeerNetwork<S, V>,
    abort_request: &SnapshotWriteAbort,
) -> Result<EffectAbort, LifecycleProjectionError> {
    let capability = abort_request.capability();
    let generation = capability.persistence_generation();
    let abort = network.snapshot_effect_ledger.abort_exact(capability);
    if abort == EffectAbort::Aborted {
        network.checkpoint_evidence.note_aborted(
            generation,
            abort_request.failed_at(),
            abort_request.failure(),
        );
    }
    Ok(abort)
}

fn complete_checkpoint_snapshot_effect<
    S: ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
>(
    network: &mut ManagedPeerNetwork<S, V>,
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

fn complete_peer_effect<S: ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>(
    network: &mut ManagedPeerNetwork<S, V>,
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
    if is_fresh {
        apply_fresh_tx_response_transport_written(network, maybe_evidence);
        Ok(EffectCompletion::Applied)
    } else {
        Ok(EffectCompletion::AchievedButStale)
    }
}

fn apply_fresh_tx_response_transport_written<
    S: ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
>(
    network: &mut ManagedPeerNetwork<S, V>,
    maybe_evidence: Option<PeerEmissionEvidence>,
) {
    let Some(evidence) = maybe_evidence else {
        return;
    };
    if !evidence.is_transaction_response() {
        return;
    }
    let Some(member) = evidence.maybe_member() else {
        return;
    };
    let clear = MempoolRetryClear {
        member,
        cause: MempoolRetryClearCause::TransportWritten,
    };
    network.unbroadcast_members.remove(&clear.member);
    network.lifecycle_evidence.retry_clears =
        network.lifecycle_evidence.retry_clears.saturating_add(1);
    network.maybe_last_transport_written_clear = Some(clear);
}
