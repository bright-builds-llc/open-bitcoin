// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::super::super::ownership::basic_filter_lock_map_cost;
use super::*;
use crate::storage::fjall_store::prune::{PRUNE_LOCKS_KEY, decode_prune_locks, encode_prune_locks};

#[test]
fn phase159_append_raw_clone_corruption_revokes_proof_and_genuine_recovery_refuses() {
    // Arrange
    let path = temp_path("append-raw-clone-recovery");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    append(&store, &records[..2]);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let clone = store.clone();
    let key = codec::record_key(records[1].identity().block_hash());
    let mut bytes = codec::encode_record(&records[1]);
    let previous = open_bitcoin_core::primitives::FilterHeader::default();
    bytes[70..102].copy_from_slice(previous.as_bytes());
    let header = open_bitcoin_core::consensus::compute_filter_header(
        records[1].identity().filter_hash(),
        previous,
    );
    bytes[134..166].copy_from_slice(header.as_bytes());
    // Act
    clone
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, bytes.clone())
        .expect("self-consistent row with incompatible parent header");
    // Assert raw invalidation before recovery can itself revoke any authority.
    assert!(
        store
            .check_basic_filter_append_proof(&proof)
            .expect_err("every clone loses the captured proof")
            .to_string()
            .contains("invalidated")
    );
    assert!(
        clone
            .maybe_basic_filter_append_proof_with_budget(budget())
            .expect("no live proof")
            .is_none()
    );
    // Act: only genuine complete recovery may attempt to restore authority.
    let recovery = store.configure_basic_filter_index_before_prune(
        Some(positions.last().expect("tip").block_hash),
        BasicFilterStartupMode::Enabled,
    );
    // Assert
    assert!(
        recovery
            .expect_err("full recovery must reject the edge")
            .to_string()
            .contains("predecessor")
    );
    assert_eq!(
        store
            .get_bytes(StorageNamespace::BlockIndex, &key)
            .expect("corruption remains untouched"),
        Some(bytes)
    );
    assert!(
        store
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[2].identity().block_hash())
            )
            .expect("no new immutable row")
            .is_none()
    );
    drop(clone);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_preparation_reserves_acquisition_plus_recheck_before_map_decode() {
    // Arrange
    let path = temp_path("append-acquisition-total");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let mut maximum = budget();
    maximum.checkpoint_operations = 2000;
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(maximum)
        .expect("one check fits")
        .expect("proof");
    let mut raw = 1_u32.to_le_bytes().to_vec();
    raw.extend_from_slice(&1_u16.to_le_bytes());
    raw.push(255);
    raw.extend_from_slice(&[0; 8]);
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, PRUNE_LOCKS_KEY, raw)
        .expect("malformed map of small admitted size");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records);
    // Assert
    assert!(
        result
            .err()
            .expect("before decoder")
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
fn phase157_append_lock_map_cost_refuses_comparison_overflow() {
    // Arrange / Act
    let result = basic_filter_lock_map_cost(u64::MAX, 4);
    // Assert
    assert!(result.is_err());
}

#[test]
fn phase157_append_lock_map_cost_refuses_allocation_overflow() {
    // Arrange / Act
    let result = basic_filter_lock_map_cost(1, u64::MAX);
    // Assert
    assert!(result.is_err());
}

#[test]
fn phase157_append_lock_decoder_preserves_parse_before_duplicate_error_order() {
    // Arrange
    let mut raw = 3_u32.to_le_bytes().to_vec();
    for name in [b"same".as_slice(), b"same".as_slice(), &[255]] {
        raw.extend_from_slice(&(name.len() as u16).to_le_bytes());
        raw.extend_from_slice(name);
        raw.extend_from_slice(&[0; 8]);
    }
    // Act
    let result = decode_prune_locks(&raw);
    // Assert
    assert!(
        result
            .expect_err("malformed later row")
            .to_string()
            .contains("not utf-8")
    );
}

#[test]
fn phase157_append_lock_codec_still_refuses_duplicate_names() {
    // Arrange
    let lock = PruneLockInfo {
        name: "same".to_owned(),
        height_first: 0,
        height_last: 1,
    };
    let mut raw = 2_u32.to_le_bytes().to_vec();
    let encoded = encode_prune_locks(std::slice::from_ref(&lock)).expect("single valid lock");
    raw.extend_from_slice(&encoded[4..]);
    raw.extend_from_slice(&encoded[4..]);
    // Act
    let encode_result = encode_prune_locks(&[lock.clone(), lock]);
    let decode_result = decode_prune_locks(&raw);
    // Assert
    assert!(
        encode_result
            .expect_err("encode duplicate")
            .to_string()
            .contains("duplicated")
    );
    assert!(
        decode_result
            .expect_err("decode duplicate")
            .to_string()
            .contains("duplicated")
    );
}

#[test]
fn phase157_append_tiny_row_budget_refuses_before_candidate_effect_allocation() {
    // Arrange
    let path = temp_path("append-row-budget");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let mut maximum = budget();
    maximum.encoded_bytes = 1;
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(maximum)
        .expect("bounded metadata")
        .expect("proof");
    let before = store.maybe_basic_filter_state().expect("before");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records);
    // Assert
    assert!(
        result
            .err()
            .expect("refused")
            .to_string()
            .contains("work budget")
    );
    assert_eq!(
        store.maybe_basic_filter_state().expect("safe unchanged"),
        before
    );
    assert_eq!(
        store.basic_filter_artifacts().expect("no rows"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_metadata_reservation_retains_saved_projection_and_lock_map() {
    // Arrange
    let path = temp_path("append-map-refusal-saved");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(2);
    recovered(&store, &positions);
    append(&store, &records[..1]);
    let before = store.maybe_basic_filter_state().expect("before");
    let locks = store
        .get_bytes(StorageNamespace::BlockIndex, PRUNE_LOCKS_KEY)
        .expect("lock bytes");
    let mut maximum = budget();
    maximum.checkpoint_operations = 1;
    // Act
    let result = store.maybe_basic_filter_append_proof_with_budget(maximum);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.maybe_basic_filter_state().expect("safe unchanged"),
        before
    );
    assert_eq!(
        store
            .get_bytes(StorageNamespace::BlockIndex, PRUNE_LOCKS_KEY)
            .expect("same lock map"),
        locks
    );
    assert_eq!(
        store.basic_filter_projection(0).expect("saved projection"),
        records[0].identity().block_hash()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
