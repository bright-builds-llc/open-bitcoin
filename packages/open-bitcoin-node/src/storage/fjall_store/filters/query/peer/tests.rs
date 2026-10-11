// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::*;
use crate::storage::fjall_store::filters::tests::query::recovered_fixture;

#[test]
fn phase161_read_fault_callers_peer_policy_and_budget_errors_never_latch() {
    // Arrange
    let (path, store, records) = recovered_fixture("peer-read-fault-negative", 4);
    let hash = records[3].identity().block_hash();
    let expected = store.basic_filter_retention().expect("complete");
    // Act / Assert
    for refusal in 0..3 {
        let result = store.with_basic_filter_query(|_| match refusal {
            0 => check_envelope(BASIC_FILTER_QUERY_MAX_RECORD_BYTES + 1),
            1 => checked_response_hex_bytes(usize::MAX).map(|_| ()),
            _ => charge_hash_work(&mut BasicFilterQueryWork::default(), usize::MAX, true),
        });
        assert!(matches!(result, Err(StorageError::Corruption { .. })));
        assert!(
            store
                .basic_filter_operator_observation()
                .expect("policy callback")
                .maybe_read_fault
                .is_none()
        );
    }
    for axis in [
        Axis::RecordReads,
        Axis::MetadataReads,
        Axis::ValidatedEnvelopeBytes,
        Axis::HashPaddedBytes,
        Axis::HeaderOperations,
        Axis::LogicalCopyBytes,
        Axis::EncodedSuffixBytes,
    ] {
        let mut budget = BasicPeerServeBudget::default().with_limit(axis, 0);
        assert!(
            store
                .with_basic_peer_query(&mut budget, |reader, meter| reader
                    .maybe_peer_record(hash, meter, true))
                .is_err()
        );
        assert!(
            store
                .basic_filter_operator_observation()
                .expect("capture")
                .maybe_read_fault
                .is_none()
        );
        assert_eq!(store.basic_filter_retention(), Ok(expected));
    }
    let mut budget = BasicPeerServeBudget::default();
    assert!(
        store
            .with_basic_peer_query(&mut budget, |reader, meter| reader.maybe_peer_record(
                BlockHash::from_byte_array([99; 32]),
                meter,
                true
            ))
            .expect("unknown")
            .is_none()
    );
    let wrong = [open_bitcoin_network::policy::FilterBranchIdentity {
        hash,
        height: 1,
        parent: Default::default(),
    }];
    assert!(matches!(
        store.with_basic_peer_query(&mut BasicPeerServeBudget::default(), |reader, meter| reader
            .maybe_peer_records(&wrong, meter, true)),
        Err(BasicPeerReadError::Storage(StorageError::Corruption { .. }))
    ));
    assert!(
        store
            .basic_filter_operator_observation()
            .expect("capture")
            .maybe_read_fault
            .is_none()
    );
    let key = codec::record_key(hash);
    let original = store.block_index.get(&key).expect("get").expect("row");
    store
        .block_index
        .insert(&key, vec![0; BASIC_FILTER_QUERY_MAX_RECORD_BYTES + 1])
        .expect("oversize under earned epoch");
    assert!(store.maybe_basic_filter_point_query(hash).is_err());
    assert!(
        store
            .with_basic_peer_query(&mut BasicPeerServeBudget::default(), |reader, meter| reader
                .maybe_peer_record(hash, meter, true))
            .is_err()
    );
    assert!(
        store
            .basic_filter_operator_observation()
            .expect("capture")
            .maybe_read_fault
            .is_none()
    );
    store
        .block_index
        .insert(key, original)
        .expect("restore policy fixture");
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_budgeted_read_disabled_and_integrity_epoch_are_distinct() {
    // Arrange
    let path = std::env::temp_dir().join(format!("phase160-peer-disabled-{}", std::process::id()));
    let store = FjallNodeStore::open(&path).expect("store");
    let mut budget = BasicPeerServeBudget::default();
    // Act / Assert
    assert!(matches!(
        store.with_basic_peer_query(&mut budget, |reader, meter| {
            reader.maybe_peer_record(Default::default(), meter, true)
        }),
        Err(BasicPeerReadError::Disabled)
    ));
    assert_eq!(budget.work().get(Axis::RecordReads), 0);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");

    let (path, store, records) = recovered_fixture("peer-epoch", 4);
    let clone = store.clone();
    clone
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            &codec::record_key(records[0].identity().block_hash()),
            codec::encode_record(&records[0]),
        )
        .expect("raw clone write invalidates every reader");
    let mut budget = BasicPeerServeBudget::default();
    assert!(matches!(
        store.with_basic_peer_query(&mut budget, |reader, meter| {
            reader.maybe_peer_record(records[3].identity().block_hash(), meter, true)
        }),
        Err(BasicPeerReadError::Storage(StorageError::Corruption { .. }))
    ));
    assert_eq!(budget.work().get(Axis::RecordReads), 0);
    drop(clone);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_budgeted_read_complete_batch_or_no_prefix() {
    // Arrange
    let (path, store, records) = recovered_fixture("peer-batch", 4);
    let identities: Vec<_> = records
        .iter()
        .map(|record| {
            let id = record.identity();
            open_bitcoin_network::policy::FilterBranchIdentity {
                hash: id.block_hash(),
                parent: id.parent_hash(),
                height: id.height(),
            }
        })
        .collect();
    let mut budget = BasicPeerServeBudget::default();
    // Act / Assert
    let batch = store
        .with_basic_peer_query(&mut budget, |reader, meter| {
            reader.maybe_peer_records(&identities, meter, true)
        })
        .expect("query")
        .expect("complete rows");
    assert_eq!(batch.len(), 4);
    store
        .inject_query_record_fault_for_test(
            records[3].identity().block_hash(),
            QueryRecordFault::MissingTarget,
        )
        .expect("tamper after early successful rows");
    let mut budget = BasicPeerServeBudget::default();
    assert!(
        store
            .with_basic_peer_query(&mut budget, |reader, meter| {
                reader.maybe_peer_records(&identities, meter, true)
            })
            .expect("genuine missing target")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_budgeted_read_exact_and_one_over_hidden_parent_bytes() {
    // Arrange
    let (path, store, records) = recovered_fixture("peer-exact", 4);
    let hash = records[3].identity().block_hash();
    let expected = store
        .maybe_basic_filter_point_query(hash)
        .expect("RPC")
        .expect("row");
    let bytes = expected.work().validated_envelope_bytes as u64;
    // Act / Assert
    for (limit, success) in [(bytes, true), (bytes - 1, false)] {
        let mut budget =
            BasicPeerServeBudget::default().with_limit(Axis::ValidatedEnvelopeBytes, limit);
        let result = store.with_basic_peer_query(&mut budget, |reader, meter| {
            reader.maybe_peer_record(hash, meter, true)
        });
        assert_eq!(result.is_ok(), success);
        if success {
            assert_eq!(result.expect("exact").expect("row"), expected);
        } else {
            assert!(matches!(result, Err(BasicPeerReadError::Resource(_))));
        }
    }
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_budgeted_read_every_charged_axis_boundary_and_identity_mismatch() {
    // Arrange
    let (path, store, records) = recovered_fixture("peer-axes", 4);
    let hash = records[3].identity().block_hash();
    let mut observed = BasicPeerServeBudget::default();
    store
        .with_basic_peer_query(&mut observed, |reader, meter| {
            reader.maybe_peer_record(hash, meter, true)
        })
        .expect("observed")
        .expect("row");
    // Act / Assert
    for axis in [
        Axis::RecordReads,
        Axis::MetadataReads,
        Axis::ValidatedEnvelopeBytes,
        Axis::HashPaddedBytes,
        Axis::HeaderOperations,
        Axis::LogicalCopyBytes,
        Axis::EncodedSuffixBytes,
    ] {
        let exact = observed.work().get(axis);
        for (limit, success) in [(exact, true), (exact - 1, false)] {
            let mut budget = BasicPeerServeBudget::default().with_limit(axis, limit);
            let result = store.with_basic_peer_query(&mut budget, |reader, meter| {
                reader.maybe_peer_record(hash, meter, true)
            });
            assert_eq!(result.is_ok(), success, "{axis:?}");
            if !success {
                assert!(
                    matches!(result, Err(BasicPeerReadError::Resource(BasicPeerResourceRefusal(actual))) if actual == axis)
                );
            }
        }
    }
    let wrong = [open_bitcoin_network::policy::FilterBranchIdentity {
        hash,
        height: 2,
        parent: Default::default(),
    }];
    let mut budget = BasicPeerServeBudget::default();
    assert!(matches!(
        store.with_basic_peer_query(&mut budget, |reader, meter| reader
            .maybe_peer_records(&wrong, meter, true)),
        Err(BasicPeerReadError::Storage(_))
    ));
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_budgeted_read_failure_classes_and_sanitized_backend_fault() {
    for fault in [
        QueryRecordFault::CorruptTarget,
        QueryRecordFault::CorruptParent,
        QueryRecordFault::MissingParent,
    ] {
        // Arrange
        let (path, store, records) = recovered_fixture("peer-corruption", 4);
        let hash = records[3].identity().block_hash();
        store
            .inject_query_record_fault_for_test(hash, fault)
            .expect("fault");
        let mut budget = BasicPeerServeBudget::default();
        // Act / Assert
        let error = store
            .with_basic_peer_query(&mut budget, |reader, meter| {
                reader.maybe_peer_record(hash, meter, true)
            })
            .expect_err("present corrupt row never absence");
        assert!(matches!(
            error,
            BasicPeerReadError::Storage(StorageError::Corruption { .. })
        ));
        assert_eq!(error.to_string(), "BASIC peer storage failure");
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
    let (path, store, records) = recovered_fixture("peer-backend-fault", 4);
    store.set_basic_filter_fault(super::super::super::FilterPublicationFault::BeforeQueryRead);
    let mut budget = BasicPeerServeBudget::default();
    let error = store
        .with_basic_peer_query(&mut budget, |reader, meter| {
            reader.maybe_peer_record(records[3].identity().block_hash(), meter, true)
        })
        .expect_err("backend fault");
    assert!(matches!(
        error,
        BasicPeerReadError::Storage(StorageError::BackendFailure { .. })
    ));
    assert!(!error.to_string().contains("private"));
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_budgeted_read_hidden_large_predecessor_refuses_before_parent_hash() {
    use open_bitcoin_core::{
        codec::block_filter::encode_basic_filter_values,
        consensus::{block_filter::compute_filter_header, crypto::double_sha256},
        primitives::{FilterHash, FilterHeader},
    };
    // Arrange: deliberate valid large parent under an already earned epoch.
    let (path, store, records) = recovered_fixture("peer-hidden-large", 2);
    let encoded = encode_basic_filter_values(&vec![0; 400_000]).expect("large legal encoding");
    let hash = FilterHash::from_byte_array(double_sha256(&encoded));
    let header = compute_filter_header(hash, FilterHeader::default());
    let mut parent = codec::encode_record(&records[0]);
    parent.truncate(codec::RECORD_OVERHEAD);
    parent[102..134].copy_from_slice(hash.as_bytes());
    parent[134..166].copy_from_slice(header.as_bytes());
    parent[166..170].copy_from_slice(&(encoded.len() as u32).to_le_bytes());
    parent.extend_from_slice(&encoded);
    let parent_len = parent.len();
    store
        .block_index
        .insert(
            codec::record_key(records[0].identity().block_hash()),
            parent,
        )
        .expect("parent");
    let mut target = codec::encode_record(&records[1]);
    let filter_hash = FilterHash::from_byte_array(target[102..134].try_into().expect("hash"));
    target[70..102].copy_from_slice(header.as_bytes());
    target[134..166].copy_from_slice(compute_filter_header(filter_hash, header).as_bytes());
    let target_len = target.len();
    store
        .block_index
        .insert(
            codec::record_key(records[1].identity().block_hash()),
            target,
        )
        .expect("target");
    let mut budget = BasicPeerServeBudget::default().with_limit(
        Axis::ValidatedEnvelopeBytes,
        (target_len + parent_len - 1) as u64,
    );
    // Act
    let result = store.with_basic_peer_query(&mut budget, |reader, meter| {
        reader.maybe_peer_record(records[1].identity().block_hash(), meter, true)
    });
    // Assert
    assert!(matches!(
        result,
        Err(BasicPeerReadError::Resource(BasicPeerResourceRefusal(
            Axis::ValidatedEnvelopeBytes
        )))
    ));
    assert_eq!(
        budget.work().get(Axis::ValidatedEnvelopeBytes),
        target_len as u64
    );
    assert_eq!(budget.work().get(Axis::HashPaddedBytes), 512);
    assert_eq!(budget.work().get(Axis::RecordReads), 2);
    assert!(
        store
            .maybe_basic_filter_point_query(records[1].identity().block_hash())
            .expect("intact point query")
            .is_some()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
