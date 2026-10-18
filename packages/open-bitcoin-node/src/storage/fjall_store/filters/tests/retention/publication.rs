// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::super::super::FilterPublicationFault;
use super::*;
use crate::chainstate::ReorgFixture;

fn prepared_append(
    store: &FjallNodeStore,
    records: &[StoredFilterRecord],
) -> super::super::super::PreparedBasicFilterAppend {
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("budgeted proof")
        .expect("recovered authority");
    store
        .prepare_basic_filter_append(proof, records)
        .expect("prepare")
}

fn empty_index(
    name: &str,
) -> (
    PathBuf,
    FjallNodeStore,
    Vec<ChainPosition>,
    Vec<StoredFilterRecord>,
) {
    let path = temp_path(name);
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, records) = fixtures(3);
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("initialize");
    for position in &positions {
        store
            .save_block(
                &fixture_block(position.previous_block_hash(), position.height),
                PersistMode::Sync,
            )
            .expect("body");
        if position.height != 0 {
            store
                .save_undo(
                    position.block_hash,
                    &BlockUndo::default(),
                    PersistMode::Sync,
                )
                .expect("undo");
        }
    }
    store
        .enable_basic_filter_index(&fence(&positions))
        .expect("exclusive append authority");
    (path, store, positions, records)
}

#[test]
fn phase161_retention_publication_success_grows_once() {
    // Arrange
    let (path, store, _, _) = retained_store("retention-publication-growth", 3);
    // Act
    let measured = store.basic_filter_retention();
    // Assert
    assert_eq!(measured, Ok(observed_rows(&store)));
    assert_eq!(measured.expect("complete").retained_record_count, 3);
    cleanup(path, store);
}

#[test]
fn phase161_retention_publication_identical_retry_preserves_totals() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-publication-retry", 3);
    recover(&store, &positions);
    let expected = observed_rows(&store);
    // Act
    append_current(&store, &records).expect("identical retry");
    publish(&store, &positions, &records);
    // Assert
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    cleanup(path, store);
}

#[test]
fn phase161_retention_publication_identical_batch_duplicates_count_once() {
    // Arrange
    let path = temp_path("retention-publication-duplicates");
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, records) = fixtures(1);
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("initialize");
    // Act
    append_current(&store, &[records[0].clone(), records[0].clone()]).expect("duplicate batch");
    // Assert
    assert_eq!(store.basic_filter_retention(), Ok(observed_rows(&store)));
    assert_eq!(observed_rows(&store).retained_record_count, 1);
    cleanup(path, store);
}

fn conflicting_genesis(record: &StoredFilterRecord) -> StoredFilterRecord {
    let key = codec::record_key(record.identity().block_hash());
    let mut bytes = codec::encode_record(record);
    let hash = open_bitcoin_core::primitives::FilterHash::from_byte_array(
        open_bitcoin_core::consensus::crypto::double_sha256(&[0]),
    );
    let header = open_bitcoin_core::consensus::compute_filter_header(hash, Default::default());
    bytes.truncate(codec::RECORD_OVERHEAD);
    bytes[102..134].copy_from_slice(hash.as_bytes());
    bytes[134..166].copy_from_slice(header.as_bytes());
    bytes[166..170].copy_from_slice(&1_u32.to_le_bytes());
    bytes.push(0);
    codec::decode_record(&key, &bytes, None).expect("self-consistent conflicting identity")
}

#[test]
fn phase161_retention_publication_conflicting_batch_duplicates_have_no_effects() {
    // Arrange
    let path = temp_path("retention-publication-conflict");
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, records) = fixtures(1);
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("initialize");
    let before = store.basic_filter_retention();
    let conflicting = conflicting_genesis(&records[0]);
    // Act
    let result = append_current(&store, &[records[0].clone(), conflicting]);
    // Assert
    assert!(result.is_err());
    assert_eq!(observed_rows(&store).retained_record_count, 0);
    assert_eq!(store.basic_filter_retention(), before);
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    cleanup(path, store);
}

#[test]
fn phase161_retention_publication_primary_append_and_retry_have_exact_totals() {
    // Arrange
    let (path, store, _, records) = empty_index("retention-primary");
    // Act
    let achieved = store
        .complete_basic_filter_append(prepared_append(&store, &records))
        .expect("append");
    let expected = observed_rows(&store);
    let reads = store.basic_filter_point_reads_for_test();
    for _ in 0..64 {
        assert_eq!(store.basic_filter_retention(), Ok(expected));
    }
    assert_eq!(store.basic_filter_point_reads_for_test(), reads);
    let retried = store
        .complete_basic_filter_append(prepared_append(&store, &records))
        .expect("retry");
    // Assert
    assert_eq!(expected.retained_record_count, 3);
    assert_eq!(store.basic_filter_retention(), Ok(expected));
    assert_eq!(retried.batch_bytes, 0);
    assert_eq!(retried.safe_checkpoint, achieved.safe_checkpoint);
    cleanup(path, store);
}

fn publication_faults(publisher: u8, faults: &[FilterPublicationFault]) {
    for &fault in faults {
        // Arrange
        let (path, store, positions, records) = empty_index("retention-publication-fault");
        let prepared = (publisher == 0).then(|| prepared_append(&store, &records));
        store.set_basic_filter_fault(fault);
        // Act
        let result = match publisher {
            0 => store
                .complete_basic_filter_append(prepared.expect("prepared"))
                .map(|_| ()),
            1 => append_current(&store, &records),
            _ => publish_current(
                &store,
                &fence(&positions),
                checkpoint(&records[2]),
                checkpoint(&records[2]).input_protection(),
                &records,
            ),
        };
        // Assert
        assert!(result.is_err(), "fault {fault:?} must fire");
        let expected_count = if fault == FilterPublicationFault::AfterCommit {
            3
        } else {
            0
        };
        let after = observed_rows(&store);
        assert_eq!(after.retained_record_count, expected_count);
        let reads = store.basic_filter_point_reads_for_test();
        for _ in 0..64 {
            assert_eq!(
                store.basic_filter_retention(),
                Err(BasicIndexUnavailableReason::RetentionFailed)
            );
        }
        assert_eq!(store.basic_filter_point_reads_for_test(), reads);
        assert_eq!(observed_rows(&store), after);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("all-handle reopen");
        assert!(reopened.basic_filter_retention().is_err());
        recover(&reopened, &positions);
        assert_eq!(reopened.basic_filter_retention(), Ok(after));
        cleanup(path, reopened);
    }
}

#[test]
fn phase161_retention_publication_primary_faults_require_reopen() {
    publication_faults(
        0,
        &[
            FilterPublicationFault::BeforeRecords,
            FilterPublicationFault::BeforeCheckpoint,
            FilterPublicationFault::BeforeProtection,
            FilterPublicationFault::BeforeCommit,
            FilterPublicationFault::AfterCommit,
        ],
    );
}

#[test]
fn phase161_retention_publication_record_only_faults_require_reopen() {
    publication_faults(
        1,
        &[
            FilterPublicationFault::BeforeRecords,
            FilterPublicationFault::BeforeCommit,
            FilterPublicationFault::AfterCommit,
        ],
    );
}

#[test]
fn phase161_retention_publication_compatibility_checkpoint_faults_require_reopen() {
    publication_faults(
        2,
        &[
            FilterPublicationFault::BeforeRecords,
            FilterPublicationFault::BeforeCheckpoint,
            FilterPublicationFault::BeforeProtection,
            FilterPublicationFault::BeforeCommit,
            FilterPublicationFault::AfterCommit,
        ],
    );
}

#[test]
fn phase161_retention_publication_unknown_totals_never_rebuild_from_delta() {
    for unavailable in [
        RetainedBasicFilterObservation::Incomplete,
        RetainedBasicFilterObservation::Failed,
        RetainedBasicFilterObservation::BudgetExceeded,
    ] {
        for primary in [true, false] {
            // Arrange
            let (path, store, _, records) = empty_index("retention-unknown-delta");
            store.filter_publication_guard().expect("control").retention = unavailable;
            // Act
            if primary {
                store
                    .complete_basic_filter_append(prepared_append(&store, &records))
                    .expect("append");
            } else {
                append_current(&store, &records).expect("record-only append");
            }
            // Assert
            assert_eq!(store.basic_filter_retention(), unavailable.measurement());
            assert_eq!(observed_rows(&store).retained_record_count, 3);
            cleanup(path, store);
        }
    }
}

#[test]
fn phase161_retention_publication_overflow_refuses_before_all_effects() {
    for field in 0..3 {
        for primary in [true, false] {
            // Arrange
            let (path, store, _, records) = empty_index("retention-overflow");
            let prepared = primary.then(|| prepared_append(&store, &records));
            let mut total = store.basic_filter_retention().expect("complete empty");
            match field {
                0 => total.retained_record_count = u64::MAX,
                1 => total.encoded_filter_payload_bytes = u64::MAX,
                _ => total.retained_record_key_value_bytes = u64::MAX,
            }
            store.filter_publication_guard().expect("control").retention =
                RetainedBasicFilterObservation::Complete(total);
            let revision = store.filter_publication_guard().expect("control").revision;
            let locks = store.load_prune_locks().expect("locks");
            // Act
            let result = if primary {
                store
                    .complete_basic_filter_append(prepared.expect("prepared"))
                    .map(|_| ())
            } else {
                append_current(&store, &records)
            };
            // Assert
            assert!(
                result
                    .expect_err("checked overflow")
                    .to_string()
                    .contains("retention arithmetic overflow")
            );
            assert_eq!(
                store.basic_filter_retention(),
                Err(BasicIndexUnavailableReason::RetentionFailed)
            );
            assert_eq!(observed_rows(&store).retained_record_count, 0);
            assert_eq!(
                store.filter_publication_guard().expect("control").revision,
                revision
            );
            assert_eq!(store.load_prune_locks().expect("locks"), locks);
            assert_eq!(
                store.maybe_basic_filter_checkpoint().expect("checkpoint"),
                Some(FilterCheckpoint::new(IndexPrefix::Empty))
            );
            cleanup(path, store);
        }
    }
}

#[test]
fn phase161_retention_publication_raw_clone_interleave_revokes_prepared_completion() {
    // Arrange
    let (path, store, positions, records) = empty_index("retention-raw-stale");
    let prepared = prepared_append(&store, &records);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    let clone = store.clone();
    // Act
    clone
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            &codec::record_key(records[0].identity().block_hash()),
            codec::encode_record(&records[0]),
        )
        .expect("raw insert");
    let refused = store.complete_basic_filter_append(prepared);
    // Assert
    assert!(refused.is_err());
    assert!(
        !store
            .basic_peer_epoch_is_current(&epoch)
            .expect("stale epoch")
    );
    assert_eq!(observed_rows(&store).retained_record_count, 1);
    let reads = store.basic_filter_point_reads_for_test();
    for _ in 0..64 {
        assert_eq!(
            store.basic_filter_retention(),
            Err(BasicIndexUnavailableReason::RetentionIncomplete)
        );
    }
    assert_eq!(store.basic_filter_point_reads_for_test(), reads);
    drop(epoch);
    drop(clone);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("real reopen");
    recover(&reopened, &positions);
    assert_eq!(
        reopened.basic_filter_retention(),
        Ok(observed_rows(&reopened))
    );
    cleanup(path, reopened);
}

#[test]
fn phase161_retention_publication_failed_raw_put_and_remove_stay_unavailable() {
    for remove in [true, false] {
        // Arrange
        let (path, store, _, records) = retained_store("retention-raw-failure", 1);
        let before = observed_rows(&store);
        let clone = store.clone();
        store.set_basic_filter_revision_for_test(u64::MAX);
        let key = codec::record_key(records[0].identity().block_hash());
        // Act
        let result = if remove {
            clone.remove_bytes(StorageNamespace::BlockIndex, &key, PersistMode::Sync)
        } else {
            clone.put_bytes(
                StorageNamespace::BlockIndex,
                &key,
                codec::encode_record(&records[0]),
                PersistMode::Sync,
            )
        };
        // Assert
        assert!(result.is_err());
        assert_eq!(
            store.basic_filter_retention(),
            Err(BasicIndexUnavailableReason::RetentionFailed)
        );
        assert_eq!(observed_rows(&store), before);
        drop(clone);
        cleanup(path, store);
    }
}

#[test]
fn phase161_retention_publication_raw_remove_failed_reopen_never_earns_zero() {
    // Arrange
    let (path, store, positions, records) = retained_store("retention-raw-remove", 1);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    let clone = store.clone();
    // Act
    clone
        .remove_bytes(
            StorageNamespace::BlockIndex,
            &codec::record_key(records[0].identity().block_hash()),
            PersistMode::Sync,
        )
        .expect("raw remove");
    // Assert
    assert_eq!(
        store.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionIncomplete)
    );
    assert!(
        !store
            .basic_peer_epoch_is_current(&epoch)
            .expect("stale epoch")
    );
    drop(epoch);
    drop(clone);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("real reopen");
    assert!(
        reopened
            .recover_basic_filter_index_before_prune(Some(positions[0].block_hash))
            .is_err()
    );
    assert_eq!(
        reopened.basic_filter_retention(),
        Err(BasicIndexUnavailableReason::RetentionFailed)
    );
    cleanup(path, reopened);
}

#[test]
fn phase161_retention_publication_coins_metadata_and_disable_preserve_totals_revoke_epoch() {
    // Arrange
    let (path, store, positions, _) = retained_store("retention-coins-disable", 3);
    let before = observed_rows(&store);
    let epoch = store
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("epoch");
    let mut clone = store.clone();
    // Act
    clone
        .coins_view()
        .batch_write(
            open_bitcoin_core::chainstate::CoinsBatch {
                entries: Default::default(),
            },
            Some(positions[2].block_hash),
        )
        .expect("coins flush");
    crate::chainstate::FlushPersistSink::persist_chain_meta(&mut clone, &positions)
        .expect("metadata flush");
    assert_eq!(store.basic_filter_retention(), Ok(before));
    assert!(
        store
            .basic_peer_epoch_is_current(&epoch)
            .expect("ordinary revision preserves epoch")
    );
    store.disable_basic_filter_index().expect("disable");
    // Assert
    assert_eq!(store.basic_filter_retention(), Ok(before));
    assert!(
        !store
            .basic_peer_epoch_is_current(&epoch)
            .expect("disable revokes capture")
    );
    drop(epoch);
    drop(clone);
    cleanup(path, store);
}

#[test]
fn phase161_retention_publication_real_reorg_reuses_retained_branch_rows_once() {
    // Arrange
    let mut fixture = ReorgFixture::new("retention-real-reorg", 3);
    let path = fixture.path().to_owned();
    let before = observed_rows(&fixture.store);
    assert_eq!(fixture.store.basic_filter_retention(), Ok(before));
    let (staged, replacement) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare rewind");
    // Act
    fixture.publish(staged, prepared).expect("validated reorg");
    assert_eq!(fixture.store.basic_filter_retention(), Ok(before));
    fixture
        .append_replacement(&replacement)
        .expect("append replacement");
    let expected = observed_rows(&fixture.store);
    let current = fixture.manager.chainstate();
    let disconnect = current.active_chain()[1..]
        .iter()
        .rev()
        .map(|position| {
            fixture
                .store
                .load_block(position.block_hash)
                .expect("body read")
                .expect("retained replacement body")
        })
        .collect::<Vec<_>>();
    let connected = fixture.records[1..]
        .iter()
        .map(|record| open_bitcoin_core::chainstate::AnchoredBlock {
            block: fixture
                .store
                .load_block(record.identity().block_hash())
                .expect("body read")
                .expect("retained original body"),
            chain_work: open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(
                u128::from(record.identity().height()) + 100,
            ),
        })
        .collect::<Vec<_>>();
    let staged = current
        .stage_reorg(
            &disconnect,
            &connected,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("genuine return stage");
    let current_proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&current_proof, &staged)
        .expect("return rewind");
    fixture
        .publish(staged, prepared)
        .expect("return to retained original branch");
    let reused = fixture
        .append_replacement(&fixture.records[1..])
        .expect("reuse retained original rows");
    // Assert
    assert_eq!(
        expected.retained_record_count,
        before.retained_record_count + replacement.len() as u64
    );
    assert_eq!(fixture.store.basic_filter_retention(), Ok(expected));
    assert_eq!(reused.processed, checkpoint(&fixture.records[2]));
    assert_eq!(observed_rows(&fixture.store), expected);
    drop(current_proof);
    drop(proof);
    fixture = fixture.reopen();
    assert_eq!(fixture.store.basic_filter_retention(), Ok(expected));
    assert_eq!(observed_rows(&fixture.store), expected);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}
