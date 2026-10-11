// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
use open_bitcoin_primitives::MerkleRoot;

fn header(parent: BlockHash, nonce: u32, bits: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash: parent,
        merkle_root: MerkleRoot::default(),
        time: nonce,
        bits,
        nonce,
    }
}

#[test]
fn phase160_header_proof_nonuniform_fork_and_recovery_match() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(header(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    let easy = store
        .insert_header(header(root.block_hash, 2, 0x207f_ffff))
        .expect("easy");
    let easy_tip = store
        .insert_header(header(easy.block_hash, 3, 0x207f_ffff))
        .expect("tip");
    let hard = store
        .insert_header(header(root.block_hash, 4, 0x1d00_ffff))
        .expect("hard");
    let mut entries: Vec<_> = store.entries().cloned().collect();
    entries.reverse();
    // Act
    let recovered = HeaderStore::from_entries(entries).expect("recover");
    // Assert
    assert_eq!(
        store.best_tip().expect("synthetic").block_hash,
        easy_tip.block_hash
    );
    assert_eq!(
        store.maybe_best_proof_header().expect("exact").block_hash,
        hard.block_hash
    );
    assert_eq!(
        store.maybe_best_proof_header(),
        recovered.maybe_best_proof_header()
    );
    assert_eq!(store.proof_facts, recovered.proof_facts);
}

#[test]
fn phase160_header_proof_invalid_parent_and_identity_are_unavailable() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(header(BlockHash::default(), 1, 0))
        .expect("invalid target fixture");
    let child = store
        .insert_header(header(root.block_hash, 2, 0x207f_ffff))
        .expect("child");
    let raw = HeaderEntry {
        block_hash: BlockHash::from_byte_array([9; 32]),
        header: header(BlockHash::default(), 3, 0x207f_ffff),
        height: 0,
        chain_work: 999,
    };
    // Act
    let recovered = HeaderStore::from_entries([raw]).expect("raw fixture");
    // Assert
    assert!(store.maybe_proof_facts(root.block_hash).is_none());
    assert!(store.maybe_proof_facts(child.block_hash).is_none());
    assert!(recovered.maybe_best_proof_header().is_none());
}

#[test]
fn phase160_header_proof_height_mismatch_and_ties_are_deterministic() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(header(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root");
    let a = store
        .insert_header(header(root.block_hash, 2, 0x207f_ffff))
        .expect("a");
    let b = store
        .insert_header(header(root.block_hash, 3, 0x207f_ffff))
        .expect("b");
    let recovered_tie = HeaderStore::from_entries(
        store
            .entries()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev(),
    )
    .expect("recovered tie");
    assert_eq!(
        recovered_tie.maybe_best_proof_header(),
        store.maybe_best_proof_header()
    );
    let mut entries: Vec<_> = store.entries().cloned().collect();
    for entry in &mut entries {
        if entry.block_hash == a.block_hash {
            entry.height = 5;
        }
    }
    // Act
    let recovered = HeaderStore::from_entries(entries).expect("fixture");
    // Assert
    assert_eq!(
        store.maybe_best_proof_header().expect("tie").block_hash,
        a.block_hash.max(b.block_hash)
    );
    assert!(recovered.maybe_proof_facts(a.block_hash).is_none());
    assert_eq!(
        recovered
            .maybe_best_proof_header()
            .expect("valid b")
            .block_hash,
        b.block_hash
    );
}

#[test]
fn phase160_header_proof_seed_and_admitted_positions_have_same_exact_sum() {
    // Arrange
    let root = ChainPosition::new(header(BlockHash::default(), 1, 0x207f_ffff), 0, 999, 0);
    let child = ChainPosition::new(header(root.block_hash, 2, 0x1d00_ffff), 1, 1000, 0);
    let positions = [root.clone(), child.clone()];
    let mut store = HeaderStore::default();
    // Act
    store.seed_from_chain(&positions);
    let recovered = HeaderStore::from_entries(store.entries().cloned()).expect("recover");
    // Assert
    assert_eq!(
        store
            .maybe_proof_facts(child.block_hash)
            .expect("proof")
            .cumulative,
        block_proof(root.header.bits)
            .expect("root work")
            .wrapping_add(block_proof(child.header.bits).expect("child work"))
    );
    assert_eq!(store.proof_facts, recovered.proof_facts);
    store.seed_from_chain(&[]);
    assert!(store.maybe_best_proof_header().is_none());
}

#[test]
fn phase160_coverage_unknown_proof_hash_preserves_existing_facts() {
    // Arrange
    let mut store = HeaderStore::default();
    let root = store
        .insert_header(header(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root fixture");
    store
        .insert_header(header(root.block_hash, 2, 0x1d00_ffff))
        .expect("child fixture");
    let unknown = BlockHash::from_byte_array([99; 32]);
    let before = store.clone();

    // Act
    store.record_proof_facts(unknown);

    // Assert
    assert!(store.maybe_proof_facts(unknown).is_none());
    assert_eq!(store, before);
}

#[test]
fn phase160_coverage_malformed_genesis_parent_withholds_descendant_proof() {
    // Arrange
    let mut valid = HeaderStore::default();
    let root = valid
        .insert_header(header(BlockHash::default(), 1, 0x207f_ffff))
        .expect("root fixture");
    valid
        .insert_header(header(root.block_hash, 2, 0x1d00_ffff))
        .expect("valid child fixture");
    let malformed_header = header(root.block_hash, 3, 0x1d00_ffff);
    let malformed_hash = block_hash(&malformed_header);
    let descendant_header = header(malformed_hash, 4, 0x1d00_ffff);
    let descendant_hash = block_hash(&descendant_header);
    let mut entries: Vec<_> = valid.entries().cloned().collect();
    entries.extend([
        HeaderEntry {
            block_hash: malformed_hash,
            header: malformed_header,
            height: 0,
            chain_work: u128::MAX - 1,
        },
        HeaderEntry {
            block_hash: descendant_hash,
            header: descendant_header,
            height: 1,
            chain_work: u128::MAX,
        },
    ]);

    // Act
    // Raw reconstruction tests arithmetic refusal, not consensus acceptance.
    let recovered = HeaderStore::from_entries(entries).expect("raw reconstruction fixture");

    // Assert
    assert!(recovered.maybe_proof_facts(malformed_hash).is_none());
    assert!(recovered.maybe_proof_facts(descendant_hash).is_none());
    assert_eq!(recovered.proof_facts, valid.proof_facts);
    assert_eq!(
        recovered.maybe_best_proof_header(),
        valid.maybe_best_proof_header()
    );
}
