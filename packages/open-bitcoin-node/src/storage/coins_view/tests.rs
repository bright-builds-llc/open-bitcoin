// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{ChainstateError, Coin, CoinsBatch, CoinsCacheEntry, CoinsView},
    primitives::{Amount, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid},
};

use super::{FjallCoinsView, map_optional_read, map_storage};
use crate::storage::{
    FjallNodeStore, StorageError, StorageNamespace, StorageRecoveryAction,
    coins_codec::{
        encode_best_block_key, encode_coin_key, encode_coin_value, encode_head_blocks_key,
        encode_head_blocks_value, estimated_encoded_bytes,
    },
};

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-coins-view-{test_name}-{}-{timestamp}",
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

fn sample_coin() -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: true,
        created_height: 100,
        created_median_time_past: 1_700_000_000,
    }
}

fn sample_outpoint(seed: u8) -> OutPoint {
    OutPoint {
        txid: Txid::from_byte_array([seed; 32]),
        vout: u32::from(seed),
    }
}

fn dirty_unspent_batch(pairs: &[(OutPoint, Coin)]) -> CoinsBatch {
    let mut entries = HashMap::new();
    for (outpoint, coin) in pairs {
        entries.insert(
            outpoint.clone(),
            CoinsCacheEntry::unspent_dirty(coin.clone()),
        );
    }
    CoinsBatch { entries }
}

fn assert_coins_storage_detail(error: ChainstateError, needle: &str) {
    let ChainstateError::CoinsStorage { detail } = error else {
        panic!("expected CoinsStorage, got {error:?}");
    };
    assert!(
        detail.to_ascii_lowercase().contains(needle),
        "detail {detail:?} should contain {needle:?}"
    );
}

#[test]
fn batch_write_writes_h_then_coins_then_b() {
    // Arrange
    let path = temp_store_path("h-then-coins-then-b");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);
    let outpoint = sample_outpoint(1);
    let coin = sample_coin();
    let new_tip = BlockHash::from_byte_array([7_u8; 32]);
    let implicit_old = BlockHash::from_byte_array([0_u8; 32]);
    let encoded_heads = encode_head_blocks_value(&[new_tip, implicit_old]).expect("encode H");
    assert_eq!(encoded_heads.len(), 65);

    // Act
    view.batch_write(
        dirty_unspent_batch(&[(outpoint.clone(), coin.clone())]),
        Some(new_tip),
    )
    .expect("batch_write dirty unspent");

    // Assert
    assert_eq!(view.best_block().expect("best_block"), Some(new_tip));
    assert_eq!(
        view.head_blocks().expect("head_blocks"),
        Vec::<BlockHash>::new()
    );
    assert_eq!(view.get_coin(&outpoint).expect("get_coin"), Some(coin));
    assert_eq!(
        view.read_raw_bytes(&encode_head_blocks_key())
            .expect("read H"),
        None
    );
    assert_eq!(
        view.read_raw_bytes(&encode_best_block_key())
            .expect("read B"),
        Some(vec![7_u8; 32])
    );
    remove_dir_if_exists(&path);
}

#[test]
fn spent_dirty_deletes_coin_key() {
    // Arrange
    let path = temp_store_path("spent-dirty-delete");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);
    let outpoint = sample_outpoint(2);
    let coin = sample_coin();
    view.batch_write(
        dirty_unspent_batch(&[(outpoint.clone(), coin)]),
        Some(BlockHash::from_byte_array([7_u8; 32])),
    )
    .expect("write unspent");
    let mut spent = HashMap::new();
    spent.insert(outpoint.clone(), CoinsCacheEntry::spent_dirty());

    // Act
    view.batch_write(
        CoinsBatch { entries: spent },
        Some(BlockHash::from_byte_array([8_u8; 32])),
    )
    .expect("spend dirty");

    // Assert
    assert_eq!(view.get_coin(&outpoint).expect("get_coin"), None);
    assert!(!view.have_coin(&outpoint).expect("have_coin"));
    assert!(
        !view
            .contains_raw_key(&encode_coin_key(&outpoint))
            .expect("contains C")
    );
    remove_dir_if_exists(&path);
}

#[test]
fn get_coin_decode_failure_is_coins_storage_not_none() {
    // Arrange
    let path = temp_store_path("decode-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let view = FjallCoinsView::from_store(&store);
    let outpoint = sample_outpoint(3);
    view.write_raw_bytes(&encode_coin_key(&outpoint), vec![0x00])
        .expect("plant corrupt coin");

    // Act
    let error = view
        .get_coin(&outpoint)
        .expect_err("decode must fail closed");

    // Assert
    let ChainstateError::CoinsStorage { detail } = error else {
        panic!("expected CoinsStorage, got {error:?}");
    };
    let lower = detail.to_ascii_lowercase();
    assert!(
        lower.contains("corruption") || lower.contains("coins"),
        "detail {detail:?} should mention corruption or coins"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn have_coin_backend_error_is_not_false() {
    // Arrange
    let backend = StorageError::BackendFailure {
        namespace: StorageNamespace::Coins,
        message: "injected".into(),
        action: StorageRecoveryAction::Restart,
    };

    // Act
    let mapped_err = map_optional_read::<()>(Err(backend));
    let mapped_none = map_optional_read::<()>(Ok(None));
    let mapped_some = map_optional_read(Ok(Some(())));

    // Assert
    assert!(matches!(
        mapped_err,
        Err(ChainstateError::CoinsStorage { .. })
    ));
    assert_eq!(mapped_none, Ok(None));
    assert_eq!(mapped_some, Ok(Some(())));
}

#[test]
fn two_element_h_without_b_is_interrupted_write() {
    // Arrange
    let path = temp_store_path("interrupted-h");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);
    let new_tip = BlockHash::from_byte_array([7_u8; 32]);
    let old_tip = BlockHash::from_byte_array([1_u8; 32]);
    let heads = encode_head_blocks_value(&[new_tip, old_tip]).expect("encode H");
    view.write_raw_bytes(&encode_head_blocks_key(), heads)
        .expect("plant H");
    let outpoint = sample_outpoint(4);
    let coin = sample_coin();

    // Act
    let best_error = view.best_block().expect_err("best_block fail closed");
    let heads_error = view.head_blocks().expect_err("head_blocks fail closed");
    let write_error = view
        .batch_write(
            dirty_unspent_batch(&[(outpoint.clone(), coin)]),
            Some(BlockHash::from_byte_array([9_u8; 32])),
        )
        .expect_err("batch_write refuses present H");

    // Assert
    assert_coins_storage_detail(best_error, "interrupted");
    assert_coins_storage_detail(heads_error, "interrupted");
    assert_coins_storage_detail(write_error, "interrupted");
    assert_eq!(
        view.get_coin(&outpoint).expect("new coin must stay absent"),
        None
    );
    remove_dir_if_exists(&path);
}

#[test]
fn head_blocks_count_three_is_corruption() {
    // Arrange
    let path = temp_store_path("h-count-three");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let view = FjallCoinsView::from_store(&store);
    let mut planted = vec![0x03];
    planted.extend_from_slice(&[0x11; 96]);
    view.write_raw_bytes(&encode_head_blocks_key(), planted)
        .expect("plant count-3 H");

    // Act
    let error = view
        .head_blocks()
        .expect_err("count 3 is not auto-repaired");

    // Assert
    assert_coins_storage_detail(error, "corruption");
    remove_dir_if_exists(&path);
}

#[test]
fn batch_write_splits_when_next_item_would_exceed_cap() {
    // Arrange
    let path = temp_store_path("split-cap-80");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);
    let first = sample_outpoint(5);
    let second = sample_outpoint(6);
    let coin = sample_coin();
    let first_value = encode_coin_value(&coin).expect("encode first coin");
    let second_value = encode_coin_value(&coin).expect("encode second coin");
    assert!(estimated_encoded_bytes(&encode_coin_key(&first), Some(&first_value)) > 40);
    assert!(estimated_encoded_bytes(&encode_coin_key(&second), Some(&second_value)) > 40);
    let new_tip = BlockHash::from_byte_array([7_u8; 32]);

    // Act
    view.batch_write_with_limit(
        dirty_unspent_batch(&[
            (first.clone(), coin.clone()),
            (second.clone(), coin.clone()),
        ]),
        Some(new_tip),
        80,
    )
    .expect("capped batch_write");

    // Assert
    assert_eq!(view.partial_buffered_commits(), 1);
    assert_eq!(
        view.get_coin(&first).expect("first coin"),
        Some(coin.clone())
    );
    assert_eq!(view.get_coin(&second).expect("second coin"), Some(coin));
    assert_eq!(view.best_block().expect("best_block"), Some(new_tip));
    assert_eq!(
        view.read_raw_bytes(&encode_head_blocks_key())
            .expect("read H"),
        None
    );
    remove_dir_if_exists(&path);
}

#[test]
fn simulate_crash_after_partial_leaves_h_and_fails_closed() {
    // Arrange
    let path = temp_store_path("crash-after-partial");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);
    view.set_simulate_crash_after_partial(true);
    let first = sample_outpoint(7);
    let second = sample_outpoint(8);
    let coin = sample_coin();
    let new_tip = BlockHash::from_byte_array([7_u8; 32]);

    // Act
    let write_error = view
        .batch_write_with_limit(
            dirty_unspent_batch(&[(first, coin.clone()), (second, coin)]),
            Some(new_tip),
            80,
        )
        .expect_err("crash seam returns interrupted");
    drop(view);
    drop(store);
    let open_error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("interrupted H must fail closed on open"),
        Err(error) => error,
    };
    let reopened = FjallNodeStore::open_without_ensure_schema_for_test(&path)
        .expect("peek interrupted markers");
    let reopened_view = FjallCoinsView::from_store(&reopened);

    // Assert
    assert_coins_storage_detail(write_error, "interrupted");
    assert!(matches!(
        open_error,
        StorageError::InterruptedWrite {
            namespace: StorageNamespace::Coins,
            action: StorageRecoveryAction::Reindex,
        }
    ));
    assert_coins_storage_detail(
        reopened_view
            .head_blocks()
            .expect_err("reopen head_blocks fail closed"),
        "interrupted",
    );
    assert_coins_storage_detail(
        reopened_view
            .best_block()
            .expect_err("reopen best_block fail closed"),
        "interrupted",
    );
    remove_dir_if_exists(&path);
}

#[test]
fn batch_write_without_new_tip_fails() {
    // Arrange
    let path = temp_store_path("missing-tip");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);

    // Act
    let error = view
        .batch_write(dirty_unspent_batch(&[]), None)
        .expect_err("missing tip");

    // Assert
    assert_coins_storage_detail(error, "batch_write requires a new best-block");
    remove_dir_if_exists(&path);
}

#[test]
fn coins_view_source_has_no_write_batch_len_as_bytes() {
    // Arrange
    let source = include_str!("../coins_view.rs");

    // Act / Assert
    assert!(source.contains("estimated_encoded_bytes"));
    assert!(source.contains("DEFAULT_COINS_DB_BATCH_BYTES"));
    assert!(
        !source.contains("batch.len()"),
        "Fjall WriteBatch::len() is item count, not encoded bytes"
    );
    assert!(
        !source.contains(".len() as"),
        "do not cast batch len next to DEFAULT_COINS_DB_BATCH_BYTES"
    );
}

#[test]
fn map_storage_preserves_storage_detail() {
    // Arrange
    let error = StorageError::InterruptedWrite {
        namespace: StorageNamespace::Coins,
        action: StorageRecoveryAction::Reindex,
    };

    // Act
    let mapped = map_storage(error);

    // Assert
    assert_coins_storage_detail(mapped, "interrupted");
}
