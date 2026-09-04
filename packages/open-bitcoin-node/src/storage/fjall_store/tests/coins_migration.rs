// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

use std::collections::HashMap;
use std::str;

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin, CoinsView, TxUndo},
    primitives::{
        Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, TransactionOutput, Txid,
    },
};

use super::*;
use crate::storage::blob_schema_is_readable;
use crate::storage::coins_codec::{
    decode_coin_key, encode_best_block_key, encode_best_block_value, encode_coin_key,
    encode_coin_value, encode_head_blocks_key, encode_head_blocks_value,
};
use crate::storage::coins_view::FjallCoinsView;
use crate::storage::snapshot_codec::{decode_chainstate_snapshot, encode_chainstate_snapshot};

fn sample_coin(height: u32, mtp: i64) -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(5_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: false,
        created_height: height,
        created_median_time_past: mtp,
    }
}

fn leftover_snapshot() -> (ChainstateSnapshot, OutPoint, Coin, BlockHash, BlockHash) {
    let tip = ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0xcc; 32]),
            time: 1,
            bits: 0x207f_ffff,
            nonce: 1,
        },
        0,
        1,
        1,
    );
    let tip_hash = tip.block_hash;
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([0xaa; 32]),
        vout: 1,
    };
    let coin = sample_coin(5, 42);
    let undo_hash = BlockHash::from_byte_array([0xbb; 32]);
    let undo = BlockUndo {
        transactions: vec![TxUndo {
            restored_inputs: vec![coin.clone()],
        }],
    };
    let mut utxos = HashMap::new();
    utxos.insert(outpoint.clone(), coin.clone());
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(undo_hash, undo);
    let mut snapshot = ChainstateSnapshot::new(vec![tip], utxos, undo_by_block);
    snapshot.maybe_confirmed_txid_counts = Some(HashMap::new());
    (snapshot, outpoint, coin, tip_hash, undo_hash)
}

fn raw_schema_version(store: &FjallNodeStore) -> String {
    let bytes = store
        .get_bytes(StorageNamespace::Schema, "schema_version")
        .expect("read schema")
        .expect("schema present");
    str::from_utf8(&bytes).expect("utf8 schema").to_string()
}

#[test]
fn schema_version_current_is_two() {
    // Arrange / Act
    let current = SchemaVersion::CURRENT.get();

    // Assert
    assert_eq!(current, 2);
}

#[test]
fn fresh_open_writes_schema_two_with_empty_coins_and_no_snapshot() {
    // Arrange
    let path = temp_store_path("fresh-schema-two");
    remove_dir_if_exists(&path);

    // Act
    let store = FjallNodeStore::open(&path).expect("open fresh store");

    // Assert
    assert_eq!(raw_schema_version(&store), "2");
    assert_eq!(store.hydrate_chainstate_for_open().expect("hydrate"), None);
    assert_eq!(
        store.load_chainstate_snapshot().expect("leftover snapshot"),
        None
    );
    assert_eq!(
        FjallCoinsView::from_store(&store)
            .best_block()
            .expect("best_block"),
        None
    );
    remove_dir_if_exists(&path);
}

#[test]
fn schema_one_leftover_snapshot_migrates_once_into_coins_and_undo() {
    // Arrange
    let path = temp_store_path("schema-one-migrate");
    remove_dir_if_exists(&path);
    let (snapshot, outpoint, coin, tip_hash, undo_hash) = leftover_snapshot();
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(1)
            .expect("plant schema 1");
        store
            .save_chainstate_snapshot(&snapshot, PersistMode::Sync)
            .expect("save leftover");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen migrates");
    let view = FjallCoinsView::from_store(&store);

    // Assert
    assert_eq!(raw_schema_version(&store), "2");
    assert_eq!(
        view.get_coin(&outpoint).expect("get_coin"),
        Some(coin.clone())
    );
    assert_eq!(view.best_block().expect("best_block"), Some(tip_hash));
    assert_eq!(
        store.load_undo(undo_hash).expect("load_undo"),
        Some(snapshot.undo_by_block[&undo_hash].clone())
    );
    assert!(
        store
            .load_chainstate_snapshot()
            .expect("leftover kept")
            .is_some()
    );
    let hydrated = store
        .hydrate_chainstate_for_open()
        .expect("hydrate")
        .expect("migrated snapshot");
    assert_eq!(hydrated.utxos.get(&outpoint), Some(&coin));
    remove_dir_if_exists(&path);
}

#[test]
fn schema_two_leftover_utxos_do_not_hydrate_after_coins_exist() {
    // Arrange
    let path = temp_store_path("leftover-non-authority");
    remove_dir_if_exists(&path);
    let (snapshot, aa_outpoint, aa_coin, _tip_hash, _undo_hash) = leftover_snapshot();
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(1)
            .expect("plant schema 1");
        store
            .save_chainstate_snapshot(&snapshot, PersistMode::Sync)
            .expect("save leftover");
    }
    let store = FjallNodeStore::open(&path).expect("migrate");
    let dd_outpoint = OutPoint {
        txid: Txid::from_byte_array([0xdd; 32]),
        vout: 0,
    };
    let mut overwritten = snapshot.clone();
    overwritten.utxos.clear();
    overwritten
        .utxos
        .insert(dd_outpoint.clone(), sample_coin(9, 9));
    store
        .save_chainstate_snapshot(&overwritten, PersistMode::Sync)
        .expect("overwrite leftover");
    drop(store);

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen");
    let view = FjallCoinsView::from_store(&store);
    let hydrated = store
        .hydrate_chainstate_for_open()
        .expect("hydrate")
        .expect("coins snapshot");

    // Assert
    assert_eq!(hydrated.utxos.get(&aa_outpoint), Some(&aa_coin));
    assert!(!hydrated.utxos.contains_key(&dd_outpoint));
    assert_eq!(view.get_coin(&aa_outpoint).expect("aa coin"), Some(aa_coin));
    assert_eq!(view.get_coin(&dd_outpoint).expect("dd miss"), None);
    remove_dir_if_exists(&path);
}

#[test]
fn schema_two_leftover_plus_empty_coins_fails_closed_without_remigrate() {
    // Arrange
    let path = temp_store_path("schema-two-leftover-empty");
    remove_dir_if_exists(&path);
    let (snapshot, outpoint, _coin, _tip, _undo) = leftover_snapshot();
    {
        let store = FjallNodeStore::open(&path).expect("fresh schema 2");
        store
            .save_chainstate_snapshot(&snapshot, PersistMode::Sync)
            .expect("leftover only");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("schema 2 leftover plus empty coins must fail closed"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Coins | StorageNamespace::Chainstate,
            action: StorageRecoveryAction::RestoreFromBackup,
            ..
        }
    ));
    let peek =
        FjallNodeStore::open_without_ensure_schema_for_test(&path).expect("peek without remigrate");
    let view = FjallCoinsView::from_store(&peek);
    assert_eq!(
        view.read_raw_bytes(&encode_coin_key(&outpoint))
            .expect("raw C"),
        None
    );
    remove_dir_if_exists(&path);
}

#[test]
fn schema_one_leftover_plus_nonempty_coins_is_corruption_without_remigrate() {
    // Arrange
    let path = temp_store_path("schema-one-mixed");
    remove_dir_if_exists(&path);
    let leftover_outpoint = OutPoint {
        txid: Txid::from_byte_array([0x11; 32]),
        vout: 0,
    };
    let planted_outpoint = OutPoint {
        txid: Txid::from_byte_array([0x22; 32]),
        vout: 0,
    };
    let planted_coin = sample_coin(1, 1);
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(1)
            .expect("plant schema 1");
        let mut leftover = leftover_snapshot().0;
        leftover.utxos.clear();
        leftover
            .utxos
            .insert(leftover_outpoint.clone(), sample_coin(2, 2));
        store
            .save_chainstate_snapshot(&leftover, PersistMode::Sync)
            .expect("leftover");
        let view = FjallCoinsView::from_store(&store);
        view.write_raw_bytes(
            &encode_coin_key(&planted_outpoint),
            encode_coin_value(&planted_coin).expect("encode planted"),
        )
        .expect("plant C");
        view.write_raw_bytes(
            &encode_best_block_key(),
            encode_best_block_value(BlockHash::from_byte_array([0x33; 32])),
        )
        .expect("plant B");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("mixed schema 1 leftover and coins must fail closed"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
    let peek =
        FjallNodeStore::open_without_ensure_schema_for_test(&path).expect("peek after failed open");
    let view = FjallCoinsView::from_store(&peek);
    assert_eq!(
        view.read_raw_bytes(&encode_coin_key(&leftover_outpoint))
            .expect("leftover must not remigrate"),
        None
    );
    assert_eq!(
        decode_coin_key(&encode_coin_key(&planted_outpoint)).expect("planted key"),
        planted_outpoint
    );
    assert_eq!(
        view.get_coin(&planted_outpoint).expect("planted coin"),
        Some(planted_coin)
    );
    remove_dir_if_exists(&path);
}

#[test]
fn hydrate_two_element_h_without_b_fails_closed() {
    // Arrange
    let path = temp_store_path("interrupted-h");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("schema 2");
    let view = FjallCoinsView::from_store(&store);
    let heads = encode_head_blocks_value(&[
        BlockHash::from_byte_array([0x44; 32]),
        BlockHash::from_byte_array([0x55; 32]),
    ])
    .expect("encode H");
    view.write_raw_bytes(&encode_head_blocks_key(), heads)
        .expect("plant H");
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([0x66; 32]),
        vout: 0,
    };
    view.write_raw_bytes(
        &encode_coin_key(&outpoint),
        encode_coin_value(&sample_coin(3, 3)).expect("encode C"),
    )
    .expect("plant C");

    // Act
    let error = store
        .hydrate_chainstate_for_open()
        .expect_err("two-element H without B");

    // Assert
    assert!(matches!(
        error,
        StorageError::InterruptedWrite {
            namespace: StorageNamespace::Coins,
            action: StorageRecoveryAction::Reindex,
        }
    ));
    remove_dir_if_exists(&path);
}

#[test]
fn schema_three_is_schema_mismatch() {
    // Arrange
    let path = temp_store_path("schema-three");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("open");
        store
            .write_schema_version_for_test(3)
            .expect("plant schema 3");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("schema 3 must mismatch"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::SchemaMismatch {
            expected: SchemaVersion::CURRENT,
            actual,
        } if actual.get() == 3
    ));
    remove_dir_if_exists(&path);
}

#[test]
fn decode_versioned_accepts_blob_schema_one_after_current_is_two() {
    // Arrange
    let snapshot = leftover_snapshot().0;
    let encoded = encode_chainstate_snapshot(&snapshot).expect("encode current");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    value["schema_version"] = serde_json::json!(1);
    let schema_one = serde_json::to_vec_pretty(&value).expect("schema 1 bytes");

    // Act
    let decoded = decode_chainstate_snapshot(&schema_one).expect("decode schema 1 blob");

    // Assert
    assert_eq!(decoded.utxos.len(), snapshot.utxos.len());
    assert!(blob_schema_is_readable(
        SchemaVersion::new(1).expect("schema 1")
    ));
    assert!(blob_schema_is_readable(
        SchemaVersion::new(2).expect("schema 2")
    ));
    assert!(!blob_schema_is_readable(
        SchemaVersion::new(3).expect("schema 3")
    ));
}

#[test]
fn save_block_and_load_block_still_use_block_hex_keys() {
    // Arrange
    let path = temp_store_path("block-hex-keys");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open");
    let fixture = block(BlockHash::from_byte_array([0_u8; 32]), 3);
    let source = include_str!("../../fjall_store.rs");

    // Act
    let block_hash = store
        .save_block(&fixture, PersistMode::Sync)
        .expect("save_block");
    let loaded = store.load_block(block_hash).expect("load_block");

    // Assert
    assert_eq!(loaded, Some(fixture));
    assert!(source.contains("fn save_block"));
    assert!(source.contains("fn load_block"));
    assert!(source.contains("block_key"));
    let save_start = source.find("pub fn save_block").expect("save_block");
    let load_start = source.find("pub fn load_block").expect("load_block");
    let range_start = save_start.min(load_start);
    let range_end = save_start.max(load_start);
    let after_last = source[range_end..]
        .find(
            "
    pub fn",
        )
        .expect("function after save/load_block");
    let bodies = &source[range_start..range_end + after_last];
    assert!(!bodies.contains("StorageNamespace::Coins"));
    remove_dir_if_exists(&path);
}

#[test]
fn persist_progress_source_still_writes_leftover_snapshot() {
    // Arrange
    let runtime_state = include_str!("../../../sync/runtime_state.rs");
    let chainstate = include_str!("../../../chainstate.rs");

    // Act / Assert
    assert!(runtime_state.contains("save_chainstate_snapshot"));
    assert!(chainstate.contains("save_snapshot(self.chainstate.snapshot())"));
}
