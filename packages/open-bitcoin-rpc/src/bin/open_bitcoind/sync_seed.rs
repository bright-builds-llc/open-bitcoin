// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_node::{DurableSyncRuntime, SyncLifecycleState};

use super::{DaemonSyncPreflightError, current_timestamp_unix_seconds};

pub(super) fn seed_initial_sync_state(
    sync_runtime: &DurableSyncRuntime,
) -> Result<(), DaemonSyncPreflightError> {
    let timestamp = current_timestamp_unix_seconds();
    let lifecycle = if sync_runtime
        .load_sync_control()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?
        .paused
    {
        SyncLifecycleState::Paused
    } else if sync_runtime
        .store()
        .load_recovery_marker()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?
        .is_some()
    {
        SyncLifecycleState::Recovering
    } else {
        SyncLifecycleState::Active
    };
    let state = sync_runtime
        .durable_sync_state(lifecycle, None, timestamp)
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    sync_runtime
        .persist_durable_sync_state(state)
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))
}
