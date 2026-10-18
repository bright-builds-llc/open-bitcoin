// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/pow.cpp

use super::*;
use open_bitcoin_consensus::{chain_proof::block_proof, crypto::check_proof_of_work};
use open_bitcoin_primitives::MerkleRoot;

fn mined(parent: BlockHash, time: u32, bits: u32) -> BlockHeader {
    let mut header = BlockHeader {
        version: 1,
        previous_block_hash: parent,
        merkle_root: MerkleRoot::default(),
        time,
        bits,
        nonce: 0,
    };
    while !check_proof_of_work(block_hash(&header).to_byte_array(), bits).expect("bounded target") {
        header.nonce = header.nonce.checked_add(1).expect("bounded mining");
    }
    header
}

#[test]
fn phase165_shorter_greater_work_drives_tip_and_proof_projection() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(mined(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    let easy = store
        .insert_header(mined(root.block_hash, 2, 0x207f_ffff))
        .expect("easy");
    store
        .insert_header(mined(easy.block_hash, 3, 0x207f_ffff))
        .expect("tall");
    let hard_header = mined(root.block_hash, 4, 0x201f_ffff);
    // Act
    let hard = store.insert_header(hard_header).expect("hard");
    // Assert
    assert_eq!(store.best_tip().expect("best").block_hash, hard.block_hash);
    assert_eq!(
        store.maybe_best_proof_header().expect("facts").cumulative,
        hard.chain_work
    );
    assert_eq!(
        hard.chain_work,
        block_proof(0x207f_ffff)
            .expect("root proof")
            .wrapping_add(block_proof(0x201f_ffff).expect("hard proof"))
    );
    assert_eq!(store.best_chain_entries().len(), 2);
}

#[test]
fn phase165_equal_work_preserves_real_admission_order() {
    // Arrange
    let root = mined(BlockHash::default(), 1, 0x207f_ffff);
    let a = mined(block_hash(&root), 2, 0x207f_ffff);
    let b = mined(block_hash(&root), 3, 0x207f_ffff);
    // Act / Assert
    for (first, second) in [(a.clone(), b.clone()), (b, a)] {
        let mut store = HeaderStore::default();
        store.insert_header(root.clone()).expect("root");
        let admitted = store.insert_header(first.clone()).expect("first");
        store.insert_header(second).expect("second");
        assert_eq!(
            store.best_tip().expect("tip").block_hash,
            admitted.block_hash
        );
        assert_eq!(
            store.maybe_best_proof_header().expect("facts").block_hash,
            admitted.block_hash
        );
        let before = store.clone();
        assert_eq!(
            store.insert_header(first).expect("duplicate").chain_work,
            admitted.chain_work
        );
        assert_eq!(store, before);
    }
}

#[test]
fn phase165_imported_consistent_numbers_cannot_earn_ancestry() {
    // Arrange
    let mut admitted = HeaderStore::default();
    let root = admitted
        .insert_header(mined(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    let mut imported = HeaderStore::from_entries(admitted.entries().cloned()).expect("raw import");
    let before = imported.clone();
    // Act
    let result = imported.insert_header(mined(root.block_hash, 2, 0x207f_ffff));
    // Assert
    assert!(result.is_err());
    assert_eq!(imported, before);
    assert!(imported.maybe_proof_facts(root.block_hash).is_none());
    assert!(imported.maybe_best_proof_header().is_none());
}

#[test]
fn phase165_rejected_header_inputs_preserve_all_owner_state() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(mined(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    let mut invalid_hash = mined(root.block_hash, 2, 0x207f_ffff);
    while check_proof_of_work(block_hash(&invalid_hash).to_byte_array(), invalid_hash.bits)
        .expect("target")
    {
        invalid_hash.nonce += 1;
    }
    let mut invalid_compact = invalid_hash.clone();
    invalid_compact.bits = 0x0100_0001;
    let additional_root = mined(BlockHash::default(), 3, 0x207f_ffff);
    let missing = mined(BlockHash::from_byte_array([99; 32]), 4, 0x207f_ffff);
    // Act / Assert
    for candidate in [invalid_hash, invalid_compact, additional_root, missing] {
        let before = store.clone();
        assert!(store.insert_header(candidate).is_err());
        assert_eq!(store, before);
    }
}

#[test]
fn phase165_raw_forged_parents_cannot_mint_admission() {
    // Arrange
    let root_header = mined(BlockHash::default(), 1, 0x207f_ffff);
    let root_hash = block_hash(&root_header);
    let legitimate = HeaderEntry {
        block_hash: root_hash,
        header: root_header,
        height: 0,
        chain_work: block_proof(0x207f_ffff).expect("work"),
    };
    let mut bad_hash = legitimate.clone();
    bad_hash.block_hash = BlockHash::from_byte_array([9; 32]);
    let mut bad_height = legitimate.clone();
    bad_height.height = u32::MAX;
    let mut inflated = legitimate.clone();
    inflated.chain_work = open_bitcoin_consensus::chain_proof::ProofWork::from_le_bytes([255; 32]);
    // Act / Assert
    for raw in [legitimate, bad_hash, bad_height, inflated] {
        let parent_hash = raw.block_hash;
        let mut store = HeaderStore::default();
        store.record_position(&ChainPosition {
            block_hash: raw.block_hash,
            header: raw.header,
            height: raw.height,
            chain_work: raw.chain_work,
            median_time_past: 0,
        });
        let before = store.clone();
        assert!(
            store
                .insert_header(mined(parent_hash, 2, 0x207f_ffff))
                .is_err()
        );
        assert_eq!(store, before);
        assert!(store.best_tip().is_none());
        assert!(store.maybe_proof_facts(root_hash).is_none());
    }
}

#[test]
fn phase165_selected_limit_and_context_transitions_fail_closed() {
    // Arrange
    let root = mined(BlockHash::default(), 1, 0x207f_ffff);
    let restrictive = ConsensusParams {
        pow_limit_bits: 0x201f_ffff,
        ..ConsensusParams::default()
    };
    let mut store = HeaderStore::default();
    // Act / Assert
    let before = store.clone();
    assert!(
        store
            .insert_header_with_params(root.clone(), &restrictive)
            .is_err()
    );
    assert_eq!(store, before);
    store
        .insert_header(root.clone())
        .expect("unbound acceptance");
    let before = store.clone();
    assert!(
        store
            .insert_header_with_params(root.clone(), &ConsensusParams::default())
            .is_err()
    );
    assert_eq!(store, before);
    let mut selected = HeaderStore::default();
    selected
        .insert_header_with_params(root.clone(), &ConsensusParams::default())
        .expect("selected root");
    let before = selected.clone();
    assert!(selected.insert_header(root.clone()).is_err());
    assert!(
        selected
            .insert_header_with_params(root, &restrictive)
            .is_err()
    );
    assert_eq!(selected, before);
}

#[test]
fn phase165_recorded_positions_cannot_overwrite_accepted_receipts() {
    // Arrange
    let mut store = HeaderStore::default();
    let header = mined(BlockHash::default(), 1, 0x207f_ffff);
    let root = store.insert_header(header.clone()).expect("root");
    let forged = ChainPosition::new(
        header,
        99,
        open_bitcoin_consensus::chain_proof::ProofWork::from_le_bytes([255; 32]),
        0,
    );
    let before = store.clone();
    // Act
    store.record_position(&forged);
    // Assert
    assert_eq!(store, before);
    assert_eq!(store.best_tip().expect("tip").chain_work, root.chain_work);
}

#[test]
fn phase165_taller_equal_work_branch_cannot_replace_first_best() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(mined(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    let shorter = store
        .insert_header(mined(root.block_hash, 2, 0x202a_aaaa))
        .expect("shorter proof six");
    let mut parent = root.block_hash;
    // Act
    for time in 3..=5 {
        parent = store
            .insert_header(mined(parent, time, 0x207f_ffff))
            .expect("easy proof two")
            .block_hash;
    }
    // Assert
    assert_eq!(
        store.entry(&parent).expect("tall").chain_work,
        shorter.chain_work
    );
    assert_eq!(store.entry(&parent).expect("tall").height, 3);
    assert_eq!(
        store.best_tip().expect("first best").block_hash,
        shorter.block_hash
    );
}

#[test]
fn phase165_wide_corpus_numbers_remain_only_unproved_projection() {
    // Arrange
    let expected = "8000000000000000000000000000000000000000000000000000000000000000";
    assert!(
        include_str!("../../../../../docs/parity/fixtures/phase165-work-vectors.json")
            .contains(expected)
    );
    let high =
        open_bitcoin_consensus::chain_proof::ProofWork::from_hex(expected).expect("corpus number");
    let low = ChainPosition::untrusted_fixture_work(u128::MAX);
    let a = mined(BlockHash::default(), 1, 0x207f_ffff);
    let b = mined(BlockHash::default(), 2, 0x207f_ffff);
    let high_hash = block_hash(&b);
    // Act
    let store = HeaderStore::from_entries([
        HeaderEntry {
            block_hash: block_hash(&a),
            header: a,
            height: 0,
            chain_work: low,
        },
        HeaderEntry {
            block_hash: high_hash,
            header: b,
            height: 0,
            chain_work: high,
        },
    ])
    .expect("raw numbers");
    // Assert
    assert_eq!(
        store
            .maybe_unproved_fixture_tip()
            .expect("wide raw number")
            .block_hash,
        high_hash
    );
    assert!(store.best_tip().is_none());
    assert!(store.maybe_best_proof_header().is_none());
}

#[test]
fn phase165_cycle_import_has_no_authority_and_bounded_ancestry() {
    // Arrange
    let a_hash = BlockHash::from_byte_array([1; 32]);
    let b_hash = BlockHash::from_byte_array([2; 32]);
    let work = block_proof(0x207f_ffff).expect("numerical proof");
    // Act
    let mut store = HeaderStore::from_entries([
        HeaderEntry {
            block_hash: a_hash,
            header: mined(b_hash, 1, 0x207f_ffff),
            height: 1,
            chain_work: work,
        },
        HeaderEntry {
            block_hash: b_hash,
            header: mined(a_hash, 2, 0x207f_ffff),
            height: 1,
            chain_work: work,
        },
    ])
    .expect("raw cyclic numbers");
    let before = store.clone();
    let rejected = store.insert_header(mined(a_hash, 3, 0x207f_ffff));
    // Assert
    assert!(rejected.is_err());
    assert_eq!(store, before);
    assert!(store.ancestor_at_height(a_hash, 0).is_none());
    assert!(store.best_chain_entries().is_empty());
    assert!(store.maybe_best_proof_header().is_none());
}

#[test]
fn phase165_raw_duplicate_is_not_an_admission_receipt() {
    // Arrange
    let header = mined(BlockHash::default(), 1, 0x207f_ffff);
    let position = ChainPosition::new(
        header.clone(),
        0,
        ChainPosition::untrusted_fixture_work(999),
        0,
    );
    let mut store = HeaderStore::default();
    store.record_position(&position);
    let before = store.clone();
    // Act
    let result = store.insert_header(header);
    // Assert
    assert!(result.is_err());
    assert_eq!(store, before);
    assert!(store.maybe_best_proof_header().is_none());
}

#[test]
fn phase165_header_queries_bound_a_missing_selected_entry() {
    // Arrange: corrupt internal projection state to exercise the defensive query guard.
    let mut store = HeaderStore::default();
    store
        .insert_header(mined(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    store.best_tip = Some(BlockHash::from_byte_array([99; 32]));
    // Act
    let headers = store.headers_after_locator(&BlockLocator::default(), BlockHash::default(), 1);
    // Assert
    assert!(headers.is_empty());
    assert!(store.best_tip().is_none());
}
