// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

use crate::chainstate::{FjallChainstateStore, FlushLifecycle, ManagedChainstate};
use crate::storage::coins_view::FjallCoinsView;
use open_bitcoin_core::chainstate::{Chainstate, CoinsCache, FlushPolicyTime};
use open_bitcoin_core::{
    chainstate::{ChainstateError, CoinsView},
    consensus::{ConsensusParams, ScriptVerifyFlags, chain_proof::ProofWork},
    primitives::BlockHash,
};

type Manager = ManagedChainstate<FjallChainstateStore, FjallCoinsView>;

fn fixture_dir() -> std::path::PathBuf {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "phase165-rebind-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}

fn with_target(source: &Chainstate, check: impl FnOnce(&crate::FjallNodeStore, &mut Manager)) {
    let dir = fixture_dir();
    let store = crate::FjallNodeStore::open(&dir).expect("fixture store");
    if source.tip().is_some() {
        store
            .seed_coins_from_snapshot(&source.snapshot())
            .expect("exact raw target bytes");
    }
    let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
    let target = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        positions,
        store.load_all_undo_records().expect("undo"),
        counts,
    );
    let mut manager = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        target,
        FlushLifecycle::ready(
            FlushPolicyTime::from_unix_seconds(0),
            FlushPolicyTime::from_unix_seconds(0),
            0,
            false,
        ),
    );
    check(&store, &mut manager);
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

fn live() -> Chainstate {
    let mut source = Chainstate::new();
    let block = super::build_block(BlockHash::default(), 0, 50);
    source
        .connect_block(
            &block,
            ProofWork::ZERO,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("actual genesis");
    source
}

fn handle(
    store: &crate::FjallNodeStore,
    canonical: bool,
) -> crate::ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView> {
    let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
    let target = Chainstate::from_parent(
        store.coins_view(),
        positions,
        store.load_all_undo_records().expect("undo"),
        counts,
    );
    let manager = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        target,
        FlushLifecycle::ready(
            FlushPolicyTime::from_unix_seconds(0),
            FlushPolicyTime::from_unix_seconds(0),
            0,
            false,
        ),
    );
    let mut network = crate::ManagedPeerNetwork::from_initialized_chainstate(
        manager,
        open_bitcoin_network::LocalPeerConfig::default(),
        open_bitcoin_mempool::PolicyConfig::default(),
        16,
        open_bitcoin_network::RelayActivationConfig::default(),
        open_bitcoin_network::BlockRelayActivationPolicy::default(),
        false,
    );
    if canonical {
        network.admission_scope = crate::network::canonical_admission::AdmissionScope::Canonical(
            open_bitcoin_core::consensus::CanonicalChain::Regtest,
        );
    }
    crate::ManagedNetworkHandle::new(network)
}

#[test]
fn phase165_rebind_managed_genuine_empty_owner_adoption() {
    // Arrange
    let dir = fixture_dir();
    let store = crate::FjallNodeStore::open(&dir).expect("fixture store");
    let target = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        Vec::new(),
        Default::default(),
        None,
    );
    let mut managed: ManagedChainstate<FjallChainstateStore, FjallCoinsView> =
        ManagedChainstate::from_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            target,
            FlushLifecycle::ready(
                FlushPolicyTime::from_unix_seconds(0),
                FlushPolicyTime::from_unix_seconds(0),
                0,
                false,
            ),
        );
    // Act
    let result = managed.adopt_live_fixture_owner(Chainstate::new());
    // Assert
    assert!(
        result.is_ok(),
        "complete matching empty adapter accepts genuine owner"
    );
    drop(managed);
    drop(store);
    std::fs::remove_dir_all(dir).expect("remove fixture");
}

#[test]
fn phase165_rebind_managed_populated_owner_preserves_exact_work() {
    // Arrange
    let source = live();
    with_target(&source, |_, manager| {
        let before = manager.export_chainstate_snapshot().expect("snapshot");
        // Act
        manager
            .adopt_live_fixture_owner(source.clone())
            .expect("complete exact target");
        // Assert
        assert_eq!(
            manager.export_chainstate_snapshot().expect("snapshot"),
            before
        );
        assert!(manager.chainstate().has_earned_work());
    });
}

#[test]
fn phase165_rebind_managed_raw_and_mechanical_sources_refuse() {
    // Arrange
    with_target(&Chainstate::new(), |_, manager| {
        let before = manager.export_chainstate_snapshot().expect("snapshot");
        // Act / Assert
        for source in [
            Chainstate::from_snapshot(Chainstate::new().snapshot()),
            Chainstate::new_for_mechanical_fixture(),
        ] {
            assert_eq!(
                manager.adopt_live_fixture_owner(source),
                Err(ChainstateError::UntrustedWorkLineage)
            );
            assert_eq!(
                manager.export_chainstate_snapshot().expect("snapshot"),
                before
            );
        }
    });
}

#[test]
fn phase165_rebind_managed_wrong_concrete_parent_refuses() {
    // Arrange
    with_target(&Chainstate::new(), |store, _| {
        with_target(&Chainstate::new(), |other, _| {
            let target =
                Chainstate::from_parent(other.coins_view(), Vec::new(), Default::default(), None);
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store.clone()),
                target,
                FlushLifecycle::ready(
                    FlushPolicyTime::from_unix_seconds(0),
                    FlushPolicyTime::from_unix_seconds(0),
                    0,
                    false,
                ),
            );
            let before = manager.export_chainstate_snapshot().expect("snapshot");
            // Act
            let result = manager.adopt_live_fixture_owner(Chainstate::new());
            // Assert
            assert_eq!(result, Err(ChainstateError::WorkRebindMismatch));
            assert_eq!(
                manager.export_chainstate_snapshot().expect("snapshot"),
                before
            );
        });
    });
}

#[test]
fn phase165_rebind_managed_physical_metadata_cannot_hide_behind_stale_core() {
    // Arrange
    let source = live();
    for alteration in 0..3 {
        with_target(&source, |store, manager| {
            let core_before = manager.chainstate().overlay_snapshot();
            let mut physical = source.snapshot();
            match alteration {
                0 => physical.active_chain[0].median_time_past += 1,
                1 => {
                    physical
                        .undo_by_block
                        .insert(BlockHash::default(), Default::default());
                }
                _ => physical.maybe_confirmed_txid_counts = None,
            }
            store
                .seed_coins_from_snapshot(&physical)
                .expect("changed physical metadata");
            let metadata_before = store.load_chain_meta_for_open().expect("metadata");
            let undo_before = store.load_all_undo_records().expect("undo");
            // Act
            let result = manager.adopt_live_fixture_owner(source.clone());
            // Assert
            assert_eq!(result, Err(ChainstateError::WorkRebindMismatch));
            assert_eq!(manager.chainstate().overlay_snapshot(), core_before);
            assert_eq!(
                store.load_chain_meta_for_open().expect("metadata"),
                metadata_before
            );
            assert_eq!(store.load_all_undo_records().expect("undo"), undo_before);
        });
    }
}

#[test]
fn phase165_rebind_managed_changed_physical_coin_fields_refuse() {
    // Arrange
    let source = live();
    for alteration in 0..5 {
        with_target(&source, |store, manager| {
            let (key, mut coin) = source.utxos().into_iter().next().expect("coin");
            match alteration {
                0 => {
                    coin.output.value =
                        open_bitcoin_core::primitives::Amount::from_sats(49).expect("value")
                }
                1 => {
                    coin.output.script_pubkey =
                        open_bitcoin_core::primitives::ScriptBuf::from_bytes(vec![0x52])
                            .expect("script")
                }
                2 => coin.is_coinbase = false,
                3 => coin.created_height = 99,
                _ => coin.created_median_time_past += 1,
            }
            let encoded = crate::storage::coins_codec::encode_coin_value(&coin).expect("coin");
            store
                .coins_view()
                .write_raw_bytes(
                    &crate::storage::coins_codec::encode_coin_key(&key),
                    encoded.clone(),
                )
                .expect("changed bytes");
            let before = manager.export_chainstate_snapshot().expect("snapshot");
            // Act
            let result = manager.adopt_live_fixture_owner(source.clone());
            // Assert
            assert_eq!(result, Err(ChainstateError::WorkRebindMismatch));
            assert_eq!(
                manager.export_chainstate_snapshot().expect("snapshot"),
                before
            );
            assert_eq!(
                store
                    .coins_view()
                    .read_raw_bytes(&crate::storage::coins_codec::encode_coin_key(&key))
                    .expect("bytes"),
                Some(encoded)
            );
        });
    }
}

#[test]
fn phase165_rebind_managed_corrupt_coin_or_marker_refuses_without_repair() {
    // Arrange
    let source = live();
    for marker in 0..3 {
        with_target(&source, |store, manager| {
            let key = match marker {
                0 => crate::storage::coins_codec::encode_coin_key(
                    source.utxos().keys().next().expect("coin"),
                )
                .to_vec(),
                1 => crate::storage::coins_codec::encode_best_block_key().to_vec(),
                _ => crate::storage::coins_codec::encode_head_blocks_key().to_vec(),
            };
            let bytes = vec![0xff];
            store
                .coins_view()
                .write_raw_bytes(&key, bytes.clone())
                .expect("corrupt fixture bytes");
            let before = manager.chainstate().overlay_snapshot();
            // Act
            let result = manager.adopt_live_fixture_owner(source.clone());
            // Assert
            assert!(result.is_err());
            assert_eq!(manager.chainstate().overlay_snapshot(), before);
            assert_eq!(
                store.coins_view().read_raw_bytes(&key).expect("bytes"),
                Some(bytes)
            );
        });
    }
}

#[test]
fn phase165_rebind_managed_rotation_rejects_previously_prepared_stage() {
    // Arrange
    let source = live();
    with_target(&source, |_, manager| {
        manager
            .adopt_live_fixture_owner(source.clone())
            .expect("first transfer");
        let child = super::build_block(source.tip().expect("tip").block_hash, 1, 50);
        let prepared = manager
            .prepare_connect_block_with_current_time(
                &child,
                ProofWork::ZERO,
                i64::from(child.header.time),
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("valid prepared stage");
        manager
            .adopt_live_fixture_owner(source.clone())
            .expect("same state, fresh owner");
        let before = manager.export_chainstate_snapshot().expect("snapshot");
        // Act
        let result = manager.commit_prepared_connect(prepared);
        // Assert
        assert_eq!(result, Err(ChainstateError::StageOwnerMismatch));
        assert_eq!(
            manager.export_chainstate_snapshot().expect("snapshot"),
            before
        );
        assert!(!manager.has_pending_validation_history());
    });
}

#[test]
fn phase165_rebind_fjall_complete_snapshot_includes_uncached_rows() {
    // Arrange
    let source = live();
    with_target(&source, |store, manager| {
        // Act
        let snapshot = store
            .coins_view()
            .complete_snapshot()
            .expect("complete coherent view");
        // Assert
        assert_eq!(snapshot.coins, source.utxos());
        assert_eq!(
            snapshot.maybe_best_block,
            source.tip().map(|tip| tip.block_hash)
        );
        assert!(snapshot.head_blocks.is_empty());
        assert_eq!(manager.chainstate().coins().cache_entry_count(), 0);
    });
}

#[test]
fn phase165_rebind_fixture_handle_uses_actual_managed_owner() {
    // Arrange
    let source = live();
    with_target(&source, |store, _| {
        let handle = handle(store, false);
        let before = handle.chainstate_snapshot().expect("snapshot");
        // Act
        handle
            .adopt_live_fixture_owner(source.clone())
            .expect("explicit actual owner transfer");
        // Assert
        assert!(handle.has_earned_fixture_work().expect("origin"));
        assert_eq!(handle.chainstate_snapshot().expect("snapshot"), before);
    });
}

#[test]
fn phase165_rebind_canonical_handle_cannot_use_fixture_transfer() {
    // Arrange
    let source = live();
    with_target(&source, |store, _| {
        let handle = handle(store, true);
        let before = handle.chainstate_snapshot().expect("snapshot");
        // Act
        let result = handle.adopt_live_fixture_owner(source.clone());
        // Assert
        assert!(result.is_err());
        assert!(!handle.has_earned_fixture_work().expect("origin"));
        assert_eq!(handle.chainstate_snapshot().expect("snapshot"), before);
    });
}

#[test]
fn phase165_rebind_fixture_counts_only_enrich_missing_matching_physical_state() {
    // Arrange
    let source = live();
    with_target(&source, |store, manager| {
        manager
            .adopt_live_fixture_owner(source.clone())
            .expect("genuine owner");
        let bytes = crate::storage::snapshot_codec::encode_chain_meta(source.active_chain(), None)
            .expect("production-shaped metadata");
        store
            .write_raw_for_test(crate::StorageNamespace::Chainstate, "chain_meta", bytes)
            .expect("missing counts");
        let coins_before = store.coins_view().complete_snapshot().expect("coins");
        // Act
        manager
            .persist_missing_live_fixture_counts()
            .expect("only enrich missing actual counts");
        manager
            .persist_missing_live_fixture_counts()
            .expect("matching present counts are idempotent");
        // Assert
        assert_eq!(
            store.load_chain_meta_for_open().expect("metadata").1,
            source.snapshot().maybe_confirmed_txid_counts
        );
        assert_eq!(
            store.coins_view().complete_snapshot().expect("coins"),
            coins_before
        );
    });
}

#[test]
fn phase165_rebind_fixture_counts_never_replace_conflicting_present_counts() {
    // Arrange
    let source = live();
    with_target(&source, |store, manager| {
        manager
            .adopt_live_fixture_owner(source.clone())
            .expect("genuine owner");
        let mut counts = source
            .snapshot()
            .maybe_confirmed_txid_counts
            .expect("known counts");
        *counts.values_mut().next().expect("txid") += 1;
        let bytes =
            crate::storage::snapshot_codec::encode_chain_meta(source.active_chain(), Some(&counts))
                .expect("conflicting counts");
        store
            .write_raw_for_test(crate::StorageNamespace::Chainstate, "chain_meta", bytes)
            .expect("present conflicting rows");
        let before = manager.chainstate().overlay_snapshot();
        // Act
        let result = manager.persist_missing_live_fixture_counts();
        // Assert
        assert_eq!(result, Err(ChainstateError::WorkRebindMismatch));
        assert_eq!(
            store.load_chain_meta_for_open().expect("metadata").1,
            Some(counts)
        );
        assert_eq!(manager.chainstate().overlay_snapshot(), before);
    });
}

#[test]
fn phase165_rebind_fixture_counts_require_an_actual_flushed_live_owner() {
    // Arrange
    let source = live();
    with_target(&source, |store, manager| {
        let before = store.load_chain_meta_for_open().expect("metadata");
        // Act
        let raw_result = manager.persist_missing_live_fixture_counts();
        manager
            .adopt_live_fixture_owner(source.clone())
            .expect("genuine owner");
        let child = super::build_block(source.tip().expect("tip").block_hash, 1, 50);
        manager
            .connect_block(
                &child,
                ProofWork::ZERO,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("live unflushed child");
        let unflushed_result = manager.persist_missing_live_fixture_counts();
        // Assert
        assert_eq!(raw_result, Err(ChainstateError::UntrustedWorkLineage));
        assert_eq!(unflushed_result, Err(ChainstateError::WorkRebindMismatch));
        assert_eq!(store.load_chain_meta_for_open().expect("metadata"), before);
    });
}
