// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::chainstate::ReorgFixture;

mod faults;

#[test]
fn phase158_storage_reorg_preserves_stronger_reserved_and_ordinary_locks() {
    // Arrange
    let mut fixture = ReorgFixture::new("strong-lock-reorg", 3);
    let path = fixture.path().to_owned();
    let mut locks = fixture.store.load_prune_locks().expect("locks");
    locks.retain(|lock| lock.name != open_bitcoin_core::chainstate::BASIC_INDEX_PRUNE_LOCK);
    let operator = PruneLockInfo {
        name: "operator".to_owned(),
        height_first: 7,
        height_last: 99,
    };
    locks.push(operator.clone());
    locks.push(
        IndexInputProtection::FromHeight(0)
            .maybe_prune_lock()
            .expect("reserved"),
    );
    fixture
        .store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            crate::storage::fjall_store::prune::PRUNE_LOCKS_KEY,
            crate::storage::fjall_store::prune::encode_prune_locks(&locks)
                .expect("stronger persisted lock fixture"),
        )
        .expect("fixture");
    fixture = fixture.reopen();
    let (staged, _) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    // Act
    fixture.publish(staged, prepared).expect("achieve");
    // Assert
    assert_eq!(
        fixture
            .store
            .maybe_basic_filter_state()
            .expect("state")
            .expect("present")
            .protection,
        IndexInputProtection::FromHeight(0)
    );
    assert!(
        fixture
            .store
            .load_prune_locks()
            .expect("preserved map")
            .contains(&operator)
    );
    assert!(fixture.store.sync_prune_locks(&[operator]).is_err());
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_authenticated_overwrite_retains_displaced_fence_and_headers() {
    // Arrange
    let mut fixture = ReorgFixture::new("replacement", 3);
    let old = fixture.records.clone();
    let (staged, replacement) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("rewind");
    // Act
    fixture
        .publish(staged, prepared)
        .expect("genuine achieved reorg");
    let current = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    assert!(
        fixture
            .store
            .prepare_basic_filter_append(current, &replacement)
            .is_err()
    );
    fixture
        .append_replacement(&replacement)
        .expect("sealed replacement");
    // Assert
    let current = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    assert_eq!(current.safe_checkpoint(), checkpoint(&old[0]));
    assert_eq!(
        current.processed(),
        checkpoint(replacement.last().expect("new tip"))
    );
    assert_eq!(current.protection(), IndexInputProtection::FromHeight(1));
    assert!(
        fixture
            .store
            .check_basic_filter_append_proof(&proof)
            .is_err()
    );
    for record in &old {
        assert_eq!(
            fixture
                .store
                .load_basic_filter_record(record.identity().block_hash())
                .expect("retained"),
            Some(record.clone())
        );
    }
    assert_eq!(
        fixture
            .store
            .basic_filter_projection(1)
            .expect("projection"),
        replacement[0].identity().block_hash()
    );
    assert_eq!(
        fixture
            .store
            .maybe_active_basic_filter_record(1)
            .expect("unsafe hidden"),
        None
    );
    let path = fixture.path().to_owned();
    drop(proof);
    drop(current);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_own_flush_promotes_only_replacement_endpoint() {
    // Arrange
    let mut fixture = ReorgFixture::new("replacement-flush", 3);
    let path = fixture.path().to_owned();
    let old = fixture.records.clone();
    let (staged, replacement) = fixture.stage(3);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("publish");
    fixture
        .append_replacement(&replacement)
        .expect("replacement append");
    // Act
    fixture.flush();
    fixture
        .append_replacement(&[])
        .expect("promote earned checkpoint");
    drop(proof);
    drop(fixture);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    reopened
        .recover_basic_filter_index_before_prune(Some(
            replacement.last().expect("tip").identity().block_hash(),
        ))
        .expect("verified recovery");
    // Assert
    assert_eq!(
        reopened.maybe_basic_filter_checkpoint().expect("safe"),
        Some(checkpoint(replacement.last().expect("tip")))
    );
    for record in &old {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("retained"),
            Some(record.clone())
        );
    }
    for record in &replacement {
        assert_eq!(
            reopened
                .maybe_active_basic_filter_record(record.identity().height())
                .expect("active"),
            Some(record.clone())
        );
    }
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_lagging_progress_never_jumps_to_ancestor() {
    // Arrange
    let mut fixture = ReorgFixture::new("lagging-rewind", 0);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage_from(1, 1);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    // Act
    let achieved = fixture.publish(staged, prepared).expect("achieve");
    // Assert
    assert_eq!(
        achieved.prepared().processed(),
        checkpoint(&fixture.records[0])
    );
    assert_eq!(
        achieved.prepared().safe_checkpoint(),
        checkpoint(&fixture.records[0])
    );
    assert_eq!(
        achieved.prepared().protection(),
        IndexInputProtection::FromHeight(1)
    );
    assert!(achieved.prepared().work().record_operations < 30);
    drop(achieved);
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_counter_exhaustion_refuses_before_effects() {
    // Arrange
    let fixture = ReorgFixture::new("reorg-counter", 3);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage(2);
    let before = fixture.store.maybe_basic_filter_state().expect("state");
    fixture.store.set_basic_filter_revision_for_test(u64::MAX);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    // Act
    let result = fixture.store.prepare_basic_filter_reorg(&proof, &staged);
    // Assert
    assert!(
        result
            .err()
            .expect("refuses")
            .to_string()
            .contains("revision exhausted")
    );
    assert_eq!(
        fixture.store.maybe_basic_filter_state().expect("state"),
        before
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_borrowed_positions_refuse_old_branch_records() {
    // Arrange
    let mut fixture = ReorgFixture::new("replacement-old-record", 3);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("achieve");
    // Act
    let result = fixture.append_replacement(&fixture.records[1..]);
    // Assert
    assert!(
        result
            .expect_err("refused")
            .to_string()
            .contains("accepted position")
    );
    let current = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    assert!(
        fixture
            .store
            .prepare_basic_filter_append(current, &fixture.records[1..])
            .is_err()
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_foreign_borrowed_positions_refuse_identical_facts() {
    // Arrange
    let mut first = ReorgFixture::new("positions-first", 3);
    let mut second = ReorgFixture::new("positions-second", 3);
    let first_path = first.path().to_owned();
    let second_path = second.path().to_owned();
    let (_, records) = first.stage(2);
    for fixture in [&mut first, &mut second] {
        let (staged, _) = fixture.stage(2);
        let proof = fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        let prepared = fixture
            .store
            .prepare_basic_filter_reorg(&proof, &staged)
            .expect("prepare");
        fixture.publish(staged, prepared).expect("achieve");
    }
    let first_proof = first
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    let second_proof = second
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    let positions = first
        .manager
        .authorize_basic_filter_append_positions(&first_proof, &records)
        .expect("first genuine positions");
    // Act
    let result =
        second
            .store
            .prepare_basic_filter_replacement_append(second_proof, positions, &records);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        second
            .store
            .basic_filter_projection(1)
            .expect("old projection"),
        second.records[1].identity().block_hash()
    );
    drop(first_proof);
    drop(first);
    drop(second);
    std::fs::remove_dir_all(first_path).expect("cleanup");
    std::fs::remove_dir_all(second_path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_prepared_replacement_cannot_publish_after_invalidation() {
    // Arrange
    let mut fixture = ReorgFixture::new("stale-append", 3);
    let path = fixture.path().to_owned();
    let (staged, records) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("achieve");
    let current = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    let positions = fixture
        .manager
        .authorize_basic_filter_append_positions(&current, &records)
        .expect("positions");
    let prepared = fixture
        .store
        .prepare_basic_filter_replacement_append(current, positions, &records)
        .expect("prepare");
    // Act
    fixture
        .store
        .invalidate_basic_filter_append()
        .expect("writer invalidation");
    let result = fixture.store.complete_basic_filter_append(prepared);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        fixture
            .store
            .basic_filter_projection(1)
            .expect("old projection"),
        fixture.records[1].identity().block_hash()
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_identical_immutable_retry_is_idempotent() {
    // Arrange
    let mut fixture = ReorgFixture::new("replacement-retry", 3);
    let path = fixture.path().to_owned();
    let (staged, records) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("achieve");
    fixture
        .append_replacement(&records[..1])
        .expect("first replacement");
    let current = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    // Act
    let prepared = fixture
        .store
        .prepare_basic_filter_append(current, &records[..1])
        .expect("identical retry");
    let achieved = fixture
        .store
        .complete_basic_filter_append(prepared)
        .expect("idempotent");
    // Assert
    assert_eq!(achieved.batch_bytes, 0);
    assert_eq!(achieved.safe_checkpoint, checkpoint(&fixture.records[0]));
    assert_eq!(
        fixture
            .store
            .load_basic_filter_record(records[0].identity().block_hash())
            .expect("original"),
        Some(records[0].clone())
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_conflicting_immutable_retry_never_changes_original_bytes() {
    // Arrange
    let mut fixture = ReorgFixture::new("replacement-immutable-conflict", 3);
    let path = fixture.path().to_owned();
    let (staged, records) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("achieve");
    fixture
        .append_replacement(&records[..1])
        .expect("first replacement");
    let mut bytes = codec::encode_record(&records[0]);
    let hash = open_bitcoin_core::primitives::FilterHash::from_byte_array(
        open_bitcoin_core::consensus::crypto::double_sha256(&[0]),
    );
    let header = open_bitcoin_core::consensus::compute_filter_header(
        hash,
        records[0].identity().previous_header(),
    );
    bytes.truncate(codec::RECORD_OVERHEAD);
    bytes[102..134].copy_from_slice(hash.as_bytes());
    bytes[134..166].copy_from_slice(header.as_bytes());
    bytes[166..170].copy_from_slice(&1_u32.to_le_bytes());
    bytes.push(0);
    let key = codec::record_key(records[0].identity().block_hash());
    let conflicting = codec::decode_record(&key, &bytes, Some(&fixture.records[0].identity()))
        .expect("self-consistent alternate");
    let current = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    // Act
    let result = fixture
        .store
        .prepare_basic_filter_append(current, &[conflicting]);
    // Assert
    assert!(
        result
            .err()
            .expect("refused")
            .to_string()
            .contains("conflicting immutable")
    );
    assert_eq!(
        fixture
            .store
            .get_bytes(StorageNamespace::BlockIndex, &key)
            .expect("original bytes"),
        Some(codec::encode_record(&records[0]))
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_raw_metadata_cannot_refresh_achieved_authority() {
    // Arrange
    let mut fixture = ReorgFixture::new("raw-replacement-flush", 3);
    let path = fixture.path().to_owned();
    let (staged, records) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("achieve");
    fixture.append_replacement(&records).expect("replacement");
    let before = fixture.store.maybe_basic_filter_state().expect("safe");
    // Act
    fixture
        .store
        .coins_view()
        .batch_write(
            open_bitcoin_core::chainstate::CoinsBatch {
                entries: Default::default(),
            },
            Some(records[1].identity().block_hash()),
        )
        .expect("raw writer");
    fixture
        .store
        .save_validated_chain_meta(
            fixture.manager.chainstate().active_chain(),
            PersistMode::Sync,
        )
        .expect("nominal public sink write");
    // Assert
    assert!(
        fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("no authority")
            .is_none()
    );
    assert_eq!(
        fixture
            .store
            .maybe_basic_filter_state()
            .expect("safe unchanged"),
        before
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}
