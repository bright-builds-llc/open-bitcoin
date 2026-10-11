// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::block_hash;
use open_bitcoin_primitives::{BlockHash, BlockHeader, BlockLocator, Hash32, MerkleRoot};

use crate::{HeaderStore, NetworkError};

fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

#[test]
fn header_store_builds_exponential_locators() {
    let mut store = HeaderStore::default();
    let mut previous = BlockHash::from_byte_array([0_u8; 32]);
    for height in 0..15_u32 {
        let current_header = header(previous, height + 1);
        let position =
            ChainPosition::new(current_header.clone(), height, u128::from(height + 1), 0);
        previous = position.block_hash;
        store.record_position(&position);
    }

    let locator = store.locator();
    assert_eq!(locator.block_hashes.len(), 14);
    assert_eq!(
        locator.block_hashes.first().copied(),
        Some(Hash32::from(previous)),
    );
    assert_eq!(
        locator.block_hashes.last().copied(),
        Some(locator.block_hashes[locator.block_hashes.len() - 1]),
    );
}

#[test]
fn headers_after_locator_returns_following_headers() {
    let mut store = HeaderStore::default();
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis = store
        .insert_header(genesis_header.clone())
        .expect("genesis");
    let child_header = header(genesis.block_hash, 2);
    let child = store.insert_header(child_header.clone()).expect("child");
    let descendant_header = header(child.block_hash, 3);
    store
        .insert_header(descendant_header.clone())
        .expect("descendant");

    let locator = open_bitcoin_primitives::BlockLocator {
        block_hashes: vec![genesis.block_hash.into()],
    };
    let headers = store.headers_after_locator(&locator, BlockHash::from_byte_array([0_u8; 32]), 2);

    assert_eq!(headers, vec![child_header, descendant_header]);
}

#[test]
fn headers_after_locator_respects_stop_hash() {
    let mut store = HeaderStore::default();
    let genesis = store
        .insert_header(header(BlockHash::from_byte_array([0_u8; 32]), 1))
        .expect("genesis");
    let child_header = header(genesis.block_hash, 2);
    let child = store.insert_header(child_header.clone()).expect("child");
    let descendant_header = header(child.block_hash, 3);
    let descendant = store
        .insert_header(descendant_header.clone())
        .expect("descendant");

    let locator = BlockLocator {
        block_hashes: vec![genesis.block_hash.into()],
    };
    let headers = store.headers_after_locator(&locator, descendant.block_hash, 10);
    assert_eq!(headers, vec![child_header, descendant_header]);
}

#[test]
fn empty_duplicate_and_missing_ancestor_paths_are_covered() {
    let mut store = HeaderStore::default();
    assert_eq!(store.best_height(), -1);
    assert!(store.best_tip().is_none());
    store.update_best_tip(BlockHash::from_byte_array([42_u8; 32]));
    assert!(store.best_tip().is_none());
    store.best_tip = Some(BlockHash::from_byte_array([43_u8; 32]));
    store
        .parents
        .insert(BlockHash::from_byte_array([43_u8; 32]), None);
    assert!(
        store
            .headers_after_locator(&BlockLocator::default(), BlockHash::default(), 1)
            .is_empty()
    );
    store.best_tip = None;
    store.parents.clear();
    assert!(store.locator().block_hashes.is_empty());
    assert!(
        store
            .headers_after_locator(&BlockLocator::default(), BlockHash::default(), 0)
            .is_empty()
    );
    assert!(!store.contains(&BlockHash::from_byte_array([1_u8; 32])));

    let missing_parent = BlockHash::from_byte_array([9_u8; 32]);
    assert_eq!(
        store.insert_header(header(missing_parent, 1)),
        Err(NetworkError::MissingHeaderAncestor(missing_parent)),
    );

    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 7);
    let inserted = store.insert_header(genesis_header.clone()).expect("insert");
    let duplicate = store.insert_header(genesis_header).expect("duplicate");
    assert_eq!(duplicate, inserted);
    assert!(store.contains(&inserted.block_hash));
    assert_eq!(
        store.best_tip().expect("tip").block_hash,
        inserted.block_hash
    );
}

#[test]
fn seed_and_tie_break_paths_prefer_the_expected_tip() {
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_position = ChainPosition::new(genesis_header.clone(), 0, 1, 0);
    let child_a_header = header(genesis_position.block_hash, 2);
    let child_b_header = header(genesis_position.block_hash, 3);
    let child_a_position = ChainPosition::new(child_a_header.clone(), 1, 2, 0);
    let child_b_position = ChainPosition::new(child_b_header.clone(), 1, 2, 0);

    let mut store = HeaderStore::default();
    store.seed_from_chain(&[genesis_position.clone(), child_a_position.clone()]);
    store.record_position(&child_b_position);

    let expected_tip = child_a_position.block_hash.max(child_b_position.block_hash);
    assert_eq!(store.best_tip().expect("best tip").block_hash, expected_tip);
}

#[test]
fn higher_height_wins_when_work_is_equal() {
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_position = ChainPosition::new(genesis_header, 0, 1, 0);
    let child_header = header(genesis_position.block_hash, 2);
    let descendant_header = header(block_hash(&child_header), 3);

    let mut store = HeaderStore::default();
    store.record_position(&genesis_position);
    store.record_position(&ChainPosition::new(child_header.clone(), 1, 2, 0));
    store.record_position(&ChainPosition::new(descendant_header.clone(), 2, 2, 0));

    assert_eq!(
        store.best_tip().expect("best tip").block_hash,
        block_hash(&descendant_header),
    );
}

#[test]
fn best_chain_entries_follow_the_selected_tip_order() {
    let mut store = HeaderStore::default();
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis = store
        .insert_header(genesis_header.clone())
        .expect("genesis");
    let branch_a_header = header(genesis.block_hash, 2);
    let branch_a = store.insert_header(branch_a_header).expect("branch a");
    let branch_b_header = header(genesis.block_hash, 3);
    let branch_b = store
        .insert_header(branch_b_header.clone())
        .expect("branch b");
    let branch_b_child = store
        .insert_header(header(branch_b.block_hash, 4))
        .expect("branch b child");

    let entries = store.best_chain_entries();

    assert_eq!(
        entries
            .iter()
            .map(|entry| entry.block_hash)
            .collect::<Vec<_>>(),
        vec![
            genesis.block_hash,
            branch_b.block_hash,
            branch_b_child.block_hash
        ]
    );
    assert_eq!(entries[0].height, 0);
    assert_eq!(entries[1].height, 1);
    assert_eq!(entries[2].height, 2);
    assert!(
        entries
            .iter()
            .all(|entry| entry.block_hash != branch_a.block_hash)
    );
}

#[test]
fn header_store_rebuilds_from_persisted_entries() {
    // Arrange
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis = ChainPosition::new(genesis_header.clone(), 0, 1, 0);
    let child_header = header(genesis.block_hash, 2);
    let child = ChainPosition::new(child_header, 1, 2, 0);
    let original_entries = [
        crate::HeaderEntry {
            block_hash: child.block_hash,
            header: child.header.clone(),
            height: child.height,
            chain_work: child.chain_work,
        },
        crate::HeaderEntry {
            block_hash: genesis.block_hash,
            header: genesis.header.clone(),
            height: genesis.height,
            chain_work: genesis.chain_work,
        },
    ];

    // Act
    let store = HeaderStore::from_entries(original_entries).expect("rebuild header store");
    let rebuilt_entries = store.entries().cloned().collect::<Vec<_>>();

    // Assert
    assert_eq!(store.best_height(), 1);
    assert_eq!(
        store.best_tip().expect("best tip").block_hash,
        child.block_hash
    );
    assert_eq!(rebuilt_entries.len(), 2);
    assert!(rebuilt_entries.iter().any(|entry| entry.height == 0));
    assert!(rebuilt_entries.iter().any(|entry| entry.height == 1));
}

#[test]
fn header_store_rebuild_rejects_missing_parent() {
    // Arrange
    let missing_parent = BlockHash::from_byte_array([9_u8; 32]);
    let orphan_header = header(missing_parent, 2);
    let orphan_entry = crate::HeaderEntry {
        block_hash: block_hash(&orphan_header),
        header: orphan_header,
        height: 1,
        chain_work: 2,
    };

    // Act
    let error = HeaderStore::from_entries([orphan_entry]).expect_err("missing parent");

    // Assert
    assert_eq!(error, NetworkError::MissingHeaderAncestor(missing_parent));
}

#[test]
fn entry_ancestor_and_median_time_helpers_cover_present_and_missing_paths() {
    // Arrange
    let mut store = HeaderStore::default();
    let mut previous = BlockHash::from_byte_array([0_u8; 32]);
    let mut hashes = Vec::new();
    for height in 0..12_u32 {
        let current_header = header(previous, height + 1);
        previous = block_hash(&current_header);
        hashes.push(previous);
        store
            .insert_header(current_header)
            .expect("header inserts into store");
    }

    // Act / Assert
    let tip_hash = *hashes.last().expect("tip hash");
    assert_eq!(store.entry(&tip_hash).expect("tip entry").height, 11);
    assert!(
        store
            .entry(&BlockHash::from_byte_array([99_u8; 32]))
            .is_none()
    );
    assert_eq!(
        store
            .ancestor_at_height(tip_hash, 5)
            .expect("ancestor")
            .height,
        5,
    );
    assert!(store.ancestor_at_height(tip_hash, 20).is_none());
    assert_eq!(store.median_time_past(tip_hash), Some(7));
    assert!(
        store
            .median_time_past(BlockHash::from_byte_array([100_u8; 32]))
            .is_none()
    );

    // Corrupt one parent edge to cover the helper's missing-parent fallback.
    let child_hash = hashes[2];
    store.parents.insert(child_hash, None);
    assert!(store.ancestor_at_height(tip_hash, 1).is_none());
}
