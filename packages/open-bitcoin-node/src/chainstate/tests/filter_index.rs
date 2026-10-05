// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use open_bitcoin_core::chainstate::{
    IndexInputProtection,
    filter_index::{
        catch_up::{AcceptedIndexTarget, BasicIndexProgress},
        lifecycle::IndexGeneration,
    },
};

fn attach(managed: &mut ManagedChainstate<MemoryChainstateStore>) {
    let tip = managed.chainstate().tip().expect("accepted genesis");
    let progress = BasicIndexProgress::new(
        IndexGeneration::new(1),
        tip.block_hash,
        AcceptedIndexTarget::new(tip.height, tip.block_hash),
        None,
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("owner");
    managed
        .install_basic_index_owner(progress)
        .expect("install owner");
}

#[test]
fn phase157_accepted_direct_advances_target_without_forcing_flush() {
    // Arrange
    let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    let (genesis, _) = connect_genesis(&mut managed);
    attach(&mut managed);
    let next = build_block(block_hash(&genesis.header), 1, 50);
    // Act
    managed
        .connect_block(&next, 2, ScriptVerifyFlags::P2SH, mature_params())
        .expect("connect");
    // Assert
    let progress = managed.maybe_basic_index_progress().expect("owner");
    assert_eq!(progress.accepted_target().height(), 1);
    assert_eq!(progress.maybe_next_height().expect("next"), Some(0));
    assert_eq!(progress.maybe_processed_endpoint(), None);
    assert_eq!(managed.store().best_block().expect("unflushed coins"), None);
    assert!(
        managed.maybe_accepted_basic_facts().is_none(),
        "behind accepts do not jump genesis"
    );
}

#[test]
fn phase157_accepted_rejected_prepare_leaves_owner_unchanged() {
    // Arrange
    let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    connect_genesis(&mut managed);
    attach(&mut managed);
    let before = managed.maybe_basic_index_progress();
    let invalid = build_block(BlockHash::from_byte_array([9; 32]), 1, 50);
    // Act
    let result = managed.connect_block(&invalid, 2, ScriptVerifyFlags::P2SH, mature_params());
    // Assert
    assert!(result.is_err());
    assert_eq!(managed.maybe_basic_index_progress(), before);
    assert!(managed.maybe_accepted_basic_facts().is_none());
}

#[test]
fn phase157_accepted_clone_drops_owner() {
    // Arrange
    let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    connect_genesis(&mut managed);
    attach(&mut managed);
    // Act
    let cloned = managed.clone();
    // Assert
    assert!(cloned.maybe_basic_index_progress().is_none());
}
