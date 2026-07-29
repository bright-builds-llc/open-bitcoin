// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sole mutex dispatcher for typed lifecycle commands.

use open_bitcoin_mempool::MempoolLifecycleDelta;

use super::ManagedNetworkHandle;
use crate::network::lifecycle_effects::{
    EffectCompletion, ExactEffectLedgerCompletion, PeerEffectCapability, PreparedSnapshotWrite,
};
use crate::network::lifecycle_projection::{LifecycleCommand, LifecycleProjectionError};
use crate::storage::MempoolSnapshot;
use crate::{ChainstateStore, ManagedPeerNetwork};

pub(in crate::network) enum LifecycleCommandResult {
    Lifecycle(MempoolLifecycleDelta),
    SnapshotPrepared(PreparedSnapshotWrite),
    RelayPrepared(PeerEffectCapability),
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
            let delta = plan.core.facts().delta().clone();
            let validated = network.validate_prepared_lifecycle(plan)?;
            network.apply_prepared_lifecycle(validated);
            Ok(LifecycleCommandResult::Lifecycle(delta))
        }
        LifecycleCommand::PrepareSnapshot(_request) => {
            let snapshot = MempoolSnapshot::from_mempool(network.mempool().mempool());
            Ok(LifecycleCommandResult::SnapshotPrepared(
                network.snapshot_effect_ledger.reserve_next(
                    network.authority_epoch,
                    network.lifecycle_generation,
                    snapshot,
                )?,
            ))
        }
        LifecycleCommand::PrepareRelay(request) => Ok(LifecycleCommandResult::RelayPrepared(
            network.peer_effect_ledger.reserve_next(
                network.authority_epoch,
                network.lifecycle_generation,
                request.peer_id,
                network.peer_session_generation(request.peer_id),
            )?,
        )),
        LifecycleCommand::CompletePeerEffect(receipt) => {
            let peer_id = receipt.peer_id();
            let effect_id = receipt.exact_key();
            if network.peer_effect_ledger.is_completed(effect_id) {
                return Ok(LifecycleCommandResult::PeerEffectCompleted(
                    EffectCompletion::AlreadyApplied,
                ));
            }
            let exact_completion = network.peer_effect_ledger.complete_exact(&receipt);
            if exact_completion == ExactEffectLedgerCompletion::NotPending {
                return Err(LifecycleProjectionError::InvalidEffectReceipt("peer"));
            }
            let is_fresh = receipt.authority_epoch() == network.authority_epoch
                && receipt.lifecycle_generation() == network.lifecycle_generation
                && receipt.peer_session_generation() == network.peer_session_generation(peer_id);
            let completion = if is_fresh {
                EffectCompletion::Applied
            } else {
                EffectCompletion::AchievedButStale
            };
            network.maybe_forget_peer_session_generation(peer_id);
            Ok(LifecycleCommandResult::PeerEffectCompleted(completion))
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
    }
}
