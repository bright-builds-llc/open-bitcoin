// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::super::BasicFilterAppendProof;
use super::*;
use crate::chainstate::BasicFilterStartupMode;
use open_bitcoin_core::chainstate::{CoinsBatch, CoinsCacheEntry};

fn proof(store: &FjallNodeStore) -> BasicFilterAppendProof {
    store
        .maybe_basic_filter_append_proof()
        .expect("proof loader")
        .expect("recovery proof")
}

fn recovered(store: &FjallNodeStore, positions: &[ChainPosition]) {
    seed(store, positions);
    for position in positions {
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
                    &BlockUndo {
                        transactions: Vec::new(),
                    },
                    PersistMode::Sync,
                )
                .expect("undo");
        }
    }
    store
        .configure_basic_filter_index_before_prune(
            Some(positions.last().expect("tip").block_hash),
            BasicFilterStartupMode::Enabled,
        )
        .expect("complete configured recovery");
}

#[test]
fn phase157_proof_absent_and_unverified_initialization_grant_no_append() {
    // Arrange
    let path = temp_path("proof-unverified");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    // Act / Assert
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("absent")
            .is_none()
    );
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("saved state only");
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("unverified")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_complete_recovery_mints_bounded_same_store_authority() {
    // Arrange
    let path = temp_path("proof-recovered");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(16);
    recovered(&store, &positions);
    // Act
    let proof = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    store
        .filter_integrity_reads
        .store(0, std::sync::atomic::Ordering::Relaxed);
    store
        .check_basic_filter_append_proof(&proof)
        .expect("same store");
    store
        .clone()
        .check_basic_filter_append_proof(&proof)
        .expect("shared clone");
    // Assert
    assert!(
        store
            .filter_integrity_reads
            .load(std::sync::atomic::Ordering::Relaxed)
            < 20
    );
    drop(proof);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_foreign_store_cannot_reuse_identical_frontiers() {
    // Arrange
    let first_path = temp_path("proof-first");
    let second_path = temp_path("proof-second");
    let first = FjallNodeStore::open(&first_path).expect("first");
    let second = FjallNodeStore::open(&second_path).expect("second");
    let (positions, _) = fixtures(1);
    recovered(&first, &positions);
    recovered(&second, &positions);
    let proof = first
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    let before = second.maybe_basic_filter_state().expect("before");
    // Act
    let result = second.check_basic_filter_append_proof(&proof);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        second.maybe_basic_filter_state().expect("unchanged"),
        before
    );
    drop(proof);
    drop(first);
    drop(second);
    std::fs::remove_dir_all(first_path).expect("cleanup");
    std::fs::remove_dir_all(second_path).expect("cleanup");
}

#[test]
fn phase157_proof_disable_reenable_invalidates_old_generation() {
    // Arrange
    let path = temp_path("proof-lifecycle");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    // Act
    store.disable_basic_filter_index().expect("disable");
    assert!(store.check_basic_filter_append_proof(&proof).is_err());
    store
        .enable_basic_filter_index(&fence(&positions))
        .expect("enable");
    // Assert
    assert!(store.check_basic_filter_append_proof(&proof).is_err());
    let current = store
        .maybe_basic_filter_append_proof()
        .expect("current")
        .expect("minted");
    store
        .check_basic_filter_append_proof(&current)
        .expect("new generation");
    drop(current);
    drop(proof);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_coins_only_write_invalidates_every_clone() {
    // Arrange
    let path = temp_path("proof-coins");
    let store = FjallNodeStore::open(&path).expect("open");
    let other = store.clone();
    let (positions, _) = fixtures(2);
    recovered(&store, &positions[..1]);
    let proof = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    // Act
    other
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(positions[1].block_hash),
        )
        .expect("real coins B");
    // Assert
    assert!(store.check_basic_filter_append_proof(&proof).is_err());
    assert!(
        other
            .maybe_basic_filter_append_proof()
            .expect("invalidated")
            .is_none()
    );
    assert_eq!(
        store.coins_view().best_block().expect("B"),
        Some(positions[1].block_hash)
    );
    drop(proof);
    drop(other);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_raw_matching_metadata_cannot_refresh_authority() {
    // Arrange
    let path = temp_path("proof-raw-meta");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    // Act
    store
        .save_chain_meta(&positions, PersistMode::Sync)
        .expect("raw metadata");
    // Assert
    assert!(store.check_basic_filter_append_proof(&proof).is_err());
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("invalidated")
            .is_none()
    );
    drop(proof);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_completed_metadata_remains_pending_without_managed_completion() {
    // Arrange
    let path = temp_path("proof-refresh");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(2);
    recovered(&store, &positions[..1]);
    let old = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    // Act
    store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(positions[1].block_hash),
        )
        .expect("coins flush");
    store
        .save_validated_chain_meta(&positions, PersistMode::Flush)
        .expect("trusted complete metadata");
    // Assert
    assert!(store.check_basic_filter_append_proof(&old).is_err());
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("pending only")
            .is_none()
    );
    assert_eq!(old.durable_tip(), (0, positions[0].block_hash));
    assert_eq!(old.branch_identity(), positions[0].block_hash);
    assert_eq!(
        old.generation(),
        open_bitcoin_core::chainstate::filter_index::lifecycle::IndexGeneration::new(0)
    );
    assert_eq!(old.safe_checkpoint(), old.processed());
    drop(old);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_after_coins_metadata_fault_refuses_and_reopen_reconciles() {
    // Arrange
    let path = temp_path("proof-meta-fault");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(2);
    recovered(&store, &positions[..1]);
    let before = store.maybe_basic_filter_state().expect("before");
    let old = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(positions[1].block_hash),
        )
        .expect("actual B advances");
    // Act
    store.set_basic_filter_fault(super::super::FilterPublicationFault::BeforeChainMeta);
    assert!(
        store
            .save_validated_chain_meta(&positions, PersistMode::Sync)
            .is_err()
    );
    assert!(store.clone().check_basic_filter_append_proof(&old).is_err());
    drop(old);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("real reopen");
    // Assert
    assert_eq!(
        reopened.coins_view().best_block().expect("B"),
        Some(positions[1].block_hash)
    );
    assert_eq!(
        reopened
            .maybe_basic_filter_state()
            .expect("unchanged safe state"),
        before
    );
    assert!(
        reopened
            .configure_basic_filter_index_before_prune(
                Some(positions[1].block_hash),
                BasicFilterStartupMode::Enabled
            )
            .is_err()
    );
    assert!(
        reopened
            .maybe_basic_filter_append_proof()
            .expect("no authority")
            .is_none()
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_interrupted_limited_coins_write_cannot_refresh() {
    // Arrange
    let path = temp_path("proof-partial");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(2);
    recovered(&store, &positions[..1]);
    let old = store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("minted");
    let mut view = store.coins_view();
    view.set_simulate_crash_after_partial(true);
    let coin = open_bitcoin_core::chainstate::Coin {
        output: TransactionOutput {
            value: Amount::from_sats(1).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        },
        created_height: 0,
        created_median_time_past: 0,
        is_coinbase: true,
    };
    let entries = [(OutPoint::null(), CoinsCacheEntry::unspent_dirty(coin))]
        .into_iter()
        .collect();
    // Act
    assert!(
        view.batch_write_with_limit(CoinsBatch { entries }, Some(positions[1].block_hash), 1)
            .is_err()
    );
    // Assert
    assert_eq!(view.head_blocks().expect("actual H").len(), 2);
    assert!(store.check_basic_filter_append_proof(&old).is_err());
    assert!(
        store
            .save_validated_chain_meta(&positions, PersistMode::Sync)
            .is_err()
    );
    drop(view);
    drop(old);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_raw_coins_and_public_sink_cannot_forge_trusted_flush() {
    // Arrange
    let path = temp_path("proof-public-forgery");
    let mut store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(2);
    recovered(&store, &positions[..1]);
    // Act
    store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(positions[1].block_hash),
        )
        .expect("raw caller coins");
    crate::chainstate::FlushPersistSink::persist_chain_meta(&mut store, &positions)
        .expect("public sink");
    // Assert
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("no forge")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_failed_history_preflight_never_mints_authority() {
    // Arrange
    let path = temp_path("proof-missing-history");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(2);
    seed(&store, &positions);
    // Act
    let result = store.enable_basic_filter_index(&fence(&positions));
    // Assert
    assert!(result.is_err());
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .is_none()
    );
    assert!(
        store
            .maybe_basic_filter_state()
            .expect("no publication")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_full_forest_corruption_refuses_recovery_mint() {
    // Arrange
    let path = temp_path("proof-forest-corruption");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(2);
    recovered(&store, &positions);
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            &codec::record_key(records[1].identity().block_hash()),
            vec![255],
        )
        .expect("hidden corrupt row");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    // Act
    let result = reopened.enable_basic_filter_index(&fence(&positions));
    // Assert
    assert!(result.is_err());
    assert!(
        reopened
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .is_none()
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_reopen_incarnation_rejects_previous_instance() {
    // Arrange
    let path = temp_path("proof-incarnation");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(1);
    recovered(&store, &positions);
    let old = proof(&store);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    reopened
        .enable_basic_filter_index(&fence(&positions))
        .expect("full recovery/preflight");
    // Act / Assert
    assert!(reopened.check_basic_filter_append_proof(&old).is_err());
    reopened
        .check_basic_filter_append_proof(&proof(&reopened))
        .expect("new incarnation");
    drop(old);
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_changed_saved_branch_refuses_without_prefix_scan() {
    // Arrange
    let path = temp_path("proof-stale-branch");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, _) = fixtures(2);
    recovered(&store, &positions);
    let old = proof(&store);
    let mut state = store
        .maybe_basic_filter_state()
        .expect("state")
        .expect("saved");
    state.fence_hash = positions[0].block_hash;
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            codec::STATE_KEY,
            codec::encode_state(state),
        )
        .expect("changed saved branch");
    // Act / Assert
    assert!(store.check_basic_filter_append_proof(&old).is_err());
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("unchanged after refusal"),
        Some(state)
    );
    drop(old);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_poisoned_publication_invalidates_clone_work() {
    // Arrange
    let path = temp_path("proof-poisoned");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let old = proof(&store);
    let before = store.maybe_basic_filter_state().expect("state");
    // Act
    store.set_basic_filter_fault(super::super::FilterPublicationFault::BeforeRecords);
    assert!(append_current(&store, &records).is_err());
    // Assert
    assert!(store.clone().check_basic_filter_append_proof(&old).is_err());
    assert_eq!(store.maybe_basic_filter_state().expect("unchanged"), before);
    drop(old);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_proof_prepare_and_complete_use_constant_endpoint_reads() {
    // Arrange
    let mut counts = Vec::new();
    for count in [16, 256] {
        let path = temp_path("proof-point-bounds");
        let store = FjallNodeStore::open(&path).expect("open");
        let (positions, records) = fixtures(count);
        recovered(&store, &positions);
        for batch in records.chunks(128) {
            publish(&store, &positions, batch);
        }
        store
            .enable_basic_filter_index(&fence(&positions))
            .expect("full recovered prefix");
        // Act
        store
            .filter_integrity_reads
            .store(0, std::sync::atomic::Ordering::Relaxed);
        let work = proof(&store);
        store
            .check_basic_filter_append_proof(&work)
            .expect("bounded complete checks");
        counts.push(
            store
                .filter_integrity_reads
                .load(std::sync::atomic::Ordering::Relaxed),
        );
        // Assert
        assert_eq!(
            work.processed(),
            checkpoint(records.last().expect("endpoint"))
        );
        drop(work);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
    assert_eq!(counts[0], counts[1]);
    assert!(counts[0] > 0 && counts[0] < 20);
}
