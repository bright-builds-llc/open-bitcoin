// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::LocalPeerConfig;

use crate::MemoryChainstateStore;
use crate::network::{EffectCompletion, ManagedNetworkHandle, ManagedPeerNetwork};

fn empty_network_handle() -> ManagedNetworkHandle {
    ManagedNetworkHandle::from_network_fixture(ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    ))
}

#[test]
fn fjall_store_reopens_saved_snapshots_and_metadata() {
    // Arrange
    let path = temp_store_path("reopen-snapshots");
    remove_dir_if_exists(&path);
    let chainstate = chainstate_snapshot();
    let wallet = wallet_snapshot();
    let headers = header_entries();
    let block = block(headers[0].block_hash, 3);
    let block_hash = block_hash(&block.header);
    let metrics = MetricsStorageSnapshot {
        samples: vec![MetricSample::new(MetricKind::SyncHeight, 1.0, 2)],
    };
    let metadata = RuntimeMetadata {
        last_clean_shutdown: true,
        ..RuntimeMetadata::default()
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_chainstate_snapshot(&chainstate, PersistMode::Sync)
            .expect("save chainstate");
        store
            .save_wallet_snapshot(&wallet, PersistMode::Sync)
            .expect("save wallet");
        store
            .save_header_entries(&headers, PersistMode::Sync)
            .expect("save headers");
        store
            .save_block(&block, PersistMode::Sync)
            .expect("save block");
        store
            .save_metrics_snapshot(&metrics, PersistMode::Sync)
            .expect("save metrics");
        store
            .save_runtime_metadata(&metadata, PersistMode::Sync)
            .expect("save runtime metadata");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_chainstate_snapshot()
            .expect("load chainstate"),
        Some(chainstate)
    );
    assert_eq!(
        reopened.load_wallet_snapshot().expect("load wallet"),
        Some(wallet)
    );
    assert_eq!(
        reopened
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries,
        headers
    );
    assert_eq!(
        reopened
            .load_block_index_entries()
            .expect("load block index")
            .expect("block index")
            .entries,
        headers
    );
    assert_eq!(
        reopened
            .load_header_store()
            .expect("load header store")
            .expect("header store")
            .best_height(),
        1
    );
    assert_eq!(
        reopened.load_block(block_hash).expect("load block"),
        Some(block)
    );
    assert_eq!(
        reopened
            .load_block(BlockHash::from_byte_array([99_u8; 32]))
            .expect("load missing block"),
        None
    );
    assert_eq!(
        reopened.load_metrics_snapshot().expect("load metrics"),
        Some(metrics)
    );
    assert_eq!(
        reopened.load_runtime_metadata().expect("load metadata"),
        Some(metadata)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_round_trips_after_reopen() {
    // Arrange
    let path = temp_store_path("mempool-reopen");
    remove_dir_if_exists(&path);
    let snapshot = mempool_snapshot();

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_mempool_snapshot(&snapshot, PersistMode::Sync)
            .expect("save mempool snapshot");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_mempool_snapshot()
            .expect("load mempool snapshot"),
        Some(snapshot)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_remove_clears_persisted_state() {
    // Arrange
    let path = temp_store_path("mempool-clear");
    remove_dir_if_exists(&path);
    let snapshot = mempool_snapshot();
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .save_mempool_snapshot(&snapshot, PersistMode::Sync)
        .expect("save mempool snapshot");

    // Act
    store
        .clear_mempool_snapshot(PersistMode::Sync)
        .expect("clear mempool snapshot");

    // Assert
    assert_eq!(
        store
            .load_mempool_snapshot()
            .expect("load cleared mempool snapshot"),
        None
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_reports_corruption() {
    // Arrange
    let path = temp_store_path("mempool-corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Mempool,
            SNAPSHOT_KEY,
            b"{not-json".to_vec(),
        )
        .expect("write corrupt mempool snapshot");

    // Act
    let error = store
        .load_mempool_snapshot()
        .expect_err("corrupt mempool snapshot should fail");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_persists_before_receipt_completion() {
    // Arrange
    let path = temp_store_path("prepared-mempool-success");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write()
        .expect("snapshot should prepare");

    // Act
    let receipt = store
        .execute_prepared_mempool_snapshot_write(prepared, PersistMode::Sync)
        .expect("snapshot executor should persist");
    let completion = handle
        .complete_snapshot_write(receipt)
        .expect("snapshot completion should dispatch");

    // Assert
    assert_eq!(completion, EffectCompletion::Applied);
    assert_eq!(
        store
            .load_mempool_snapshot()
            .expect("load persisted snapshot"),
        Some(MempoolSnapshot::default())
    );
    assert!(
        handle.prepare_mempool_snapshot_write().is_ok(),
        "successful completion should release the pending slot"
    );

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_returns_no_receipt_after_write_failure() {
    // Arrange
    let path = temp_store_path("prepared-mempool-write-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write()
        .expect("snapshot should prepare");
    let expected = StorageError::BackendFailure {
        namespace: StorageNamespace::Mempool,
        message: "injected mempool snapshot write failure".to_string(),
        action: StorageRecoveryAction::Restart,
    };

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        prepared,
        PersistMode::Sync,
        |_, _| Err(expected.clone()),
    );

    // Assert
    assert_eq!(result, Err(expected));
    assert!(
        handle.prepare_mempool_snapshot_write().is_err(),
        "failed execution must not complete the pending snapshot"
    );
    assert_eq!(
        store
            .load_mempool_snapshot()
            .expect("load after write failure"),
        None
    );

    remove_dir_if_exists(&path);
}
