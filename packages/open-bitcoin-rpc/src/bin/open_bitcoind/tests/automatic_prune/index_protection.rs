// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Explicit accepted same-Fjall fixture history exercises real cache eviction and
//! physical retention. This is not ordinary canonical startup qualification.

use super::*;
use open_bitcoin_node::core::{
    chainstate::{
        Chainstate, ChainstateSnapshot, FilterCheckpoint, FlushMode, FlushPolicyTime,
        IndexInputProtection, IndexPrefix,
        filter_index::lifecycle::{EffectiveIndexOwnership, IndexGeneration, IndexLifecycle},
    },
    consensus::{
        ConsensusParams, ScriptVerifyFlags,
        block::{block_subsidy, check_block_header_with_params},
        block_hash, block_merkle_root,
        chain_proof::ProofWork,
    },
    primitives::{
        Block, BlockHash, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    },
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallChainstateStore, FjallCoinsView, ManagedChainstate,
    ManagedNetworkHandle, SyncNetwork, SyncRuntimeConfig,
    chainstate::{BasicFilterStartupMode, FlushLifecycle, PruneProtectionSnapshot},
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
    fn block(parent: BlockHash, height: u32, params: &ConsensusParams) -> Block {
        let mut number = height.to_le_bytes().to_vec();
        while number.last() == Some(&0) {
            number.pop();
        }
        if number.last().is_some_and(|byte| byte & 0x80 != 0) {
            number.push(0);
        }
        let mut script = vec![u8::try_from(number.len()).expect("bounded script number")];
        script.extend(number);
        script.push(0x51);
        let transactions = vec![Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::from_bytes(script).expect("minimal full height"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: block_subsidy(height, params),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true script"),
            }],
            lock_time: 0,
        }];
        let mut header = position(height).header;
        header.previous_block_hash = parent;
        header.merkle_root = block_merkle_root(&transactions).expect("actual merkle").0;
        header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                header.nonce = *nonce;
                check_block_header_with_params(&header, params).is_ok()
            })
            .expect("selected valid PoW");
        Block {
            header,
            transactions,
        }
    }

    fn seed(store: &FjallNodeStore) -> (Self, DurableSyncRuntime) {
        let params = SyncNetwork::Regtest.consensus_params();
        assert_eq!(params.coinbase_maturity, 100);
        let now = FlushPolicyTime::new(0);
        let mut core = Chainstate::from_parent(store.coins_view(), vec![], HashMap::new(), None);
        Chainstate::new()
            .rebind_into(&mut core)
            .expect("genuine empty transfer");
        let mut managed = ManagedChainstate::from_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            core,
            FlushLifecycle::ready(now, now, 0, false),
        );
        let mut blocks = Vec::new();
        for height in 0..=1_001 {
            let parent = blocks.last().map_or(BlockHash::default(), |block: &Block| {
                block_hash(&block.header)
            });
            let block = Self::block(parent, height, &params);
            let accepted = managed
                .connect_block(&block, ProofWork::ZERO, ScriptVerifyFlags::P2SH, params)
                .expect("actual dense acceptance");
            assert_eq!(accepted.height, height);
            assert_eq!(accepted.block_hash, block_hash(&block.header));
            blocks.push(block);
        }
        let (mut sink, mut core) = managed.into_parts();
        let truth = core
            .admission_snapshot()
            .expect("complete actually derived dense facts");
        assert_eq!(truth.active_chain.len(), 1_002);
        assert_eq!(truth.undo_by_block.len(), 1_002);
        let mut body_bytes = 0;
        let mut undo_bytes = 0;
        let pair_sizes = blocks
            .iter()
            .map(|block| {
                let body = u64::try_from(
                    open_bitcoin_codec::encode_block(block)
                        .expect("actual body encoding")
                        .len(),
                )
                .expect("body bytes");
                let undo = truth
                    .undo_by_block
                    .get(&block_hash(&block.header))
                    .expect("derived undo INCLUDING root");
                let encoded_undo = encoded_undo_size(undo);
                body_bytes += body;
                undo_bytes += encoded_undo;
                body + encoded_undo
            })
            .collect::<Vec<_>>();
        let positions = truth.active_chain.clone();
        let undo = truth
            .undo_by_block
            .iter()
            .map(|(hash, undo)| (*hash, undo.clone()))
            .collect::<Vec<_>>();
        let mut lifecycle = FlushLifecycle::ready(now, now, 0, false);
        assert!(
            lifecycle
                .execute_flush(
                    &mut sink,
                    &mut core,
                    FlushMode::Always,
                    FlushPolicyTime::new(1),
                    u64::MAX,
                    &undo,
                    &[],
                    &blocks,
                    &positions,
                )
                .expect("genuine root-inclusive physical flush")
                .wrote_coins
        );
        let measured = store
            .retained_payload_usage(&positions)
            .expect("actual encoded pair sizes");
        assert_eq!(measured.current_usage_bytes, body_bytes + undo_bytes);
        assert_eq!(
            measured.height_sizes.values().copied().collect::<Vec<_>>(),
            pair_sizes
        );
        let runtime = DurableSyncRuntime::from_live_fixture_source(
            ManagedChainstate::from_chainstate(sink, core, lifecycle),
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
            params,
            BasicFilterStartupMode::Disabled,
            Default::default(),
            Default::default(),
            false,
        )
        .expect("same genuine dense fixture without a startup index turn");
        (
            Self {
                truth,
                blocks,
                body_bytes,
                undo_bytes,
                pair_sizes,
            },
            runtime,
        )
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
                (height >= first_retained)
                    .then(|| self.truth.undo_by_block[&position.block_hash].clone()),
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
                ScriptVerifyFlags::P2SH,
                SyncNetwork::Regtest.consensus_params(),
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
fn index_protection_genuine_fixture_retains_then_deletes_only_after_disable() {
    // Arrange: one bounded dense fixture and one reusable nonactive bulk buffer.
    let temp = TempDir::new("index-protection-legal-target");
    let runtime_config = config(&temp);
    assert!(!runtime_config.sync.is_enabled());
    assert!(!runtime_config.inbound.enabled);
    let store = open_runtime_store(&runtime_config)
        .expect("store")
        .expect("durable");
    let (history, runtime) = DenseHistory::seed(&store);
    let initial_bytes = populate_threshold(&store, &history.truth);
    assert!(initial_bytes > TARGET);
    let opened = crate::assemble_durable_runtime(&runtime_config, runtime)
        .expect("configured explicit genuine fixture")
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
        "index protection legal target: target={TARGET} logical_before={initial_bytes} logical_after={} active_body_bytes={} undo_total_bytes={} active_pairs=1002 deleted_pairs=714 deleted_bytes={deleted_bytes}",
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
