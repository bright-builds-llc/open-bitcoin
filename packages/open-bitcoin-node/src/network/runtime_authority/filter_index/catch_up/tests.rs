// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::chainstate::ReorgFixture;
use crate::network::ManagedPeerNetwork;
use open_bitcoin_core::primitives::{NetworkAddress, NetworkMagic};
use open_bitcoin_network::LocalPeerConfig;
mod faults;

#[test]
fn phase158_preflight_publication_failure_exposes_new_accepted_target_and_keeps_persistence_pause()
{
    // Arrange
    let fixture = ReorgFixture::new("driver-visible-failed", 3);
    let path = fixture.path().to_owned();
    let store = fixture.store.clone();
    let (disconnect, replacement) = branch(&fixture, 2);
    let target = open_bitcoin_core::consensus::block_hash(&replacement[1].block.header);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    let old = network
        .maybe_basic_index_progress()
        .expect("old")
        .expect("owner");
    store.set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeRecords,
    );
    // Act
    assert!(reorg(&network, &disconnect, &replacement).is_err());
    let observed = network
        .read(|network| network.chainstate().maybe_basic_index_accepted_target())
        .expect("serialized observation");
    let turn = network.drive_basic_filter_index_turn();
    // Assert
    assert_eq!(
        observed.expect("genuine accepted replacement").block_hash(),
        target
    );
    assert!(
        turn.expect_err("pending publication cannot advance")
            .to_string()
            .contains("pending publication")
    );
    let progress = network
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("owner");
    assert_eq!(
        progress.maybe_processed_endpoint(),
        old.maybe_processed_endpoint()
    );
    assert_eq!(
        progress.state(),
        BasicIndexState::Paused(
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexPause::Persistence
        )
    );
    drop(network);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_full_disconnect_ordinary_turn_reports_absent_accepted_target() {
    // Arrange
    let fixture = ReorgFixture::new("driver-full-disconnect", 3);
    let path = fixture.path().to_owned();
    let disconnect: Vec<_> = fixture
        .manager
        .chainstate()
        .active_chain()
        .iter()
        .rev()
        .map(|p| {
            fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained")
        })
        .collect();
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    // Act
    reorg(&network, &disconnect, &[]).expect("full genuine disconnect");
    let turn = network
        .drive_basic_filter_index_turn()
        .expect("no nonempty turn");
    // Assert
    assert!(turn.maybe_accepted_target.is_none());
    assert!(turn.maybe_accepted_lag.is_none());
    assert_eq!(turn.work, TurnWork::default());
    assert!(matches!(
        turn.maybe_progress.expect("conservative owner").state(),
        BasicIndexState::Paused(_)
    ));
    drop(network);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_genuine_later_connect_advances_visible_target_and_historical_facts() {
    // Arrange
    let fixture = ReorgFixture::new("driver-later-connect", 3);
    let path = fixture.path().to_owned();
    let (disconnect, replacement) = branch(&fixture, 3);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    reorg(&network, &disconnect, &replacement[..2]).expect("genuine replacement");
    network
        .drive_basic_filter_index_turn()
        .expect("reach replacement tip");
    // Act
    network
        .connect_stored_block(
            &replacement[2].block,
            replacement[2].chain_work,
            2000,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("genuine next child");
    let facts = network
        .accepted_basic_facts_for_test()
        .expect("facts")
        .expect("next-height capture");
    let turn = network
        .drive_basic_filter_index_turn()
        .expect("same ordered owner");
    // Assert
    assert_eq!(facts.spent_scripts.len(), 2);
    assert_eq!(
        turn.maybe_accepted_target.expect("accepted").block_hash(),
        open_bitcoin_core::consensus::block_hash(&replacement[2].block.header)
    );
    assert_eq!(turn.maybe_accepted_lag, Some(0));
    assert_eq!(turn.body_reads, 0);
    assert_eq!(turn.generations, 1);
    drop(network);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn reorg(
    network: &ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>,
    disconnect: &[open_bitcoin_core::primitives::Block],
    replacement: &[open_bitcoin_core::chainstate::AnchoredBlock],
) -> Result<open_bitcoin_core::chainstate::ChainTransition, ManagedNetworkAuthorityError> {
    network.reorg_to_branch(
        disconnect,
        replacement,
        open_bitcoin_mempool::ReorgLifecycleContext::new(
            open_bitcoin_mempool::PolicyTime::from_unix_seconds(2000),
        ),
        open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
        open_bitcoin_core::consensus::ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        },
    )
}

fn one_block_budget() -> BasicIndexTurnBudget {
    let budget = production_budget().expect("budget");
    let mut normal = budget.normal();
    normal.blocks = 1;
    BasicIndexTurnBudget::new(normal, budget.absolute_singleton()).expect("one admitted block")
}

fn handle(fixture: ReorgFixture) -> ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView> {
    let local = LocalPeerConfig {
        magic: NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        services: Default::default(),
        address: NetworkAddress {
            services: 0,
            address_bytes: [0; 16],
            port: 18444,
        },
        nonce: 1,
        relay: false,
        user_agent: "/phase158/".to_owned(),
    };
    ManagedNetworkHandle::from_network_fixture(ManagedPeerNetwork::from_initialized_chainstate(
        fixture.manager,
        local,
        Default::default(),
        16,
        Default::default(),
        Default::default(),
        false,
    ))
}

fn branch(
    fixture: &ReorgFixture,
    count: usize,
) -> (
    Vec<open_bitcoin_core::primitives::Block>,
    Vec<open_bitcoin_core::chainstate::AnchoredBlock>,
) {
    let (stage, _) = fixture.stage(count);
    let disconnect = stage
        .transition()
        .disconnected
        .iter()
        .map(|p| {
            fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained")
        })
        .collect();
    let replacement = stage
        .transition()
        .connected
        .iter()
        .map(|p| open_bitcoin_core::chainstate::AnchoredBlock {
            block: fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained"),
            chain_work: p.chain_work,
        })
        .collect();
    (disconnect, replacement)
}

#[test]
fn phase158_preflight_production_reorg_scheduled_turn_reaches_equal_height_target() {
    // Arrange
    let fixture = ReorgFixture::new("driver-equal", 3);
    let path = fixture.path().to_owned();
    let store = fixture.store.clone();
    let (disconnect, replacement) = branch(&fixture, 2);
    let target = open_bitcoin_core::consensus::block_hash(&replacement[1].block.header);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    // Act
    network
        .reorg_to_branch(
            &disconnect,
            &replacement,
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(2000),
            ),
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("actual production reorg");
    let outcome = network
        .drive_basic_filter_index_turn()
        .expect("scheduled production turn");
    // Assert
    assert_eq!(
        outcome
            .maybe_progress
            .expect("progress")
            .maybe_processed_endpoint()
            .expect("processed")
            .block_hash(),
        target
    );
    assert!(outcome.work.checkpoint_operations > 0);
    drop(network);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_later_scheduled_turn_uses_accepted_chain_after_stage_consumed() {
    // Arrange
    let fixture = ReorgFixture::new("driver-later", 1);
    let path = fixture.path().to_owned();
    let (disconnect, replacement) = branch(&fixture, 2);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    reorg(&network, &disconnect, &replacement).expect("accepted");
    // Act
    let first = network
        .drive_basic_filter_index_turn_with_budget(one_block_budget())
        .expect("first");
    let second = network
        .drive_basic_filter_index_turn_with_budget(one_block_budget())
        .expect("later, no stage retained");
    // Assert
    assert_eq!(first.work.blocks, 1);
    assert_eq!(first.maybe_accepted_lag, Some(1));
    assert_eq!(second.work.blocks, 1);
    assert_eq!(second.maybe_accepted_lag, Some(0));
    assert_eq!(
        second
            .maybe_progress
            .expect("progress")
            .maybe_processed_endpoint()
            .expect("processed")
            .block_hash(),
        open_bitcoin_core::consensus::block_hash(&replacement[1].block.header)
    );
    drop(network);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_lagging_common_immutable_row_reuses_unavailable_payload() {
    // Arrange
    let fixture = ReorgFixture::new("driver-immutable", 1);
    let path = fixture.path().to_owned();
    let store = fixture.store.clone();
    let (stage, _) = fixture.stage_from(1, 1);
    let replacement: Vec<_> = stage
        .transition()
        .connected
        .iter()
        .map(|p| open_bitcoin_core::chainstate::AnchoredBlock {
            block: store
                .load_block(p.block_hash)
                .expect("body")
                .expect("present"),
            chain_work: p.chain_work,
        })
        .collect();
    let disconnect: Vec<_> = stage
        .transition()
        .disconnected
        .iter()
        .map(|p| {
            store
                .load_block(p.block_hash)
                .expect("body")
                .expect("present")
        })
        .collect();
    let old = &fixture.records[1];
    let old_hash = old.identity().block_hash();
    store
        .write_raw_for_test(
            crate::StorageNamespace::BlockIndex,
            &crate::storage::filter_index::record_key(old.identity().block_hash()),
            crate::storage::filter_index::encode_record(old),
        )
        .expect("genuine immutable ahead row");
    store
        .delete_basic_input_for_test(old.identity().block_hash(), false)
        .expect("physical missing body");
    store
        .delete_basic_input_for_test(old.identity().block_hash(), true)
        .expect("physical missing undo");
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    // Act
    reorg(&network, &disconnect, &replacement)
        .expect("no consensus disconnect at pruned common height");
    let outcome = network
        .drive_basic_filter_index_turn()
        .expect("reuse + replacement");
    // Assert
    assert_eq!(outcome.reused_records, 1);
    assert_eq!(outcome.generations, 1);
    assert_eq!(outcome.maybe_accepted_lag, Some(0));
    assert!(!store.has_block(old_hash).expect("still absent"));
    drop(network);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_composed_preparation_work_contains_native_sources_and_capture() {
    // Arrange
    let fixture = ReorgFixture::new("driver-preparation-ledger", 3);
    let path = fixture.path().to_owned();
    let (disconnect, replacement) = branch(&fixture, 2);
    // Act
    let prepared = fixture
        .manager
        .prepare_reorg(
            &disconnect,
            &replacement,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("genuine preparation");
    let work = prepared
        .maybe_basic_filter_preparation_work()
        .expect("complete preparation ledger");
    // Assert
    assert!(work.body_bytes > 0);
    assert!(work.undo_bytes > 0);
    assert!(work.cloned_bytes > work.body_bytes);
    assert!(work.checkpoint_operations > 0);
    assert!(work.record_operations >= 4);
    drop(prepared);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_production_reorg_revokes_already_owned_old_append() {
    // Arrange
    let fixture = ReorgFixture::new("driver-stale-append", 2);
    let path = fixture.path().to_owned();
    let store = fixture.store.clone();
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("active");
    let old_append = store
        .prepare_basic_filter_append(proof, &fixture.records[2..])
        .expect("old owned append");
    let (disconnect, replacement) = branch(&fixture, 2);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    // Act
    reorg(&network, &disconnect, &replacement).expect("genuine network reorg");
    let before = network.maybe_basic_index_progress().expect("before");
    let stale = store.complete_basic_filter_append(old_append);
    // Assert
    assert!(stale.is_err());
    assert_eq!(network.maybe_basic_index_progress().expect("after"), before);
    assert_eq!(
        network
            .drive_basic_filter_index_turn()
            .expect("current scheduled work")
            .maybe_accepted_lag,
        Some(0)
    );
    drop(network);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
