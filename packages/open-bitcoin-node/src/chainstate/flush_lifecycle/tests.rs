// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

mod error_map;

use std::{
    cell::Cell,
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainstateError, Coin, CoinsBatch, CoinsCache, CoinsCacheEntry, CoinsView,
        FlushDecision, FlushMode, FlushPolicyTime, RecoveryDecision,
    },
    consensus::block_hash,
    primitives::{
        Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
        Transaction, TransactionInput, TransactionOutput, Txid,
    },
};
use open_bitcoin_network::HeaderEntry;

use super::{
    COINS_DB_CACHE_CAP_BYTES, DEFAULT_KERNEL_CACHE_BYTES, FlushLifecycle, FlushPersistSink,
    MIN_DBCACHE_BYTES, ManagerReadiness, apply_recovery_decision, default_coins_cache_byte_limit,
    initialize, probe_disk_free_bytes,
};
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
        "open-bitcoin-flush-lifecycle-{test_name}-{}-{timestamp}",
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

fn policy_now() -> FlushPolicyTime {
    FlushPolicyTime::from_unix_seconds(1_700_000_000)
}

fn open_temp_store(test_name: &str) -> (PathBuf, FjallNodeStore) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    (path, store)
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

fn sample_coin() -> Coin {
    Coin {
        output: output(50),
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 1_700_000_001,
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

fn plant_interrupted_heads(view: &FjallCoinsView, new_hash: BlockHash, old_hash: BlockHash) {
    let heads = encode_head_blocks_value(&[new_hash, old_hash]).expect("encode H");
    view.write_raw_bytes(&encode_head_blocks_key(), heads)
        .expect("plant H");
    view.delete_raw_bytes(&encode_best_block_key())
        .expect("remove B");
}

fn initialize_ready(
    store: &FjallNodeStore,
) -> (FlushLifecycle, FjallCoinsView, CoinsCache<FjallCoinsView>) {
    initialize(store, policy_now(), policy_now(), 0, false, u64::MAX).expect("initialize")
}

fn expect_error<T>(result: Result<T, StorageError>, message: &str) -> StorageError {
    match result {
        Ok(_) => panic!("{message}: expected error"),
        Err(error) => error,
    }
}

struct SucceedingSink;

impl FlushPersistSink for SucceedingSink {
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_undo(&mut self, _hash: BlockHash, _undo: &BlockUndo) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_chain_meta(
        &mut self,
        _active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
    ) -> Result<(), StorageError> {
        Ok(())
    }
}

struct UndoFailingSink;

impl FlushPersistSink for UndoFailingSink {
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_undo(&mut self, _hash: BlockHash, _undo: &BlockUndo) -> Result<(), StorageError> {
        Err(StorageError::BackendFailure {
            namespace: StorageNamespace::Chainstate,
            message: "undo persist failed".to_string(),
            action: StorageRecoveryAction::Repair,
        })
    }

    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_chain_meta(
        &mut self,
        _active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
    ) -> Result<(), StorageError> {
        Ok(())
    }
}

struct RecordingCoinsView {
    writes: Cell<usize>,
}

impl CoinsView for RecordingCoinsView {
    fn get_coin(&self, _outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Ok(None)
    }

    fn have_coin(&self, _outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        Ok(false)
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Ok(None)
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Ok(Vec::new())
    }

    fn batch_write(
        &mut self,
        _writes: CoinsBatch,
        _maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.writes.set(self.writes.get().saturating_add(1));
        Ok(())
    }
}

fn dirty_recording_cache() -> CoinsCache<RecordingCoinsView> {
    let mut cache = CoinsCache::from_parent(RecordingCoinsView {
        writes: Cell::new(0),
    });
    cache
        .add_coin(
            OutPoint {
                txid: Txid::from_byte_array([0x11; 32]),
                vout: 0,
            },
            sample_coin(),
            true,
        )
        .expect("dirty overlay");
    cache
}

#[test]
fn default_coins_cache_byte_limit_is_442_mib() {
    // Arrange
    let flush_src = include_str!("../../../../open-bitcoin-chainstate/src/coins/flush.rs");

    // Act
    let limit = default_coins_cache_byte_limit();

    // Assert
    assert_eq!(limit, 442 * 1024 * 1024);
    assert_eq!(DEFAULT_KERNEL_CACHE_BYTES, 450 * 1024 * 1024);
    assert_eq!(MIN_DBCACHE_BYTES, 4 * 1024 * 1024);
    assert_eq!(COINS_DB_CACHE_CAP_BYTES, 8 * 1024 * 1024);
    assert!(
        !flush_src.contains("DEFAULT_KERNEL_CACHE_BYTES")
            && !flush_src.contains("MIN_DBCACHE_BYTES")
            && !flush_src.contains("COINS_DB_CACHE_CAP_BYTES"),
        "flush.rs must stay unpinned from shell cache defaults"
    );
}

#[test]
fn initialize_consistent_store_is_ready_to_flush() {
    // Arrange
    let (path, store) = open_temp_store("consistent-ready");

    // Act
    let (lifecycle, _view, cache) = initialize_ready(&store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_eq!(cache.cache_entry_count(), 0);
    remove_dir_if_exists(&path);
}

#[test]
fn initialize_interrupted_without_bodies_is_not_ready_to_flush() {
    // Arrange
    let (path, store) = open_temp_store("interrupted-no-bodies");
    let view = FjallCoinsView::from_store(&store);
    plant_interrupted_heads(
        &view,
        BlockHash::from_byte_array([0xaa; 32]),
        BlockHash::from_byte_array([0xbb; 32]),
    );

    // Act
    let error = expect_error(
        initialize(&store, policy_now(), policy_now(), 0, false, u64::MAX),
        "missing bodies fail-closed",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::InterruptedWrite {
                namespace: StorageNamespace::Coins,
                action: StorageRecoveryAction::Reindex,
            }
        ),
        "expected InterruptedWrite, got {error:?}"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn initialize_interrupted_with_bodies_is_ready_after_replay() {
    // Arrange
    let (path, store) = open_temp_store("interrupted-with-bodies");
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
    let (lifecycle, view, cache) = initialize_ready(&store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_eq!(view.best_block().expect("best"), Some(new_hash));
    assert_eq!(cache.cache_entry_count(), 0);
    remove_dir_if_exists(&path);
}

#[test]
fn execute_flush_none_does_not_write_coins() {
    // Arrange
    let (path, store) = open_temp_store("none-no-write");
    let tip = BlockHash::from_byte_array([7_u8; 32]);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    };
    let mut planted = FjallCoinsView::from_store(&store);
    planted
        .batch_write(dirty_unspent_batch(&[(outpoint, sample_coin())]), Some(tip))
        .expect("plant consistent tip");
    let (mut lifecycle, view, mut cache) = initialize_ready(&store);
    let before = view.best_block().expect("best before");

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::IfNeeded,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("none flush");

    // Assert
    assert!(matches!(execution.decision, FlushDecision::None(_)));
    assert!(!execution.wrote_coins);
    assert_eq!(view.best_block().expect("best after"), before);
    assert_eq!(before, Some(tip));
    remove_dir_if_exists(&path);
}

#[test]
fn execute_flush_aborts_coins_when_undo_save_fails() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let mut cache = dirty_recording_cache();
    let undo_hash = BlockHash::from_byte_array([0x22; 32]);

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut UndoFailingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[(undo_hash, BlockUndo::default())],
            &[],
            &[],
            &[],
        ),
        "undo persist aborts",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::BackendFailure {
                ref message,
                ..
            } if message == "undo persist failed"
        ),
        "expected undo persist failure, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn execute_flush_always_uses_cache_flush_kind_from_decide_flush() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    lifecycle.set_next_write(policy_now());
    let mut cache = dirty_recording_cache();

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("always flush");

    // Assert
    assert!(
        matches!(execution.decision, FlushDecision::Flush(_)),
        "Always must map to cache Flush, got {:?}",
        execution.decision
    );
    assert!(execution.wrote_coins);
    assert_eq!(cache.parent().writes.get(), 1);
}

#[test]
fn execute_flush_before_ready_does_not_write() {
    // Arrange
    let mut lifecycle = FlushLifecycle::not_ready_for_test();
    let mut cache = dirty_recording_cache();

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        ),
        "flush before ready",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail == "flush before ready"
        ),
        "expected flush before ready, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn execute_flush_refuse_disk_space_does_not_write_coins() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let mut cache = dirty_recording_cache();

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            0,
            &[],
            &[],
            &[],
            &[],
        ),
        "refuse disk space",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail == "refuse disk space"
        ),
        "expected refuse disk space, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn apply_recovery_one_head_and_inconsistent_count() {
    // Arrange
    let (path, store) = open_temp_store("recovery-decision");
    let view = FjallCoinsView::from_store(&store);

    // Act
    let kept = match apply_recovery_decision(&store, view, RecoveryDecision::OneHead) {
        Ok(view) => view,
        Err(error) => panic!("one head: {error:?}"),
    };
    let inconsistent = expect_error(
        apply_recovery_decision(
            &store,
            FjallCoinsView::from_store(&store),
            RecoveryDecision::InconsistentOtherCount { count: 3 },
        ),
        "inconsistent",
    );

    // Assert
    assert_eq!(kept.head_blocks().expect("heads"), Vec::<BlockHash>::new());
    assert!(
        matches!(
            inconsistent,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail.contains("unexpected head_blocks count 3")
        ),
        "expected inconsistent count, got {inconsistent:?}"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn fjall_store_implements_flush_persist_sink() {
    // Arrange
    let (path, mut store) = open_temp_store("fjall-sink");
    let block_header = header(BlockHash::from_byte_array([0_u8; 32]), 9);
    let block = Block {
        header: block_header.clone(),
        transactions: vec![coinbase(0, 50)],
    };
    let hash = block_hash(&block_header);

    // Act
    let block_ok = store.persist_block(&block);
    let undo_ok = store.persist_undo(hash, &BlockUndo::default());
    let empty_headers = store.persist_header_entries(&[]);
    let headers_ok = store.persist_header_entries(&[header_entry(block_header, 0, 1)]);

    // Assert
    assert!(block_ok.is_ok(), "{block_ok:?}");
    assert!(undo_ok.is_ok(), "{undo_ok:?}");
    assert!(empty_headers.is_ok(), "{empty_headers:?}");
    assert!(headers_ok.is_ok(), "{headers_ok:?}");
    remove_dir_if_exists(&path);
}

#[test]
fn probe_disk_free_bytes_is_defined() {
    // Arrange / Act / Assert
    assert_eq!(probe_disk_free_bytes(Path::new("/")), u64::MAX);
}

#[test]
fn execute_flush_periodic_due_uses_sync_from_decide_flush() {
    // Arrange
    let mut lifecycle = FlushLifecycle::ready_for_test(
        default_coins_cache_byte_limit(),
        0,
        FlushPolicyTime::from_unix_seconds(1),
        false,
    );
    let mut cache = CoinsCache::from_parent(RecordingCoinsView {
        writes: Cell::new(0),
    });

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(2),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("periodic sync");

    // Assert
    assert!(
        matches!(execution.decision, FlushDecision::Sync(_)),
        "Periodic due must map to cache Sync, got {:?}",
        execution.decision
    );
    assert!(execution.wrote_coins);
    assert_eq!(cache.parent().writes.get(), 1);
}

#[test]
fn persist_calls_execute_flush_ifneeded() {
    // Arrange / Act / Assert
    let persist_src = include_str!("../../chainstate.rs");
    assert!(
        persist_src.contains("self.flush_lifecycle.execute_flush"),
        "ManagedChainstate::persist must call execute_flush"
    );
    assert!(
        persist_src.contains("FlushMode::IfNeeded"),
        "ManagedChainstate::persist must use IfNeeded"
    );
    assert!(
        !persist_src.contains("save_snapshot(self.chainstate.snapshot())"),
        "ManagedChainstate::persist must not write leftover snapshots"
    );
}
