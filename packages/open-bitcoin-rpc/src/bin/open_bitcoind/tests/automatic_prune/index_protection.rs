// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real ordinary daemon retention. Dense ancestry and historical correspondence
//! are codec-valid deletion evidence, not consensus-accepted sync history.

use super::*;
use open_bitcoin_node::core::{
    chainstate::{
        BasicFilterInputs, BlockUndo, ChainPosition, ChainstateSnapshot, FilterCheckpoint,
        HistoricalBlockUndo, IndexInputProtection, IndexPrefix,
        filter_index::lifecycle::{EffectiveIndexOwnership, IndexGeneration, IndexLifecycle},
    },
    consensus::{ConsensusParams, ScriptVerifyFlags, block_merkle_root},
    primitives::{
        Amount, Block, BlockHash, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_node::{
    FjallChainstateStore, FjallCoinsView, ManagedNetworkHandle, chainstate::PruneProtectionSnapshot,
};
use std::collections::HashMap;

type DurableHandle = ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>;

struct DenseHistory {
    truth: ChainstateSnapshot,
    blocks: Vec<Block>,
    body_bytes: u64,
    undo_bytes: u64,
    pair_sizes: Vec<u64>,
}

impl DenseHistory {
    fn seed(store: &FjallNodeStore) -> Self {
        let mut positions = Vec::new();
        let mut blocks = Vec::new();
        let mut parent = BlockHash::default();
        let undo = BlockUndo::default();
        let mut body_bytes = 0;
        for height in 0..=1_001 {
            let transactions = vec![Transaction {
                version: 1,
                inputs: vec![TransactionInput {
                    previous_output: OutPoint::null(),
                    script_sig: ScriptBuf::from_bytes(vec![1, height as u8])
                        .expect("bounded script"),
                    sequence: TransactionInput::SEQUENCE_FINAL,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: Amount::from_sats(5_000_000_000).expect("amount"),
                    script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
                }],
                lock_time: 0,
            }];
            let mut header = position(height).header;
            header.previous_block_hash = parent;
            header.merkle_root = block_merkle_root(&transactions).expect("merkle").0;
            let block = Block {
                header,
                transactions,
            };
            let position = ChainPosition::new(
                block.header.clone(),
                height,
                u128::from(height) + 1,
                i64::from(block.header.time),
            );
            let maybe_history = (height != 0).then_some(HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo: &undo,
            });
            BasicFilterInputs::from_historical(&block, &position, maybe_history)
                .expect("body/position/undo correspondence");
            body_bytes += u64::try_from(
                open_bitcoin_codec::encode_block(&block)
                    .expect("encoded body")
                    .len(),
            )
            .expect("body length");
            store
                .save_block(&block, PersistMode::Buffered)
                .expect("real body value");
            if height != 0 {
                store
                    .save_undo(position.block_hash, &undo, PersistMode::Buffered)
                    .expect("real non-genesis undo");
            }
            parent = position.block_hash;
            positions.push(position);
            blocks.push(block);
        }
        let truth = ChainstateSnapshot::new(positions, HashMap::new(), HashMap::new());
        store
            .seed_coins_from_snapshot(&truth)
            .expect("compatible durable B and metadata");
        let measured = store
            .retained_payload_usage(&truth.active_chain)
            .expect("actual encoded pair sizes");
        let one_body = u64::try_from(
            open_bitcoin_codec::encode_block(&blocks[1])
                .expect("body")
                .len(),
        )
        .expect("length");
        let undo_bytes = measured.height_sizes[&1]
            .checked_sub(one_body)
            .expect("actual encoded undo length");
        let pair_sizes: Vec<_> = blocks
            .iter()
            .enumerate()
            .map(|(height, block)| {
                let body =
                    u64::try_from(open_bitcoin_codec::encode_block(block).expect("body").len())
                        .expect("length");
                body + if height == 0 { 0 } else { undo_bytes }
            })
            .collect();
        assert_eq!(
            measured.current_usage_bytes,
            body_bytes + 1_001 * undo_bytes
        );
        assert_eq!(
            measured.height_sizes.values().copied().collect::<Vec<_>>(),
            pair_sizes
        );
        Self {
            truth,
            blocks,
            body_bytes,
            undo_bytes,
            pair_sizes,
        }
    }

    fn assert_payloads(&self, store: &FjallNodeStore, first_retained: usize) {
        for (height, position) in self.truth.active_chain.iter().enumerate() {
            assert_eq!(
                store.load_block(position.block_hash).expect("actual body"),
                (height >= first_retained).then(|| self.blocks[height].clone()),
                "body height {height}"
            );
            assert_eq!(
                store.load_undo(position.block_hash).expect("actual undo"),
                (height != 0 && height >= first_retained).then(BlockUndo::default),
                "undo height {height}"
            );
        }
    }

    fn cache(&self, handle: &DurableHandle, height: usize) {
        let position = &self.truth.active_chain[height];
        handle
            .connect_stored_block(
                &self.blocks[height],
                position.chain_work,
                position.median_time_past,
                ScriptVerifyFlags::NONE,
                ConsensusParams::default(),
            )
            .expect("already active block cache");
        assert!(
            handle
                .cached_block_present(position.block_hash)
                .expect("cache present")
        );
    }
}

fn protection(store: &FjallNodeStore) -> PruneProtectionSnapshot {
    store
        .load_prune_protection()
        .expect("current validated protection")
}

fn owned(store: &FjallNodeStore) -> EffectiveIndexOwnership {
    protection(store)
        .maybe_owner()
        .expect("saved index ownership")
}

fn assert_unearned(store: &FjallNodeStore) {
    assert!(!store.load_have_pruned().expect("unearned have-pruned"));
    let counts = store
        .load_operator_support_counts()
        .expect("unearned counts");
    assert_eq!(counts.successful_batch_count, 0);
    assert_eq!(counts.pruned_height_count, 0);
    assert_eq!(counts.maybe_last_prune_height, None);
}

#[test]
fn index_protection_ordinary_legal_target_retains_then_deletes_only_after_disable() {
    // Arrange: one bounded dense fixture and one reusable nonactive bulk buffer.
    let temp = TempDir::new("index-protection-legal-target");
    let runtime_config = config(&temp);
    assert!(!runtime_config.sync.is_enabled());
    assert!(!runtime_config.inbound.enabled);
    let store = open_runtime_store(&runtime_config)
        .expect("store")
        .expect("durable");
    let history = DenseHistory::seed(&store);
    let initial_bytes = populate_threshold(&store, &history.truth);
    assert!(initial_bytes > TARGET);
    let opened = open_authoritative_network_runtime(&runtime_config, Some(store.clone()))
        .expect("ordinary production runtime")
        .expect_durable();
    let handle = opened.network.clone();
    handle
        .set_coins_next_write(FlushPolicyTime::new(10_000))
        .expect("future deadline");
    handle
        .enable_basic_filter_index()
        .expect("trusted host establishes Active Empty protection");
    let initial_owner = owned(&store);
    assert_eq!(
        initial_owner.lifecycle(),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        }
    );
    assert_eq!(
        initial_owner.checkpoint().checkpoint(),
        FilterCheckpoint::new(IndexPrefix::Empty)
    );
    assert_eq!(
        initial_owner.maybe_effective_protection(),
        Some(IndexInputProtection::FromHeight(0))
    );
    for height in [0, 1, 713, 714] {
        history.cache(&handle, height);
    }

    // Act: ordinary offline Periodic retention while index work is stalled.
    let stalled = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("ordinary protected turn");

    // Assert: no candidate means no forced coins write or earned deletion.
    assert!(!stalled.wrote_coins);
    assert!(stalled.deleted_block_hashes.is_empty());
    history.assert_payloads(&store, 0);
    assert_unearned(&store);
    assert_eq!(
        store
            .retained_payload_usage(&history.truth.active_chain)
            .expect("protected actual total")
            .current_usage_bytes,
        initial_bytes
    );
    let always = flush_cycle(
        &handle,
        FlushMode::Always,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("protected shutdown-style Always");
    assert!(always.wrote_coins);
    assert!(always.deleted_block_hashes.is_empty());
    assert_unearned(&store);
    handle
        .set_coins_next_write(FlushPolicyTime::new(10_000))
        .expect("restore future deadline");

    // Arrange: same-range lifecycle changes with no payload change, same second.
    handle
        .disable_basic_filter_index()
        .expect("invalidate before release");
    handle
        .enable_basic_filter_index()
        .expect("reacquire before ordinary application");
    assert_eq!(
        owned(&store).lifecycle(),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(2)
        }
    );
    let reenabled = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("same-second protected turn");
    assert!(!reenabled.wrote_coins);
    assert!(reenabled.deleted_block_hashes.is_empty());
    history.assert_payloads(&store, 0);
    assert_unearned(&store);

    // Act: release now invalidates the same-second throttle; real pairs delete.
    handle
        .disable_basic_filter_index()
        .expect("explicit release");
    let disabled = owned(&store);
    assert_eq!(
        disabled.lifecycle(),
        IndexLifecycle::Disabled {
            generation: IndexGeneration::new(3)
        }
    );
    assert_eq!(disabled.checkpoint(), initial_owner.checkpoint());
    assert!(disabled.maybe_effective_protection().is_none());
    let deleted = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("same-second ordinary genuine deletion");

    let remaining_bytes =
        assert_deletion(&store, &handle, &history, initial_bytes, deleted, disabled);
    drop(handle);
    drop(opened);
    drop(store);

    assert_disabled_reopen(&runtime_config, &history, disabled, remaining_bytes);
    let deleted_bytes: u64 = history.pair_sizes[..=713].iter().sum();
    eprintln!(
        "index protection legal target: target={TARGET} logical_before={initial_bytes} logical_after={} active_body_bytes={} undo_value_bytes={} active_pairs=1002 deleted_pairs=714 deleted_bytes={deleted_bytes}",
        remaining_bytes, history.body_bytes, history.undo_bytes
    );
}

fn assert_deletion(
    store: &FjallNodeStore,
    handle: &DurableHandle,
    history: &DenseHistory,
    initial_bytes: u64,
    deleted: open_bitcoin_node::chainstate::FlushExecution,
    disabled: EffectiveIndexOwnership,
) -> u64 {
    // Assert: exact ordered receipts, paired removal, cache/undo cleanup and bytes.
    let expected: Vec<_> = history.truth.active_chain[..=713]
        .iter()
        .map(|position| position.block_hash)
        .collect();
    assert!(
        deleted.wrote_coins,
        "actual pruning forces coins despite future deadline"
    );
    assert_eq!(deleted.deleted_block_hashes, expected);
    history.assert_payloads(store, 714);
    let snapshot = handle
        .chainstate_snapshot()
        .expect("receipt-cleaned memory");
    for height in [0, 1, 713] {
        let hash = history.truth.active_chain[height].block_hash;
        assert!(
            !handle
                .cached_block_present(hash)
                .expect("earned cache cleanup")
        );
        assert!(!snapshot.undo_by_block.contains_key(&hash));
    }
    assert!(
        handle
            .cached_block_present(history.truth.active_chain[714].block_hash)
            .expect("retained cache")
    );
    assert!(
        snapshot
            .undo_by_block
            .contains_key(&history.truth.active_chain[714].block_hash)
    );
    let deleted_bytes: u64 = history.pair_sizes[..=713].iter().sum();
    let remaining = store
        .retained_payload_usage(&history.truth.active_chain)
        .expect("actual post-delete bytes");
    assert_eq!(remaining.current_usage_bytes, initial_bytes - deleted_bytes);
    assert!(
        remaining.current_usage_bytes > TARGET,
        "nonactive usage keeps legal target unattainable"
    );
    assert!(store.load_have_pruned().expect("earned marker"));
    let counts = store
        .load_operator_support_counts()
        .expect("earned support");
    assert_eq!(counts.successful_batch_count, 1);
    assert_eq!(counts.pruned_height_count, 714);
    assert_eq!(counts.maybe_last_prune_height, Some(713));
    let repeated = flush_cycle(
        handle,
        FlushMode::Always,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("repeated Always");
    assert!(repeated.deleted_block_hashes.is_empty());
    assert_eq!(
        store
            .load_operator_support_counts()
            .expect("no extra receipts"),
        counts
    );
    assert_eq!(owned(store), disabled);
    drop(snapshot);
    remaining.current_usage_bytes
}

fn assert_disabled_reopen(
    runtime_config: &RuntimeConfig,
    history: &DenseHistory,
    disabled: EffectiveIndexOwnership,
    remaining_bytes: u64,
) {
    // Act: close every clone and reopen the real ordinary daemon runtime.
    let store = open_runtime_store(runtime_config)
        .expect("actual reopen")
        .expect("store");
    let reopened = open_authoritative_network_runtime(runtime_config, Some(store.clone()))
        .expect("Disabled history-loss reopen")
        .expect_durable();
    let counts = store
        .load_operator_support_counts()
        .expect("durable earned support");
    assert_eq!(counts.successful_batch_count, 1);
    assert_eq!(counts.pruned_height_count, 714);
    assert_eq!(counts.maybe_last_prune_height, Some(713));
    history.assert_payloads(&store, 714);
    assert_eq!(owned(&store), disabled);
    let error = reopened
        .network
        .enable_basic_filter_index()
        .expect_err("missing required body forbids acquisition");
    assert!(error.to_string().contains("missing BASIC activation body"));
    assert_eq!(owned(&store), disabled);
    assert_eq!(
        store
            .load_operator_support_counts()
            .expect("unchanged support"),
        counts
    );
    assert_eq!(
        store
            .retained_payload_usage(&history.truth.active_chain)
            .expect("no history mutation")
            .current_usage_bytes,
        remaining_bytes
    );
    assert_eq!(
        store
            .wallet_scan_chainstate_snapshot()
            .expect("real coins/meta")
            .expect("truth")
            .active_chain,
        history.truth.active_chain
    );
    drop(reopened);
    drop(store);
    let store = open_runtime_store(runtime_config)
        .expect("refusal remains durable")
        .expect("store");
    let reopened = open_authoritative_network_runtime(runtime_config, Some(store.clone()))
        .expect("second Disabled reopen")
        .expect_durable();
    assert_eq!(owned(&store), disabled);
    history.assert_payloads(&store, 714);
    drop(reopened);
    drop(store);
}
