// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/index/base.cpp

use open_bitcoin_core::{
    chainstate::{
        ChainPosition, CoinsView, FlushMode, FlushPolicyTime, MIN_BLOCKS_TO_KEEP, ManualPruneInput,
        ManualPruneRefusal, PruneMode, plan_manual_prune,
    },
    consensus::{CanonicalChain, block_hash},
};

use super::{DurableSyncRuntime, SyncNetwork, phase167_support as canonical};
use crate::chainstate::validation_history::tests::phase167 as mining;

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

fn assert_pairs(runtime: &DurableSyncRuntime, chain: &[ChainPosition], deleted: u32) {
    for position in &chain[1..] {
        let present = position.height > deleted;
        assert_eq!(
            runtime
                .store()
                .has_block(position.block_hash)
                .expect("body"),
            present,
            "body at {}",
            position.height
        );
        assert_eq!(
            runtime.store().has_undo(position.block_hash).expect("undo"),
            present,
            "undo at {}",
            position.height
        );
    }
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
    assert_pairs(&runtime, &chain, 0);
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
    assert_pairs(&runtime, &chain, 0);
    assert!(!store.load_have_pruned().expect("no deletion"));

    let caught_up = (0..=TIP).any(|_| {
        runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("BASIC catch-up")
            .maybe_accepted_lag
            == Some(0)
    });
    assert!(caught_up);
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
    assert_pairs(&runtime, &chain, LAST_PRUNABLE);
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
    assert_pairs(&runtime, &chain, LAST_PRUNABLE);
    eprintln!(
        "CINT-01 tip={TIP} protected_pairs={} deleted_pairs={} retained_pairs={} logical_bytes_before={} logical_bytes_after={}",
        prefix.len(),
        deletion.deleted_block_hashes.len(),
        TIP - LAST_PRUNABLE,
        before.current_usage_bytes,
        after.current_usage_bytes
    );
}
