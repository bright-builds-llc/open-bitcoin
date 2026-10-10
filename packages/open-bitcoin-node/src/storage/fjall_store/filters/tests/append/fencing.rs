// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::{Chainstate, FlushMode, FlushPolicyTime};
use open_bitcoin_core::consensus::{
    ConsensusParams, ScriptVerifyFlags, block_hash, check_block_header,
};

#[test]
fn phase157_append_unbudgeted_flush_proof_cannot_start_turn() {
    // Arrange
    let path = temp_path("append-budget-required");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof()
        .expect("ordinary flush proof")
        .expect("recovered");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_artifacts().expect("no effects"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_lock_map_size_budget_refuses_before_malformed_name_decode() {
    // Arrange
    let path = temp_path("append-lock-size");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    recovered(&store, &positions);
    let mut raw = 1_u32.to_le_bytes().to_vec();
    raw.extend_from_slice(&5000_u16.to_le_bytes());
    raw.extend(std::iter::repeat_n(255, 5000));
    raw.extend_from_slice(&[0; 8]);
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            super::super::super::super::prune::PRUNE_LOCKS_KEY,
            raw,
        )
        .expect("oversized malformed map fixture");
    let mut maximum = budget();
    maximum.cloned_bytes = 4096;
    // Act
    let result = store.maybe_basic_filter_append_proof_with_budget(maximum);
    // Assert
    assert!(
        result
            .err()
            .expect("refused")
            .to_string()
            .contains("work budget")
    );
    assert_eq!(
        store.basic_filter_artifacts().expect("no effects"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_lock_map_count_budget_refuses_before_utf_decode() {
    // Arrange
    let path = temp_path("append-lock-count");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    recovered(&store, &positions);
    let mut raw = 8_u32.to_le_bytes().to_vec();
    for _ in 0..8 {
        raw.extend_from_slice(&1_u16.to_le_bytes());
        raw.push(255);
        raw.extend_from_slice(&[0; 8]);
    }
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            super::super::super::super::prune::PRUNE_LOCKS_KEY,
            raw,
        )
        .expect("wide invalid UTF map fixture");
    let mut maximum = budget();
    maximum.checkpoint_operations = 30;
    // Act
    let result = store.maybe_basic_filter_append_proof_with_budget(maximum);
    // Assert
    assert!(
        result
            .err()
            .expect("refused")
            .to_string()
            .contains("work budget")
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_malformed_huge_lock_count_refuses_without_allocating_entries() {
    // Arrange
    let path = temp_path("append-lock-header");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    recovered(&store, &positions);
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            super::super::super::super::prune::PRUNE_LOCKS_KEY,
            u32::MAX.to_le_bytes().to_vec(),
        )
        .expect("bad count header");
    // Act
    let result = store.maybe_basic_filter_append_proof_with_budget(budget());
    // Assert
    assert!(
        result
            .err()
            .expect("refused")
            .to_string()
            .contains("count exceeds envelope")
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_larger_operator_map_refuses_prepared_completion_before_effects() {
    // Arrange
    let path = temp_path("append-lock-growth");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let mut maximum = budget();
    maximum.checkpoint_operations = 5000;
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(maximum)
        .expect("proof")
        .expect("bounded");
    let prepared = store
        .prepare_basic_filter_append(proof, &records)
        .expect("prepare small map");
    let mut locks = store.load_prune_locks().expect("reserved lock");
    for index in 0..200 {
        locks.push(PruneLockInfo {
            name: format!("operator-{index}"),
            height_first: 0,
            height_last: 99,
        });
    }
    store
        .sync_prune_locks(&locks)
        .expect("general operator CRUD remains supported");
    let before = store.maybe_basic_filter_state().expect("before");
    // Act
    let result = store.complete_basic_filter_append(prepared);
    // Assert
    assert!(
        result
            .expect_err("refused")
            .to_string()
            .contains("work budget")
    );
    assert_eq!(
        store.load_prune_locks().expect("operator map preserved"),
        locks
    );
    assert_eq!(
        store.maybe_basic_filter_state().expect("safe unchanged"),
        before
    );
    assert_eq!(
        store
            .basic_filter_artifacts()
            .expect("no candidate effects"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_interrupted_actual_coins_batch_refuses_prepared_work_and_reopen() {
    // Arrange
    let path = temp_path("append-interrupted-coins");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(2);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, &records)
        .expect("prepare");
    let before = store.maybe_basic_filter_state().expect("before");
    let mut view = store.coins_view();
    view.set_simulate_crash_after_partial(true);
    let coin = open_bitcoin_core::chainstate::Coin {
        output: TransactionOutput {
            value: Amount::from_sats(1).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        },
        created_height: 0,
        created_median_time_past: 0,
        is_coinbase: true,
    };
    let entries = [(
        OutPoint::null(),
        open_bitcoin_core::chainstate::CoinsCacheEntry::unspent_dirty(coin),
    )]
    .into_iter()
    .collect();
    // Act
    assert!(
        view.batch_write_with_limit(
            open_bitcoin_core::chainstate::CoinsBatch { entries },
            Some(positions[0].block_hash),
            1
        )
        .is_err()
    );
    assert!(store.complete_basic_filter_append(prepared).is_err());
    drop(view);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    // Assert
    assert_eq!(
        reopened
            .coins_view()
            .head_blocks()
            .expect("achieved interrupted H")
            .len(),
        2
    );
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("safe unchanged"),
        before
    );
    assert_eq!(
        reopened.basic_filter_artifacts().expect("no rows"),
        (false, false)
    );
    assert!(
        reopened
            .configure_basic_filter_index_before_prune(
                Some(positions[1].block_hash),
                BasicFilterStartupMode::Enabled
            )
            .is_err()
    );
    assert!(
        reopened
            .maybe_basic_filter_append_proof_with_budget(budget())
            .expect("no minted authority")
            .is_none()
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_more_than_128_candidates_refuses_before_allocation() {
    // Arrange
    let path = temp_path("append-envelope-count");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(129);
    recovered(&store, &positions[..1]);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_artifacts().expect("absent"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_foreign_preparation_cannot_publish_in_identical_store() {
    // Arrange
    let first_path = temp_path("append-foreign-first");
    let second_path = temp_path("append-foreign-second");
    let first = FjallNodeStore::open(&first_path).expect("first");
    let second = FjallNodeStore::open(&second_path).expect("second");
    let (positions, records) = fixtures(1);
    recovered(&first, &positions);
    recovered(&second, &positions);
    let proof = first
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = first
        .prepare_basic_filter_append(proof, &records)
        .expect("prepare");
    // Act
    let result = second.complete_basic_filter_append(prepared);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        second.basic_filter_artifacts().expect("absent"),
        (false, false)
    );
    drop(first);
    drop(second);
    std::fs::remove_dir_all(first_path).expect("cleanup");
    std::fs::remove_dir_all(second_path).expect("cleanup");
}

#[test]
fn phase157_append_disabled_generation_refuses_prepared_completion() {
    // Arrange
    let path = temp_path("append-disabled");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, &records)
        .expect("prepare");
    // Act
    store.disable_basic_filter_index().expect("actual disable");
    let result = store.complete_basic_filter_append(prepared);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.basic_filter_artifacts().expect("absent"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_wrong_previous_header_in_saved_row_refuses_local_edge() {
    // Arrange
    let path = temp_path("append-header-edge");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    append(&store, &records[..2]);
    let key = codec::record_key(records[1].identity().block_hash());
    let mut bytes = codec::encode_record(&records[1]);
    let previous = open_bitcoin_core::primitives::FilterHeader::default();
    bytes[70..102].copy_from_slice(previous.as_bytes());
    let header = open_bitcoin_core::consensus::compute_filter_header(
        records[1].identity().filter_hash(),
        previous,
    );
    bytes[134..166].copy_from_slice(header.as_bytes());
    // Deliberately bypass raw publication to test local edge validation with a
    // live proof. Raw clone invalidation and real recovery have separate coverage.
    store
        .block_index
        .insert(&key, bytes)
        .expect("own commitment valid, local edge wrong");
    // Act
    let result = store.maybe_basic_filter_append_proof_with_budget(budget());
    // Assert
    assert!(
        result
            .err()
            .expect("local edge must fail")
            .to_string()
            .contains("predecessor")
    );
    assert!(
        store
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[2].identity().block_hash())
            )
            .expect("absent")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_processed_projection_tampering_refuses_completion() {
    // Arrange
    let path = temp_path("append-processed-projection");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    append(&store, &records[..2]);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, &records[2..])
        .expect("prepare");
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            &codec::active_key(1),
            codec::encode_projection(1, records[0].identity().block_hash()),
        )
        .expect("endpoint corruption");
    // Act
    let result = store.complete_basic_filter_append(prepared);
    // Assert
    assert!(result.is_err());
    assert!(
        store
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[2].identity().block_hash())
            )
            .expect("absent")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn validated_history(
    count: u32,
) -> (
    Vec<Block>,
    Vec<StoredFilterRecord>,
    open_bitcoin_core::chainstate::ChainstateSnapshot,
) {
    let mut state = Chainstate::default();
    let mut blocks = Vec::new();
    let mut records: Vec<StoredFilterRecord> = Vec::new();
    let mut maybe_genesis = None;
    for height in 0..count {
        let mut block = fixture_block(
            state
                .tip()
                .map(|position| position.block_hash)
                .unwrap_or_default(),
            height,
        );
        let mut magnitude = height.to_le_bytes().to_vec();
        while magnitude.last() == Some(&0) {
            magnitude.pop();
        }
        if magnitude.last().is_some_and(|byte| byte & 0x80 != 0) {
            magnitude.push(0);
        }
        let mut height_script = vec![magnitude.len() as u8];
        height_script.extend(magnitude);
        height_script.push(0x51);
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_bytes(height_script).expect("height script");
        block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("easy proof of work");
        state
            .connect_block_with_current_time(
                &block,
                u128::from(height) + 1,
                i64::from(block.header.time) + 1,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("actual consensus acceptance");
        let position = state.tip().expect("accepted tip");
        let maybe_history = (height != 0).then(|| HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &state.undo_by_block()[&position.block_hash],
        });
        let inputs = BasicFilterInputs::from_historical(&block, position, maybe_history)
            .expect("actual historical inputs");
        records.push(
            StoredFilterRecord::generate(
                &inputs,
                position,
                records.last().map(|record| record.identity()).as_ref(),
            )
            .expect("generated"),
        );
        if height == 0 {
            maybe_genesis = Some(state.snapshot());
        }
        blocks.push(block);
    }
    (blocks, records, maybe_genesis.expect("genesis"))
}

#[test]
fn phase157_append_actual_runtime_deferred_flush_earns_bounded_zero_record_release() {
    // Arrange
    let mut measured = Vec::new();
    for count in [17, 129] {
        let path = temp_path("append-own-flush-release");
        let store = FjallNodeStore::open(&path).expect("open");
        let (blocks, records, genesis) = validated_history(count);
        store
            .seed_coins_from_snapshot(&genesis)
            .expect("genesis B and metadata");
        for block in &blocks {
            store
                .save_block(block, PersistMode::Sync)
                .expect("retained body");
        }
        let runtime = crate::DurableSyncRuntime::open_configured(
            store,
            crate::SyncRuntimeConfig::default(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("actual recovered runtime");
        let store = runtime.store();
        for candidates in records.chunks(128) {
            append(store, candidates);
        }
        assert_eq!(
            store
                .maybe_basic_filter_state()
                .expect("safe")
                .expect("saved")
                .maybe_endpoint,
            Some((0, block_hash(&blocks[0].header)))
        );
        for (index, block) in blocks.iter().enumerate().skip(1) {
            runtime
                .network_handle()
                .connect_stored_block(
                    block,
                    index as u128 + 1,
                    i64::from(block.header.time) + 1,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                )
                .expect("ordinary accepted connect");
        }
        // Act
        runtime
            .network_handle()
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX,
            )
            .expect("actual tracked own flush");
        store
            .filter_integrity_reads
            .store(0, std::sync::atomic::Ordering::Relaxed);
        let achieved = append(store, &[]);
        // Assert
        assert_eq!(
            achieved.processed,
            checkpoint(records.last().expect("processed endpoint"))
        );
        assert_eq!(achieved.safe_checkpoint, achieved.processed);
        assert_eq!(achieved.work.blocks, 0);
        let reads = store
            .filter_integrity_reads
            .load(std::sync::atomic::Ordering::Relaxed);
        assert!(achieved.work.record_operations <= 32);
        assert!(reads < 32);
        measured.push((achieved.work, achieved.batch_bytes, reads));
        assert_eq!(
            store
                .maybe_basic_filter_state()
                .expect("safe")
                .expect("saved")
                .protection,
            IndexInputProtection::FromHeight(count)
        );
        drop(runtime);
        let reopened = FjallNodeStore::open(&path).expect("genuine reopen");
        assert_eq!(
            reopened
                .maybe_basic_filter_checkpoint()
                .expect("durable earned safe"),
            Some(checkpoint(records.last().expect("durable endpoint")))
        );
        drop(reopened);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
    assert_eq!(measured[0], measured[1]);
    eprintln!("phase157 actual-runtime release work (17/129): {measured:?}");
}
