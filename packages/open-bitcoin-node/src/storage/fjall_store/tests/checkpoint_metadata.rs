// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

use super::*;
use crate::storage::snapshot_codec::{decode_chain_meta, encode_chain_meta};

const META: &str = "chain_meta";

fn store(label: &str) -> (std::path::PathBuf, FjallNodeStore) {
    let path = temp_store_path(label);
    let store = FjallNodeStore::open(&path).expect("owned metadata-only fixture");
    (path, store)
}

fn ancestry() -> Vec<ChainPosition> {
    let mut positions = leftover_snapshot().0.active_chain;
    let mut header = positions[0].header.clone();
    header.previous_block_hash = positions[0].block_hash;
    header.time += 600;
    positions.push(ChainPosition::new(
        header,
        1,
        ChainPosition::untrusted_fixture_work(2),
        601,
    ));
    positions
}

fn seed(
    store: &FjallNodeStore,
    positions: &[ChainPosition],
    maybe_counts: Option<&HashMap<Txid, u32>>,
) {
    store
        .chainstate
        .insert(
            META,
            encode_chain_meta(positions, maybe_counts).expect("controlled current-format data"),
        )
        .expect("test-only bytes; no live authority");
}

fn saved(store: &FjallNodeStore) -> crate::storage::snapshot_codec::HydratedChainMeta {
    decode_chain_meta(
        &store
            .get_bytes(StorageNamespace::Chainstate, META)
            .expect("metadata read")
            .expect("actual persisted metadata"),
    )
    .expect("current metadata")
}

fn cleanup(path: std::path::PathBuf, store: FjallNodeStore) {
    drop(store);
    std::fs::remove_dir_all(path).expect("owned fixture cleanup");
}

#[test]
fn phase166_checkpoint_metadata_identical_ancestry_preserves_explicit_empty_counts() {
    // Arrange
    let (path, store) = store("checkpoint-empty-counts");
    let positions = ancestry();
    seed(&store, &positions, Some(&HashMap::new()));
    // Act
    store
        .save_validated_chain_meta(&positions, PersistMode::Sync)
        .expect("same ancestry checkpoint");
    // Assert
    assert_eq!(saved(&store), (positions, Some(HashMap::new())));
    cleanup(path, store);
}

#[test]
fn phase166_checkpoint_metadata_identical_ancestry_preserves_nonempty_counts() {
    // Arrange
    let (path, store) = store("checkpoint-nonempty-counts");
    let positions = ancestry();
    let counts = HashMap::from([(Txid::from_byte_array([0x42; 32]), 2)]);
    seed(&store, &positions, Some(&counts));
    // Act
    store
        .save_validated_chain_meta(&positions, PersistMode::Sync)
        .expect("same ancestry checkpoint");
    // Assert
    assert_eq!(saved(&store), (positions, Some(counts)));
    cleanup(path, store);
}

#[test]
fn phase166_checkpoint_metadata_changed_ancestry_discards_counts_even_when_tip_agrees() {
    for change_interior in [false, true] {
        // Arrange
        let (path, store) = store(if change_interior {
            "checkpoint-interior"
        } else {
            "checkpoint-tip"
        });
        let positions = ancestry();
        seed(
            &store,
            &positions,
            Some(&HashMap::from([(Txid::from_byte_array([0x42; 32]), 2)])),
        );
        let mut proposed = positions.clone();
        if change_interior {
            proposed[0].median_time_past += 1;
            assert_eq!(
                proposed.last(),
                positions.last(),
                "same tip alone cannot retain counts"
            );
        } else {
            proposed.pop();
        }
        // Act
        store
            .save_validated_chain_meta(&proposed, PersistMode::Sync)
            .expect("changed ancestry checkpoint");
        // Assert
        assert_eq!(saved(&store), (proposed, None));
        cleanup(path, store);
    }
}

#[test]
fn phase166_checkpoint_metadata_missing_or_legacy_counts_are_never_inferred_empty() {
    for legacy_present in [false, true] {
        // Arrange
        let (path, store) = store(if legacy_present {
            "checkpoint-legacy"
        } else {
            "checkpoint-missing"
        });
        let positions = ancestry();
        assert!(
            store
                .get_bytes(StorageNamespace::Chainstate, META)
                .expect("actual absence")
                .is_none()
        );
        if legacy_present {
            seed(&store, &positions, None);
        }
        // Act
        store
            .save_validated_chain_meta(&positions, PersistMode::Sync)
            .expect("metadata checkpoint");
        // Assert
        assert_eq!(saved(&store), (positions, None));
        cleanup(path, store);
    }
}

#[test]
fn phase166_checkpoint_metadata_malformed_bytes_fail_before_replacement_or_invalidation() {
    // Arrange
    let (path, store) = store("checkpoint-malformed");
    let positions = ancestry();
    store
        .chainstate
        .insert(META, b"{malformed-private-payload")
        .expect("test-only malformed bytes");
    let before = store
        .get_bytes(StorageNamespace::Chainstate, META)
        .expect("saved bytes");
    let revision = store.filter_publication_guard().expect("revision").revision;
    let sequence = store.db.seqno();
    // Act
    let result = store.save_validated_chain_meta(&positions, PersistMode::Sync);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store
            .get_bytes(StorageNamespace::Chainstate, META)
            .expect("unchanged bytes"),
        before
    );
    assert_eq!(
        store
            .filter_publication_guard()
            .expect("same revision")
            .revision,
        revision
    );
    assert_eq!(
        store.db.seqno(),
        sequence,
        "no metadata or coverage replacement"
    );
    cleanup(path, store);
}

#[test]
fn phase166_checkpoint_metadata_public_raw_writer_keeps_invalidation_contract() {
    // Arrange
    let path = temp_store_path("checkpoint-public-invalidation");
    let chain = open_bitcoin_core::consensus::CanonicalChain::Regtest;
    let opened =
        FjallNodeStore::open_canonical_genesis(&path, chain).expect("product canonical source");
    let runtime = crate::DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        crate::SyncRuntimeConfig {
            network: crate::SyncNetwork::Regtest,
            ..Default::default()
        },
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("actual index owner");
    let proof = runtime
        .store()
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("genuine append authority");
    let (positions, _) = runtime
        .store()
        .load_chain_meta_for_open()
        .expect("actual ancestry");
    // Act
    runtime
        .store()
        .save_chain_meta(&positions, PersistMode::Sync)
        .expect("explicit public raw write");
    // Assert
    assert!(
        runtime
            .store()
            .check_basic_filter_append_proof(&proof)
            .is_err()
    );
    assert!(
        runtime
            .store()
            .load_chain_meta_for_open()
            .expect("raw metadata")
            .1
            .is_none()
    );
    drop(proof);
    drop(runtime);
    assert!(matches!(
        FjallNodeStore::reopen_canonical_genesis_existing(&path, chain),
        Err(crate::storage::CanonicalBootstrapRefusal::MissingRecord)
    ));
    std::fs::remove_dir_all(path).expect("owned fixture cleanup");
}
