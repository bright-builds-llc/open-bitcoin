// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::validation_history::tests::block;
use crate::{FjallChainstateStore, FjallCoinsView, FjallNodeStore, PersistMode};
use crate::{ManagedChainstate, MemoryChainstateStore};
use open_bitcoin_core::chainstate::{Chainstate, CoinsCache, FlushPolicyTime};
use open_bitcoin_core::{chainstate::ChainPosition, primitives::BlockHash};
use open_bitcoin_network::HeaderStore;

fn persistent(
    name: &str,
) -> (
    std::path::PathBuf,
    FjallNodeStore,
    ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
) {
    let dir = std::env::temp_dir().join(format!(
        "phase160-branch-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    let store = FjallNodeStore::open(&dir).expect("store");
    let mut state = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        Vec::new(),
        Default::default(),
        None,
    );
    Chainstate::new()
        .rebind_into(&mut state)
        .expect("genuine empty source");
    let manager = ManagedChainstate::from_recovered_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        state,
        crate::chainstate::FlushLifecycle::ready(
            FlushPolicyTime::from_unix_seconds(0),
            FlushPolicyTime::from_unix_seconds(0),
            0,
            false,
        ),
    )
    .expect("recovered");
    (dir, store, manager)
}

fn network<S: ChainstateStore, V: CoinsView>(
    manager: ManagedChainstate<S, V>,
    headers: HeaderStore,
) -> ManagedNetworkHandle<S, V> {
    let mut network = ManagedPeerNetwork::from_initialized_chainstate(
        manager,
        Default::default(),
        Default::default(),
        16,
        Default::default(),
        Default::default(),
        false,
    );
    network
        .try_admit_fixture_headers(headers, i64::MAX, ConsensusParams::default())
        .expect("actual selected fixture headers");
    network.set_basic_filter_serving(true);
    network.connect_outbound_peer(1, 0).expect("session");
    ManagedNetworkHandle::new(network)
}

fn limit() -> BasicFilterBranchLimit {
    BasicFilterBranchLimit {
        branch: BranchWorkLimit {
            ancestry_steps: 10_000,
            identity_bytes: 1_000_000,
        },
        metadata_units: 10_000,
    }
}

#[test]
fn phase160_branch_active_uses_own_session_and_direct_identity() {
    // Arrange
    let genesis = block(BlockHash::default(), 0, 0x51);
    let position = ChainPosition::new(
        genesis.header.clone(),
        0,
        open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
        0,
    );
    let state = open_bitcoin_core::chainstate::Chainstate::from_coins_cache(
        open_bitcoin_core::chainstate::CoinsCache::from_parent(
            open_bitcoin_core::chainstate::MemoryCoinsView::default(),
        ),
        vec![position.clone()],
        Default::default(),
        None,
    );
    let manager = ManagedChainstate::from_chainstate(
        MemoryChainstateStore::default(),
        state,
        crate::chainstate::FlushLifecycle::ready(
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
            0,
            false,
        ),
    );
    let mut network = ManagedPeerNetwork::from_initialized_chainstate(
        manager,
        Default::default(),
        Default::default(),
        16,
        Default::default(),
        Default::default(),
        false,
    );
    network.set_basic_filter_serving(true);
    network.connect_outbound_peer(1, 0).expect("advertise");
    let handle = ManagedNetworkHandle::new(network);
    // Act
    let outcome = handle
        .capture_basic_filter_branch(
            1,
            FilterRequest::Filters {
                filter_type: 0,
                start_height: 0,
                stop_hash: position.block_hash,
            },
            ConsensusParams::default(),
            limit(),
        )
        .expect("capture");
    // Assert
    let BasicFilterBranchOutcome::Authorized(capture) = outcome else {
        panic!("expected active authorization: {outcome:?}");
    };
    assert_eq!(capture.branch.identities[0].hash, position.block_hash);
    assert!(
        capture
            .own_services()
            .contains(ServiceFlags::COMPACT_FILTERS)
    );
    assert_eq!(capture.peer_id(), 1);
    assert!(capture.maybe_index_generation().is_none());
}

#[test]
fn phase160_branch_genuine_disconnected_scripts_valid_survives_paired_pruning() {
    // Arrange
    let (dir, store, mut manager) = persistent("genuine-pruned");
    let genesis = block(BlockHash::default(), 0, 0x51);
    let genesis_position = manager
        .connect_block(
            &genesis,
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("genuine genesis");
    let stale = block(genesis_position.block_hash, 1, 0x51);
    let stale_position = manager
        .connect_block(
            &stale,
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("genuine child");
    manager
        .disconnect_tip(&stale)
        .expect("retain accepted history");
    let mut headers = HeaderStore::default();
    headers.record_position(&genesis_position);
    headers.record_position(&stale_position);
    store
        .save_block(&stale, PersistMode::Sync)
        .expect("body present");
    assert_eq!(
        store
            .validation_provenance(stale_position.block_hash)
            .expect("provenance"),
        ValidationProvenance::ScriptsValid
    );
    store
        .commit_paired_delete(1, stale_position.block_hash)
        .expect("prune paired payload");
    let handle = network(manager, headers);
    // Act
    let outcome = handle
        .capture_basic_filter_branch(
            1,
            FilterRequest::Headers {
                filter_type: 0,
                start_height: 1,
                stop_hash: stale_position.block_hash,
            },
            ConsensusParams::default(),
            limit(),
        )
        .expect("capture");
    // Assert
    assert!(
        store
            .load_block(stale_position.block_hash)
            .expect("body query")
            .is_none()
    );
    let BasicFilterBranchOutcome::Authorized(capture) = outcome else {
        panic!("accepted stale history: {outcome:?}");
    };
    assert_eq!(capture.branch.identities[0].hash, stale_position.block_hash);
    assert_eq!(
        capture.branch.maybe_predecessor.expect("parent").hash,
        genesis_position.block_hash
    );
    assert_eq!(
        capture.metadata_units,
        MAX_VALIDATION_IDENTITIES as u64 + 10
    );
    assert_eq!(
        capture.authority_incarnation(),
        handle
            .read(|network| network.authority_epoch.raw())
            .expect("authority")
    );
    drop(handle);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase160_branch_header_body_and_legacy_rows_never_grant_scripts_validation() {
    // Arrange
    let (dir, store, mut manager) = persistent("negative-provenance");
    let genesis = block(BlockHash::default(), 0, 0x51);
    let position = manager
        .connect_block(
            &genesis,
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("genuine root");
    let header_only = block(position.block_hash, 1, 0x52);
    let mut headers = HeaderStore::default();
    headers
        .insert_header(genesis.header.clone())
        .expect("actual root header");
    let stop = headers
        .insert_header(header_only.header.clone())
        .expect("header only")
        .block_hash;
    let handle = network(manager, headers);
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: stop,
    };
    // Act / Assert: each source variant remains insufficient.
    assert_eq!(
        store.validation_provenance(stop).expect("never connected"),
        ValidationProvenance::NeverConnected
    );
    assert_eq!(
        handle
            .capture_basic_filter_branch(1, request, ConsensusParams::default(), limit())
            .expect("header only"),
        BasicFilterBranchOutcome::Invalid(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
    store
        .save_block(&header_only, PersistMode::Sync)
        .expect("raw body");
    assert_eq!(
        handle
            .capture_basic_filter_branch(1, request, ConsensusParams::default(), limit())
            .expect("body presence"),
        BasicFilterBranchOutcome::Invalid(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
    let entries = handle.header_entries().expect("snapshot");
    store
        .save_header_entries(&entries, PersistMode::Sync)
        .expect("raw snapshot invalidates coverage");
    assert_eq!(
        store.validation_provenance(stop).expect("raw legacy"),
        ValidationProvenance::UnknownLegacy
    );
    assert_eq!(
        handle
            .capture_basic_filter_branch(1, request, ConsensusParams::default(), limit())
            .expect("legacy"),
        BasicFilterBranchOutcome::Invalid(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
    drop(handle);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase160_branch_invalid_gate_precedence_and_metadata_budget() {
    // Arrange
    let (dir, store, mut manager) = persistent("budget");
    let root = block(BlockHash::default(), 0, 0x51);
    let position = manager
        .connect_block(
            &root,
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("root");
    let mut headers = HeaderStore::default();
    headers.record_position(&position);
    let handle = network(manager, headers);
    let saturated = BasicFilterBranchLimit {
        branch: BranchWorkLimit {
            ancestry_steps: 0,
            identity_bytes: 0,
        },
        metadata_units: 0,
    };
    // Act / Assert
    for (peer, kind, stop, start, reason) in [
        (
            99,
            0,
            BlockHash::default(),
            9,
            FilterRequestInvalid::UnsupportedTypeOrCapability,
        ),
        (
            1,
            255,
            BlockHash::default(),
            9,
            FilterRequestInvalid::UnsupportedTypeOrCapability,
        ),
        (
            1,
            0,
            BlockHash::default(),
            9,
            FilterRequestInvalid::UnknownOrDisallowedStop,
        ),
        (
            1,
            0,
            position.block_hash,
            9,
            FilterRequestInvalid::ReversedRange,
        ),
    ] {
        assert_eq!(
            handle
                .capture_basic_filter_branch(
                    peer,
                    FilterRequest::Filters {
                        filter_type: kind,
                        start_height: start,
                        stop_hash: stop
                    },
                    ConsensusParams::default(),
                    saturated
                )
                .expect("invalid before saturation"),
            BasicFilterBranchOutcome::Invalid(reason)
        );
    }
    assert_eq!(
        handle
            .capture_basic_filter_branch(
                1,
                FilterRequest::Filters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: position.block_hash
                },
                ConsensusParams::default(),
                saturated
            )
            .expect("local resource"),
        BasicFilterBranchOutcome::ResourceRefused(BranchResourceRefusal::MetadataWork)
    );
    drop(handle);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}
