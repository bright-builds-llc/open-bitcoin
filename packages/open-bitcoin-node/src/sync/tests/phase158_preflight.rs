// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::chainstate::ReorgFixture;

fn runtime(name: &str) -> (std::path::PathBuf, DurableSyncRuntime, Vec<AnchoredBlock>) {
    let mut fixture = ReorgFixture::new(name, 3);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage(3);
    let replacement: Vec<_> = staged
        .transition()
        .connected
        .iter()
        .map(|position| AnchoredBlock {
            block: fixture
                .store
                .load_block(position.block_hash)
                .expect("body")
                .expect("retained"),
            chain_work: position.chain_work,
        })
        .collect();
    let positions = fixture
        .manager
        .chainstate()
        .active_chain()
        .iter()
        .chain(staged.transition().connected.iter());
    let headers = open_bitcoin_network::HeaderStore::from_entries(positions.map(|position| {
        open_bitcoin_network::HeaderEntry {
            block_hash: position.block_hash,
            header: position.header.clone(),
            height: position.height,
            chain_work: position.chain_work,
        }
    }))
    .expect("genuine accepted fork headers");
    fixture
        .store
        .save_header_entries(
            &headers.entries().cloned().collect::<Vec<_>>(),
            crate::PersistMode::Sync,
        )
        .expect("header store");
    fixture
        .manager
        .persist_missing_live_fixture_counts()
        .expect("actual live flushed source counts");
    drop(staged);
    drop(fixture);
    let store = crate::FjallNodeStore::open(&path).expect("actual reopen");
    let mut runtime = DurableSyncRuntime::open_configured(
        store,
        super::super::SyncRuntimeConfig {
            network: super::super::SyncNetwork::Regtest,
            ..Default::default()
        },
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("production startup");
    runtime.consensus_params.coinbase_maturity = 1;
    let snapshot = runtime
        .network
        .chainstate_snapshot()
        .expect("actual recovered target");
    let mut genuine = open_bitcoin_core::chainstate::Chainstate::new();
    let mut active_bodies = Vec::new();
    for position in snapshot.active_chain {
        let block = runtime
            .store
            .load_block(position.block_hash)
            .expect("actual retained prefix read")
            .expect("complete actual fixture prefix");
        genuine
            .connect_block(
                &block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("actual full active fixture validation");
        active_bodies.push(block);
    }
    runtime
        .network
        .adopt_live_fixture_owner(genuine)
        .expect("complete matching fixture transfer only");
    runtime
        .network_handle()
        .try_admit_fixture_headers(headers, i64::MAX, runtime.consensus_params)
        .expect("explicit actual fork header admission; raw reopen earns no authority");
    for block in active_bodies
        .iter()
        .chain(replacement.iter().map(|candidate| &candidate.block))
    {
        runtime
            .network
            .connect_stored_block(
                block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                i64::from(block.header.time),
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("actual checked retained fixture body history; restart tie remains unknown");
    }
    (path, runtime, replacement)
}

#[test]
fn phase158_preflight_sync_reconcile_uses_production_reorg_and_scheduled_owner() {
    // Arrange
    let (path, mut runtime, replacement) = runtime("sync-driver-reorg");
    // Act
    let result = reconcile_best_chain(&mut runtime, 2000).expect("actual sync reorg");
    let turn = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("idle ordinary turn");
    // Assert
    assert!(matches!(result, SyncReconcileProgress::ReorgPersisted(_)));
    assert_eq!(
        turn.maybe_accepted_target.expect("accepted").block_hash(),
        block_hash(&replacement[2].block.header)
    );
    assert_eq!(turn.maybe_accepted_lag, Some(0));
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_preflight_sync_missing_undo_is_explicit_storage_refusal_before_preview() {
    // Arrange
    let (path, mut runtime, _) = runtime("sync-driver-loss");
    let old = runtime.network.chainstate_snapshot().expect("before");
    runtime
        .store
        .delete_basic_input_for_test(old.active_chain[2].block_hash, true)
        .expect("actual missing durable mate");
    // Act
    let result = reconcile_best_chain(&mut runtime, 2000);
    // Assert
    assert!(
        matches!(result, Err(SyncRuntimeError::Storage(StorageError::Corruption { ref detail, .. })) if detail.contains("required undo"))
    );
    assert_eq!(runtime.network.chainstate_snapshot().expect("after"), old);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}
