// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sole mutex dispatcher for typed lifecycle commands.

use open_bitcoin_mempool::MempoolLifecycleDelta;

use super::ManagedNetworkHandle;
use crate::network::lifecycle_effects::{
    EffectCompletion, PeerEffectCapability, PreparedSnapshotWrite, SnapshotIdentity,
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
            let effect_id = network.snapshot_effect_ledger.reserve_next()?;
            let snapshot_identity = SnapshotIdentity::from_effect_id(effect_id);
            let snapshot = MempoolSnapshot::from_mempool(network.mempool().mempool());
            Ok(LifecycleCommandResult::SnapshotPrepared(
                PreparedSnapshotWrite::new(
                    network.authority_epoch,
                    network.lifecycle_generation,
                    effect_id,
                    snapshot_identity,
                    snapshot,
                ),
            ))
        }
        LifecycleCommand::PrepareRelay(request) => {
            let effect_id = network.peer_effect_ledger.reserve_next()?;
            Ok(LifecycleCommandResult::RelayPrepared(
                PeerEffectCapability::new(
                    network.authority_epoch,
                    network.lifecycle_generation,
                    effect_id,
                    request.peer_id,
                    network.peer_session_generation,
                ),
            ))
        }
        LifecycleCommand::CompletePeerEffect(receipt) => {
            let effect_id = receipt.effect_id();
            if network.peer_effect_ledger.is_completed(effect_id) {
                return Ok(LifecycleCommandResult::PeerEffectCompleted(
                    EffectCompletion::AlreadyApplied,
                ));
            }
            let was_pending = network.peer_effect_ledger.is_pending(effect_id);
            network.peer_effect_ledger.record_completed(effect_id);
            let is_fresh = was_pending
                && receipt.authority_epoch() == network.authority_epoch
                && receipt.lifecycle_generation() == network.lifecycle_generation
                && receipt.peer_session_generation() == network.peer_session_generation;
            let completion = if is_fresh {
                EffectCompletion::Applied
            } else {
                EffectCompletion::AchievedButStale
            };
            Ok(LifecycleCommandResult::PeerEffectCompleted(completion))
        }
        LifecycleCommand::CompleteSnapshotEffect(receipt) => {
            let effect_id = receipt.effect_id();
            if network.snapshot_effect_ledger.is_completed(effect_id) {
                return Ok(LifecycleCommandResult::SnapshotEffectCompleted(
                    EffectCompletion::AlreadyApplied,
                ));
            }
            let was_pending = network.snapshot_effect_ledger.is_pending(effect_id);
            network.snapshot_effect_ledger.record_completed(effect_id);
            let is_fresh = was_pending
                && receipt.authority_epoch() == network.authority_epoch
                && receipt.persistence_generation() == network.lifecycle_generation
                && receipt.snapshot_identity() == SnapshotIdentity::from_effect_id(effect_id);
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
