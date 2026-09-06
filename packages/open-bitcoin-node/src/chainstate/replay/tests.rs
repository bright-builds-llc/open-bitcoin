// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{BlockUndo, Coin, CoinsBatch, CoinsCacheEntry, CoinsView, TxUndo},
    consensus::{block_hash, transaction_txid},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
        Transaction, TransactionInput, TransactionOutput, Txid,
    },
};
use open_bitcoin_network::HeaderEntry;

use super::replay_interrupted_flush;
use crate::storage::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace, StorageRecoveryAction,
    coins_codec::{encode_best_block_key, encode_head_blocks_key, encode_head_blocks_value},
    coins_view::FjallCoinsView,
};

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-replay-{test_name}-{}-{timestamp}",
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

fn script() -> ScriptBuf {
    ScriptBuf::from_bytes(vec![0x51]).expect("valid script")
}

fn output(value: i64) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(value).expect("valid amount"),
        script_pubkey: script(),
    }
}

fn coin(value: i64, created_height: u32, created_median_time_past: i64) -> Coin {
    Coin {
        output: output(value),
        is_coinbase: false,
        created_height,
        created_median_time_past,
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

fn header_entry(block_header: BlockHeader, height: u32, chain_work: u128) -> HeaderEntry {
    HeaderEntry {
        block_hash: block_hash(&block_header),
        header: block_header,
        height,
        chain_work,
    }
}

fn coinbase(height: u32, value: i64) -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![output(value)],
        lock_time: height,
    }
}

fn spend_tx(previous: OutPoint, value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: previous,
            script_sig: script(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![output(value)],
        lock_time: 0,
    }
}

fn dirty_unspent_batch(pairs: &[(OutPoint, Coin)]) -> CoinsBatch {
    let mut entries = HashMap::new();
    for (outpoint, planted) in pairs {
        entries.insert(
            outpoint.clone(),
            CoinsCacheEntry::unspent_dirty(planted.clone()),
        );
    }
    CoinsBatch { entries }
}

fn assert_interrupted_write(error: StorageError) {
    assert!(
        matches!(
            error,
            StorageError::InterruptedWrite {
                namespace: StorageNamespace::Coins,
                action: StorageRecoveryAction::Reindex,
            }
        ),
        "expected InterruptedWrite Reindex, got {error:?}"
    );
}

fn expect_replay_error(
    result: Result<FjallCoinsView, StorageError>,
    message: &str,
) -> StorageError {
    match result {
        Ok(_) => panic!("{message}: expected error"),
        Err(error) => error,
    }
}

struct BranchFixture {
    path: PathBuf,
    store: FjallNodeStore,
    view: FjallCoinsView,
    new_hash: BlockHash,
    restored_outpoint: OutPoint,
    restored_coin: Coin,
    spent_on_new: OutPoint,
}

fn plant_interrupted_heads(view: &FjallCoinsView, new_hash: BlockHash, old_hash: BlockHash) {
    let heads = encode_head_blocks_value(&[new_hash, old_hash]).expect("encode H");
    view.write_raw_bytes(&encode_head_blocks_key(), heads)
        .expect("plant H");
    view.delete_raw_bytes(&encode_best_block_key())
        .expect("remove B");
}

fn two_branch_fixture(persist_new_body: bool, persist_old_undo: bool) -> BranchFixture {
    let path = temp_store_path("two-branch");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);

    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_hash = block_hash(&genesis_header);
    let old_header = header(genesis_hash, 2);
    let new_header = header(genesis_hash, 3);
    let old_hash = block_hash(&old_header);
    let new_hash = block_hash(&new_header);

    let restored_outpoint = OutPoint {
        txid: Txid::from_byte_array([0xaa; 32]),
        vout: 0,
    };
    let restored_coin = coin(50, 0, 1_700_000_001);
    let spent_on_new = OutPoint {
        txid: Txid::from_byte_array([0xbb; 32]),
        vout: 0,
    };
    let spent_on_new_coin = coin(40, 0, 1_700_000_001);

    let old_spend = spend_tx(restored_outpoint.clone(), 45);
    let old_created = OutPoint {
        txid: transaction_txid(&old_spend).expect("old spend txid"),
        vout: 0,
    };
    let old_created_coin = coin(45, 1, 1_700_000_002);
    let old_block = Block {
        header: old_header.clone(),
        transactions: vec![coinbase(1, 50), old_spend],
    };
    let new_spend = spend_tx(spent_on_new.clone(), 35);
    let new_block = Block {
        header: new_header.clone(),
        transactions: vec![coinbase(1, 50), new_spend],
    };

    store
        .save_header_entries(
            &[
                header_entry(genesis_header, 0, 1),
                header_entry(old_header, 1, 2),
                header_entry(new_header, 1, 2),
            ],
            PersistMode::Sync,
        )
        .expect("save headers");
    store
        .save_block(&old_block, PersistMode::Sync)
        .expect("save old body");
    if persist_new_body {
        store
            .save_block(&new_block, PersistMode::Sync)
            .expect("save new body");
    }
    if persist_old_undo {
        store
            .save_undo(
                old_hash,
                &BlockUndo {
                    transactions: vec![TxUndo {
                        restored_inputs: vec![restored_coin.clone()],
                    }],
                },
                PersistMode::Sync,
            )
            .expect("save old undo");
    }

    view.batch_write(
        dirty_unspent_batch(&[
            (old_created, old_created_coin),
            (spent_on_new.clone(), spent_on_new_coin),
        ]),
        Some(old_hash),
    )
    .expect("plant old-tip coins");
    plant_interrupted_heads(&view, new_hash, old_hash);

    BranchFixture {
        path,
        store,
        view,
        new_hash,
        restored_outpoint,
        restored_coin,
        spent_on_new,
    }
}

#[test]
fn replay_is_noop_when_heads_empty_and_b_present() {
    // Arrange
    let path = temp_store_path("noop-consistent");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let mut view = FjallCoinsView::from_store(&store);
    let tip = BlockHash::from_byte_array([7_u8; 32]);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    };
    let planted = coin(50, 1, 9);
    view.batch_write(
        dirty_unspent_batch(&[(outpoint.clone(), planted.clone())]),
        Some(tip),
    )
    .expect("consistent B");

    // Act
    let view = replay_interrupted_flush(&store, view).expect("noop replay");

    // Assert
    assert_eq!(view.head_blocks().expect("heads"), Vec::<BlockHash>::new());
    assert_eq!(view.best_block().expect("best"), Some(tip));
    assert_eq!(view.get_coin(&outpoint).expect("coin"), Some(planted));
    remove_dir_if_exists(&path);
}

#[test]
fn replay_rolls_back_old_branch_and_forward_new_when_bodies_exist() {
    // Arrange
    let fixture = two_branch_fixture(true, true);

    // Act
    let view = replay_interrupted_flush(&fixture.store, fixture.view).expect("replay");

    // Assert
    assert_eq!(view.best_block().expect("best"), Some(fixture.new_hash));
    assert_eq!(view.head_blocks().expect("heads"), Vec::<BlockHash>::new());
    assert_eq!(view.get_coin(&fixture.spent_on_new).expect("spent"), None);
    assert_eq!(
        view.get_coin(&fixture.restored_outpoint).expect("restored"),
        Some(fixture.restored_coin)
    );
    remove_dir_if_exists(&fixture.path);
}

#[test]
fn replay_first_flush_zero_old_tip_rollforwards_only() {
    // Arrange
    let path = temp_store_path("first-flush");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    let view = FjallCoinsView::from_store(&store);
    let new_header = header(BlockHash::from_byte_array([0_u8; 32]), 4);
    let new_hash = block_hash(&new_header);
    let new_block = Block {
        header: new_header.clone(),
        transactions: vec![coinbase(0, 50)],
    };
    store
        .save_header_entries(&[header_entry(new_header, 0, 1)], PersistMode::Sync)
        .expect("save first-flush header");
    store
        .save_block(&new_block, PersistMode::Sync)
        .expect("save first-flush body");
    plant_interrupted_heads(&view, new_hash, BlockHash::from_byte_array([0_u8; 32]));

    // Act
    let view = replay_interrupted_flush(&store, view).expect("first-flush replay");

    // Assert
    assert_eq!(view.best_block().expect("best"), Some(new_hash));
    assert_eq!(view.head_blocks().expect("heads"), Vec::<BlockHash>::new());
    remove_dir_if_exists(&path);
}

#[test]
fn replay_missing_block_body_is_interrupted_write_and_keeps_h() {
    // Arrange
    let fixture = two_branch_fixture(false, true);

    // Act
    let error = expect_replay_error(
        replay_interrupted_flush(&fixture.store, fixture.view),
        "missing new body fail-closes",
    );
    drop(fixture.store);
    let reopened = FjallNodeStore::open(&fixture.path).expect("reopen");
    let view = FjallCoinsView::from_store(&reopened);

    // Assert
    assert_interrupted_write(error);
    assert_eq!(view.head_blocks().expect("kept H").len(), 2);
    assert_eq!(view.best_block().expect("still no B"), None);
    remove_dir_if_exists(&fixture.path);
}

#[test]
fn replay_missing_undo_is_interrupted_write_and_keeps_h() {
    // Arrange
    let fixture = two_branch_fixture(true, false);

    // Act
    let error = expect_replay_error(
        replay_interrupted_flush(&fixture.store, fixture.view),
        "missing old undo fail-closes",
    );
    drop(fixture.store);
    let reopened = FjallNodeStore::open(&fixture.path).expect("reopen");
    let view = FjallCoinsView::from_store(&reopened);

    // Assert
    assert_interrupted_write(error);
    assert_eq!(view.head_blocks().expect("kept H").len(), 2);
    assert_eq!(view.best_block().expect("still no B"), None);
    remove_dir_if_exists(&fixture.path);
}

#[test]
fn replay_does_not_call_connect_block() {
    // Arrange
    let source = include_str!("../replay.rs");

    // Act / Assert
    assert!(
        !source.contains("connect_block"),
        "replay must stay apply-only"
    );
    assert!(
        !source.contains("FlushForPrune"),
        "replay must not mention FlushForPrune"
    );
}

#[test]
fn flush_rs_still_has_no_replay_or_block_hash() {
    // Arrange
    let flush_src = include_str!("../../../../open-bitcoin-chainstate/src/coins/flush.rs");

    // Act
    let mentions_forbidden = flush_src.contains("ReplayBlocks")
        || flush_src.contains("BlockHash")
        || flush_src.contains("FlushForPrune");

    // Assert
    assert!(
        !mentions_forbidden,
        "flush.rs must stay count-only and prune-flush-free"
    );
}
