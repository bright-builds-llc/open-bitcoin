// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::{Chainstate, PruneMode};

/// Derive an earlier real coins checkpoint using every genuine disconnect undo.
fn seed_at_checkpoint(fixture: &TurnHistory, prefix: usize, height: usize) -> FjallNodeStore {
    let store = fixture.seed(prefix);
    let mut state = Chainstate::from_snapshot(fixture.history.full.clone());
    for block in fixture.history.blocks[height + 1..].iter().rev() {
        state.disconnect_tip(block).expect("genuine validated undo");
    }
    store
        .seed_coins_from_snapshot(&state.snapshot())
        .expect("earlier validated coins/metadata");
    store
}

#[test]
fn phase157_store_retention_accepted_unflushed_reopen_loses_only_uncheckpointed_tip() {
    // Arrange
    let history = ValidatedHistory::new("evidence-accepted-unflushed", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("old recovered owner");
    // Act: default connect must not force the 442 MiB coins policy to write.
    runtime
        .network
        .connect_local_block(&history.blocks[2], ScriptVerifyFlags::P2SH, params())
        .expect("ordinary accepted connect");
    let ahead = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("complete accepted facts process ahead");
    // Assert
    assert_eq!(ahead.work.blocks, 1);
    assert_eq!(ahead.body_reads, 0);
    let progress = ahead.maybe_progress.expect("owner");
    assert_eq!(
        progress.maybe_processed_endpoint(),
        Some(history.records[2].identity())
    );
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        Some(history.records[1].identity())
    );
    assert_eq!(
        runtime
            .store()
            .coins_view()
            .best_block()
            .expect("old durable coins"),
        Some(history.old.active_chain[1].block_hash)
    );
    drop(runtime);
    let runtime = configured(&history.path).expect("old authority is recoverable");
    assert_eq!(
        runtime
            .network
            .maybe_chain_tip()
            .expect("tip")
            .expect("safe")
            .height,
        1
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe"),
        Some(checkpoint(&history.records[1]))
    );
    assert_eq!(
        runtime
            .store()
            .load_basic_filter_record(history.records[2].identity().block_hash())
            .expect("ahead immutable record survives"),
        Some(history.records[2].clone())
    );
    assert_eq!(
        runtime
            .store()
            .maybe_active_basic_filter_record(2)
            .expect("not active"),
        None
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_store_retention_normal_periodic_and_always_checkpoint_then_safe_manual_release() {
    for mode in [FlushMode::Periodic, FlushMode::Always] {
        // Arrange: tip 19 is genuinely durable; 20..400 are ordinary later accepts.
        let fixture = TurnHistory::new(16, 385, 1);
        let runtime = DurableSyncRuntime::open_configured(
            seed_at_checkpoint(&fixture, 16, 19),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("old recovered owner");
        for block in &fixture.history.blocks[20..] {
            runtime
                .network
                .connect_local_block(block, ScriptVerifyFlags::P2SH, params())
                .expect("ordinary contiguous accepted block");
        }
        finish(&runtime);
        let progress = runtime
            .network
            .maybe_basic_index_progress()
            .expect("read")
            .expect("owner");
        assert_eq!(
            progress.maybe_processed_endpoint(),
            Some(fixture.history.records[400].identity())
        );
        assert_eq!(
            progress.maybe_safe_durable_endpoint(),
            Some(fixture.history.records[19].identity())
        );
        assert_eq!(
            runtime
                .store()
                .coins_view()
                .best_block()
                .expect("unflushed B"),
            Some(fixture.history.full.active_chain[19].block_hash)
        );
        assert!(
            runtime
                .store()
                .commit_paired_delete(20, fixture.history.full.active_chain[20].block_hash)
                .is_err(),
            "required suffix cannot be released by ahead records"
        );
        assert_pair(runtime.store(), &fixture.history, 20, true);
        // Act: inject a due timestamp, preserving production policy/cadence.
        let flushed = runtime
            .network
            .flush_coins(mode, FlushPolicyTime::new(u64::MAX), u64::MAX)
            .expect("existing normal checkpoint owner");
        assert!(flushed.wrote_coins);
        let released = runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("same owner zero-record release");
        let deletion = manual(&runtime, &[20]);
        // Assert
        assert_eq!(released.work.blocks, 0);
        assert_eq!(released.persistence_batches, 1);
        assert_eq!(
            released
                .maybe_progress
                .expect("progress")
                .maybe_safe_durable_endpoint(),
            Some(fixture.history.records[400].identity())
        );
        assert_eq!(
            deletion.deleted_block_hashes,
            vec![fixture.history.full.active_chain[20].block_hash]
        );
        assert_pair(runtime.store(), &fixture.history, 20, false);
        assert!(
            runtime
                .store()
                .load_have_pruned()
                .expect("earned paired deletion")
        );
        drop(runtime);
        let runtime =
            configured(&fixture.history.path).expect("genuine checkpoint and paired-delete reopen");
        assert_pair(runtime.store(), &fixture.history, 20, false);
        assert_eq!(
            runtime
                .store()
                .load_basic_filter_record(fixture.history.records[20].identity().block_hash())
                .expect("retained filter"),
            Some(fixture.history.records[20].clone())
        );
        assert_eq!(
            runtime
                .store()
                .coins_view()
                .best_block()
                .expect("new durable B"),
            Some(fixture.history.full.active_chain[400].block_hash)
        );
        drop(runtime);
        fixture.history.cleanup();
    }
}

/// Nonactive codec/storage volume, explicitly not accepted-branch evidence.
/// The active deletion candidates still come from the continuous validated chain.
fn add_nonactive_volume(store: &FjallNodeStore) -> Vec<BlockHash> {
    let large = TurnHistory::legal_large_singleton();
    let mut body = large
        .history
        .blocks
        .last()
        .expect("legal singleton shape")
        .clone();
    let mut hashes = Vec::new();
    for nonce in 100_000..100_584 {
        body.header.nonce = nonce;
        store
            .save_block(&body, PersistMode::Buffered)
            .expect("real nonactive body writer");
        hashes.push(block_hash(&body.header));
    }
    large.history.cleanup();
    hashes
}

/// Stream all actual body values without retaining a duplicate 550 MiB cache.
/// Exact small-history refusal snapshots use raw values; this volume control
/// compares complete sorted keys, lengths and first-party SHA256d commitments.
fn body_commitments(path: &Path) -> Vec<(Vec<u8>, usize, BlockHash)> {
    let mut rows = Vec::new();
    raw_index(path, |space| {
        for guard in space.prefix(b"block:") {
            let (key, value) = guard.into_inner().expect("actual volume payload");
            rows.push((
                key.to_vec(),
                value.len(),
                BlockHash::from_byte_array(open_bitcoin_core::consensus::crypto::double_sha256(
                    &value,
                )),
            ));
        }
    });
    rows
}

#[test]
fn phase157_store_retention_ordinary_automatic_owner_earns_actual_pairs_after_checkpoint() {
    // Arrange: actual policy threshold and actual logical bytes; no accounting override.
    let fixture = TurnHistory::new(0, 1_002, 1);
    let store = fixture.seed(0);
    let nonactive = add_nonactive_volume(&store);
    drop(store);
    let mut runtime = configured(&fixture.history.path).expect("genuine protected startup");
    runtime
        .network
        .set_prune_mode(PruneMode::Automatic { target_mib: 550 })
        .expect("legal shipped target");
    runtime
        .network
        .set_prune_network(SyncNetwork::Regtest)
        .expect("actual chain threshold");
    let usage = runtime
        .store()
        .retained_payload_usage(&fixture.history.full.active_chain)
        .expect("real all-key measurement");
    assert!(
        usage.current_usage_bytes > 550 * 1024 * 1024,
        "volume must earn target pressure"
    );
    // Act: the normal Periodic owner is blocked by the fresh safe Empty protection.
    let protected = runtime
        .network
        .flush_coins(FlushMode::Periodic, FlushPolicyTime::new(0), u64::MAX)
        .expect("ordinary protected retention");
    // Assert
    assert!(protected.deleted_block_hashes.is_empty());
    assert_pair(runtime.store(), &fixture.history, 20, true);
    assert!(
        !runtime
            .store()
            .load_have_pruned()
            .expect("no invented prune")
    );
    finish(&runtime);
    let deletion = runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(1), u64::MAX)
        .expect("ordinary automatic owner after exact-tip checkpoint");
    assert!(deletion.wrote_coins);
    assert!(!deletion.deleted_block_hashes.is_empty());
    assert!(
        deletion
            .deleted_block_hashes
            .contains(&fixture.history.full.active_chain[20].block_hash)
    );
    assert_pair(runtime.store(), &fixture.history, 20, false);
    assert_pair(runtime.store(), &fixture.history, 1_001, true);
    assert!(runtime.store().load_have_pruned().expect("achieved pairs"));
    assert_eq!(
        runtime
            .store()
            .load_operator_support_counts()
            .expect("earned counts")
            .pruned_height_count,
        deletion.deleted_block_hashes.len() as u64
    );
    for hash in &nonactive {
        assert!(
            runtime
                .store()
                .has_block(*hash)
                .expect("nonactive bytes are not candidates")
        );
    }
    let remaining = runtime
        .store()
        .retained_payload_usage(&fixture.history.full.active_chain)
        .expect("actual remaining bytes");
    assert!(remaining.current_usage_bytes < usage.current_usage_bytes);
    assert_eq!(deletion.deleted_block_hashes.len(), 714);
    eprintln!(
        "phase157 retention evidence: actual logical bytes before={} after={}; active paired deletes={}; nonactive codec bodies={}",
        usage.current_usage_bytes,
        remaining.current_usage_bytes,
        deletion.deleted_block_hashes.len(),
        nonactive.len()
    );
    assert_eq!(
        runtime.store().maybe_prune_intent().expect("complete"),
        None
    );
    drop(runtime);
    let before = body_commitments(&fixture.history.path);
    let runtime = configured(&fixture.history.path)
        .expect("actual volume, paired loss and retained index reopen");
    assert_pair(runtime.store(), &fixture.history, 20, false);
    assert_eq!(
        runtime
            .store()
            .load_basic_filter_record(fixture.history.records[20].identity().block_hash())
            .expect("retained record"),
        Some(fixture.history.records[20].clone())
    );
    assert_eq!(
        runtime
            .store()
            .retained_payload_usage(&fixture.history.full.active_chain)
            .expect("reopened actual accounting")
            .current_usage_bytes,
        remaining.current_usage_bytes
    );
    drop(runtime);
    assert_eq!(body_commitments(&fixture.history.path), before);
    fixture.history.cleanup();
}
