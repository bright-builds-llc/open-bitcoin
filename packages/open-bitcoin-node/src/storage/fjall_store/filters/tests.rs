// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::*;
use crate::storage::{
    PersistMode,
    coins_codec::{encode_best_block_key, encode_best_block_value},
};
use open_bitcoin_core::{
    chainstate::{
        BasicFilterInputs, BlockUndo, ChainPosition, CoinsView, HistoricalBlockUndo, PruneLockInfo,
    },
    consensus::block_merkle_root,
    primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

mod append;
mod append_proof;
mod faults;
mod lifecycle;
mod ownership;
mod reorg;
use faults::{append_current, publish_current, seed_orphan_records};

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "open-bitcoin-filter-index-{name}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn fixture_block(parent: BlockHash, height: u32) -> Block {
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![1, height as u8]).expect("script"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        }],
        lock_time: 0,
    };
    let transactions = vec![transaction];
    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: parent,
            merkle_root: block_merkle_root(&transactions).expect("merkle").0,
            time: 1_000 + height,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    }
}

/// Codec-valid historical coinbase-only fixtures; no claim of consensus validation.
fn fixtures(count: u32) -> (Vec<ChainPosition>, Vec<StoredFilterRecord>) {
    let mut positions = Vec::new();
    let mut records: Vec<StoredFilterRecord> = Vec::new();
    let mut parent = BlockHash::default();
    for height in 0..count {
        let block = fixture_block(parent, height);
        let position = ChainPosition::new(
            block.header.clone(),
            height,
            u128::from(height) + 1,
            i64::from(block.header.time),
        );
        let undo = BlockUndo {
            transactions: Vec::new(),
        };
        let maybe_history = (height != 0).then_some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &undo,
        });
        let inputs = BasicFilterInputs::from_historical(&block, &position, maybe_history)
            .expect("historical input");
        let maybe_predecessor = records.last().map(|record| record.identity());
        records.push(
            StoredFilterRecord::generate(&inputs, &position, maybe_predecessor.as_ref())
                .expect("generated record"),
        );
        parent = position.block_hash;
        positions.push(position);
    }
    (positions, records)
}

fn seed(store: &FjallNodeStore, positions: &[ChainPosition]) {
    store
        .save_chain_meta(positions, PersistMode::Sync)
        .expect("metadata");
    store
        .coins_view()
        .write_raw_bytes(
            &encode_best_block_key(),
            encode_best_block_value(positions.last().expect("tip").block_hash),
        )
        .expect("coins B");
}
fn fence(positions: &[ChainPosition]) -> VerifiedChainstateFence<'_> {
    VerifiedChainstateFence::new(
        Some(positions.last().expect("tip").block_hash),
        Some(positions),
    )
    .expect("fence")
}
fn checkpoint(record: &StoredFilterRecord) -> FilterCheckpoint {
    FilterCheckpoint::new(IndexPrefix::Committed(record.identity()))
}
fn publish(store: &FjallNodeStore, positions: &[ChainPosition], records: &[StoredFilterRecord]) {
    let checkpoint = checkpoint(records.last().expect("endpoint"));
    publish_current(
        store,
        &fence(positions),
        checkpoint,
        checkpoint.input_protection(),
        records,
    )
    .expect("publish");
}

#[test]
fn filter_index_atomic_publication_and_idempotent_records_survive_real_reopen() {
    // Arrange
    let path = temp_path("success");
    let (positions, records) = fixtures(3);
    let store = FjallNodeStore::open(&path).expect("first open");
    seed(&store, &positions);
    let operator = PruneLockInfo {
        name: "operator".to_owned(),
        height_first: 0,
        height_last: 50,
    };
    store
        .sync_prune_locks(std::slice::from_ref(&operator))
        .expect("operator lock");
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("initialize");
    // Act
    publish(&store, &positions, &records);
    append_current(&store, &records).expect("idempotent");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    // Assert
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(checkpoint(&records[2]))
    );
    for (height, record) in records.iter().enumerate() {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable"),
            Some(record.clone())
        );
        assert_eq!(
            reopened
                .maybe_active_basic_filter_record(height as u32)
                .expect("active"),
            Some(record.clone())
        );
    }
    assert!(
        reopened
            .load_prune_locks()
            .expect("locks")
            .contains(&operator)
    );
    assert_eq!(
        reopened
            .maybe_basic_filter_state()
            .expect("state")
            .expect("present")
            .protection,
        IndexInputProtection::FromHeight(3)
    );
    assert_eq!(
        reopened.coins_view().best_block().expect("B"),
        Some(positions[2].block_hash)
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_orphan_record_fixture_cannot_create_or_advance_checkpoint() {
    // Arrange
    let path = temp_path("records-only");
    let (positions, records) = fixtures(3);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    // Act
    seed_orphan_records(&store, &records);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("second open");
    // Assert
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        None
    );
    assert_eq!(
        reopened
            .maybe_active_basic_filter_record(0)
            .expect("active"),
        None
    );
    assert!(reopened.load_prune_locks().expect("locks").is_empty());
    assert!(
        reopened
            .initialize_basic_filter_state(&fence(&positions))
            .is_err()
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(records[2].identity().block_hash())
            .expect("record"),
        Some(records[2].clone())
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_rewind_hides_suffix_and_preserves_rows_and_operator_locks() {
    // Arrange
    let path = temp_path("rewind");
    let (positions, records) = fixtures(3);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records);
    // Act
    seed(&store, &positions[..1]);
    publish_current(
        &store,
        &fence(&positions[..1]),
        checkpoint(&records[0]),
        IndexInputProtection::FromHeight(1),
        &[],
    )
    .expect("rewind");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("second open");
    // Assert
    assert_eq!(
        reopened
            .maybe_active_basic_filter_record(1)
            .expect("suffix hidden"),
        None
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(records[2].identity().block_hash())
            .expect("immutable suffix"),
        Some(records[2].clone())
    );
    assert_eq!(
        reopened
            .basic_filter_projection(2)
            .expect("retained projection"),
        records[2].identity().block_hash()
    );
    assert_eq!(
        reopened
            .maybe_basic_filter_state()
            .expect("state")
            .expect("present")
            .protection,
        IndexInputProtection::FromHeight(1)
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_stale_fence_and_oversized_batch_refuse_before_publication() {
    // Arrange
    let path = temp_path("bounds");
    let (positions, records) = fixtures(129);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions[..1]);
    store
        .initialize_basic_filter_state(&fence(&positions[..1]))
        .expect("init");
    // Act
    let oversized = append_current(&store, &records);
    let stale = publish_current(
        &store,
        &fence(&positions),
        checkpoint(&records[0]),
        IndexInputProtection::FromHeight(1),
        &records[..1],
    );
    // Assert
    assert!(oversized.is_err());
    assert!(stale.is_err());
    assert_eq!(
        store.basic_filter_artifacts().expect("artifacts"),
        (false, false)
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_missing_predecessor_refuses_entire_record_batch() {
    // Arrange
    let path = temp_path("missing-parent");
    let (positions, records) = fixtures(2);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    // Act
    let result = append_current(&store, &records[1..]);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_artifacts().expect("artifacts"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_conflicting_immutable_rewrite_refuses_and_retains_existing_bytes_after_reopen() {
    // Arrange
    let path = temp_path("conflict");
    let (positions, records) = fixtures(1);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    append_current(&store, &records).expect("initial");
    let key = codec::record_key(records[0].identity().block_hash());
    let mut alternate = codec::encode_record(&records[0]);
    let hash = open_bitcoin_core::primitives::FilterHash::from_byte_array(
        open_bitcoin_core::consensus::crypto::double_sha256(&[0]),
    );
    let header = open_bitcoin_core::consensus::compute_filter_header(hash, Default::default());
    alternate.truncate(codec::RECORD_OVERHEAD);
    alternate[102..134].copy_from_slice(hash.as_bytes());
    alternate[134..166].copy_from_slice(header.as_bytes());
    alternate[166..170].copy_from_slice(&1_u32.to_le_bytes());
    alternate.push(0);
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, alternate.clone())
        .expect("self-consistent alternate record");
    let previous = store
        .load_basic_filter_record(records[0].identity().block_hash())
        .expect("alternate validates")
        .expect("present");
    // Act
    let error = append_current(&store, &records).expect_err("immutable conflict");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("second open");
    // Assert
    assert!(error.to_string().contains("conflicting immutable BASIC"));
    assert_eq!(
        reopened
            .get_bytes(StorageNamespace::BlockIndex, &key)
            .expect("bytes"),
        Some(alternate)
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(previous.identity().block_hash())
            .expect("retained"),
        Some(previous)
    );
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("empty state"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn malformed_reopen(category: &str) {
    // Arrange
    let path = temp_path(category);
    let (positions, records) = fixtures(3);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records[..2]);
    append_current(&store, &records[2..]).expect("suffix");
    let untouched = store
        .get_bytes(
            StorageNamespace::BlockIndex,
            &codec::record_key(records[0].identity().block_hash()),
        )
        .expect("prefix bytes");
    let state = store
        .get_bytes(StorageNamespace::BlockIndex, codec::STATE_KEY)
        .expect("state bytes");
    let locks = store
        .get_bytes(
            StorageNamespace::BlockIndex,
            super::super::prune::PRUNE_LOCKS_KEY,
        )
        .expect("lock bytes");
    let (key, bytes) = match category {
        "record" => (
            codec::record_key(records[2].identity().block_hash()),
            vec![2, 0],
        ),
        "state" => (codec::STATE_KEY.to_owned(), vec![1, 0, 2]),
        "projection" => (
            codec::active_key(1),
            codec::encode_projection(2, records[1].identity().block_hash()),
        ),
        "protection" => (super::super::prune::PRUNE_LOCKS_KEY.to_owned(), vec![255]),
        _ => panic!("unknown fixture"),
    };
    // Act
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, bytes.clone())
        .expect("corrupt fixture");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    let result = match category {
        "record" => reopened.validate_basic_filter_records(),
        "state" | "projection" => reopened.maybe_basic_filter_checkpoint().map(|_| ()),
        "protection" => reopened.recover_basic_filter_index_before_prune(Some(
            positions.last().expect("tip").block_hash,
        )),
        _ => panic!("unknown fixture"),
    };
    // Assert
    let error = result.expect_err("corruption refuses");
    assert_eq!(
        error.recovery_action(),
        Some(super::super::StorageRecoveryAction::Repair)
    );
    assert_eq!(
        reopened
            .get_bytes(StorageNamespace::BlockIndex, &key)
            .expect("corrupt bytes untouched"),
        Some(bytes)
    );
    assert_eq!(
        reopened
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[0].identity().block_hash())
            )
            .expect("prefix bytes"),
        untouched
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(records[0].identity().block_hash())
            .expect("prefix readable"),
        Some(records[0].clone())
    );
    if category != "state" {
        assert_eq!(
            reopened
                .get_bytes(StorageNamespace::BlockIndex, codec::STATE_KEY)
                .expect("state"),
            state
        );
    }
    if category != "protection" {
        assert_eq!(
            reopened
                .get_bytes(
                    StorageNamespace::BlockIndex,
                    super::super::prune::PRUNE_LOCKS_KEY
                )
                .expect("locks"),
            locks
        );
    }
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_malformed_suffix_record_refuses_after_real_reopen() {
    malformed_reopen("record");
}
#[test]
fn filter_index_malformed_state_refuses_after_real_reopen() {
    malformed_reopen("state");
}
#[test]
fn filter_index_misplaced_projection_refuses_after_real_reopen() {
    malformed_reopen("projection");
}
#[test]
fn filter_index_malformed_protection_refuses_after_real_reopen() {
    malformed_reopen("protection");
}

#[test]
fn filter_index_missing_immutable_predecessor_after_reopen_preserves_untouched_prefix() {
    // Arrange
    let path = temp_path("missing-durable-predecessor");
    let (positions, records) = fixtures(3);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records);
    let state = store.maybe_basic_filter_state().expect("state");
    let locks = store.load_prune_locks().expect("locks");
    // Act
    store
        .block_index
        .remove(codec::record_key(records[1].identity().block_hash()))
        .expect("missing predecessor fixture");
    store
        .persist(StorageNamespace::BlockIndex, PersistMode::Sync)
        .expect("sync fixture");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    // Assert
    assert!(
        reopened
            .load_basic_filter_record(records[2].identity().block_hash())
            .expect_err("missing predecessor")
            .to_string()
            .contains("predecessor")
    );
    assert_eq!(reopened.maybe_basic_filter_state().expect("state"), state);
    assert_eq!(reopened.load_prune_locks().expect("locks"), locks);
    assert_eq!(
        reopened
            .load_basic_filter_record(records[0].identity().block_hash())
            .expect("prefix"),
        Some(records[0].clone())
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_legacy_schema_one_migrates_without_creating_index_state() {
    // Arrange
    let path = temp_path("schema-one");
    let (positions, _) = fixtures(1);
    let legacy = FjallNodeStore::open_without_ensure_schema_for_test(&path).expect("legacy open");
    let snapshot = open_bitcoin_core::chainstate::ChainstateSnapshot::new(
        positions.clone(),
        Default::default(),
        Default::default(),
    );
    legacy
        .save_chainstate_snapshot(&snapshot, PersistMode::Sync)
        .expect("legacy snapshot");
    legacy
        .save_block(&fixture_block(BlockHash::default(), 0), PersistMode::Sync)
        .expect("migration confirmation evidence");
    legacy.write_schema_version_for_test(1).expect("schema one");
    // Act
    drop(legacy);
    let reopened = FjallNodeStore::open(&path).expect("migration reopen");
    // Assert
    assert_eq!(
        reopened.coins_view().best_block().expect("migrated B"),
        Some(positions[0].block_hash)
    );
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("legacy state"),
        None
    );
    assert_eq!(
        reopened.basic_filter_artifacts().expect("legacy artifacts"),
        (false, false)
    );
    assert!(reopened.load_prune_locks().expect("locks").is_empty());
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}
