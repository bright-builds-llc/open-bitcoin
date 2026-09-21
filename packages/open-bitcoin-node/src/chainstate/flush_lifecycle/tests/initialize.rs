// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

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
