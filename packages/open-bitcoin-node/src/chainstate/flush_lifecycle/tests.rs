// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

mod durability;
mod error_map;
mod execute_flush;
mod initialize;
mod recovery_and_persist;

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

pub(super) fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-flush-lifecycle-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

pub(super) fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

pub(super) fn policy_now() -> FlushPolicyTime {
    FlushPolicyTime::from_unix_seconds(1_700_000_000)
}

pub(super) fn open_temp_store(test_name: &str) -> (PathBuf, FjallNodeStore) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");
    (path, store)
}

pub(super) fn script() -> ScriptBuf {
    ScriptBuf::from_bytes(vec![0x51]).expect("valid script")
}

pub(super) fn output(value: i64) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(value).expect("valid amount"),
        script_pubkey: script(),
    }
}

pub(super) fn sample_coin() -> Coin {
    Coin {
        output: output(50),
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 1_700_000_001,
    }
}

pub(super) fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_700_000_000 + nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

pub(super) fn header_entry(
    block_header: BlockHeader,
    height: u32,
    chain_work: u128,
) -> HeaderEntry {
    HeaderEntry {
        block_hash: block_hash(&block_header),
        header: block_header,
        height,
        chain_work,
    }
}

pub(super) fn coinbase(height: u32, value: i64) -> Transaction {
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

pub(super) fn dirty_unspent_batch(pairs: &[(OutPoint, Coin)]) -> CoinsBatch {
    let mut entries = HashMap::new();
    for (outpoint, planted) in pairs {
        entries.insert(
            outpoint.clone(),
            CoinsCacheEntry::unspent_dirty(planted.clone()),
        );
    }
    CoinsBatch { entries }
}

pub(super) fn plant_interrupted_heads(
    view: &FjallCoinsView,
    new_hash: BlockHash,
    old_hash: BlockHash,
) {
    let heads = encode_head_blocks_value(&[new_hash, old_hash]).expect("encode H");
    view.write_raw_bytes(&encode_head_blocks_key(), heads)
        .expect("plant H");
    view.delete_raw_bytes(&encode_best_block_key())
        .expect("remove B");
}

pub(super) fn initialize_ready(
    store: &FjallNodeStore,
) -> (FlushLifecycle, FjallCoinsView, CoinsCache<FjallCoinsView>) {
    initialize(store, policy_now(), policy_now(), 0, false, u64::MAX).expect("initialize")
}

pub(super) fn expect_error<T>(result: Result<T, StorageError>, message: &str) -> StorageError {
    match result {
        Ok(_) => panic!("{message}: expected error"),
        Err(error) => error,
    }
}

pub(super) struct SucceedingSink;

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

pub(super) struct UndoFailingSink;

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

pub(super) struct RecordingCoinsView {
    pub(super) writes: Cell<usize>,
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

pub(super) fn dirty_recording_cache() -> CoinsCache<RecordingCoinsView> {
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
