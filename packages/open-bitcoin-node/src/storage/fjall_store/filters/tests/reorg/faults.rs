// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

fn interrupted_rewind(point: FilterPublicationFault) {
    // Arrange
    let mut fixture = ReorgFixture::new("rewind-fault", 3);
    let path = fixture.path().to_owned();
    let old = fixture.records.clone();
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
    let authorization = fixture.authorize(staged, &prepared).expect("accepted");
    fixture.store.set_basic_filter_fault(point);
    // Act
    assert!(
        fixture
            .store
            .complete_basic_filter_reorg(prepared, authorization)
            .is_err()
    );
    assert!(
        fixture
            .manager
            .flush_with_mode(
                open_bitcoin_core::chainstate::FlushMode::Always,
                open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(20),
                u64::MAX
            )
            .is_err()
    );
    assert!(fixture.store.maybe_basic_filter_append_proof().is_err());
    drop(proof);
    drop(fixture);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    // Assert
    let safe = if point == FilterPublicationFault::AfterCommit {
        &old[0]
    } else {
        &old[2]
    };
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("atomic checkpoint"),
        Some(checkpoint(safe))
    );
    for record in &old {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable"),
            Some(record.clone())
        );
    }
    assert_eq!(
        reopened
            .basic_filter_projection(2)
            .expect("unchanged projection"),
        old[2].identity().block_hash()
    );
    assert!(
        reopened
            .load_prune_protection()
            .expect("protection")
            .protects_height(1)
            || point != FilterPublicationFault::AfterCommit
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_before_records_fault_reopens_old_prefix() {
    interrupted_rewind(FilterPublicationFault::BeforeRecords);
}
#[test]
fn phase158_storage_reorg_before_checkpoint_fault_reopens_old_prefix() {
    interrupted_rewind(FilterPublicationFault::BeforeCheckpoint);
}
#[test]
fn phase158_storage_reorg_before_protection_fault_reopens_old_prefix() {
    interrupted_rewind(FilterPublicationFault::BeforeProtection);
}
#[test]
fn phase158_storage_reorg_after_commit_fault_reopens_masked_prefix() {
    interrupted_rewind(FilterPublicationFault::AfterCommit);
}

fn interrupted_append(point: FilterPublicationFault, subsequent: bool) {
    // Arrange
    let mut fixture = ReorgFixture::new("append-fault", 3);
    let path = fixture.path().to_owned();
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
        .expect("prepare");
    fixture.publish(staged, prepared).expect("rewind");
    let next = usize::from(subsequent);
    if subsequent {
        fixture
            .append_replacement(&replacement[..1])
            .expect("first turn");
    }
    let current = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    let positions = fixture
        .manager
        .authorize_basic_filter_append_positions(&current, &replacement[next..next + 1])
        .expect("seal next turn");
    let prepared = fixture
        .store
        .prepare_basic_filter_replacement_append(current, positions, &replacement[next..next + 1])
        .expect("prepare");
    fixture.store.set_basic_filter_fault(point);
    // Act
    assert!(
        fixture
            .store
            .complete_basic_filter_append(prepared)
            .is_err()
    );
    assert!(fixture.store.maybe_basic_filter_append_proof().is_err());
    drop(proof);
    drop(fixture);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    // Assert
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("shared safe"),
        Some(checkpoint(&old[0]))
    );
    assert_eq!(
        reopened
            .maybe_active_basic_filter_record(1)
            .expect("hidden"),
        None
    );
    for record in &old {
        assert_eq!(
            reopened
                .get_bytes(
                    StorageNamespace::BlockIndex,
                    &codec::record_key(record.identity().block_hash())
                )
                .expect("original bytes"),
            Some(codec::encode_record(record))
        );
    }
    let hash = if point == FilterPublicationFault::AfterCommit {
        replacement[next].identity().block_hash()
    } else {
        old[next + 1].identity().block_hash()
    };
    assert_eq!(
        reopened
            .basic_filter_projection(next as u32 + 1)
            .expect("atomic projection"),
        hash
    );
    reopened
        .validate_basic_filter_records()
        .expect("complete fork integrity");
    assert!(
        reopened
            .load_prune_protection()
            .expect("protected")
            .protects_height(1)
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_first_append_before_records_is_atomic() {
    interrupted_append(FilterPublicationFault::BeforeRecords, false);
}
#[test]
fn phase158_storage_reorg_first_append_before_checkpoint_is_atomic() {
    interrupted_append(FilterPublicationFault::BeforeCheckpoint, false);
}
#[test]
fn phase158_storage_reorg_first_append_before_protection_is_atomic() {
    interrupted_append(FilterPublicationFault::BeforeProtection, false);
}
#[test]
fn phase158_storage_reorg_first_append_after_commit_masks_suffix() {
    interrupted_append(FilterPublicationFault::AfterCommit, false);
}
#[test]
fn phase158_storage_reorg_later_append_before_records_is_atomic() {
    interrupted_append(FilterPublicationFault::BeforeRecords, true);
}
#[test]
fn phase158_storage_reorg_later_append_before_checkpoint_is_atomic() {
    interrupted_append(FilterPublicationFault::BeforeCheckpoint, true);
}
#[test]
fn phase158_storage_reorg_later_append_before_protection_is_atomic() {
    interrupted_append(FilterPublicationFault::BeforeProtection, true);
}
#[test]
fn phase158_storage_reorg_later_append_after_commit_masks_suffix() {
    interrupted_append(FilterPublicationFault::AfterCommit, true);
}

#[test]
fn phase158_storage_reorg_recovered_batch_later_hidden_row_requires_positions() {
    // Arrange
    let mut fixture = ReorgFixture::new("recovered-hidden-hole", 3);
    let path = fixture.path().to_owned();
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
        .expect("prepare");
    fixture.publish(staged, prepared).expect("rewind");
    fixture.flush();
    fixture
        .store
        .block_index
        .remove(codec::active_key(1))
        .expect("sparse masked projection fixture");
    fixture
        .store
        .persist(StorageNamespace::BlockIndex, PersistMode::Sync)
        .expect("persist sparse fixture");
    drop(proof);
    let fixture = fixture.reopen();
    let before = fixture.store.maybe_basic_filter_state().expect("before");
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    // Act
    let result = fixture.store.prepare_basic_filter_append(proof, &old[1..]);
    // Assert
    assert!(
        result
            .err()
            .expect("later hidden row refuses")
            .to_string()
            .contains("recovered projection advance requires accepted positions")
    );
    assert_eq!(
        fixture.store.maybe_basic_filter_state().expect("unchanged"),
        before
    );
    assert!(fixture.store.basic_filter_projection(1).is_err());
    fixture
        .append_replacement(&replacement)
        .expect("genuine canonical rebinding crosses sparse hidden rows");
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_recovered_foreign_positions_refuse_identical_public_facts() {
    // Arrange
    let first = ReorgFixture::new("recovered-positions-first", 1).reopen();
    let second = ReorgFixture::new("recovered-positions-second", 1).reopen();
    let first_path = first.path().to_owned();
    let second_path = second.path().to_owned();
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
    assert_eq!(first_proof.durable_tip(), second_proof.durable_tip());
    assert_eq!(first_proof.generation(), second_proof.generation());
    let positions = first
        .manager
        .authorize_basic_filter_append_positions(&first_proof, &first.records[1..])
        .expect("genuine recovered positions");
    // Act
    let result = second.store.prepare_basic_filter_replacement_append(
        second_proof,
        positions,
        &second.records[1..],
    );
    // Assert
    assert!(result.is_err());
    assert!(second.store.basic_filter_projection(1).is_err());
    drop(first_proof);
    drop(first);
    drop(second);
    std::fs::remove_dir_all(first_path).expect("cleanup");
    std::fs::remove_dir_all(second_path).expect("cleanup");
}
