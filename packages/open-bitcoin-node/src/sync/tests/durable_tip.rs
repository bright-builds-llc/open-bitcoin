// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;
use crate::sync::block_reconcile;

#[test]
fn durable_tip_direct_sync_emits_only_final_durable_best_tip() {
    // Arrange
    let path = temp_store_path("durable-tip-direct-final-only");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let captured = durable_tip_capture(&mut runtime);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone(), child.header.clone()],
        }),
        WireNetworkMessage::Block(genesis),
        WireNetworkMessage::Block(child.clone()),
        WireNetworkMessage::Block(child.clone()),
    ]]);

    // Act
    runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("direct durable tip sync");

    // Assert
    assert_eq!(
        *captured.lock().expect("durable tip capture lock"),
        vec![block_hash(&child.header)]
    );
    assert!(
        runtime
            .store()
            .load_block(block_hash(&child.header))
            .is_ok_and(|block| block.is_some())
    );
    remove_dir_if_exists(&path);
}

#[test]
fn durable_tip_live_reconcile_collapses_multiple_blocks_to_final_tip() {
    // Arrange
    let path = temp_store_path("durable-tip-reconcile-final-only");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    save_chain_headers_snapshot_and_blocks(
        &path,
        &[(&genesis, 0), (&child, 1), (&grandchild, 2)],
        &[(&genesis, 0)],
        &[(&genesis, 0), (&child, 1), (&grandchild, 2)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let captured = durable_tip_capture(&mut runtime);

    // Act
    let progress = block_reconcile::reconcile_best_chain_for_live_session(
        &mut runtime,
        i64::from(grandchild.header.time),
    )
    .expect("live reconciliation");
    runtime
        .persist_progress_and_dispatch_tip()
        .expect("persist reconciled tip");

    // Assert
    assert_eq!(
        progress,
        SyncReconcileProgress::ExtendedActiveChain { connected_count: 2 }
    );
    assert_eq!(
        *captured.lock().expect("durable tip capture lock"),
        vec![block_hash(&grandchild.header)]
    );
    remove_dir_if_exists(&path);
}
