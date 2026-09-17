// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_core::{consensus::block_hash, primitives::BlockHash};

use super::*;

#[test]
fn has_block_is_true_after_save_block_without_calling_load_block() {
    // Arrange
    let path = temp_store_path("has-block-presence");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open");
    let fixture = block(BlockHash::from_byte_array([0_u8; 32]), 3);
    let other_hash = BlockHash::from_byte_array([0xff; 32]);
    let source = include_str!("../blocks.rs");

    // Act
    let saved_hash = store
        .save_block(&fixture, PersistMode::Sync)
        .expect("save_block");
    let saved_present = store.has_block(saved_hash).expect("has_block saved");
    let missing_present = store.has_block(other_hash).expect("has_block missing");

    // Assert
    assert_eq!(saved_hash, block_hash(&fixture.header));
    assert!(saved_present);
    assert!(!missing_present);
    assert!(source.contains("contains_key"));
    assert!(
        !source.contains("load_block"),
        "has_block must not decode via load_block"
    );
    assert!(
        !source.contains("parse_block"),
        "has_block must not decode via parse_block"
    );
    remove_dir_if_exists(&path);
}
