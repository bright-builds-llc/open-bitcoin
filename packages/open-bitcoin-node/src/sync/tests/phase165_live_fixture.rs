// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::live_fixture_support::*;
use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::storage::{
    coins_codec::{
        encode_best_block_key, encode_best_block_value, encode_head_blocks_key,
        encode_head_blocks_value,
    },
    fjall_store::validation_history::HistoryPublicationFault,
    snapshot_codec::encode_chain_meta,
};
use crate::sync::live_fixture::{
    capture_live_fixture_source, check_captured_live_fixture_source, take_stage_observations,
};
use crate::{FjallChainstateStore, ManagedChainstate};
use open_bitcoin_core::{
    chainstate::{AnchoredBlock, BlockUndo, Chainstate, TxUndo},
    consensus::{CanonicalChain, ConsensusParams, ScriptVerifyFlags, chain_proof::ProofWork},
};
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
#[test]
fn phase165_live_fixture_unit_none_counts_remain_unknown_with_basic_enabled() {
    // Arrange
    let mut fixture = Fixture::new(1);
    assert!(
        fixture
            .store
            .load_chain_meta_for_open()
            .expect("metadata")
            .1
            .is_none()
    );
    let counts = fixture
        .source()
        .chainstate()
        .overlay_snapshot()
        .maybe_confirmed_txid_counts;
    // Act
    let runtime =
        open(fixture.take(), params(), BasicFilterStartupMode::Enabled).expect("normal BASIC");
    assert_eq!(runtime.consensus_params, params());
    // Assert
    assert_eq!(
        runtime
            .network
            .chainstate_snapshot()
            .expect("same counts")
            .maybe_confirmed_txid_counts,
        counts
    );
    assert!(
        fixture
            .store
            .load_chain_meta_for_open()
            .expect("no enrichment")
            .1
            .is_none()
    );
    assert!(
        fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("normal proof")
            .is_some()
    );
    assert_eq!(take_stage_observations(), [1, 1]);
    drop(runtime);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_empty_disabled_has_no_root_binding() {
    // Arrange
    let mut fixture = Fixture::new(0);
    // Act
    let runtime = open(fixture.take(), params(), BasicFilterStartupMode::Disabled).expect("empty");
    // Assert
    assert!(runtime.network.maybe_chain_tip().expect("read").is_none());
    assert!(
        runtime
            .network
            .header_entries()
            .expect("headers")
            .is_empty()
    );
    drop(runtime);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_mechanical_refuses_before_read() {
    // Arrange
    let mut fixture = Fixture::new(1);
    let (sink, core) = fixture.take().into_parts();
    let snapshot = core.overlay_snapshot();
    let mechanical = Chainstate::from_parent_for_mechanical_fixture(
        fixture.store.coins_view(),
        snapshot.active_chain,
        snapshot.undo_by_block,
        snapshot.maybe_confirmed_txid_counts,
    );
    fixture.maybe_source = Some(ManagedChainstate::from_chainstate(
        sink,
        mechanical,
        lifecycle(),
    ));
    fixture.refuse(params(), true);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_preview_refuses_before_read() {
    // Arrange
    let mut fixture = Fixture::new(2);
    let (sink, mut core) = fixture.take().into_parts();
    let side = build_branch_block(block_hash(&fixture.blocks[0].header), 1, 10);
    let stage = core
        .stage_reorg(
            &fixture.blocks[1..],
            &[AnchoredBlock {
                block: side,
                chain_work: ProofWork::ZERO,
            }],
            ScriptVerifyFlags::P2SH,
            params(),
        )
        .expect("preview stage");
    core.install_staged_reorg_preview(&stage)
        .expect("actual preview");
    fixture.maybe_source = Some(ManagedChainstate::from_chainstate(sink, core, lifecycle()));
    fixture.refuse(params(), true);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_pending_publication_refuses_before_read() {
    // Arrange
    let mut fixture = Fixture::new(0);
    fixture
        .store
        .set_validation_history_fault(HistoryPublicationFault::BeforeCommit);
    let root = build_block(BlockHash::default(), 0);
    let mut source = fixture.take();
    assert!(
        source
            .connect_block(&root, ProofWork::ZERO, ScriptVerifyFlags::P2SH, params())
            .is_err()
    );
    assert!(source.has_pending_validation_history());
    source
        .chainstate()
        .validate_live_origin()
        .expect("accepted genuine pending core");
    fixture.maybe_source = Some(source);
    fixture.refuse(params(), true);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_foreign_parent_refuses_before_init() {
    // Arrange
    let mut fixture = Fixture::new(1);
    let other = Fixture::new(0);
    let (_, core) = fixture.take().into_parts();
    fixture.maybe_source = Some(ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(other.store.clone()),
        core,
        lifecycle(),
    ));
    take_stage_observations();
    // Act
    assert!(open(fixture.take(), params(), BasicFilterStartupMode::Disabled).is_err());
    // Assert
    assert_eq!(take_stage_observations(), [1, 0]);
    fixture.cleanup();
    other.cleanup();
}
#[test]
fn phase165_live_fixture_unit_full_params_beyond_maturity_refuse() {
    // Arrange
    let mut fixture = Fixture::new(1);
    let different = ConsensusParams {
        enforce_bip34_height_in_coinbase: false,
        ..params()
    };
    fixture.refuse(different, true);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_unflushed_source_refuses_before_init() {
    // Arrange
    let mut fixture =
        Fixture::from_blocks(vec![build_block(BlockHash::default(), 0)], params(), false);
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_alias_extra_refuses() {
    aliased(0);
}
#[test]
fn phase165_live_fixture_unit_alias_missing_refuses() {
    aliased(1);
}
#[test]
fn phase165_live_fixture_unit_alias_changed_refuses() {
    aliased(2);
}
#[test]
fn phase165_live_fixture_unit_physical_best_block_refuses() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .coins_view()
        .write_raw_bytes(
            &encode_best_block_key(),
            encode_best_block_value(BlockHash::from_byte_array([77; 32])),
        )
        .expect("changed B");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_physical_heads_refuse() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .coins_view()
        .write_raw_bytes(
            &encode_head_blocks_key(),
            encode_head_blocks_value(&[BlockHash::from_byte_array([77; 32])]).expect("heads bytes"),
        )
        .expect("changed heads");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_physical_positions_refuse() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            "chain_meta",
            encode_chain_meta(&[], None).expect("encoded metadata"),
        )
        .expect("physical positions");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_conflicting_some_counts_refuse() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            "chain_meta",
            encode_chain_meta(
                fixture.source().chainstate().active_chain(),
                Some(&Default::default()),
            )
            .expect("conflicting counts"),
        )
        .expect("physical Some counts");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_missing_body_refuses_without_repair() {
    let mut fixture = Fixture::new(2);
    remove(
        &fixture.store,
        StorageNamespace::BlockIndex,
        &key("block:", block_hash(&fixture.blocks[0].header)),
    );
    fixture.refuse(params(), false);
    assert!(
        !fixture
            .store
            .has_block(block_hash(&fixture.blocks[0].header))
            .expect("still absent")
    );
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_missing_undo_refuses_without_repair() {
    let mut fixture = Fixture::new(2);
    remove(
        &fixture.store,
        StorageNamespace::Chainstate,
        &key("undo:", block_hash(&fixture.blocks[1].header)),
    );
    fixture.refuse(params(), false);
    assert!(
        !fixture
            .store
            .has_undo(block_hash(&fixture.blocks[1].header))
            .expect("still absent")
    );
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_changed_undo_refuses() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .save_undo(
            block_hash(&fixture.blocks[0].header),
            &BlockUndo {
                transactions: vec![TxUndo::default()],
            },
            PersistMode::Sync,
        )
        .expect("changed undo");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_malformed_body_refuses() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            &key("block:", block_hash(&fixture.blocks[0].header)),
            vec![0xff],
        )
        .expect("malformed body");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_validation_identity_refuses() {
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .invalidate_validation_coverage()
        .expect("unknown coverage");
    fixture.refuse(params(), false);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_canonical_mainnet_pre_read() {
    canonical_case(CanonicalChain::Mainnet);
}
#[test]
fn phase165_live_fixture_unit_canonical_testnet3_pre_read() {
    canonical_case(CanonicalChain::Testnet3);
}
#[test]
fn phase165_live_fixture_unit_canonical_signet_pre_read() {
    canonical_case(CanonicalChain::Signet);
}
#[test]
fn phase165_live_fixture_unit_canonical_regtest_pre_read() {
    canonical_case(CanonicalChain::Regtest);
}

#[test]
fn phase165_live_fixture_unit_post_graph_stage_writer_refuses_captured_identity() {
    // Arrange: actual production stages, not whole-public-constructor concurrency.
    let mut fixture = Fixture::new(1);
    let captured =
        capture_live_fixture_source(fixture.source(), params()).expect("independent capture");
    let network = prepared(fixture.take(), &captured);
    poison(&fixture.store, network.chainstate(), 2);
    network
        .chainstate()
        .chainstate()
        .validate_live_origin()
        .expect("still Live");
    // Act
    let result = check_captured_live_fixture_source(network.chainstate(), &captured);
    // Assert
    assert!(
        result.is_err(),
        "final check compares immutable independently validated capture"
    );
    drop(network);
    fixture.cleanup();
}
#[test]
fn phase165_live_fixture_unit_post_graph_stages_unchanged_positive() {
    // Arrange
    let mut fixture = Fixture::new(2);
    let captured = capture_live_fixture_source(fixture.source(), params()).expect("capture");
    let network = prepared(fixture.take(), &captured);
    // Act
    check_captured_live_fixture_source(network.chainstate(), &captured)
        .expect("stable full identity");
    // Assert
    assert!(
        network
            .chainstate()
            .maybe_accepted_body_preference()
            .is_none()
    );
    assert!(
        network
            .maybe_body_eligibility(block_hash(&fixture.blocks[0].header))
            .expect("root linkage")
            .maybe_sequence
            .is_some()
    );
    assert!(
        network
            .maybe_body_eligibility(block_hash(&fixture.blocks[1].header))
            .expect("historic child")
            .maybe_sequence
            .is_none()
    );
    drop(network);
    fixture.cleanup();
}

#[test]
fn phase165_live_fixture_unit_genesis_graph_new_events_earn_real_earlier_tie() {
    // Arrange
    let mut fixture = Fixture::new(1);
    let a1 = build_block(block_hash(&fixture.blocks[0].header), 1);
    let a2 = build_block(block_hash(&a1.header), 2);
    let b2 = build_branch_block(block_hash(&a1.header), 2, 100);
    let captured = capture_live_fixture_source(fixture.source(), params()).expect("capture");
    let mut network = prepared(fixture.take(), &captured);
    assert!(
        network
            .chainstate()
            .maybe_accepted_body_preference()
            .is_none()
    );
    network
        .connect_outbound_peer(70, 2_000_000_000)
        .expect("modeled peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage::default()),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![a1.header.clone(), a2.header.clone()],
        }),
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![b2.header.clone()],
        }),
    ] {
        network
            .receive_sync_message(
                70,
                message,
                2_000_000_000,
                ScriptVerifyFlags::P2SH,
                params(),
            )
            .expect("normal nonroot contiguous ingress");
    }
    assert!(matches!(
        network
            .connect_stored_block(
                &b2,
                ProofWork::ZERO,
                2_000_000_000,
                ScriptVerifyFlags::P2SH,
                params()
            )
            .expect("actual pending B2"),
        crate::network::BlockConnectDisposition::NonExtending { .. }
    ));
    assert!(
        network
            .maybe_body_eligibility(block_hash(&b2.header))
            .expect("pending B")
            .maybe_sequence
            .is_none()
    );
    network
        .connect_local_block(&a1, ScriptVerifyFlags::P2SH, params())
        .expect("A1 releases B2");
    let b_order = network
        .maybe_body_eligibility(block_hash(&b2.header))
        .expect("eligible B")
        .maybe_sequence
        .expect("earned B order");
    network
        .connect_local_block(&a2, ScriptVerifyFlags::P2SH, params())
        .expect("later A2 acceptance");
    assert!(
        network
            .chainstate()
            .maybe_accepted_body_preference()
            .is_some()
    );
    assert!(
        b_order
            < network
                .maybe_body_eligibility(block_hash(&a2.header))
                .expect("A facts")
                .maybe_sequence
                .expect("A order")
    );
    for body in [&a1, &a2, &b2] {
        fixture
            .store
            .save_block(body, PersistMode::Sync)
            .expect("retain checked bodies for BASIC");
    }
    for (hash, undo) in network.chainstate().chainstate().undo_by_block() {
        fixture
            .store
            .save_undo(*hash, undo, PersistMode::Sync)
            .expect("retain actual accepted undo for BASIC");
    }
    // Act
    network
        .reorg_to_branch(
            &[a2],
            &[AnchoredBlock {
                block: b2.clone(),
                chain_work: ProofWork::ZERO,
            }],
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(2_000_000_000)),
            ScriptVerifyFlags::P2SH,
            params(),
        )
        .expect("genuinely earlier eligible equal-work reorg");
    // Assert
    assert_eq!(
        network.maybe_chain_tip().expect("tip").block_hash,
        block_hash(&b2.header)
    );
    drop(network);
    fixture.cleanup();
}

#[test]
fn phase165_live_fixture_unit_masked_extra_refuses() {
    masked(0);
}
#[test]
fn phase165_live_fixture_unit_masked_missing_refuses() {
    masked(1);
}
#[test]
fn phase165_live_fixture_unit_masked_changed_refuses() {
    masked(2);
}

#[test]
fn phase165_live_fixture_unit_raw_empty_refuses_before_read() {
    // Arrange
    let mut fixture = Fixture::new(0);
    let (sink, _core) = fixture.take().into_parts();
    let raw = Chainstate::from_parent(fixture.store.coins_view(), vec![], Default::default(), None);
    fixture.maybe_source = Some(ManagedChainstate::from_chainstate(sink, raw, lifecycle()));
    // Act / Assert
    fixture.refuse(params(), true);
    fixture.cleanup();
}

#[test]
fn phase165_live_fixture_unit_actual_pruned_prefix_refuses_without_restoration() {
    // Arrange
    let mut fixture = Fixture::new(2);
    let hash = block_hash(&fixture.blocks[0].header);
    fixture
        .store
        .commit_paired_delete(0, hash)
        .expect("actual paired body/undo prune");
    assert!(fixture.store.load_have_pruned().expect("pruned"));
    // Act / Assert
    fixture.refuse(params(), false);
    assert!(!fixture.store.has_block(hash).expect("no restoration"));
    assert!(!fixture.store.has_undo(hash).expect("no undo restoration"));
    fixture.cleanup();
}

#[test]
fn phase165_live_fixture_unit_malformed_undo_refuses() {
    // Arrange
    let mut fixture = Fixture::new(1);
    fixture
        .store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            &key("undo:", block_hash(&fixture.blocks[0].header)),
            vec![0xff],
        )
        .expect("malformed undo");
    // Act / Assert
    fixture.refuse(params(), false);
    fixture.cleanup();
}

fn key(prefix: &str, hash: BlockHash) -> String {
    format!(
        "{prefix}{}",
        hash.as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    )
}
fn remove(store: &FjallNodeStore, namespace: StorageNamespace, key: &str) {
    let space = store
        .database()
        .keyspace(namespace.as_str(), fjall::KeyspaceCreateOptions::default)
        .expect("existing namespace");
    space.remove(key).expect("remove physical input");
}

#[test]
fn phase165_live_fixture_unit_historical_order_stays_unknown_after_duplicates_and_copy() {
    historical_order_controls();
}
