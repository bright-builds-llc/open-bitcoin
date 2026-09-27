// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::path::PathBuf;

use open_bitcoin_core::chainstate::{ChainPosition, PruneLockInfo};

use super::*;
use crate::storage::coins_codec::{encode_best_block_key, encode_best_block_value};
use crate::storage::fjall_store::{HAVE_PRUNED_KEY, PruneIntent, resume_prune_intent};

#[test]
fn default_coins_cache_byte_limit_is_442_mib() {
    // Arrange
    let flush_src = include_str!("../../../../../open-bitcoin-chainstate/src/coins/flush.rs");

    // Act
    let limit = default_coins_cache_byte_limit();

    // Assert
    assert_eq!(limit, 442 * 1024 * 1024);
    assert_eq!(DEFAULT_KERNEL_CACHE_BYTES, 450 * 1024 * 1024);
    assert_eq!(MIN_DBCACHE_BYTES, 4 * 1024 * 1024);
    assert_eq!(COINS_DB_CACHE_CAP_BYTES, 8 * 1024 * 1024);
    assert!(
        !flush_src.contains("DEFAULT_KERNEL_CACHE_BYTES")
            && !flush_src.contains("MIN_DBCACHE_BYTES")
            && !flush_src.contains("COINS_DB_CACHE_CAP_BYTES"),
        "flush.rs must stay unpinned from shell cache defaults"
    );
}

#[test]
fn initialize_consistent_store_is_ready_to_flush() {
    // Arrange
    let (path, store) = open_temp_store("consistent-ready");

    // Act
    let (lifecycle, _view, cache) = initialize_ready(&store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_eq!(cache.cache_entry_count(), 0);
    remove_dir_if_exists(&path);
}

#[test]
fn initialize_interrupted_without_bodies_is_not_ready_to_flush() {
    // Arrange
    let (path, store) = open_temp_store("interrupted-no-bodies");
    let view = FjallCoinsView::from_store(&store);
    plant_interrupted_heads(
        &view,
        BlockHash::from_byte_array([0xaa; 32]),
        BlockHash::from_byte_array([0xbb; 32]),
    );

    // Act
    let error = expect_error(
        initialize(&store, policy_now(), policy_now(), 0, false, u64::MAX),
        "missing bodies fail-closed",
    );

    // Assert
    let display = error.to_string();
    assert!(
        display.contains("fail_closed"),
        "missing-body replay must say fail_closed, got {display}"
    );
    assert!(
        display.contains("interrupted"),
        "missing-body replay must say interrupted, got {display}"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn initialize_interrupted_with_bodies_is_ready_after_replay() {
    // Arrange
    let (path, store) = open_temp_store("interrupted-with-bodies");
    let view = FjallCoinsView::from_store(&store);
    let new_header = header(BlockHash::from_byte_array([0_u8; 32]), 4);
    let new_hash = block_hash(&new_header);
    let new_block = Block {
        header: new_header.clone(),
        transactions: vec![coinbase(0, 50)],
    };
    store
        .save_header_entries(&[header_entry(new_header, 0, 1)], PersistMode::Sync)
        .expect("save first-flush header");
    store
        .save_block(&new_block, PersistMode::Sync)
        .expect("save first-flush body");
    plant_interrupted_heads(&view, new_hash, BlockHash::from_byte_array([0_u8; 32]));

    // Act
    let (lifecycle, view, cache) = initialize_ready(&store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_eq!(view.best_block().expect("best"), Some(new_hash));
    assert_eq!(cache.cache_entry_count(), 0);
    remove_dir_if_exists(&path);
}

struct ResumeChain {
    path: PathBuf,
    store: FjallNodeStore,
    eligible: ChainPosition,
    tip: ChainPosition,
}

fn position(height: u32, nonce: u32) -> ChainPosition {
    let block_header = header(BlockHash::from_byte_array([0_u8; 32]), nonce);
    ChainPosition::new(block_header, height, u128::from(height), 1_700_000_000)
}

fn open_resume_chain(test_name: &str, eligible_height: u32, tip_height: u32) -> ResumeChain {
    let (path, store) = open_temp_store(test_name);
    let eligible = position(eligible_height, eligible_height.saturating_add(3));
    let tip = position(tip_height, tip_height.saturating_add(11));
    store
        .save_chain_meta(&[eligible.clone(), tip.clone()], PersistMode::Sync)
        .expect("save chain meta");
    ResumeChain {
        path,
        store,
        eligible,
        tip,
    }
}

fn plant_block(chain: &ResumeChain) {
    let body = Block {
        header: chain.eligible.header.clone(),
        transactions: Vec::new(),
    };
    let saved = chain
        .store
        .save_block(&body, PersistMode::Sync)
        .expect("save block");
    assert_eq!(saved, chain.eligible.block_hash);
}

fn plant_undo(chain: &ResumeChain) {
    chain
        .store
        .save_undo(
            chain.eligible.block_hash,
            &BlockUndo::default(),
            PersistMode::Sync,
        )
        .expect("save undo");
}

fn record_intent(chain: &ResumeChain, block_hash: BlockHash) {
    chain
        .store
        .sync_prune_intent(PruneIntent {
            height: chain.eligible.height,
            block_hash,
        })
        .expect("sync prune intent");
}

fn plant_best_block(store: &FjallNodeStore, block_hash: BlockHash) {
    FjallCoinsView::from_store(store)
        .write_raw_bytes(
            &encode_best_block_key(),
            encode_best_block_value(block_hash),
        )
        .expect("plant best block");
}

fn plant_have_pruned(store: &FjallNodeStore) {
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, HAVE_PRUNED_KEY, vec![1])
        .expect("plant have_pruned");
}

fn assert_both_mates_absent(store: &FjallNodeStore, block_hash: BlockHash) {
    assert!(!store.has_block(block_hash).expect("has_block"));
    assert!(!store.has_undo(block_hash).expect("has_undo"));
    assert!(store.load_block(block_hash).expect("load block").is_none());
    assert!(store.load_undo(block_hash).expect("load undo").is_none());
}

fn assert_mates_remain(store: &FjallNodeStore, block_hash: BlockHash) {
    assert!(store.has_block(block_hash).expect("has_block"));
    assert!(store.has_undo(block_hash).expect("has_undo"));
}

fn assert_intent_absent(store: &FjallNodeStore) {
    assert!(store.maybe_prune_intent().expect("intent").is_none());
}

fn assert_repair_refusal(error: &StorageError) {
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                action: StorageRecoveryAction::Repair,
                ..
            }
        ),
        "expected repair corruption, got {error}"
    );
    assert_eq!(error.recovery_action(), Some(StorageRecoveryAction::Repair));
    assert!(!matches!(error, StorageError::InterruptedWrite { .. }));
    let display = error.to_string();
    assert!(display.contains("fail_closed"), "{display}");
    assert!(display.contains("stopped closed"), "{display}");
    assert!(display.contains("did not reindex"), "{display}");
}

fn refuse_initialize(store: &FjallNodeStore) -> StorageError {
    expect_error(
        initialize(store, policy_now(), policy_now(), 0, false, u64::MAX),
        "resume refuses",
    )
}

#[test]
fn initialize_intent_with_both_mates_finishes_ready() {
    // Arrange
    let chain = open_resume_chain("resume-both-mates", 1, 300);
    plant_block(&chain);
    plant_undo(&chain);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, chain.eligible.block_hash);

    // Act
    let (lifecycle, _view, _cache) = initialize_ready(&chain.store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_both_mates_absent(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.load_have_pruned().expect("have_pruned"));
    assert_intent_absent(&chain.store);
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_intent_with_one_mate_removes_the_remaining_key() {
    // Arrange
    let chain = open_resume_chain("resume-one-mate", 1, 300);
    plant_block(&chain);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, chain.eligible.block_hash);

    // Act
    let (lifecycle, _view, _cache) = initialize_ready(&chain.store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_both_mates_absent(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.load_have_pruned().expect("have_pruned"));
    assert_intent_absent(&chain.store);
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_intent_with_have_pruned_and_one_mate_finishes() {
    // Arrange
    let chain = open_resume_chain("resume-flag-one-mate", 1, 300);
    plant_undo(&chain);
    plant_have_pruned(&chain.store);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, chain.eligible.block_hash);

    // Act
    let (lifecycle, _view, _cache) = initialize_ready(&chain.store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert_both_mates_absent(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.load_have_pruned().expect("have_pruned"));
    assert_intent_absent(&chain.store);
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_intent_with_both_mates_absent_clears_intent() {
    // Arrange
    let chain = open_resume_chain("resume-both-absent", 1, 300);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, chain.eligible.block_hash);

    // Act
    let (lifecycle, _view, _cache) = initialize_ready(&chain.store);

    // Assert
    assert_eq!(lifecycle.readiness(), ManagerReadiness::ReadyToFlush);
    assert!(!chain.store.load_have_pruned().expect("have_pruned"));
    assert_intent_absent(&chain.store);
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_intent_hash_mismatch_refuses_repair() {
    // Arrange
    let chain = open_resume_chain("resume-hash-mismatch", 1, 300);
    plant_block(&chain);
    plant_undo(&chain);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, BlockHash::from_byte_array([0xab; 32]));

    // Act
    let error = refuse_initialize(&chain.store);

    // Assert
    assert_repair_refusal(&error);
    assert_mates_remain(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.maybe_prune_intent().expect("intent").is_some());
    assert!(!chain.store.load_have_pruned().expect("have_pruned"));
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_intent_inside_keep_window_refuses_repair() {
    // Arrange
    let chain = open_resume_chain("resume-keep-window", 10, 20);
    plant_block(&chain);
    plant_undo(&chain);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, chain.eligible.block_hash);

    // Act
    let error = refuse_initialize(&chain.store);

    // Assert
    assert_repair_refusal(&error);
    assert_mates_remain(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.maybe_prune_intent().expect("intent").is_some());
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_best_block_other_than_tip_refuses_repair() {
    // Arrange
    let chain = open_resume_chain("resume-best-block", 1, 300);
    plant_block(&chain);
    plant_undo(&chain);
    plant_best_block(&chain.store, BlockHash::from_byte_array([0xcd; 32]));
    record_intent(&chain, chain.eligible.block_hash);

    // Act
    let error = refuse_initialize(&chain.store);

    // Assert
    assert_repair_refusal(&error);
    assert_mates_remain(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.maybe_prune_intent().expect("intent").is_some());
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_lock_forbidden_intent_refuses_repair() {
    // Arrange
    let chain = open_resume_chain("resume-lock", 5, 400);
    plant_block(&chain);
    plant_undo(&chain);
    plant_best_block(&chain.store, chain.tip.block_hash);
    record_intent(&chain, chain.eligible.block_hash);
    let locks = [PruneLockInfo {
        name: "rescan".to_string(),
        height_first: 5,
        height_last: 5,
    }];

    // Act
    let error = expect_error(
        resume_prune_intent(&chain.store, &locks),
        "lock forbids resume",
    );

    // Assert
    assert_repair_refusal(&error);
    assert_mates_remain(&chain.store, chain.eligible.block_hash);
    assert!(chain.store.maybe_prune_intent().expect("intent").is_some());
    remove_dir_if_exists(&chain.path);
}

#[test]
fn initialize_intent_on_empty_chain_refuses_repair() {
    // Arrange
    let (path, store) = open_temp_store("resume-empty-chain");
    store
        .sync_prune_intent(PruneIntent {
            height: 1,
            block_hash: BlockHash::from_byte_array([0x07; 32]),
        })
        .expect("sync prune intent");

    // Act
    let error = refuse_initialize(&store);

    // Assert
    assert_repair_refusal(&error);
    assert!(store.maybe_prune_intent().expect("intent").is_some());
    remove_dir_if_exists(&path);
}
