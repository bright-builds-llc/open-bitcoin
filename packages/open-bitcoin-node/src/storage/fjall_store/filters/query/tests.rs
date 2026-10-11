// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::super::query::{
    BASIC_FILTER_QUERY_MAX_HEX_BYTES, BASIC_FILTER_QUERY_MAX_RECORD_BYTES, charge_hash_work,
    check_envelope, checked_response_hex_bytes,
};
use super::*;

#[test]
fn phase160_query_epoch_recovery_same_generation_never_revives_token() {
    // Arrange
    let (path, store, records) = recovered_fixture("epoch-recovery", 4);
    let clone = store.clone();
    let before = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    let generation = store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("owner")
        .expect("present")
        .generation();
    let epoch_value = store
        .filter_publication_guard()
        .expect("guard")
        .read_integrity_epoch();
    // Act
    clone
        .recover_basic_filter_index_before_prune(Some(
            records.last().expect("tip").identity().block_hash(),
        ))
        .expect("recover");
    let after = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("new epoch");
    // Assert
    assert_ne!(
        before, after,
        "same generation recovery must not revive old capabilities"
    );
    assert_eq!(
        store
            .filter_publication_guard()
            .expect("guard")
            .read_integrity_epoch(),
        epoch_value + 2
    );
    assert_eq!(
        store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner")
            .expect("present")
            .generation(),
        generation
    );
    drop(clone);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_query_epoch_raw_put_and_delete_invalidate_all_clones() {
    // Arrange
    for delete in [false, true] {
        let (path, store, records) =
            recovered_fixture(if delete { "epoch-delete" } else { "epoch-put" }, 4);
        let clone = store.clone();
        let epoch = store
            .with_basic_filter_query(|reader| reader.integrity_epoch())
            .expect("epoch");
        let key = codec::record_key(records[1].identity().block_hash());
        // Act
        if delete {
            clone
                .remove_bytes(StorageNamespace::BlockIndex, &key, PersistMode::Sync)
                .expect("delete");
        } else {
            clone
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    &key,
                    codec::encode_record(&records[1]),
                )
                .expect("put");
        }
        // Assert
        assert!(
            !store
                .basic_peer_epoch_is_current(&epoch)
                .expect("freshness")
        );
        assert!(
            !clone
                .basic_peer_epoch_is_current(&epoch)
                .expect("clone freshness")
        );
        assert!(
            store
                .with_basic_filter_query(|reader| reader.integrity_epoch())
                .is_err()
        );
        drop(epoch);
        drop(clone);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase160_query_epoch_trusted_append_preserves_capture_and_rpc_record() {
    // Arrange
    let path = temp_path("epoch-append");
    let (positions, records) = fixtures(4);
    let store = FjallNodeStore::open(&path).expect("store");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records[..2]);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    // Act
    publish(&store, &positions, &records[2..]);
    // Assert
    assert!(store.basic_peer_epoch_is_current(&epoch).expect("current"));
    let record = store
        .maybe_basic_filter_point_query(records[3].identity().block_hash())
        .expect("query")
        .expect("record");
    assert_eq!(record.encoded_bytes(), records[3].encoded_bytes());
    drop(epoch);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_query_epoch_failed_disable_invalidates_before_writes() {
    // Arrange
    let (path, store, _) = recovered_fixture("epoch-disable-fault", 4);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    store.set_basic_filter_fault(super::super::FilterPublicationFault::BeforeDisable);
    // Act
    assert!(store.disable_basic_filter_index().is_err());
    // Assert
    assert!(store.basic_peer_epoch_is_current(&epoch).is_err());
    drop(epoch);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_query_epoch_overflow_permanently_denies_authority_before_raw_write() {
    // Arrange
    let (path, store, records) = recovered_fixture("epoch-overflow", 4);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    store
        .filter_publication_guard()
        .expect("guard")
        .exhaust_integrity_epoch_for_test();
    let key = codec::record_key(records[1].identity().block_hash());
    let before = store
        .block_index
        .get(&key)
        .expect("read")
        .expect("row")
        .to_vec();
    // Act
    let error = store
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, vec![0])
        .expect_err("overflow");
    // Assert
    assert!(error.to_string().contains("epoch exhausted"));
    assert_eq!(
        store
            .block_index
            .get(&key)
            .expect("read")
            .expect("row")
            .as_ref(),
        before.as_slice()
    );
    assert!(store.basic_peer_epoch_is_current(&epoch).is_err());
    assert!(
        store
            .with_basic_filter_query(|reader| reader.integrity_epoch())
            .is_err()
    );
    assert!(
        store
            .recover_basic_filter_index_before_prune(Some(records[3].identity().block_hash()))
            .is_err()
    );
    drop(epoch);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_query_epoch_memory_only_validation_and_charged_capture() {
    // Arrange
    let (path, store, _) = recovered_fixture("epoch-memory", 4);
    let mut budget = open_bitcoin_network::basic_peer_resource::BasicPeerServeBudget::default();
    let epoch = store
        .with_basic_peer_query(&mut budget, |reader, meter| {
            reader.capture_basic_peer_epoch(meter)
        })
        .expect("capture");
    let reads = store.basic_filter_point_reads_for_test();
    // Act / Assert
    for _ in 0..100 {
        assert!(store.basic_peer_epoch_is_current(&epoch).expect("current"));
    }
    assert_eq!(reads, store.basic_filter_point_reads_for_test());
    assert_eq!(
        budget
            .work()
            .get(open_bitcoin_network::basic_peer_resource::BasicPeerServeAxis::MetadataReads),
        2
    );
    let mut refused = open_bitcoin_network::basic_peer_resource::BasicPeerServeBudget::default()
        .with_limit(
            open_bitcoin_network::basic_peer_resource::BasicPeerServeAxis::MetadataReads,
            1,
        );
    assert!(
        store
            .with_basic_peer_query(&mut refused, |reader, meter| reader
                .capture_basic_peer_epoch(meter))
            .is_err()
    );
    drop(epoch);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_query_epoch_disabled_and_foreign_store_never_grant_read_authority() {
    // Arrange
    let (path, store, _) = recovered_fixture("epoch-disabled", 4);
    let (foreign_path, foreign, _) = recovered_fixture("epoch-foreign", 4);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    // Act / Assert
    assert!(
        !foreign
            .basic_peer_epoch_is_current(&epoch)
            .expect("foreign")
    );
    store.disable_basic_filter_index().expect("disable");
    assert!(
        !store
            .basic_peer_epoch_is_current(&epoch)
            .expect("old epoch")
    );
    assert!(
        store
            .with_basic_filter_query(|reader| reader.integrity_epoch())
            .is_err()
    );
    drop(epoch);
    drop(store);
    drop(foreign);
    std::fs::remove_dir_all(path).expect("cleanup");
    std::fs::remove_dir_all(foreign_path).expect("cleanup");
}

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

pub(crate) fn recovered_fixture(
    name: &str,
    count: u32,
) -> (PathBuf, FjallNodeStore, Vec<StoredFilterRecord>) {
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
    let mut peer_budget =
        open_bitcoin_network::basic_peer_resource::BasicPeerServeBudget::default();
    assert!(matches!(
        store.with_basic_peer_query(&mut peer_budget, |reader, meter| {
            reader.maybe_peer_record(records[1].identity().block_hash(), meter, true)
        }),
        Err(super::super::query::peer::BasicPeerReadError::WirePayload)
    ));
    assert_eq!(
        peer_budget
            .work()
            .get(open_bitcoin_network::basic_peer_resource::BasicPeerServeAxis::HashPaddedBytes),
        0
    );
    assert_eq!(
        store
            .block_index
            .get(&key)
            .expect("unchanged stored row")
            .expect("present")
            .len(),
        codec::RECORD_OVERHEAD + encoded.len()
    );
    eprintln!(
        "phase159 QUERY codec_capacity elapsed_us={} work={:?}",
        start.elapsed().as_micros(),
        view.work()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
