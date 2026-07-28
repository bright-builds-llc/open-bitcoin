// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sole mutex dispatcher for typed lifecycle commands.

use open_bitcoin_mempool::MempoolLifecycleDelta;

use super::ManagedNetworkHandle;
use crate::network::lifecycle_projection::{
    LifecycleCommand, LifecycleProjectionError, OwnedPeerRelayEffects, OwnedSnapshotEffect,
};
use crate::{ChainstateStore, ManagedPeerNetwork};

#[allow(dead_code)] // Effect variants are consumed by later Phase 134 dispatcher plans.
pub(in crate::network) enum LifecycleCommandResult {
    Lifecycle(MempoolLifecycleDelta),
    SnapshotPrepared(OwnedSnapshotEffect),
    RelayPrepared(OwnedPeerRelayEffects),
    PeerEffectCompleted,
    SnapshotEffectCompleted,
}

impl ManagedNetworkHandle {
    #[allow(dead_code)] // The handle facade is activated when owned effects leave the lock.
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
        LifecycleCommand::PrepareSnapshot(request) => Ok(LifecycleCommandResult::SnapshotPrepared(
            request.into_owned(),
        )),
        LifecycleCommand::PrepareRelay(request) => {
            Ok(LifecycleCommandResult::RelayPrepared(request.into_owned()))
        }
        LifecycleCommand::CompletePeerEffect(_receipt) => {
            Ok(LifecycleCommandResult::PeerEffectCompleted)
        }
        LifecycleCommand::CompleteSnapshotEffect(_receipt) => {
            Ok(LifecycleCommandResult::SnapshotEffectCompleted)
        }
    }
}
