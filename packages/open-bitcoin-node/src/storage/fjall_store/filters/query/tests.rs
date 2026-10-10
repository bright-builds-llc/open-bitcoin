// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::super::query::{
    BASIC_FILTER_QUERY_MAX_HEX_BYTES, BASIC_FILTER_QUERY_MAX_RECORD_BYTES, charge_hash_work,
    check_envelope, checked_response_hex_bytes,
};
use super::*;

#[test]
fn phase159_basic_point_query_requires_recovered_integrity() {
    // Arrange
    let path = std::env::temp_dir().join(format!("phase159-point-red-{}", std::process::id()));
    let store = FjallNodeStore::open(&path).expect("store");
    // Act
    let result = store.maybe_basic_filter_point_query(Default::default());
    // Assert
    assert!(result.is_err());
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn recovered_fixture(name: &str, count: u32) -> (PathBuf, FjallNodeStore, Vec<StoredFilterRecord>) {
    let path = temp_path(name);
    let (positions, records) = fixtures(count);
    let store = FjallNodeStore::open(&path).expect("store");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    for chunk in records.chunks(128) {
        publish(&store, &positions, chunk);
    }
    store
        .recover_basic_filter_index_before_prune(Some(positions.last().expect("tip").block_hash))
        .expect("recovery");
    (path, store, records)
}

#[test]
fn phase159_basic_point_query_fixed_target_parent_reads_and_absence() {
    // Arrange
    let (path, store, records) = recovered_fixture("query-points", 401);
    for height in [0, 20, 400] {
        let before = store.basic_filter_point_reads_for_test();
        // Act
        let view = store
            .maybe_basic_filter_point_query(records[height].identity().block_hash())
            .expect("query")
            .expect("found");
        // Assert
        assert_eq!(view.identity(), records[height].identity());
        assert_eq!(view.encoded_bytes(), records[height].encoded_bytes());
        assert_eq!(view.work().record_reads, if height == 0 { 1 } else { 2 });
        assert_eq!(
            store.basic_filter_point_reads_for_test() - before,
            view.work().record_reads
        );
    }
    assert!(
        store
            .maybe_basic_filter_point_query(BlockHash::from_byte_array([42; 32]))
            .expect("genuine absence")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase159_basic_point_query_raw_clone_ancestor_mutation_invalidates_every_reader() {
    // Arrange
    let (path, store, records) = recovered_fixture("query-clone", 4);
    let clone = store.clone();
    let key = codec::record_key(records[0].identity().block_hash());
    let mut bytes = codec::encode_record(&records[0]);
    bytes[codec::RECORD_OVERHEAD] ^= 1;
    // Act
    clone
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, bytes)
        .expect("raw mutation");
    // Assert
    assert!(
        store
            .maybe_basic_filter_point_query(records[3].identity().block_hash())
            .expect_err("no stale epoch")
            .to_string()
            .contains("read integrity")
    );
    assert!(
        clone
            .maybe_basic_filter_point_query(records[3].identity().block_hash())
            .is_err()
    );
    drop(clone);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert!(
        reopened
            .recover_basic_filter_index_before_prune(Some(records[3].identity().block_hash()))
            .is_err()
    );
    assert!(
        reopened
            .maybe_basic_filter_point_query(records[3].identity().block_hash())
            .is_err()
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase159_basic_point_query_target_parent_corruption_and_missing_parent_fail_closed() {
    for (name, height, maybe_offset) in [
        ("bad-target", 3, Some(134)),
        ("bad-parent", 2, Some(102)),
        ("missing-parent", 2, None),
    ] {
        // Arrange
        let (path, store, records) = recovered_fixture(name, 4);
        let key = codec::record_key(records[height].identity().block_hash());
        // Deliberate private backend corruption bypasses raw invalidation to prove
        // both requested envelopes receive full validation even under a live epoch.
        if let Some(offset) = maybe_offset {
            let mut bytes = codec::encode_record(&records[height]);
            bytes[offset] ^= 1;
            store.block_index.insert(key, bytes).expect("fixture");
        } else {
            store.block_index.remove(key).expect("fixture");
        }
        // Act / Assert
        assert!(
            store
                .maybe_basic_filter_point_query(records[3].identity().block_hash())
                .is_err()
        );
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase159_basic_query_measure_envelope_and_hex_exact_bound_and_overflow() {
    // Arrange / Act / Assert
    assert!(check_envelope(BASIC_FILTER_QUERY_MAX_RECORD_BYTES).is_ok());
    assert!(check_envelope(BASIC_FILTER_QUERY_MAX_RECORD_BYTES + 1).is_err());
    assert_eq!(
        checked_response_hex_bytes(open_bitcoin_core::codec::MAX_SIZE as usize).expect("boundary"),
        BASIC_FILTER_QUERY_MAX_HEX_BYTES
    );
    assert!(checked_response_hex_bytes(open_bitcoin_core::codec::MAX_SIZE as usize + 1).is_err());
    assert!(checked_response_hex_bytes(usize::MAX).is_err());
    let maximum = open_bitcoin_core::codec::MAX_SIZE as usize;
    let mut work = crate::BasicFilterQueryWork {
        copied_filter_bytes: maximum,
        ..Default::default()
    };
    charge_hash_work(&mut work, maximum, true).expect("target maximum");
    charge_hash_work(&mut work, maximum, false).expect("parent maximum");
    assert_eq!(work.logical_copy_bytes, 3 * maximum + 352);
    assert_eq!(work.hash_padded_bytes, 2 * maximum + 832);
    assert!(charge_hash_work(&mut work, 1, false).is_err());
}

#[test]
fn phase159_basic_query_measure_codec_maximum_singleton_is_admitted() {
    use open_bitcoin_core::{
        codec::{MAX_SIZE, block_filter::encode_basic_filter_values},
        consensus::{block_filter::compute_filter_header, crypto::double_sha256},
        primitives::FilterHash,
    };
    // Arrange: exercise the full admitted codec capacity, independently of the
    // genuine large-block generator fixture in the authority measurement test.
    let (path, store, records) = recovered_fixture("query-codec-maximum", 2);
    let count = ((MAX_SIZE - 5) * 8 / 20) as usize;
    let mut mapped = vec![0_u64; count];
    let extra_bits = ((MAX_SIZE - 5) * 8 - count as u64 * 20) as u32;
    *mapped.last_mut().expect("nonempty") = u64::from(extra_bits) << 19;
    let encoded = encode_basic_filter_values(&mapped).expect("maximum codec output");
    drop(mapped);
    assert_eq!(encoded.len(), MAX_SIZE as usize);
    let hash = FilterHash::from_byte_array(double_sha256(&encoded));
    let header = compute_filter_header(hash, records[0].identity().filter_header());
    let mut envelope = codec::encode_record(&records[1]);
    envelope.truncate(codec::RECORD_OVERHEAD);
    envelope[102..134].copy_from_slice(hash.as_bytes());
    envelope[134..166].copy_from_slice(header.as_bytes());
    envelope[166..170].copy_from_slice(&(encoded.len() as u32).to_le_bytes());
    envelope.extend_from_slice(&encoded);
    let key = codec::record_key(records[1].identity().block_hash());
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, envelope)
        .expect("capacity fixture");
    store
        .recover_basic_filter_index_before_prune(Some(records[1].identity().block_hash()))
        .expect("complete forest recovery");
    let start = std::time::Instant::now();
    // Act
    let view = store
        .maybe_basic_filter_point_query(records[1].identity().block_hash())
        .expect("capacity query")
        .expect("found");
    // Assert
    assert_eq!(view.encoded_bytes(), encoded);
    assert_eq!(view.work().copied_filter_bytes, MAX_SIZE as usize);
    assert_eq!(
        view.work().response_hex_bytes,
        BASIC_FILTER_QUERY_MAX_HEX_BYTES
    );
    assert_eq!(view.work().record_reads, 2);
    eprintln!(
        "phase159 QUERY codec_capacity elapsed_us={} work={:?}",
        start.elapsed().as_micros(),
        view.work()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
