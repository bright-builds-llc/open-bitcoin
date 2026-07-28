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

#[allow(dead_code)] // Plans 134-06 through 134-08 route lifecycle and effect facades here.
pub(super) enum LifecycleCommandResult {
    Lifecycle(MempoolLifecycleDelta),
    SnapshotPrepared(OwnedSnapshotEffect),
    RelayPrepared(OwnedPeerRelayEffects),
    PeerEffectCompleted,
    SnapshotEffectCompleted,
}

impl ManagedNetworkHandle {
    #[allow(dead_code)] // Plans 134-06 through 134-08 route lifecycle and effect facades here.
    pub(super) fn apply_lifecycle_command(
        &self,
        command: LifecycleCommand,
    ) -> Result<LifecycleCommandResult, LifecycleProjectionError> {
        let mut network = self
            .authority
            .lock()
            .map_err(|_| LifecycleProjectionError::AuthorityUnavailable)?;
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
            LifecycleCommand::PrepareSnapshot(request) => Ok(
                LifecycleCommandResult::SnapshotPrepared(request.into_owned()),
            ),
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
}
