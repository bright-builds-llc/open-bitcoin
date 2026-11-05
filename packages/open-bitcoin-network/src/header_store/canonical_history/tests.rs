// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/pow.cpp

use super::CanonicalHeaderAdmissionError;
use crate::BodyEligibilityError;
use crate::header_store::*;
use open_bitcoin_consensus::chain_spec::{CanonicalChain, CanonicalGenesisProof};
use open_bitcoin_primitives::ScriptBuf;

fn root() -> HeaderStore {
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proved root");
    HeaderStore::from_canonical_genesis(&proof).expect("root store")
}

#[test]
fn canonical_body_without_earned_descendant_header_refuses_before_receipt() {
    // Arrange
    let mut store = root();
    let params = store.maybe_consensus_params.expect("selected params");
    let mut block = CanonicalChain::Regtest.spec().genesis_block();
    block.header.previous_block_hash = block_hash(&block.header);
    block.header.version = 4;
    block.header.time += 1;
    block.transactions[0].inputs[0].script_sig = ScriptBuf::from_serialized_bytes(vec![0x51, 0]);
    block.header.merkle_root =
        open_bitcoin_consensus::crypto::block_merkle_root(&block.transactions)
            .expect("real merkle")
            .0;
    block.header.nonce = 0;
    while open_bitcoin_consensus::block::check_block_header_with_params(&block.header, &params)
        .is_err()
    {
        block.header.nonce += 1;
    }
    let hash = block_hash(&block.header);
    let before = store.clone();

    // Act
    let error = store
        .check_body_eligibility_detailed(&block, params, 2_000_000_000)
        .expect_err("unadmitted body header");

    // Assert
    assert!(
        matches!(error, BodyEligibilityError::Network(NetworkError::InvalidHeader { reject_reason, .. })
        if reject_reason == "unproved-canonical-body-header")
    );
    assert_eq!(store, before);
    assert!(store.maybe_body_eligibility(hash).is_none());
}

fn mined(parent: &BlockHeader, time: u32, version: i32) -> BlockHeader {
    let mut header = BlockHeader {
        version,
        previous_block_hash: block_hash(parent),
        time,
        bits: parent.bits,
        ..BlockHeader::default()
    };
    while !check_proof_of_work(block_hash(&header).to_byte_array(), header.bits)
        .expect("valid bits")
    {
        header.nonce += 1;
    }
    header
}

#[test]
fn phase167_header_history_arithmetic_api_cannot_extend_canonical_root() {
    // Arrange
    let mut store = root();
    let parent = store.best_tip().expect("root").header.clone();
    let header = mined(&parent, parent.time + 1, 1);
    let before = store.clone();
    let params = *store.maybe_consensus_params.as_ref().expect("selected");
    // Act
    let result = store.insert_header_with_params(header, &params);
    // Assert
    assert!(
        result.is_err(),
        "arithmetic API has no contextual clock or version authority"
    );
    assert_eq!(store, before);
}

#[test]
fn phase167_header_history_truncated_mtp_is_refused() {
    // Arrange
    let header = BlockHeader::default();
    let hash = block_hash(&header);
    let mut store = HeaderStore::default();
    store.entries.insert(
        hash,
        HeaderEntry {
            block_hash: hash,
            header,
            height: 8,
            chain_work: open_bitcoin_consensus::chain_proof::ProofWork::ZERO,
        },
    );
    store.parents.insert(hash, None);
    // Act / Assert
    assert_eq!(store.median_time_past(hash), None);
}

fn append(store: &mut HeaderStore, parent: &BlockHeader, time: u32) -> BlockHeader {
    let header = mined(parent, time, 4);
    store
        .insert_canonical_header(header.clone(), i64::from(time))
        .expect("admitted descendant");
    header
}

#[test]
fn phase167_header_history_earned_descendants_and_body_clock() {
    // Arrange
    let mut store = root();
    let genesis = store.best_tip().expect("root").header.clone();
    let header = mined(&genesis, genesis.time + 1, 4);
    let now = i64::from(header.time) - 7_200;
    // Act
    let inserted = store
        .insert_canonical_header(header.clone(), now)
        .expect("future equality");
    let history = store
        .canonical_header_history(inserted.block_hash)
        .expect("earned context");
    let context = store
        .canonical_header_validation_context(&header)
        .expect("admission evidence");
    // Assert
    assert_eq!(history.height(), 1);
    assert_eq!(history.parent().block_hash, block_hash(&genesis));
    assert_eq!(context.current_time, now);
    assert_eq!(context.previous_median_time_past, i64::from(genesis.time));
    assert_eq!(
        inserted.chain_work,
        block_proof(genesis.bits)
            .expect("proof")
            .wrapping_add(block_proof(header.bits).expect("proof"))
    );
    assert!(store.is_canonically_admitted(inserted.block_hash));
}

#[test]
fn phase167_header_history_invalid_version_preserves_exact_store() {
    // Arrange
    let mut store = root();
    let parent = store.best_tip().expect("root").header.clone();
    // Act / Assert
    for version in [i32::MIN, 1, 2, 3] {
        let header = mined(&parent, parent.time + 1, version);
        let before = store.clone();
        assert!(
            store
                .insert_canonical_header(header, i64::from(parent.time))
                .is_err()
        );
        assert_eq!(store, before);
    }
}

#[test]
fn phase167_header_history_arithmetic_and_imports_never_mint_lineage() {
    // Arrange
    let canonical = root();
    let root_entry = canonical.best_tip().expect("root").clone();
    let mut arithmetic = HeaderStore::default();
    arithmetic
        .insert_header(root_entry.header.clone())
        .expect("arithmetic root");
    let mut imported = HeaderStore::from_entries([root_entry.clone()]).expect("import");
    let header = mined(&root_entry.header, root_entry.header.time + 1, 4);
    // Act / Assert
    for store in [&mut arithmetic, &mut imported] {
        let before = store.clone();
        assert!(
            store
                .insert_canonical_header(header.clone(), i64::from(header.time))
                .is_err()
        );
        assert!(!store.is_canonically_admitted(root_entry.block_hash));
        assert!(
            store
                .canonical_header_history(root_entry.block_hash)
                .is_err()
        );
        assert_eq!(*store, before);
    }
    assert!(
        arithmetic
            .maybe_proof_facts(root_entry.block_hash)
            .is_some()
    );
    assert!(imported.maybe_proof_facts(root_entry.block_hash).is_none());
}

#[test]
fn phase167_header_history_raw_position_parent_never_mints_lineage() {
    // Arrange
    let mut store = root();
    let genesis = store.best_tip().expect("root").header.clone();
    let raw = mined(&genesis, genesis.time + 1, 4);
    let raw_hash = block_hash(&raw);
    store.record_position(&ChainPosition::new(
        raw.clone(),
        1,
        block_proof(raw.bits).expect("proof"),
        i64::from(raw.time),
    ));
    let child = mined(&raw, raw.time + 1, 4);
    let before = store.clone();
    // Act
    let result = store.insert_canonical_header(child, i64::from(raw.time));
    // Assert
    assert!(result.is_err());
    assert_eq!(store, before);
    assert!(!store.is_canonically_admitted(raw_hash));
}

#[test]
fn phase167_header_history_explicit_readmission_recomputes_work() {
    // Arrange
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("root");
    let header = mined(proof.header(), proof.header().time + 1, 4);
    let raw = HeaderEntry {
        block_hash: BlockHash::from_byte_array([99; 32]),
        header: header.clone(),
        height: 99,
        chain_work: open_bitcoin_consensus::chain_proof::ProofWork::from_le_bytes([255; 32]),
    };
    // Act
    let store = HeaderStore::readmit_canonical_entries(&proof, [raw], i64::from(header.time))
        .expect("explicit re-admission");
    // Assert
    let tip = store.best_tip().expect("tip");
    assert_eq!(tip.height, 1);
    assert_eq!(tip.block_hash, block_hash(&header));
    assert_eq!(
        tip.chain_work,
        proof
            .work()
            .wrapping_add(block_proof(header.bits).expect("proof"))
    );
    assert!(store.is_canonically_admitted(tip.block_hash));
}

#[test]
fn phase167_header_history_branch_local_mtp_and_first_arrival_tie() {
    // Arrange
    let mut store = root();
    let genesis = store.best_tip().expect("root").header.clone();
    let a = append(&mut store, &genesis, genesis.time + 100);
    let b = append(&mut store, &genesis, genesis.time + 10);
    assert_eq!(
        store.best_tip().expect("tie tip").block_hash,
        block_hash(&a)
    );
    let b2 = append(&mut store, &b, genesis.time + 11);
    // Act
    let context = store
        .canonical_header_history(block_hash(&b2))
        .expect("branch context");
    // Assert
    assert_eq!(context.parent().block_hash, block_hash(&b));
    assert_eq!(context.parent_median_time_past(), i64::from(b.time));
    assert_eq!(
        store.best_tip().expect("more work").block_hash,
        block_hash(&b2)
    );
    assert_eq!(
        store.median_time_past(block_hash(&b2)),
        Some(i64::from(b.time))
    );
}

#[test]
fn phase167_header_history_public_network_prerequisites_remain_refused() {
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
    ] {
        // Arrange
        let spec = chain.spec();
        let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("root");
        let mut store = HeaderStore::from_canonical_genesis(&proof).expect("store");
        let mut header = proof.header().clone();
        header.previous_block_hash = proof.hash();
        header.time += 1;
        header.version = 4;
        let before = store.clone();
        // Act
        let result = store.insert_canonical_header(header, i64::from(proof.header().time));
        // Assert
        assert!(result.is_err());
        assert_eq!(store, before);
        assert_eq!(store.maybe_canonical_chain(), Some(chain));
    }
}

#[test]
fn phase167_header_history_suffix_is_earned_row_by_row() {
    // Arrange
    let mut store = root();
    let mut tip = store.best_tip().expect("root").header.clone();
    // Act
    for _ in 0..170 {
        let time = tip.time + 1;
        tip = append(&mut store, &tip, time);
    }
    let history = store
        .canonical_header_history(block_hash(&tip))
        .expect("owned suffix");
    // Assert
    assert!(matches!(
        history.origin(),
        open_bitcoin_consensus::HistoryOrigin::CheckedSuffix { .. }
    ));
    assert_eq!(history.branch().len(), 155);
    assert!(
        history
            .branch()
            .iter()
            .all(|row| store.is_canonically_admitted(row.block_hash))
    );
}

#[test]
fn phase167_header_rules_expiry_uses_parent_144_lookback_upper_median() {
    // Arrange
    let mut store = root();
    let mut tip = store.best_tip().expect("root").header.clone();
    for height in 1..=145 {
        tip = append(&mut store, &tip, 1_857_151_480 + height);
    }
    let header = mined(&tip, tip.time + 1, 4);
    let before = store.clone();
    // Act
    let result = store.insert_canonical_header(header.clone(), i64::from(header.time));
    // Assert
    assert!(
        matches!(result, Err(NetworkError::InvalidHeader { reject_reason, .. }) if reject_reason == "node-expired")
    );
    assert_eq!(store, before);
}

#[test]
fn phase167_header_rules_mtp_and_future_equality_edges_preserve_rejections() {
    // Arrange
    let original = root();
    let parent = original.best_tip().expect("root").header.clone();
    for (time, now, accepted) in [
        (parent.time, i64::from(parent.time), false),
        (parent.time + 1, i64::from(parent.time), true),
        (parent.time + 7200, i64::from(parent.time), true),
        (parent.time + 7201, i64::from(parent.time), false),
    ] {
        let mut store = original.clone();
        let header = mined(&parent, time, 4);
        // Act
        let result = store.insert_canonical_header(header, now);
        // Assert
        assert_eq!(result.is_ok(), accepted);
        if !accepted {
            assert_eq!(store, original);
        }
    }
}

#[test]
fn phase167_header_rules_invalid_work_bits_and_parent_preserve_exact_store() {
    // Arrange
    let original = root();
    let parent = original.best_tip().expect("root").header.clone();
    let valid = mined(&parent, parent.time + 1, 4);
    let mut high_hash = valid.clone();
    while check_proof_of_work(block_hash(&high_hash).to_byte_array(), high_hash.bits)
        .expect("target")
    {
        high_hash.nonce += 1;
    }
    let mut wrong_bits = valid.clone();
    wrong_bits.bits = 0x206f_ffff;
    while !check_proof_of_work(block_hash(&wrong_bits).to_byte_array(), wrong_bits.bits)
        .expect("target")
    {
        wrong_bits.nonce += 1;
    }
    let mut bad_bits = valid.clone();
    bad_bits.bits = 0;
    let mut foreign_parent = valid.clone();
    foreign_parent.previous_block_hash = BlockHash::from_byte_array([19; 32]);
    // Act / Assert
    for header in [high_hash, wrong_bits, bad_bits, foreign_parent] {
        let mut store = original.clone();
        assert!(
            store
                .insert_canonical_header(header, i64::from(parent.time))
                .is_err()
        );
        assert_eq!(store, original);
    }
}

#[test]
fn phase167_header_rules_body_recheck_requires_held_admission() {
    // Arrange
    let mut store = root();
    let parent = store.best_tip().expect("root").header.clone();
    let header = mined(&parent, parent.time + 1, 4);
    assert!(store.canonical_header_validation_context(&header).is_err());
    let now = i64::from(header.time) - 7200;
    store
        .insert_canonical_header(header.clone(), now)
        .expect("admit");
    let mut changed = header.clone();
    changed.time += 1;
    // Act
    let context = store
        .canonical_header_validation_context(&header)
        .expect("retained clock");
    let changed_result = store.canonical_header_validation_context(&changed);
    // Assert
    assert_eq!(context.current_time, now);
    assert!(changed_result.is_err());
    let imported = HeaderStore::from_entries(store.entries().cloned()).expect("raw copy");
    assert!(
        imported
            .canonical_header_validation_context(&header)
            .is_err()
    );
}

#[test]
fn phase167_header_rules_default_regtest_earned_143_144_145_287_288_289() {
    // Arrange
    let mut store = root();
    let mut tip = store.best_tip().expect("root").header.clone();
    let mut boundaries = Vec::new();
    // Act
    for height in 1..=289 {
        let time = tip.time + if height % 2 == 0 { 1201 } else { 1200 };
        tip = append(&mut store, &tip, time);
        if [143, 144, 145, 287, 288, 289].contains(&height) {
            boundaries.push((
                height,
                store
                    .canonical_header_history(block_hash(&tip))
                    .expect("earned"),
            ));
        }
    }
    // Assert
    assert_eq!(boundaries.len(), 6);
    for (height, history) in boundaries {
        assert_eq!(history.height(), height);
        assert_eq!(history.candidate().bits, 0x207f_ffff);
        assert_eq!(
            history
                .rules()
                .network_rules()
                .difficulty_adjustment_interval(),
            144
        );
        assert!(
            history
                .branch()
                .iter()
                .all(|row| store.is_canonically_admitted(row.block_hash))
        );
    }
}

#[test]
fn phase167_header_rules_missing_owned_window_or_link_never_accepts() {
    // Arrange
    let mut complete = root();
    let mut tip = complete.best_tip().expect("root").header.clone();
    for _ in 0..160 {
        let time = tip.time + 1;
        tip = append(&mut complete, &tip, time);
    }
    let header = mined(&tip, tip.time + 1, 4);
    let ancestor = complete
        .ancestor_at_height(block_hash(&tip), 15)
        .expect("window")
        .block_hash;
    let mut missing = complete.clone();
    missing.entries.remove(&ancestor);
    let mut wrong_height = complete.clone();
    wrong_height
        .entries
        .get_mut(&ancestor)
        .expect("entry")
        .height += 1;
    let mut unsealed = complete.clone();
    unsealed
        .maybe_canonical_history
        .as_mut()
        .expect("lineage")
        .admitted
        .remove(&ancestor);
    // Act / Assert
    for mut store in [missing, wrong_height, unsealed] {
        let before = store.clone();
        assert!(
            store
                .insert_canonical_header(header.clone(), i64::from(header.time))
                .is_err()
        );
        assert_eq!(store, before);
    }
}

#[test]
fn phase167_header_history_fixture_reseed_discards_canonical_membership() {
    // Arrange
    let mut store = root();
    let entry = store.best_tip().expect("root").clone();
    let position = ChainPosition::new(entry.header, entry.height, entry.chain_work, 0);
    // Act
    store.seed_from_chain(&[position]);
    // Assert
    assert_eq!(store.maybe_canonical_chain(), None);
    assert!(!store.is_canonically_admitted(entry.block_hash));
    assert!(store.maybe_proof_facts(entry.block_hash).is_none());
}

#[test]
fn phase167_header_rules_duplicate_admission_retains_original_clock() {
    // Arrange
    let mut store = root();
    let genesis = store.best_tip().expect("root").header.clone();
    let header = append(&mut store, &genesis, genesis.time + 1);
    let before = store.clone();
    // Act
    store
        .insert_canonical_header(header.clone(), 0)
        .expect("already admitted");
    let context = store
        .canonical_header_validation_context(&header)
        .expect("held evidence");
    // Assert
    assert_eq!(store, before);
    assert_eq!(context.current_time, i64::from(header.time));
}

#[test]
fn phase167_header_history_mtp_includes_tip_and_only_ten_parents() {
    // Arrange
    let mut store = root();
    let mut tip = store.best_tip().expect("root").header.clone();
    let genesis_time = tip.time;
    // Act / Assert
    for height in 0_u32..=12 {
        if height > 0 {
            tip = append(&mut store, &tip, genesis_time + height);
        }
        let expected_height = height.saturating_sub(10) + height.min(10).div_ceil(2);
        assert_eq!(
            store.median_time_past(block_hash(&tip)),
            Some(i64::from(genesis_time + expected_height))
        );
    }
}

#[test]
fn detailed_header_admission_keeps_typed_refusal_then_earns_linked_child() {
    // Arrange
    let mut store = root();
    let parent = store.best_tip().expect("proved root").header.clone();
    let before = store.clone();
    let invalid = mined(&parent, parent.time + 1, 1);
    // Act
    let error = store
        .insert_canonical_header_detailed(invalid, i64::from(parent.time) + 1)
        .expect_err("version1 cannot earn contextual admission");
    // Assert
    assert!(matches!(
        error,
        CanonicalHeaderAdmissionError::Validation { .. }
    ));
    assert_eq!(store, before);
    let valid = mined(&parent, parent.time + 2, 4);
    let inserted = store
        .insert_canonical_header_detailed(valid, i64::from(parent.time) + 2)
        .expect("linked contextual admission");
    assert!(store.is_canonically_admitted(inserted.block_hash));
}

#[test]
fn readmit_duplicate_proved_genesis_then_child_earns_only_replayed_membership() {
    // Arrange
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proved root");
    let mut source = HeaderStore::from_canonical_genesis(&proof).expect("source root");
    let genesis = source.best_tip().expect("root entry").clone();
    let child = mined(&genesis.header, genesis.header.time + 1, 4);
    let inserted = source
        .insert_canonical_header(child.clone(), i64::from(child.time))
        .expect("earned child");
    let child_entry = source
        .entry(&inserted.block_hash)
        .expect("child entry")
        .clone();
    let imported = HeaderStore::from_entries([genesis.clone(), child_entry.clone()])
        .expect("arithmetic import");
    assert!(!imported.is_canonically_admitted(child_entry.block_hash));
    // Act
    let replay = HeaderStore::readmit_canonical_entries(
        &proof,
        [genesis, child_entry],
        i64::from(child.time),
    )
    .expect("checked parent-first replay");
    // Assert
    assert!(replay.is_canonically_admitted(proof.hash()));
    assert!(replay.is_canonically_admitted(inserted.block_hash));
    assert_eq!(
        replay.best_tip().expect("replayed tip").block_hash,
        inserted.block_hash
    );
}
