// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::super::super::ownership::basic_filter_lock_map_cost;
use super::*;
use crate::storage::fjall_store::prune::{PRUNE_LOCKS_KEY, decode_prune_locks, encode_prune_locks};

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
