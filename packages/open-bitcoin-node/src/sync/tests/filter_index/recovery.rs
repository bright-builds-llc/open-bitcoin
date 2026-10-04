// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Genuine staged validation and historical spend facts, distinct from sparse prune fixtures.

use super::*;
use open_bitcoin_core::{
    chainstate::{Chainstate, CoinsBatch, CoinsCacheEntry, CoinsView},
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid},
};

pub(super) struct ValidatedHistory {
    pub path: PathBuf,
    pub blocks: Vec<Block>,
    pub records: Vec<StoredFilterRecord>,
    pub old: ChainstateSnapshot,
    pub full: ChainstateSnapshot,
}

impl ValidatedHistory {
    pub fn new(name: &str, replacement: bool) -> Self {
        let mut chainstate = Chainstate::default();
        let genesis = validated_block(BlockHash::default(), 0, vec![reward(0, 0x55)]);
        let funding = validated_block(block_hash(&genesis.header), 1, vec![reward(1, 0x51)]);
        let first = spend(&funding.transactions[0], 4_999_999_000, 0x52);
        let second = spend(&first, 4_999_998_000, if replacement { 0x56 } else { 0x53 });
        let spending = validated_block(
            block_hash(&funding.header),
            2,
            vec![reward(2, 0x54), first, second],
        );
        let blocks = vec![genesis, funding, spending];
        let mut records: Vec<StoredFilterRecord> = Vec::new();
        let mut maybe_old = None;
        for (height, block) in blocks.iter().enumerate() {
            let staged = chainstate
                .stage_connect_block_with_current_time(
                    block,
                    height as u128 + 1,
                    i64::from(block.header.time) + 1,
                    ScriptVerifyFlags::P2SH
                        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                    ConsensusParams {
                        coinbase_maturity: 1,
                        ..ConsensusParams::default()
                    },
                )
                .expect("actual consensus stage");
            if height == 2 {
                assert_eq!(staged.undo.transactions.len(), 2);
                let inputs = staged
                    .basic_filter_inputs(block)
                    .expect("validated spend inputs");
                assert_eq!(
                    inputs.spent_scripts().collect::<Vec<_>>(),
                    vec![&[0x51][..], &[0x52][..]]
                );
            }
            chainstate
                .commit_staged_connect(staged)
                .expect("validated commit");
            let position = chainstate.tip().expect("validated tip");
            let maybe_history = (height != 0).then(|| HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo: &chainstate.undo_by_block()[&position.block_hash],
            });
            let inputs = BasicFilterInputs::from_historical(block, position, maybe_history)
                .expect("retained validated history");
            let maybe_previous = records.last().map(StoredFilterRecord::identity);
            records.push(
                StoredFilterRecord::generate(&inputs, position, maybe_previous.as_ref())
                    .expect("historical record"),
            );
            if height == 1 {
                maybe_old = Some(chainstate.snapshot());
            }
        }
        let full = chainstate.snapshot();
        let old = maybe_old.expect("funding checkpoint");
        assert!(
            !full
                .utxos
                .contains_key(&outpoint(&blocks[1].transactions[0]))
        );
        assert!(
            !full
                .utxos
                .contains_key(&outpoint(&blocks[2].transactions[1]))
        );
        Self {
            path: temp_store_path(name),
            blocks,
            records,
            old,
            full,
        }
    }

    pub fn seed(&self, endpoint: usize) -> FjallNodeStore {
        let store = FjallNodeStore::open(&self.path).expect("real store");
        self.persist_payloads(&store);
        store
            .seed_coins_from_snapshot(&self.old)
            .expect("actual coins/meta checkpoint");
        store
            .initialize_basic_filter_state(&fence(&self.old.active_chain))
            .expect("empty index");
        let cp = checkpoint(&self.records[endpoint]);
        store
            .publish_basic_filter_checkpoint(
                &fence(&self.old.active_chain),
                cp,
                cp.input_protection(),
                &self.records[..=endpoint],
            )
            .expect("old safe checkpoint");
        store
    }

    pub fn persist_payloads(&self, store: &FjallNodeStore) {
        for block in &self.blocks {
            store.save_block(block, PersistMode::Sync).expect("body");
        }
        for (hash, undo) in &self.full.undo_by_block {
            store
                .save_undo(*hash, undo, PersistMode::Sync)
                .expect("validated undo");
        }
        let headers: Vec<_> = self
            .full
            .active_chain
            .iter()
            .map(|p| HeaderEntry {
                block_hash: p.block_hash,
                header: p.header.clone(),
                height: p.height,
                chain_work: p.chain_work,
            })
            .collect();
        store
            .save_header_entries(&headers, PersistMode::Sync)
            .expect("replay headers");
    }

    pub fn spend_batch(&self) -> CoinsBatch {
        let mut entries = std::collections::HashMap::new();
        for key in self.old.utxos.keys() {
            if !self.full.utxos.contains_key(key) {
                entries.insert(key.clone(), CoinsCacheEntry::spent_dirty());
            }
        }
        for (key, coin) in &self.full.utxos {
            if self.old.utxos.get(key) != Some(coin) {
                entries.insert(key.clone(), CoinsCacheEntry::unspent_dirty(coin.clone()));
            }
        }
        CoinsBatch { entries }
    }

    pub fn reopen(&self) -> Result<DurableSyncRuntime, SyncRuntimeError> {
        DurableSyncRuntime::open(
            FjallNodeStore::open(&self.path).expect("real Fjall reopen"),
            sync_config(),
        )
    }

    pub fn assert_rows_and_payloads(&self, store: &FjallNodeStore) {
        for record in &self.records {
            assert_eq!(
                store
                    .load_basic_filter_record(record.identity().block_hash())
                    .expect("immutable"),
                Some(record.clone())
            );
        }
        for block in &self.blocks {
            assert_eq!(
                store.load_block(block_hash(&block.header)).expect("body"),
                Some(block.clone())
            );
        }
        let hash = self.full.active_chain[2].block_hash;
        assert_eq!(
            store.load_undo(hash).expect("non-genesis historical undo"),
            Some(self.full.undo_by_block[&hash].clone())
        );
    }

    pub fn cleanup(self) {
        fs::remove_dir_all(self.path).expect("cleanup");
    }
}

fn reward(height: u8, output: u8) -> Transaction {
    let mut transaction = fixture_block(BlockHash::default(), u32::from(height))
        .transactions
        .remove(0);
    transaction.inputs[0].script_sig = ScriptBuf::from_bytes(if height == 0 {
        vec![0, 0x51]
    } else {
        vec![1, height, 0x51]
    })
    .expect("height script");
    transaction.outputs[0].script_pubkey = ScriptBuf::from_bytes(vec![output]).expect("output");
    transaction
}

fn outpoint(transaction: &Transaction) -> OutPoint {
    OutPoint {
        txid: transaction_txid(transaction).expect("txid"),
        vout: 0,
    }
}

fn spend(previous: &Transaction, value: i64, output: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: outpoint(previous),
            script_sig: ScriptBuf::default(),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("value"),
            script_pubkey: ScriptBuf::from_bytes(vec![output]).expect("output"),
        }],
        lock_time: 0,
    }
}

fn validated_block(parent: BlockHash, height: u32, transactions: Vec<Transaction>) -> Block {
    let mut block = fixture_block(parent, height);
    block.transactions = transactions;
    block.header.time = 1_000 + height * 100;
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy proof of work");
    block
}

#[test]
fn filter_index_production_validated_ahead_records_do_not_mint_cursor() {
    // Arrange
    let history = ValidatedHistory::new("filter-validated-ahead", false);
    let store = history.seed(1);
    store
        .persist_basic_filter_records(&history.records[2..])
        .expect("ahead rows before deferred coins flush");
    drop(store);
    let before = snapshot_index(&history.path);
    // Act
    let runtime = history.reopen().expect("old authority startup");
    // Assert
    let store = runtime.store();
    assert_eq!(
        store.coins_view().best_block().expect("B"),
        Some(history.old.active_chain[1].block_hash)
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("cursor"),
        Some(checkpoint(&history.records[1]))
    );
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("fence")
            .expect("state")
            .fence_hash,
        history.old.active_chain[1].block_hash
    );
    assert_eq!(
        store
            .maybe_active_basic_filter_record(2)
            .expect("visible projection"),
        None
    );
    assert_eq!(
        store.load_prune_locks().expect("protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert_eq!(store.maybe_prune_intent().expect("intent"), None);
    history.assert_rows_and_payloads(store);
    drop(runtime);
    assert_eq!(snapshot_index(&history.path), before);
    history.cleanup();
}

#[test]
fn filter_index_production_validated_missing_prefix_preserves_history_and_intent() {
    for missing_projection in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("filter-validated-missing-prefix", false);
        let store = history.seed(1);
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("current validated authority");
        let cp = checkpoint(&history.records[2]);
        store
            .publish_basic_filter_checkpoint(
                &fence(&history.full.active_chain),
                cp,
                cp.input_protection(),
                &history.records[2..],
            )
            .expect("saved complete prefix");
        let intent = PruneIntent {
            height: 2,
            block_hash: history.full.active_chain[2].block_hash,
        };
        store.sync_prune_intent(intent).expect("live input intent");
        drop(store);
        raw_index(&history.path, |index| {
            let key = if missing_projection {
                codec::active_key(1)
            } else {
                codec::record_key(history.records[1].identity().block_hash())
            };
            index.remove(key).expect("missing prefix seam");
        });
        let before = snapshot_index(&history.path);
        // Act
        let result = history.reopen();
        // Assert
        assert_filter_refusal(
            result,
            if missing_projection {
                "missing BASIC project"
            } else {
                "missing BASIC predecessor"
            },
        );
        assert_eq!(snapshot_index(&history.path), before);
        let store = FjallNodeStore::open(&history.path).expect("inspect refusal");
        assert_eq!(
            store.load_block(intent.block_hash).expect("exact body"),
            Some(history.blocks[2].clone())
        );
        assert_eq!(
            store
                .load_undo(intent.block_hash)
                .expect("exact non-genesis undo"),
            Some(history.full.undo_by_block[&intent.block_hash].clone())
        );
        assert_eq!(
            store.maybe_prune_intent().expect("retained live intent"),
            Some(intent)
        );
        assert_eq!(
            store.load_prune_locks().expect("unchanged protection"),
            vec![
                IndexInputProtection::FromHeight(3)
                    .maybe_prune_lock()
                    .expect("lock")
            ]
        );
        drop(store);
        history.cleanup();
    }
}

#[test]
fn filter_index_production_validated_equal_height_fork_retains_common_prefix() {
    // Arrange
    let original = ValidatedHistory::new("filter-validated-fork", false);
    let replacement = ValidatedHistory::new("filter-unused-replacement", true);
    assert_eq!(original.old, replacement.old);
    let store = original.seed(1);
    store
        .seed_coins_from_snapshot(&original.full)
        .expect("original durable authority");
    let cp = checkpoint(&original.records[2]);
    store
        .publish_basic_filter_checkpoint(
            &fence(&original.full.active_chain),
            cp,
            cp.input_protection(),
            &original.records[2..],
        )
        .expect("original branch checkpoint");
    replacement.persist_payloads(&store);
    store
        .persist_basic_filter_records(&replacement.records[2..])
        .expect("alternative immutable row");
    store
        .seed_coins_from_snapshot(&replacement.full)
        .expect("validated replacement coins/meta");
    drop(store);
    // Act
    let runtime = original.reopen().expect("common indexed prefix");
    // Assert
    let store = runtime.store();
    assert_eq!(
        store.coins_view().best_block().expect("B"),
        Some(replacement.full.active_chain[2].block_hash)
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("cursor"),
        Some(checkpoint(&original.records[1]))
    );
    assert_eq!(
        store
            .maybe_active_basic_filter_record(2)
            .expect("hidden displaced projection"),
        None
    );
    assert_eq!(
        store.basic_filter_projection(2).expect("retained suffix"),
        original.full.active_chain[2].block_hash
    );
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("fence")
            .expect("state")
            .fence_hash,
        replacement.full.active_chain[2].block_hash
    );
    assert_eq!(
        store.load_prune_locks().expect("protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    original.assert_rows_and_payloads(store);
    replacement.assert_rows_and_payloads(store);
    assert_eq!(store.maybe_prune_intent().expect("intent"), None);
    drop(runtime);
    original.cleanup();
}
