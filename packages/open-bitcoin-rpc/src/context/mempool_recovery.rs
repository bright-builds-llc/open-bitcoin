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

use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::status::SyncRecoveryCategory;
use open_bitcoin_node::{
    FjallNodeStore, ManagedNetworkAuthorityError, ManagedNetworkHandle, ManagedPeerNetwork,
    MemoryChainstateStore,
};

use crate::config::RuntimeConfig;

pub(super) fn recover_mempool_snapshot_from_store(
    config: &RuntimeConfig,
    maybe_store: Option<&FjallNodeStore>,
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) {
    let store;
    let store = match maybe_store {
        Some(store) => store,
        None => {
            let Some(data_dir) = config.maybe_data_dir.as_ref() else {
                return;
            };
            let opened_store = match FjallNodeStore::open(data_dir) {
                Ok(store) => store,
                Err(error) => {
                    network.record_mempool_recovery_storage_error(&error);
                    return;
                }
            };
            store = opened_store;
            &store
        }
    };

    #[allow(
        deprecated,
        reason = "Phase 135 Plan 06 migrates and removes this startup compatibility call"
    )]
    match store.load_mempool_snapshot() {
        Ok(Some(snapshot)) => {
            let recovery_result =
                network.recover_mempool_snapshot(&snapshot, verify_flags, consensus_params);
            if recovery_result.is_err() {
                network.record_mempool_recovery_unavailable(SyncRecoveryCategory::InvalidPeerData);
            }
        }
        Ok(None) => {}
        Err(error) => network.record_mempool_recovery_storage_error(&error),
    }
}

pub(super) fn recover_mempool_snapshot_from_store_handle(
    config: &RuntimeConfig,
    maybe_store: Option<&FjallNodeStore>,
    network: &ManagedNetworkHandle,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<(), ManagedNetworkAuthorityError> {
    let store;
    let store = match maybe_store {
        Some(store) => store,
        None => {
            let Some(data_dir) = config.maybe_data_dir.as_ref() else {
                return Ok(());
            };
            let opened_store = match FjallNodeStore::open(data_dir) {
                Ok(store) => store,
                Err(error) => {
                    network.record_mempool_recovery_storage_error(&error)?;
                    return Ok(());
                }
            };
            store = opened_store;
            &store
        }
    };

    #[allow(
        deprecated,
        reason = "Phase 135 Plan 06 migrates and removes this startup compatibility call"
    )]
    match store.load_mempool_snapshot() {
        Ok(Some(snapshot)) => {
            let recovery_result =
                network.recover_mempool_snapshot(&snapshot, verify_flags, consensus_params);
            if recovery_result.is_err() {
                network
                    .record_mempool_recovery_unavailable(SyncRecoveryCategory::InvalidPeerData)?;
            }
        }
        Ok(None) => {}
        Err(error) => network.record_mempool_recovery_storage_error(&error)?,
    }
    Ok(())
}
