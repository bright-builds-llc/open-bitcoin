// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Sparse deletion-order fixtures plus separately validated spend/branch recovery evidence.

use super::*;
use crate::storage::{
    coins_codec::{encode_best_block_key, encode_best_block_value},
    filter_index::{self as codec, StoredFilterRecord},
    fjall_store::PruneIntent,
};
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, BasicFilterInputs, BlockUndo, CoinsView, FilterCheckpoint,
    HistoricalBlockUndo, IndexInputProtection, IndexPrefix, VerifiedChainstateFence,
};

mod accepted;
mod catch_up;
mod evidence;
mod faults;
mod lifecycle;
mod prune_coordination;
mod prune_faults;
mod recovery;
mod startup;

fn reserved_filter_path(name: &str) -> PathBuf {
    let base = temp_store_path(name);
    for suffix in 0_u64.. {
        let candidate = base.with_extension(suffix.to_string());
        match fs::create_dir(&candidate) {
            Ok(()) => return candidate,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => panic!("reserve filter fixture {}: {error}", candidate.display()),
        }
    }
    panic!("filter fixture suffix exhausted");
}

struct FilterStartupFixture {
    path: PathBuf,
    positions: Vec<ChainPosition>,
    records: Vec<StoredFilterRecord>,
    intent: PruneIntent,
}

impl FilterStartupFixture {
    fn new(name: &str, intent_height: u32, maybe_endpoint: Option<u32>) -> Self {
        let path = reserved_filter_path(name);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut positions = Vec::new();
        let mut records: Vec<StoredFilterRecord> = Vec::new();
        let mut parent = BlockHash::default();
        for height in 0..=400 {
            let block = fixture_block(parent, height);
            let position = ChainPosition::new(
                block.header.clone(),
                height,
                u128::from(height) + 1,
                i64::from(block.header.time),
            );
            let undo = BlockUndo::default();
            if maybe_endpoint.is_some_and(|endpoint| height <= endpoint) {
                let maybe_history = (height != 0).then_some(HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo: &undo,
                });
                let inputs = BasicFilterInputs::from_historical(&block, &position, maybe_history)
                    .expect("historical inputs");
                let maybe_previous = records.last().map(StoredFilterRecord::identity);
                records.push(
                    StoredFilterRecord::generate(&inputs, &position, maybe_previous.as_ref())
                        .expect("filter"),
                );
            }
            if height == intent_height {
                store
                    .save_block(&block, PersistMode::Sync)
                    .expect("intent body");
                if height != 0 {
                    store
                        .save_undo(position.block_hash, &undo, PersistMode::Sync)
                        .expect("intent undo");
                }
            }
            parent = position.block_hash;
            positions.push(position);
        }
        seed_authority(&store, &positions);
        if let Some(endpoint) = maybe_endpoint {
            store
                .initialize_basic_filter_state(&fence(&positions))
                .expect("empty state");
            let checkpoint = checkpoint(&records[endpoint as usize]);
            store
                .publish_basic_filter_checkpoint(
                    &store
                        .maybe_basic_filter_work(&fence(&positions))
                        .expect("work")
                        .expect("Active"),
                    &fence(&positions),
                    checkpoint,
                    checkpoint.input_protection(),
                    &records,
                )
                .expect("checkpoint");
        }
        let intent = PruneIntent {
            height: intent_height,
            block_hash: positions[intent_height as usize].block_hash,
        };
        seed_raw_prune_intent(&store, intent);
        drop(store);
        Self {
            path,
            positions,
            records,
            intent,
        }
    }

    fn open_runtime(&self) -> Result<DurableSyncRuntime, SyncRuntimeError> {
        DurableSyncRuntime::open(
            FjallNodeStore::open(&self.path).expect("real reopen"),
            sync_config(),
        )
    }

    fn cleanup(self) {
        fs::remove_dir_all(self.path).expect("cleanup");
    }
}

fn fixture_block(parent: BlockHash, height: u32) -> Block {
    let transactions = vec![Transaction {
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
    }];
    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: parent,
            merkle_root: block_merkle_root(&transactions).expect("merkle").0,
            time: 1_000 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    }
}

fn seed_authority(store: &FjallNodeStore, positions: &[ChainPosition]) {
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
    .expect("verified fence")
}

fn checkpoint(record: &StoredFilterRecord) -> FilterCheckpoint {
    FilterCheckpoint::new(IndexPrefix::Committed(record.identity()))
}

fn append_current(
    store: &FjallNodeStore,
    records: &[StoredFilterRecord],
) -> Result<(), StorageError> {
    let positions = store.load_chain_meta_for_open()?.0;
    let fence = VerifiedChainstateFence::new(
        store
            .coins_view()
            .best_block()
            .map_err(codec::index_corruption)?,
        Some(&positions),
    )
    .map_err(codec::index_corruption)?;
    let work = store
        .maybe_basic_filter_work(&fence)?
        .ok_or_else(|| codec::index_corruption("test requires Active BASIC work"))?;
    store.persist_basic_filter_records(&work, &fence, records)
}

fn publish_current(
    store: &FjallNodeStore,
    fence: &VerifiedChainstateFence<'_>,
    checkpoint: FilterCheckpoint,
    protection: IndexInputProtection,
    records: &[StoredFilterRecord],
) -> Result<(), StorageError> {
    let work = store
        .maybe_basic_filter_work(fence)?
        .ok_or_else(|| codec::index_corruption("test requires Active BASIC work"))?;
    store.publish_basic_filter_checkpoint(&work, fence, checkpoint, protection, records)
}

/// Deliberately unsafe persisted recovery fixture; never bypass a production guard.
fn seed_raw_prune_intent(store: &FjallNodeStore, intent: PruneIntent) {
    let mut bytes = intent.height.to_le_bytes().to_vec();
    bytes.extend_from_slice(intent.block_hash.as_bytes());
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, "prune_intent", bytes)
        .expect("raw recovered intent fixture");
}

/// Saved stronger/invalid ownership test facts, independent of operator authority.
fn seed_raw_basic_protection(store: &FjallNodeStore, protection: IndexInputProtection) {
    let lock = protection.maybe_prune_lock().expect("test range");
    let mut bytes = 1_u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&u16::try_from(lock.name.len()).expect("name").to_le_bytes());
    bytes.extend_from_slice(lock.name.as_bytes());
    bytes.extend_from_slice(&lock.height_first.to_le_bytes());
    bytes.extend_from_slice(&lock.height_last.to_le_bytes());
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, "prune_locks", bytes)
        .expect("raw recovery protection fixture");
}

/// Open only after every production store/runtime handle has been dropped.
fn raw_index(path: &Path, operation: impl FnOnce(&fjall::Keyspace)) {
    raw_namespace(path, "block_index", operation);
}

fn raw_namespace(path: &Path, namespace: &str, operation: impl FnOnce(&fjall::Keyspace)) {
    let db = fjall::Database::builder(path).open().expect("raw reopen");
    let index = db
        .keyspace(namespace, fjall::KeyspaceCreateOptions::default)
        .expect("existing index");
    operation(&index);
    db.persist(fjall::PersistMode::SyncAll).expect("raw sync");
}

fn snapshot_index(path: &Path) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut rows = Vec::new();
    raw_index(path, |index| {
        for guard in index.iter() {
            let (key, value) = guard.into_inner().expect("row");
            rows.push((key.to_vec(), value.to_vec()));
        }
    });
    rows
}

fn assert_filter_refusal(result: Result<DurableSyncRuntime, SyncRuntimeError>, category: &str) {
    let Err(SyncRuntimeError::Storage(error)) = result else {
        panic!("expected filter startup storage refusal");
    };
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                action: StorageRecoveryAction::Repair,
                ..
            }
        ),
        "{error}"
    );
    let message = error.to_string();
    assert!(
        message.contains("fail_closed BASIC index startup"),
        "{message}"
    );
    assert!(message.contains(category), "{message}");
}

fn assert_payload(store: &FjallNodeStore, fixture: &FilterStartupFixture, body: bool, undo: bool) {
    assert_eq!(
        store.has_block(fixture.intent.block_hash).expect("body"),
        body
    );
    assert_eq!(
        store
            .load_block(fixture.intent.block_hash)
            .expect("body decode")
            .is_some(),
        body
    );
    assert_eq!(
        store.has_undo(fixture.intent.block_hash).expect("undo"),
        undo
    );
    assert_eq!(
        store
            .load_undo(fixture.intent.block_hash)
            .expect("undo decode")
            .is_some(),
        undo
    );
    if body {
        assert_eq!(
            store
                .load_block(fixture.intent.block_hash)
                .expect("exact body"),
            Some(fixture_block(
                fixture.positions[fixture.intent.height as usize].previous_block_hash(),
                fixture.intent.height
            ))
        );
    }
    if undo {
        assert_eq!(
            store
                .load_undo(fixture.intent.block_hash)
                .expect("exact undo"),
            Some(BlockUndo::default())
        );
    }
}
