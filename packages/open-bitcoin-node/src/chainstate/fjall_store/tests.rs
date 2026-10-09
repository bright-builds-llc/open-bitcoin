// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::super::flush_lifecycle::managed_fixture;
use super::*;
use open_bitcoin_core::chainstate::{CoinsCache, FlushMode, FlushPolicyTime};

fn flush(manager: &mut ManagedChainstate<FjallChainstateStore, FjallCoinsView>) {
    manager
        .flush_with_mode(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(10),
            u64::MAX,
        )
        .expect("real managed flush");
}

#[path = "tests/fixture.rs"]
mod fixture;
pub(crate) use fixture::ReorgFixture;

#[test]
fn phase158_bridge_reorg_genuine_acceptance_publishes_and_pending_blocks_flush() {
    // Arrange
    let (path, store, mut manager, genesis) = managed_fixture("reorg-bridge");
    let staged = manager
        .chainstate
        .stage_reorg(
            &[genesis],
            &[],
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams::default(),
        )
        .expect("genuine disconnect");
    let proof = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    let (_, accepted) = manager.chainstate.absorb_staged_reorg_with_receipt(staged);
    // Act
    let lineage = manager.maybe_validated_lineage.as_mut().expect("tracked");
    let authorization = lineage
        .authorize_reorg(&accepted, &prepared)
        .expect("authorize");
    assert!(lineage.maybe_prepare(manager.chainstate.tip()).is_err());
    let completed = store
        .complete_basic_filter_reorg(prepared, authorization)
        .expect("publish");
    lineage.confirm_reorg(&completed).expect("confirm");
    // Assert
    assert!(store.check_basic_filter_append_proof(&proof).is_err());
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(open_bitcoin_core::chainstate::FilterCheckpoint::new(
            open_bitcoin_core::chainstate::IndexPrefix::Empty
        ))
    );
    drop(completed);
    drop(proof);
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_bridge_reorg_preview_never_earns_replacement_positions_or_flush() {
    // Arrange
    let mut fixture = ReorgFixture::new("preview-reorg", 3);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    // Act
    fixture
        .manager
        .chainstate
        .install_staged_reorg_preview(&staged);
    // Assert
    assert!(
        fixture
            .manager
            .authorize_basic_filter_append_positions(&proof, &fixture.records[1..2])
            .is_err()
    );
    assert!(
        fixture
            .manager
            .maybe_validated_lineage
            .as_ref()
            .expect("tracked")
            .maybe_prepare(fixture.manager.chainstate.tip())
            .is_err()
    );
    fixture
        .store
        .check_basic_filter_append_proof(&proof)
        .expect("preview cannot publish");
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_bridge_reorg_generic_reconstruction_cannot_earn_positions() {
    // Arrange
    let fixture = ReorgFixture::new("generic-reorg", 3);
    let path = fixture.path().to_owned();
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    let lifecycle = fixture.manager.flush_lifecycle.clone();
    let (sink, state) = fixture.manager.into_parts();
    // Act
    let generic = ManagedChainstate::from_chainstate(sink, state, lifecycle);
    // Assert
    assert!(generic.maybe_validated_lineage.is_none());
    assert!(
        generic
            .authorize_basic_filter_append_positions(&proof, &fixture.records[1..2])
            .is_err()
    );
    drop(generic);
    drop(proof);
    drop(fixture.store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_bridge_reorg_foreign_preparation_refuses_genuine_receipt() {
    // Arrange
    let mut first = ReorgFixture::new("reorg-foreign-first", 3);
    let second = ReorgFixture::new("reorg-foreign-second", 3);
    let first_path = first.path().to_owned();
    let second_path = second.path().to_owned();
    let (first_stage, _) = first.stage(2);
    let (second_stage, _) = second.stage(2);
    let proof = second
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = second
        .store
        .prepare_basic_filter_reorg(&proof, &second_stage)
        .expect("prepare");
    // Act
    let result = first.authorize(first_stage, &prepared);
    // Assert
    assert!(result.is_err());
    second
        .store
        .check_basic_filter_append_proof(&proof)
        .expect("foreign store unchanged");
    drop(prepared);
    drop(proof);
    drop(first);
    drop(second);
    std::fs::remove_dir_all(first_path).expect("cleanup");
    std::fs::remove_dir_all(second_path).expect("cleanup");
}

#[test]
fn phase158_bridge_reorg_revision_change_after_acceptance_cannot_publish_or_flush() {
    // Arrange
    let mut fixture = ReorgFixture::new("reorg-stale-after-accept", 3);
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
    let before = fixture.store.maybe_basic_filter_state().expect("state");
    let authorization = fixture
        .authorize(staged, &prepared)
        .expect("genuine accepted");
    // Act
    fixture
        .store
        .invalidate_basic_filter_append()
        .expect("concurrent invalidation");
    assert!(
        fixture
            .store
            .complete_basic_filter_reorg(prepared, authorization)
            .is_err()
    );
    // Assert
    assert!(
        fixture
            .manager
            .maybe_validated_lineage
            .as_ref()
            .expect("tracked pending")
            .maybe_prepare(fixture.manager.chainstate.tip())
            .is_err()
    );
    assert_eq!(
        fixture.store.maybe_basic_filter_state().expect("unchanged"),
        before
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_missing_achieved_authorization_cannot_refresh_displaced_fence() {
    // Arrange
    let mut fixture = ReorgFixture::new("unpublished-replacement-metadata", 3);
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
    let authorization = fixture
        .authorize(staged, &prepared)
        .expect("accepted but unpublished");
    let before = fixture.store.maybe_basic_filter_state().expect("before");
    // Act
    fixture
        .store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(records[1].identity().block_hash()),
        )
        .expect("raw coins");
    let result = fixture
        .store
        .save_validated_chain_meta(fixture.manager.chainstate.active_chain(), PersistMode::Sync);
    // Assert
    assert!(
        result
            .expect_err("displaced ancestry requires achievement")
            .to_string()
            .contains("incompatible BASIC durable")
    );
    assert_eq!(
        fixture.store.maybe_basic_filter_state().expect("unchanged"),
        before
    );
    assert!(
        fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("no authority")
            .is_none()
    );
    drop(authorization);
    drop(prepared);
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_mismatched_actual_coins_hash_cannot_refresh_fence() {
    // Arrange
    let mut fixture = ReorgFixture::new("mismatched-replacement-coins", 3);
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
    fixture.publish(staged, prepared).expect("achieved");
    let before = fixture.store.maybe_basic_filter_state().expect("before");
    // Act
    fixture
        .store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(BlockHash::from_byte_array([91; 32])),
        )
        .expect("raw coins");
    let result = fixture
        .store
        .save_validated_chain_meta(fixture.manager.chainstate.active_chain(), PersistMode::Sync);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        fixture.store.maybe_basic_filter_state().expect("unchanged"),
        before
    );
    assert!(
        fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("no authority")
            .is_none()
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_generation_exhaustion_refuses_before_effects() {
    // Arrange
    let fixture = ReorgFixture::new("reorg-generation-exhausted", 3);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage(2);
    fixture
        .store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            crate::storage::filter_index::ownership::OWNER_KEY,
            crate::storage::filter_index::ownership::encode_owner(
                open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle::Active {
                    generation: IndexGeneration::new(u64::MAX),
                },
            )
            .to_vec(),
        )
        .expect("exhausted persisted owner fixture");
    fixture
        .store
        .configure_basic_filter_index_before_prune(
            Some(fixture.records[2].identity().block_hash()),
            super::super::BasicFilterStartupMode::Enabled,
        )
        .expect("genuine complete recovery");
    let before = fixture.store.maybe_basic_filter_state().expect("before");
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
            .expect("refused")
            .to_string()
            .contains("generation exhausted")
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
fn phase157_proof_replayed_old_completion_cannot_advance_live_authority() {
    // Arrange
    let (path, store, mut manager, _) = managed_fixture("proof-replayed");
    let pending = manager
        .maybe_validated_lineage
        .as_ref()
        .expect("tracked")
        .maybe_prepare(manager.chainstate.tip())
        .expect("receipt")
        .expect("live proof");
    flush(&mut manager);
    let current = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let completed = pending
        .complete(
            manager.maybe_validated_lineage.as_ref(),
            manager.chainstate.tip(),
        )
        .expect("deliberately stale sealed receipt");
    // Act
    assert!(FlushPersistSink::confirm_validated_flush(&mut store.clone(), completed).is_err());
    // Assert
    store
        .check_basic_filter_append_proof(&current)
        .expect("current authority unchanged");
    drop(current);
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_foreign_completion_refuses_identical_public_facts() {
    // Arrange
    let (first_path, first, first_manager, _) = managed_fixture("proof-completion-first");
    let (second_path, second, second_manager, _) = managed_fixture("proof-completion-second");
    let pending = first_manager
        .maybe_validated_lineage
        .as_ref()
        .expect("tracked")
        .maybe_prepare(first_manager.chainstate.tip())
        .expect("receipt")
        .expect("live");
    let completed = pending
        .complete(
            first_manager.maybe_validated_lineage.as_ref(),
            first_manager.chainstate.tip(),
        )
        .expect("foreign test receipt");
    let current = second
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    // Act
    assert!(FlushPersistSink::confirm_validated_flush(&mut second.clone(), completed).is_err());
    // Assert
    second
        .check_basic_filter_append_proof(&current)
        .expect("second authority unchanged");
    drop(current);
    drop(first_manager);
    drop(second_manager);
    drop(first);
    drop(second);
    std::fs::remove_dir_all(first_path).expect("cleanup");
    std::fs::remove_dir_all(second_path).expect("cleanup");
}

#[test]
fn phase157_proof_reconstruction_never_inherits_private_lineage() {
    // Arrange
    let (path, store, manager, _) = managed_fixture("proof-reconstruction");
    let lifecycle = manager.flush_lifecycle.clone();
    let (sink, state) = manager.into_parts();
    // Act
    let mut reconstructed = ManagedChainstate::from_chainstate(sink, state, lifecycle);
    assert!(reconstructed.maybe_validated_lineage.is_none());
    flush(&mut reconstructed);
    // Assert
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("no inherited authority")
            .is_none()
    );
    drop(reconstructed);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_identical_test_replacement_clears_provenance() {
    // Arrange
    let (path, store, mut manager, _) = managed_fixture("proof-replacement");
    let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
    let replacement = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        positions,
        Default::default(),
        counts,
    );
    // Act
    manager.install_chainstate_for_test(replacement);
    assert!(manager.maybe_validated_lineage.is_none());
    flush(&mut manager);
    // Assert
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("no reseeding")
            .is_none()
    );
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_memory_clone_drops_even_test_injected_lineage() {
    // Arrange
    let (path, store, mut manager, _) = managed_fixture("proof-clone");
    let mut memory = ManagedChainstate::from_store(super::super::MemoryChainstateStore::default());
    memory.maybe_validated_lineage = manager.maybe_validated_lineage.take();
    // Act
    let cloned = memory.clone();
    // Assert
    assert!(memory.maybe_validated_lineage.is_some());
    assert!(cloned.maybe_validated_lineage.is_none());
    drop(cloned);
    drop(memory);
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_old_manager_cannot_follow_reenabled_generation() {
    // Arrange
    let (path, store, mut manager, _) = managed_fixture("proof-generation");
    store.disable_basic_filter_index().expect("disable");
    let positions = store.load_chain_meta_for_open().expect("metadata").0;
    let fence = open_bitcoin_core::chainstate::VerifiedChainstateFence::new(
        store.coins_view().best_block().expect("B"),
        Some(&positions),
    )
    .expect("full recovered fence");
    store
        .enable_basic_filter_index(&fence)
        .expect("complete enable/preflight");
    let current = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("new generation");
    // Act
    assert!(
        manager
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX
            )
            .is_err()
    );
    // Assert
    store
        .check_basic_filter_append_proof(&current)
        .expect("new authority unchanged");
    drop(current);
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_raw_snapshot_seed_cannot_refresh_any_clone() {
    // Arrange
    let (path, store, manager, _) = managed_fixture("proof-raw-seed");
    let old = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let snapshot = store
        .wallet_scan_chainstate_snapshot()
        .expect("snapshot")
        .expect("present");
    let before = store.maybe_basic_filter_state().expect("safe state");
    // Act
    store
        .clone()
        .seed_coins_from_snapshot(&snapshot)
        .expect("raw seed publication");
    // Assert
    assert!(store.check_basic_filter_append_proof(&old).is_err());
    assert!(
        store
            .clone()
            .maybe_basic_filter_append_proof()
            .expect("no authority")
            .is_none()
    );
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("safe state unchanged"),
        before
    );
    drop(old);
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
