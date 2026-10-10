// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::super::fixtures::{daemon_block, params};
use super::*;
use open_bitcoin_node::core::{
    chainstate::{AnchoredBlock, FlushMode, FlushPolicyTime, PruneMode, PrunePlan},
    consensus::{ScriptVerifyFlags, check_block_header},
};

pub(super) fn replace_tip(fixture: &mut DaemonFixture, ancestor: usize, count: usize) {
    let disconnect: Vec<_> = fixture.blocks[ancestor + 1..]
        .iter()
        .rev()
        .cloned()
        .collect();
    let snapshot = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("actual accepted undo");
    for block in &disconnect {
        let hash = block_hash(&block.header);
        fixture
            .store
            .save_undo(
                hash,
                &snapshot.undo_by_block[&hash],
                open_bitcoin_node::PersistMode::Sync,
            )
            .expect("actual retained disconnect undo");
    }
    let mut connected = Vec::new();
    let mut parent = fixture.blocks[ancestor].clone();
    for offset in 1..=count {
        let height = (ancestor + offset) as u32;
        let mut block = daemon_block(Some(&parent), height);
        block.header.time += 1;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("replacement proof of work");
        fixture
            .store
            .save_block(&block, open_bitcoin_node::PersistMode::Sync)
            .expect("ordinary replacement body");
        connected.push(AnchoredBlock {
            block: block.clone(),
            chain_work: u128::from(height) + 1_000,
        });
        parent = block;
    }
    fixture
        .opened
        .network
        .reorg_to_branch(
            &disconnect,
            &connected,
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(50_000),
            ),
            ScriptVerifyFlags::P2SH,
            params(),
        )
        .expect("genuine managed validated replacement");
    fixture.blocks.truncate(ancestor + 1);
    fixture
        .blocks
        .extend(connected.into_iter().map(|anchored| anchored.block));
}

#[tokio::test]
async fn phase159_daemon_rpc_retention_actual_paired_prune_stale_and_all_handle_reopen() {
    // Arrange: 401 genuinely accepted continuous blocks, not sparse metadata.
    let mut fixture = DaemonFixture::new(401);
    fixture.finish();
    assert_eq!(fixture.blocks.len(), 401);
    assert_eq!(
        fixture
            .opened
            .network
            .prune_mode()
            .expect("configured manual owner policy"),
        PruneMode::ManualOnly
    );
    assert!(!open_bitcoin_node::core::chainstate::height_inside_keep_window(400, 20));
    let pruned = block_hash(&fixture.blocks[20].header);
    let stale = block_hash(&fixture.blocks[400].header);
    let original_pruned = filter(&fixture, pruned).await;
    let original_stale = filter(&fixture, stale).await;
    let snapshot = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("accepted continuous chain");
    let before = fixture
        .store
        .retained_payload_usage(&snapshot.active_chain)
        .expect("actual bytes before")
        .current_usage_bytes;
    // Act: use the production serialized paired-prune owner and real height policy.
    let deletion = fixture
        .opened
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(50_000),
            u64::MAX,
            &PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("legal existing paired prune");
    // Assert: payloads really disappeared while lookup commitments remain exact.
    assert_eq!(deletion.deleted_block_hashes, vec![pruned]);
    assert_eq!(
        fixture.store.load_block(pruned).expect("deleted body"),
        None
    );
    assert_eq!(fixture.store.load_undo(pruned).expect("deleted undo"), None);
    assert!(
        fixture
            .store
            .load_have_pruned()
            .expect("durable earned have_pruned")
    );
    let after = fixture
        .store
        .retained_payload_usage(&snapshot.active_chain)
        .expect("actual bytes after")
        .current_usage_bytes;
    assert!(after < before);
    assert_eq!(filter(&fixture, pruned).await, original_pruned);
    // A genuinely accepted suffix displaced before indexing has no immutable row.
    let accepted_missing = fixture.accept_next();
    replace_tip(&mut fixture, 390, 11);
    fixture.finish();
    let active = block_hash(&fixture.blocks[401].header);
    assert_ne!(active, stale);
    let original_active = filter(&fixture, active).await;
    assert_eq!(filter(&fixture, stale).await, original_stale);
    assert_error(
        &filter(&fixture, accepted_missing).await,
        -32603,
        "Filter not found. This error is unexpected and indicates index corruption.",
    );
    fixture
        .opened
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(50_001), u64::MAX)
        .expect("real replacement coins fence");
    fixture.finish();
    drop(snapshot);
    assert!(
        open_bitcoin_node::FjallNodeStore::open(&fixture.path).is_err(),
        "a second open must refuse while configured handles are live"
    );
    let (path, blocks) = fixture.close();
    // No context, HTTP state, worker, runtime, store, coins view or handle survives.
    let fixture = DaemonFixture::open(path, blocks, true);
    assert_eq!(filter(&fixture, pruned).await, original_pruned);
    assert_eq!(filter(&fixture, stale).await, original_stale);
    assert_eq!(filter(&fixture, active).await, original_active);
    assert_error(
        &filter(&fixture, accepted_missing).await,
        -32603,
        "Filter not found. This error is unexpected and indicates index corruption.",
    );
    assert_eq!(
        fixture
            .store
            .load_block(pruned)
            .expect("reopened absent body"),
        None
    );
    assert_eq!(
        fixture
            .store
            .load_undo(pruned)
            .expect("reopened absent undo"),
        None
    );
    assert!(
        fixture
            .store
            .load_have_pruned()
            .expect("reopened prune bit")
    );
    assert_eq!(
        invoke(&fixture, "getindexinfo", json!([])).await["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":401}})
    );
    eprintln!(
        "phase159 daemon retention evidence: initial_accepted=401 accepted_before_reorg=402 replacement=11 deleted_pairs={} target_height=20 logical_payload_bytes_before={before} after={after} loss={} all_handles_closed=true reopened=true exact_active_stale_pruned=true accepted_missing_retained=true",
        deletion.deleted_block_hashes.len(),
        before - after
    );
    fixture.cleanup();
}
