// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
            .load_mempool_snapshot_with_limits(snapshot_decode_limits())
            .expect("load mempool snapshot"),
        Some(snapshot)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_round_trips_at_policy_transaction_capacity() {
    // Arrange
    let path = temp_store_path("mempool-policy-capacity");
    remove_dir_if_exists(&path);
    let snapshot = mempool_snapshot();
    let transaction_bytes = snapshot
        .records
        .iter()
        .map(|record| {
            open_bitcoin_core::codec::encode_transaction(
                &record.transaction,
                open_bitcoin_core::codec::TransactionEncoding::WithWitness,
            )
            .expect("encode transaction")
            .len()
        })
        .sum();
    let policy = PolicyConfig {
        mempool_capacity: open_bitcoin_mempool::MempoolCapacity::new(transaction_bytes),
        ..PolicyConfig::default()
    };
    let limits = MempoolSnapshotDecodeLimits::from_policy(&policy).expect("policy limits");

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_mempool_snapshot(&snapshot, PersistMode::Sync)
            .expect("save near-capacity snapshot");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let recovered = reopened
        .load_mempool_snapshot_with_limits(limits)
        .expect("load near-capacity snapshot");

    // Assert
    assert_eq!(recovered, Some(snapshot));
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
            .load_mempool_snapshot_with_limits(snapshot_decode_limits())
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
    let bytes_before = store
        .get_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY)
        .expect("read corrupt bytes before load");

    // Act
    let error = store
        .load_mempool_snapshot_with_limits(snapshot_decode_limits())
        .expect_err("corrupt mempool snapshot should fail");
    let bytes_after = store
        .get_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY)
        .expect("read corrupt bytes after load");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ..
        }
    ));
    assert_eq!(bytes_after, bytes_before);

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_rejects_oversized_bytes_without_mutation() {
    // Arrange
    let path = temp_store_path("mempool-oversized");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let bytes = crate::storage::snapshot_codec::encode_mempool_snapshot(&mempool_snapshot())
        .expect("encode fixture");
    store
        .write_raw_for_test(StorageNamespace::Mempool, SNAPSHOT_KEY, bytes.clone())
        .expect("write oversized fixture");
    let limits =
        MempoolSnapshotDecodeLimits::new(bytes.len() - 1, 128, 128, 1024 * 1024, 2 * 1024 * 1024);

    // Act
    let error = store
        .load_mempool_snapshot_with_limits(limits)
        .expect_err("encoded byte bound should reject the snapshot");
    let bytes_after = store
        .get_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY)
        .expect("read oversized bytes after load");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ..
        }
    ));
    assert_eq!(bytes_after, Some(bytes));

    remove_dir_if_exists(&path);
}
