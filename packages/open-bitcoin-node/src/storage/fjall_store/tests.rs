// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin, TxUndo},
    consensus::block_hash,
    primitives::{
        Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet, WalletSnapshot, WalletUtxo},
};
use open_bitcoin_network::HeaderEntry;

use super::{
    FjallNodeStore, RECOVERY_MARKER_KEY, RuntimeMetadata, SNAPSHOT_KEY, StorageError,
    StorageNamespace, StorageRecoveryAction,
};
use crate::{MetricKind, MetricSample, MetricsStorageSnapshot, PersistMode, SchemaVersion};

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-fjall-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_700_000_000 + nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

fn header_entries() -> Vec<HeaderEntry> {
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_hash = block_hash(&genesis_header);
    let child_header = header(genesis_hash, 2);
    let child_hash = block_hash(&child_header);

    vec![
        HeaderEntry {
            block_hash: genesis_hash,
            header: genesis_header,
            height: 0,
            chain_work: 1,
        },
        HeaderEntry {
            block_hash: child_hash,
            header: child_header,
            height: 1,
            chain_work: 2,
        },
    ]
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn output(value: i64) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(value).expect("valid amount"),
        script_pubkey: script(&[0x51]),
    }
}

fn chainstate_snapshot() -> ChainstateSnapshot {
    let position = ChainPosition::new(header(BlockHash::from_byte_array([0_u8; 32]), 1), 0, 1, 1);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([9; 32]),
        vout: 0,
    };
    let coin = Coin {
        output: output(5_000),
        is_coinbase: false,
        created_height: 0,
        created_median_time_past: 1,
    };
    let mut utxos = HashMap::new();
    utxos.insert(outpoint, coin.clone());
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(
        position.block_hash,
        BlockUndo {
            transactions: vec![TxUndo {
                restored_inputs: vec![coin],
            }],
        },
    );

    ChainstateSnapshot::new(vec![position], utxos, undo_by_block)
}

fn wallet_snapshot() -> WalletSnapshot {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    let descriptor_id = wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("descriptor import");
    let mut snapshot = wallet.snapshot();
    snapshot.utxos.push(WalletUtxo {
        descriptor_id,
        outpoint: OutPoint {
            txid: Txid::from_byte_array([4; 32]),
            vout: 1,
        },
        output: output(10_000),
        created_height: 2,
        created_median_time_past: 3,
        is_coinbase: false,
    });
    snapshot
}

#[test]
fn fjall_store_reopens_saved_snapshots_and_metadata() {
    // Arrange
    let path = temp_store_path("reopen-snapshots");
    remove_dir_if_exists(&path);
    let chainstate = chainstate_snapshot();
    let wallet = wallet_snapshot();
    let headers = header_entries();
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
fn incompatible_schema_version_returns_schema_mismatch() {
    // Arrange
    let path = temp_store_path("schema-mismatch");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(SchemaVersion::CURRENT.get() + 1)
            .expect("write schema version");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("expected schema mismatch"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::SchemaMismatch {
            expected: SchemaVersion::CURRENT,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn malformed_snapshot_maps_to_corruption() {
    // Arrange
    let path = temp_store_path("corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            SNAPSHOT_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed record");

    // Act
    let error = store
        .load_chainstate_snapshot()
        .expect_err("malformed chainstate");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn recovery_marker_round_trips_and_clean_shutdown_clears_it() {
    // Arrange
    let path = temp_store_path("recovery-marker");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");

    // Act
    let marker = store
        .mark_interrupted_write(
            StorageNamespace::BlockIndex,
            StorageRecoveryAction::Reindex,
            "block index write interrupted",
            PersistMode::Sync,
        )
        .expect("write recovery marker");
    let loaded = store
        .load_recovery_marker()
        .expect("load recovery marker")
        .expect("recovery marker");
    store
        .mark_clean_shutdown(PersistMode::Sync)
        .expect("mark clean shutdown");

    // Assert
    assert_eq!(loaded, marker);
    assert_eq!(store.load_recovery_marker().expect("reload marker"), None);
    assert!(
        store
            .load_runtime_metadata()
            .expect("load runtime metadata")
            .expect("runtime metadata")
            .last_clean_shutdown
    );

    remove_dir_if_exists(&path);
}

#[test]
fn malformed_recovery_marker_maps_to_runtime_corruption() {
    // Arrange
    let path = temp_store_path("recovery-marker-corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Runtime,
            RECOVERY_MARKER_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed marker");

    // Act
    let error = store
        .load_recovery_marker()
        .expect_err("malformed recovery marker");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Runtime,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}
