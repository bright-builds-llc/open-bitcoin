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

use std::time::{SystemTime, UNIX_EPOCH};

use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::mempool::{PolicyConfig, PolicyTime};
use open_bitcoin_node::status::SyncRecoveryCategory;
use open_bitcoin_node::storage::fjall_store::MempoolSnapshotDecodeLimits;
use open_bitcoin_node::{FjallNodeStore, ManagedNetworkAuthorityError, ManagedNetworkHandle};

use crate::config::RuntimeConfig;

pub(super) fn recover_mempool_snapshot_from_store_handle(
    config: &RuntimeConfig,
    maybe_store: Option<&FjallNodeStore>,
    network: &ManagedNetworkHandle,
    policy: &PolicyConfig,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<(), ManagedNetworkAuthorityError> {
    recover_mempool_snapshot_from_store_handle_at(
        config,
        maybe_store,
        network,
        policy,
        verify_flags,
        consensus_params,
        startup_policy_time(),
    )
}

#[allow(clippy::too_many_arguments)]
fn recover_mempool_snapshot_from_store_handle_at(
    config: &RuntimeConfig,
    maybe_store: Option<&FjallNodeStore>,
    network: &ManagedNetworkHandle,
    policy: &PolicyConfig,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    startup_at: Result<PolicyTime, SyncRecoveryCategory>,
) -> Result<(), ManagedNetworkAuthorityError> {
    let startup_at = match startup_at {
        Ok(startup_at) => startup_at,
        Err(category) => {
            network.record_mempool_recovery_unavailable(category)?;
            return Ok(());
        }
    };
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

    recover_mempool_snapshot_with_loader(
        network,
        policy,
        verify_flags,
        consensus_params,
        startup_at,
        |decode_limits| store.load_mempool_snapshot_with_limits(decode_limits),
    )
}

#[allow(clippy::too_many_arguments)]
fn recover_mempool_snapshot_with_loader<Load>(
    network: &ManagedNetworkHandle,
    _policy: &PolicyConfig,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    startup_at: PolicyTime,
    load: Load,
) -> Result<(), ManagedNetworkAuthorityError>
where
    Load: FnOnce(
        MempoolSnapshotDecodeLimits,
    ) -> Result<
        Option<open_bitcoin_node::storage::MempoolSnapshot>,
        open_bitcoin_node::StorageError,
    >,
{
    let decode_limits = match MempoolSnapshotDecodeLimits::for_persisted_input() {
        Ok(decode_limits) => decode_limits,
        Err(category) => {
            network.record_mempool_recovery_unavailable(category)?;
            return Ok(());
        }
    };
    match load(decode_limits) {
        Ok(Some(snapshot)) => {
            let prepared = match network.prepare_mempool_recovery_at(
                &snapshot,
                verify_flags,
                consensus_params,
                startup_at,
            ) {
                Ok(prepared) => prepared,
                Err(_) => {
                    network.record_mempool_recovery_unavailable(
                        SyncRecoveryCategory::InvalidPeerData,
                    )?;
                    return Ok(());
                }
            };
            if network.install_mempool_recovery(prepared).is_err() {
                network
                    .record_mempool_recovery_unavailable(SyncRecoveryCategory::InvalidPeerData)?;
            }
        }
        Ok(None) => {}
        Err(error) => network.record_mempool_recovery_storage_error(&error)?,
    }
    Ok(())
}

fn startup_policy_time() -> Result<PolicyTime, SyncRecoveryCategory> {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SyncRecoveryCategory::ResourceExhaustion)?;
    let seconds =
        i64::try_from(elapsed.as_secs()).map_err(|_| SyncRecoveryCategory::ResourceExhaustion)?;
    Ok(PolicyTime::new(seconds))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process;
    use std::sync::atomic::{AtomicU64, Ordering};

    use open_bitcoin_mempool::{MempoolCapacity, PolicyConfig, PolicyTime};
    use open_bitcoin_network::{BlockRelayActivationPolicy, RelayActivationConfig};
    use open_bitcoin_node::ManagedNetworkHandle;
    use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
    use open_bitcoin_node::core::primitives::NetworkMagic;
    use open_bitcoin_node::status::SyncRecoveryCategory;
    use open_bitcoin_node::storage::fjall_store::{FjallNodeStore, MempoolSnapshotDecodeLimits};
    use open_bitcoin_node::storage::mempool_snapshot::CapturedMempoolGeneration;
    use open_bitcoin_node::storage::{
        MempoolSnapshot, PersistMode, SchemaVersion, StorageError, StorageNamespace,
        StorageRecoveryAction,
    };

    use super::recover_mempool_snapshot_with_loader;

    static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn snapshot_decode_limits_are_invariant_across_current_policy_capacities() {
        // Arrange
        let expected =
            MempoolSnapshotDecodeLimits::new(268_435_456, 220_096, 5_000, 4_194_304, 67_108_864);
        let policies = [
            PolicyConfig::default(),
            PolicyConfig {
                mempool_capacity: MempoolCapacity::new(0),
                ..PolicyConfig::default()
            },
            PolicyConfig {
                mempool_capacity: MempoolCapacity::new(1),
                ..PolicyConfig::default()
            },
            PolicyConfig {
                mempool_capacity: MempoolCapacity::new(usize::MAX),
                ..PolicyConfig::default()
            },
        ];

        // Act
        let limits = policies.map(|policy| {
            let handle = transient_handle();
            let mut maybe_observed = None;
            recover_mempool_snapshot_with_loader(
                &handle,
                &policy,
                ScriptVerifyFlags::NONE,
                ConsensusParams::default(),
                PolicyTime::new(20),
                |limits| {
                    maybe_observed = Some(limits);
                    Ok(None)
                },
            )
            .expect("bounded startup load");
            maybe_observed.expect("persisted input limits supplied to loader")
        });

        // Assert
        assert!(limits.into_iter().all(|limits| limits == expected));
    }

    #[test]
    fn startup_installs_current_v2_and_legacy_v1_before_handle_publication() {
        // Arrange
        let current = MempoolSnapshot::try_new_current(
            CapturedMempoolGeneration::new(7),
            PolicyTime::new(10),
            Vec::new(),
            Default::default(),
        )
        .expect("current snapshot");
        let legacy = MempoolSnapshot::default();

        // Act
        let current_handle = recover_snapshot(current, PolicyTime::new(20));
        let legacy_handle = recover_snapshot(legacy, PolicyTime::new(20));

        // Assert
        let current_summary = current_handle
            .latest_mempool_recovery_summary()
            .expect("current recovery evidence")
            .expect("current recovery summary");
        let legacy_summary = legacy_handle
            .latest_mempool_recovery_summary()
            .expect("legacy recovery evidence")
            .expect("legacy recovery summary");
        assert!(current_summary.records.is_empty());
        assert!(legacy_summary.records.is_empty());
        let checkpoint = current_handle
            .checkpoint_evidence(PolicyTime::new(20), 300)
            .expect("checkpoint evidence");
        assert_eq!(checkpoint.current_generation, 7);
        assert_eq!(checkpoint.maybe_last_durable_generation, Some(7));
        assert_eq!(
            current_handle
                .mempool_info()
                .expect("current mempool info")
                .rolling_mempool_fee_rate_sats_per_kvb,
            0
        );
    }

    #[test]
    fn startup_rejects_terminal_captured_generation_and_still_admits() {
        // Arrange
        let crafted = serde_json::to_vec(&serde_json::json!({
            "schema_version": 1,
            "payload": {
                "format_version": 2,
                "captured_generation": 18446744073709551615_u64,
                "captured_at_unix_seconds": 10,
                "records": [],
                "unbroadcast_members": []
            }
        }))
        .expect("crafted current-v2 bytes");
        let handle = transient_handle();

        // Act
        recover_mempool_snapshot_with_loader(
            &handle,
            &PolicyConfig::default(),
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
            PolicyTime::new(20),
            |decode_limits| {
                open_bitcoin_node::storage::snapshot_codec::decode_mempool_snapshot_with_limits(
                    &crafted,
                    decode_limits.into_codec_limits(),
                )
                .map(Some)
            },
        )
        .expect("record typed startup decode failure");

        // Assert
        assert!(
            handle
                .latest_mempool_recovery_summary()
                .expect("startup recovery evidence")
                .is_none()
        );
        assert_eq!(
            handle
                .latest_mempool_recovery_storage_error()
                .expect("startup failure evidence"),
            Some(SyncRecoveryCategory::StoreCorruption)
        );
        let checkpoint = handle
            .checkpoint_evidence(PolicyTime::new(20), 300)
            .expect("fresh checkpoint evidence");
        assert_eq!(checkpoint.current_generation, 0);
        assert_ne!(checkpoint.maybe_last_durable_generation, Some(u64::MAX));
        let expire_result = handle.expire_mempool(PolicyTime::new(20));
        assert!(
            expire_result.is_ok(),
            "follow-up mutation must remain possible: {expire_result:?}"
        );
        let rendered = format!("{expire_result:?}");
        assert!(
            !rendered
                .to_ascii_lowercase()
                .contains("lifecycle generation exhausted"),
            "follow-up mutation must not exhaust generation: {rendered}"
        );
    }

    #[test]
    fn startup_records_schema_decode_and_identity_failures_without_installing() {
        // Arrange
        let cases = [
            (
                StorageError::SchemaMismatch {
                    expected: SchemaVersion::CURRENT,
                    actual: SchemaVersion::new(2).expect("schema version"),
                },
                SyncRecoveryCategory::IncompatibleSchema,
            ),
            (
                corrupt_snapshot_error("decode_failure"),
                SyncRecoveryCategory::StoreCorruption,
            ),
            (
                corrupt_snapshot_error("identity_mismatch"),
                SyncRecoveryCategory::StoreCorruption,
            ),
        ];

        // Act / Assert
        for (error, expected) in cases {
            let handle = transient_handle();
            recover_mempool_snapshot_with_loader(
                &handle,
                &PolicyConfig::default(),
                ScriptVerifyFlags::NONE,
                ConsensusParams::default(),
                PolicyTime::new(20),
                |_| Err(error),
            )
            .expect("record typed startup failure");
            assert_eq!(
                handle
                    .latest_mempool_recovery_storage_error()
                    .expect("startup failure evidence"),
                Some(expected)
            );
            assert!(
                handle
                    .latest_mempool_recovery_summary()
                    .expect("startup recovery evidence")
                    .is_none()
            );
        }
    }

    #[test]
    fn startup_limit_failure_preserves_the_stored_snapshot() {
        // Arrange
        let data_dir = test_data_dir("retained-snapshot");
        let store = FjallNodeStore::open(&data_dir).expect("open store");
        let snapshot = MempoolSnapshot::try_new_current(
            CapturedMempoolGeneration::new(3),
            PolicyTime::new(10),
            Vec::new(),
            Default::default(),
        )
        .expect("current snapshot");
        store
            .save_mempool_snapshot(&snapshot, PersistMode::Sync)
            .expect("save snapshot");
        let handle = transient_handle();
        let expected_limits =
            MempoolSnapshotDecodeLimits::for_persisted_input().expect("persisted input limits");

        // Act
        recover_mempool_snapshot_with_loader(
            &handle,
            &PolicyConfig::default(),
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
            PolicyTime::new(20),
            |limits| {
                assert_eq!(limits, expected_limits);
                store.load_mempool_snapshot_with_limits(MempoolSnapshotDecodeLimits::new(
                    0, 0, 0, 0, 0,
                ))
            },
        )
        .expect("record limit failure");
        let retained = store
            .load_mempool_snapshot_with_limits(
                MempoolSnapshotDecodeLimits::for_persisted_input().expect("persisted input limits"),
            )
            .expect("load retained snapshot");

        // Assert
        assert_eq!(retained, Some(snapshot));
        assert_eq!(
            handle
                .latest_mempool_recovery_storage_error()
                .expect("startup failure evidence"),
            Some(SyncRecoveryCategory::StoreCorruption)
        );
        drop(store);
        fs::remove_dir_all(data_dir).expect("remove store");
    }

    fn recover_snapshot(snapshot: MempoolSnapshot, startup_at: PolicyTime) -> ManagedNetworkHandle {
        let handle = transient_handle();
        recover_mempool_snapshot_with_loader(
            &handle,
            &PolicyConfig::default(),
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
            startup_at,
            |_| Ok(Some(snapshot)),
        )
        .expect("recover snapshot before publication");
        handle
    }

    fn transient_handle() -> ManagedNetworkHandle {
        ManagedNetworkHandle::transient_runtime(
            NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
            18_444,
            RelayActivationConfig::default(),
            BlockRelayActivationPolicy::default(),
            false,
        )
    }

    fn corrupt_snapshot_error(detail: &str) -> StorageError {
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            detail: detail.to_string(),
            action: StorageRecoveryAction::RestoreFromBackup,
        }
    }

    fn test_data_dir(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "open-bitcoin-startup-{name}-{}-{}",
            process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create test data dir");
        path
    }
}
