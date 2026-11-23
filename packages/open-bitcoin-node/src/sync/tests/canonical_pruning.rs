// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use open_bitcoin_core::{
    chainstate::{
        AnchoredBlock, ChainPosition, CoinsView, FlushMode, FlushPolicyTime, MIN_BLOCKS_TO_KEEP,
        ManualPruneInput, ManualPruneRefusal, PruneMode, plan_manual_prune,
    },
    consensus::{CanonicalChain, block_hash},
    primitives::{Block, ScriptBuf},
};
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};

use super::{DurableSyncRuntime, SyncNetwork, phase167_support as canonical};
use crate::{
    FjallNodeStore, PersistMode,
    chainstate::{BasicFilterStartupMode, validation_history::tests::phase167 as mining},
};

const PRUNE_AFTER: u32 = 1_000;
const TIP: u32 = 1_001;
const LAST_PRUNABLE: u32 = 713;
const CORPUS_TIP: u32 = 301;

fn extend_canonical_chain(runtime: &DurableSyncRuntime) -> Vec<ChainPosition> {
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut parent = genesis.header;
    let mut expected = vec![block_hash(&parent)];
    let corpus = canonical::corpus();
    for height in 1..=TIP {
        let block = if height <= CORPUS_TIP {
            canonical::block(canonical::row(&corpus, height as usize))
        } else {
            mining::child(&parent, height)
        };
        assert_eq!(block.header.previous_block_hash, block_hash(&parent));
        canonical::admit_and_connect(runtime, &block);
        canonical::assert_scripts_valid(runtime, &block);
        expected.push(block_hash(&block.header));
        parent = block.header;
    }
    let chain = runtime
        .network
        .chainstate_snapshot()
        .expect("accepted canonical state")
        .active_chain;
    assert_eq!(chain.len(), expected.len());
    for (height, (position, hash)) in chain.iter().zip(expected).enumerate() {
        assert_eq!(position.height as usize, height);
        assert_eq!(position.block_hash, hash);
    }
    chain
}

fn assert_pairs(store: &FjallNodeStore, chain: &[ChainPosition], deleted: u32) {
    for position in &chain[1..] {
        let present = position.height > deleted;
        assert_eq!(
            store.has_block(position.block_hash).expect("body"),
            present,
            "body at {}",
            position.height
        );
        assert_eq!(
            store.has_undo(position.block_hash).expect("undo"),
            present,
            "undo at {}",
            position.height
        );
    }
}

fn catch_up_basic(runtime: &DurableSyncRuntime, tip: u32) {
    assert!((0..=tip).any(|_| {
        runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("BASIC catch-up")
            .maybe_accepted_lag
            == Some(0)
    }));
}

#[test]
fn canonical_regtest_pruning_deletes_pairs_after_basic_catch_up() {
    // Arrange: a canonical owner with admitted bodies, earned work and durable undo.
    let source = canonical::Source::new();
    let mut runtime = canonical::open_genesis(&source);
    runtime
        .network
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("ordinary manual mode");
    let chain = extend_canonical_chain(&runtime);
    let store = runtime.store();
    runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(79_999), u64::MAX)
        .expect("checkpoint accepted block undo and coins");
    assert_pairs(store, &chain, 0);
    assert_eq!(SyncNetwork::Regtest.prune_after_height(), PRUNE_AFTER);
    assert_eq!(MIN_BLOCKS_TO_KEEP, 288);
    assert_eq!(TIP - MIN_BLOCKS_TO_KEEP, LAST_PRUNABLE);
    let input = ManualPruneInput {
        tip: chain.last().expect("active tip").height,
        prune_after_height: SyncNetwork::Regtest.prune_after_height(),
        mode: runtime.network.prune_mode().expect("effective prune mode"),
        target_height: LAST_PRUNABLE,
        maybe_present_heights: Some((1..=TIP).collect()),
        locks: vec![],
    };
    assert!(matches!(
        plan_manual_prune(&ManualPruneInput {
            tip: PRUNE_AFTER - 1,
            ..input.clone()
        }),
        Err(ManualPruneRefusal::ChainTooShort { .. })
    ));
    assert!(matches!(
        plan_manual_prune(&ManualPruneInput {
            target_height: LAST_PRUNABLE + 1,
            ..input.clone()
        }),
        Err(ManualPruneRefusal::TargetInsideKeepWindow { .. })
    ));
    let plan = plan_manual_prune(&input).expect("maximum legal target");
    let flush = |now| {
        runtime.network.flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(now),
            u64::MAX,
            &plan,
            &[],
        )
    };

    // Act and assert: the owner must apply fresh protection to a stale plan.
    let blocked = flush(80_000).expect("protect unindexed inputs");
    assert!(blocked.deleted_block_hashes.is_empty());
    assert!(
        store
            .load_prune_protection()
            .expect("owned protection")
            .protects_height(1)
    );
    assert_pairs(store, &chain, 0);
    assert!(!store.load_have_pruned().expect("no deletion"));

    catch_up_basic(&runtime, TIP);
    let protection = store.load_prune_protection().expect("caught-up protection");
    assert!(!protection.protects_height(LAST_PRUNABLE));
    let basic = store
        .maybe_basic_filter_state()
        .expect("saved BASIC checkpoint")
        .expect("enabled BASIC index");
    let before = store
        .retained_payload_usage(&chain)
        .expect("measured before deletion");
    let deletion = flush(80_001).expect("real paired pruning");
    let prefix = &chain[1..=LAST_PRUNABLE as usize];
    assert_eq!(
        deletion.deleted_block_hashes,
        prefix.iter().map(|row| row.block_hash).collect::<Vec<_>>()
    );
    assert!(deletion.wrote_coins);
    assert_pairs(store, &chain, LAST_PRUNABLE);
    assert!(store.load_have_pruned().expect("earned deletion marker"));
    assert!(
        store
            .maybe_prune_intent()
            .expect("committed deletion")
            .is_none()
    );
    let after = store
        .retained_payload_usage(&chain)
        .expect("measured after deletion");
    let removed: u64 = prefix
        .iter()
        .map(|row| before.height_sizes[&row.height])
        .sum();
    assert_eq!(
        before.current_usage_bytes - removed,
        after.current_usage_bytes
    );
    assert_eq!(
        runtime
            .network
            .maybe_chain_tip()
            .expect("unchanged active tip")
            .as_ref(),
        chain.last()
    );
    assert_eq!(
        store.coins_view().best_block().expect("durable coins tip"),
        chain.last().map(|position| position.block_hash)
    );
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("saved BASIC checkpoint"),
        Some(basic)
    );
    runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(80_002), u64::MAX)
        .expect("flush after cache eviction");
    assert_pairs(store, &chain, LAST_PRUNABLE);
    eprintln!(
        "CINT-01 tip={TIP} protected_pairs={} deleted_pairs={} retained_pairs={} logical_bytes_before={} logical_bytes_after={}",
        prefix.len(),
        deletion.deleted_block_hashes.len(),
        TIP - LAST_PRUNABLE,
        before.current_usage_bytes,
        after.current_usage_bytes
    );
}

const FORK_PARENT: u32 = 998;
const FORK_CHECKPOINT: u32 = 1_000;
const REPLACEMENT_TIP: u32 = TIP + 1;

fn activate_longer_branch(runtime: &DurableSyncRuntime, chain: &[ChainPosition]) -> Vec<Block> {
    let mut parent = chain[FORK_PARENT as usize].header.clone();
    let mut replacement = Vec::new();
    for height in FORK_PARENT + 1..=REPLACEMENT_TIP {
        let mut child = mining::child(&parent, height);
        child.transactions[0].outputs[0].script_pubkey =
            ScriptBuf::from_bytes(vec![0x52]).expect("distinct replacement coinbase script");
        mining::remine(&mut child);
        parent = child.header.clone();
        replacement.push(child);
    }
    runtime
        .network
        .receive_message(
            1,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: replacement
                    .iter()
                    .map(|block| block.header.clone())
                    .collect(),
            }),
            i64::MIN,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("admit actual replacement headers");
    let headers = runtime
        .network
        .header_entries()
        .expect("earned replacement work");
    let anchored = replacement
        .iter()
        .map(|block| {
            runtime
                .store()
                .save_block(block, PersistMode::Sync)
                .expect("candidate body");
            let hash = block_hash(&block.header);
            let entry = headers
                .iter()
                .find(|entry| entry.block_hash == hash)
                .expect("admitted replacement header");
            AnchoredBlock {
                block: block.clone(),
                chain_work: entry.chain_work,
            }
        })
        .collect::<Vec<_>>();
    let displaced = chain[(FORK_PARENT + 1) as usize..]
        .iter()
        .rev()
        .map(|position| {
            runtime
                .store()
                .load_block(position.block_hash)
                .expect("displaced lookup")
                .expect("displaced body")
        })
        .collect::<Vec<_>>();
    let transition = runtime
        .network
        .reorg_to_branch(
            &displaced,
            &anchored,
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(i64::MIN)),
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("activate longer canonical branch");
    assert_eq!(transition.disconnected.len(), displaced.len());
    assert_eq!(transition.connected.len(), replacement.len());
    for block in &replacement {
        canonical::assert_scripts_valid(runtime, block);
    }
    replacement
}

#[test]
fn canonical_pruned_reorg_cold_open_preserves_unproved_source() {
    // Arrange: earn, index and replace the original tip through managed authority.
    let source = canonical::Source::new();
    let mut runtime = canonical::open_genesis(&source);
    runtime
        .network
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("manual pruning mode");
    let chain = extend_canonical_chain(&runtime);
    runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(81_000), u64::MAX)
        .expect("original chain checkpoint");
    catch_up_basic(&runtime, TIP);
    let stale_hash = chain[FORK_CHECKPOINT as usize].block_hash;
    let store = runtime.store();
    let stale_filter = store
        .load_basic_filter_record(stale_hash)
        .expect("original filter lookup")
        .expect("original height-1000 filter");
    let replacement = activate_longer_branch(&runtime, &chain);
    runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(81_001), u64::MAX)
        .expect("replacement checkpoint");
    catch_up_basic(&runtime, REPLACEMENT_TIP);
    let active = runtime
        .network
        .chainstate_snapshot()
        .expect("replacement active chain")
        .active_chain;
    let latest = active.last().expect("replacement tip");
    assert_eq!(latest.height, REPLACEMENT_TIP);
    assert_eq!(
        latest.block_hash,
        block_hash(&replacement.last().expect("replacement block").header)
    );
    let active_hash = active[FORK_CHECKPOINT as usize].block_hash;
    assert_ne!(active_hash, stale_hash);
    assert_eq!(
        store
            .basic_filter_projection(FORK_CHECKPOINT)
            .expect("active checkpoint projection"),
        active_hash
    );
    let active_filter = store
        .load_basic_filter_record(active_hash)
        .expect("replacement filter lookup")
        .expect("replacement height-1000 filter");
    assert_ne!(
        active_filter.identity().filter_header(),
        stale_filter.identity().filter_header()
    );
    assert_eq!(
        store
            .load_basic_filter_record(stale_hash)
            .expect("displaced filter lookup"),
        Some(stale_filter.clone())
    );

    let archive = store
        .capture_regtest_replay_archive_for_test()
        .expect("complete detached archive before deletion");
    let archive_source = canonical::Source::new();
    archive.write_for_test(archive_source.path());
    let last_prunable = REPLACEMENT_TIP - MIN_BLOCKS_TO_KEEP;
    let plan = plan_manual_prune(&ManualPruneInput {
        tip: latest.height,
        prune_after_height: SyncNetwork::Regtest.prune_after_height(),
        mode: runtime.network.prune_mode().expect("effective prune mode"),
        target_height: last_prunable,
        maybe_present_heights: Some((1..=last_prunable).collect()),
        locks: runtime.network.list_prune_locks().expect("durable locks"),
    })
    .expect("post-reorg prune plan");
    let prune = runtime
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(81_002),
            u64::MAX,
            &plan,
            &[],
        )
        .expect("paired deletion after reorg");
    assert_eq!(
        prune.deleted_block_hashes,
        active[1..=last_prunable as usize]
            .iter()
            .map(|position| position.block_hash)
            .collect::<Vec<_>>()
    );
    assert_pairs(store, &active, last_prunable);
    drop(runtime);
    let before = canonical::physical_manifest(source.path());
    assert!(!before.is_empty());

    // Act: existing canonical reopen only issues authority after full replay.
    let refusal = DurableSyncRuntime::reopen_canonical_history_configured(
        source.path(),
        canonical::selected_config(),
        BasicFilterStartupMode::PreserveSaved,
    )
    .err()
    .expect("closed pruned source has no complete replay proof");

    // Assert: the failed reopen preserves every original file.
    assert_eq!(canonical::physical_manifest(source.path()), before);
    assert!(
        DurableSyncRuntime::reopen_pruned_regtest_with_archive(
            source.path(),
            source.path(),
            canonical::selected_config(),
            Default::default(),
            false,
        )
        .is_err()
    );
    assert_eq!(canonical::physical_manifest(source.path()), before);
    let archived_before = canonical::physical_manifest(archive_source.path());
    let reopened = DurableSyncRuntime::reopen_pruned_regtest_with_archive(
        source.path(),
        archive_source.path(),
        canonical::selected_config(),
        Default::default(),
        false,
    )
    .expect("independently replayed cold pruned runtime");
    assert_eq!(
        canonical::physical_manifest(archive_source.path()),
        archived_before
    );
    drop(archive_source);
    let cold_store = reopened.store();
    assert_pairs(cold_store, &active, last_prunable);
    let pruned_hash = active[last_prunable as usize].block_hash;
    assert!(
        !reopened
            .network
            .cached_block_present(pruned_hash)
            .expect("cold body cache")
    );
    assert!(
        !reopened
            .network
            .maybe_body_eligibility(pruned_hash)
            .expect("pruned header facts")
            .expect("replayed ancestry")
            .available
    );
    cold_store
        .check_pruned_regtest_mutations_for_test(&archive)
        .expect("detached tamper controls");
    let snapshot = reopened
        .network
        .chainstate_snapshot()
        .expect("rebound canonical state");
    assert_eq!(snapshot.tip(), Some(latest));
    assert_eq!(
        reopened
            .network
            .header_entries()
            .expect("replayed headers")
            .into_iter()
            .find(|entry| entry.block_hash == latest.block_hash)
            .expect("replayed header")
            .chain_work,
        latest.chain_work
    );
    assert_eq!(snapshot.undo_by_block.len(), MIN_BLOCKS_TO_KEEP as usize);
    assert!(
        prune
            .deleted_block_hashes
            .iter()
            .all(|hash| !snapshot.undo_by_block.contains_key(hash))
    );
    assert_eq!(
        cold_store
            .coins_view()
            .best_block()
            .expect("cold coins tip"),
        Some(latest.block_hash)
    );
    assert_eq!(
        cold_store
            .basic_filter_projection(FORK_CHECKPOINT)
            .expect("cold active projection"),
        active_hash
    );
    let pruned_filter = cold_store
        .load_basic_filter_record(pruned_hash)
        .expect("retained indexed record")
        .expect("pruned ancestor filter");
    for (hash, record) in [
        (active_hash, active_filter),
        (stale_hash, stale_filter),
        (pruned_hash, pruned_filter),
    ] {
        let crate::BasicFilterQuery::Found(query) = reopened
            .network
            .basic_filter_query(hash)
            .expect("managed query on reopened source")
        else {
            panic!("proved filter must be immediately readable for {hash:?}");
        };
        assert_eq!(query.identity(), record.identity());
        assert_eq!(query.encoded_bytes(), record.encoded_bytes());
        assert_eq!(
            cold_store
                .load_basic_filter_record(hash)
                .expect("cold BASIC record"),
            Some(record)
        );
    }
    eprintln!(
        "CINT-02 cold-open: tip={REPLACEMENT_TIP} deleted_pairs={} distinct_basic_checkpoint={FORK_CHECKPOINT} archive_replay=true source_preserved_on_refusal=true missing_archive_refusal={refusal}",
        prune.deleted_block_hashes.len()
    );
}
