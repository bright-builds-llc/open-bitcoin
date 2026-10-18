// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_node::core::{
    chainstate::{ChainstateError, ChainstateSnapshot, coins::CompleteCoinsSnapshot},
    consensus::chain_proof::block_proof,
};

fn assert_retained_inputs(store: &FjallNodeStore, saved: &ChainstateSnapshot, blocks: &[Block]) {
    for block in blocks {
        let hash = block_hash(&block.header);
        assert_eq!(
            store.load_block(hash).expect("retained body"),
            Some(block.clone())
        );
        assert_eq!(
            store
                .load_undo(hash)
                .expect("retained undo")
                .expect("required undo"),
            *saved.undo_by_block.get(&hash).expect("derived prefix undo")
        );
    }
}

fn assert_saved_prefix(
    store: &FjallNodeStore,
    expected: &ChainstateSnapshot,
    physical: &CompleteCoinsSnapshot,
    blocks: &[Block],
) {
    let saved = store
        .hydrate_chainstate_for_open()
        .expect("physical metadata")
        .expect("populated prefix");
    assert!(saved.maybe_confirmed_txid_counts.is_none());
    assert_eq!(&saved, expected);
    assert_eq!(
        &store
            .coins_view()
            .complete_snapshot()
            .expect("complete physical identity"),
        physical
    );
    assert_retained_inputs(store, expected, blocks);
}

fn assert_prefix_graph(runtime: &DurableSyncRuntime, prefix: &ChainstateSnapshot) {
    let handle = runtime.network_handle();
    let headers = handle.header_entries().expect("selected prefix headers");
    for position in &prefix.active_chain {
        let header = headers
            .iter()
            .find(|header| header.block_hash == position.block_hash)
            .expect("actual prefix header");
        assert_eq!(header.header, position.header);
        assert_eq!(header.height, position.height);
        assert_eq!(header.chain_work, position.chain_work);
        let facts = handle
            .maybe_body_eligibility(position.block_hash)
            .expect("body facts")
            .expect("checked prefix body");
        assert_eq!(facts.block_hash, position.block_hash);
        assert_eq!(facts.chain_work, position.chain_work);
        assert!(facts.available);
        assert!(!facts.failed);
        if position.height > 0 {
            assert_eq!(facts.maybe_sequence, None);
        }
    }
}

#[test]
fn phase165_live_fixture_requalified_prefix_extends_with_normal_maturity() {
    // Arrange: persist a genuine populated prefix, keeping only owned data across close.
    let selected = SyncNetwork::Regtest.consensus_params();
    assert_eq!(selected, SyncNetwork::Regtest.consensus_params());
    assert_eq!(selected.coinbase_maturity, 100);
    let flags = ScriptVerifyFlags::P2SH;
    let directory = FixtureDir::new();
    let root = block(BlockHash::default(), 0, 0);
    let nonroot = block(block_hash(&root.header), 1, 0);
    let child = block(block_hash(&nonroot.header), 2, 0);
    let child_hash = block_hash(&child.header);
    let prefix = [root, nonroot];
    let (store, source) = genuine(&directory, &prefix, selected);
    assert!(source.chainstate().has_earned_work());
    let saved_before_close = store
        .hydrate_chainstate_for_open()
        .expect("saved prefix")
        .expect("populated source");
    let physical_before_close = store
        .coins_view()
        .complete_snapshot()
        .expect("saved complete coins");
    assert_eq!(saved_before_close.active_chain.len(), prefix.len());
    assert_eq!(
        physical_before_close.maybe_best_block,
        Some(block_hash(&prefix[1].header))
    );
    assert!(physical_before_close.head_blocks.is_empty());
    assert_saved_prefix(&store, &saved_before_close, &physical_before_close, &prefix);
    drop((source, store));

    let reopened_store = FjallNodeStore::open(&directory.0)
        .expect("exclusive reopen after all original handles close");
    assert_saved_prefix(
        &reopened_store,
        &saved_before_close,
        &physical_before_close,
        &prefix,
    );
    let observed = reopened_store
        .hydrate_chainstate_for_open()
        .expect("reopened metadata")
        .expect("actual retained prefix");
    let physical = reopened_store
        .coins_view()
        .complete_snapshot()
        .expect("actual reopened coins B and heads");
    let raw_observation = Chainstate::from_parent(
        reopened_store.coins_view(),
        observed.active_chain.clone(),
        observed.undo_by_block.clone(),
        observed.maybe_confirmed_txid_counts.clone(),
    );
    assert!(!raw_observation.has_earned_work());
    assert_eq!(
        raw_observation.validate_live_origin(),
        Err(ChainstateError::UntrustedWorkLineage)
    );
    drop(raw_observation);

    let mut genuine_memory = Chainstate::new();
    let mut retained_blocks = Vec::new();
    for (index, position) in observed.active_chain.iter().enumerate() {
        let retained = reopened_store
            .load_block(position.block_hash)
            .expect("read actual prefix body")
            .expect("every retained active body is required");
        assert_eq!(retained, prefix[index]);
        let parent_work = genuine_memory
            .tip()
            .map_or(ProofWork::ZERO, |parent| parent.chain_work);
        let expected_work = parent_work
            .wrapping_add(block_proof(retained.header.bits).expect("actual target proof"));
        let accepted = genuine_memory
            .connect_block_with_current_time(
                &retained,
                ProofWork::ZERO,
                i64::from(retained.header.time),
                flags,
                selected,
            )
            .expect("full normal-parameter Memory validation");
        assert_eq!(
            accepted.height,
            u32::try_from(index).expect("bounded prefix height")
        );
        assert_eq!(accepted.block_hash, block_hash(&retained.header));
        assert_eq!(accepted.header, retained.header);
        assert_eq!(
            accepted.previous_block_hash(),
            retained.header.previous_block_hash
        );
        assert_eq!(accepted.chain_work, expected_work);
        assert_eq!(&accepted, position);
        retained_blocks.push(retained);
    }
    let genuine_snapshot = genuine_memory.snapshot();
    assert_eq!(genuine_snapshot.active_chain.len(), prefix.len());
    assert_eq!(genuine_snapshot.active_chain, observed.active_chain);
    assert_eq!(genuine_snapshot.undo_by_block, observed.undo_by_block);
    assert!(genuine_snapshot.maybe_confirmed_txid_counts.is_some());
    assert_eq!(
        genuine_memory
            .coins()
            .complete_snapshot()
            .expect("complete derived Memory coins"),
        physical
    );
    let expected_child_work = genuine_snapshot
        .active_chain
        .last()
        .expect("derived parent")
        .chain_work
        .wrapping_add(block_proof(child.header.bits).expect("first child target proof"));

    let mut fjall_target = Chainstate::from_parent(
        reopened_store.coins_view(),
        genuine_snapshot.active_chain.clone(),
        genuine_snapshot.undo_by_block.clone(),
        genuine_snapshot.maybe_confirmed_txid_counts.clone(),
    );
    assert!(!fjall_target.has_earned_work());
    genuine_memory
        .rebind_into(&mut fjall_target)
        .expect("consume complete genuine Memory owner");
    assert!(fjall_target.has_earned_work());
    fjall_target
        .validate_live_origin()
        .expect("requalified origin");
    fjall_target
        .bind_live_work_source(selected)
        .expect("full normal parameter binding");
    assert!(matches!(
        fjall_target.bind_live_work_source(ConsensusParams {
            coinbase_maturity: 1,
            ..selected
        }),
        Err(ChainstateError::WorkParametersMismatch)
    ));
    let requalified_source = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(reopened_store.clone()),
        fjall_target,
        lifecycle(),
    );
    let runtime = open(
        requalified_source,
        selected,
        BasicFilterStartupMode::Disabled,
    )
    .expect("public requalified constructor");
    assert_saved_prefix(
        &reopened_store,
        &saved_before_close,
        &physical,
        &retained_blocks,
    );
    assert_prefix_graph(&runtime, &genuine_snapshot);
    let handle = runtime.network_handle();
    let headers = handle.header_entries().expect("complete checked prefix");
    assert_eq!(headers.len(), genuine_snapshot.active_chain.len());
    assert!(!headers.iter().any(|header| header.block_hash == child_hash));
    assert_eq!(
        handle
            .maybe_body_eligibility(child_hash)
            .expect("no future body fact"),
        None
    );
    assert!(
        reopened_store
            .load_block(child_hash)
            .expect("no future stored body")
            .is_none()
    );

    // Act: the first future event is direct actual body connection, without header ingress.
    let accepted = handle
        .connect_local_block(&child, flags, selected)
        .expect("first direct normal-parameter extension");

    // Assert: genuine extension preserves historical UNKNOWN order and physical unknown counts.
    assert_eq!(
        accepted.height,
        u32::try_from(prefix.len()).expect("child height")
    );
    assert_eq!(accepted.block_hash, child_hash);
    assert_eq!(accepted.header, child.header);
    assert_eq!(accepted.chain_work, expected_child_work);
    let extended = handle.chainstate_snapshot().expect("actual extended owner");
    assert_eq!(
        extended.active_chain.len(),
        genuine_snapshot.active_chain.len() + 1
    );
    assert_eq!(
        &extended.active_chain[..prefix.len()],
        &genuine_snapshot.active_chain
    );
    assert_prefix_graph(&runtime, &genuine_snapshot);
    assert_retained_inputs(&reopened_store, &genuine_snapshot, &retained_blocks);
    let after_child_saved = reopened_store
        .hydrate_chainstate_for_open()
        .expect("saved counts after actual child")
        .expect("saved prefix remains");
    assert!(after_child_saved.maybe_confirmed_txid_counts.is_none());
    drop((handle, runtime, reopened_store));
}
