// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::*;
use open_bitcoin_node::core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, ChainPosition, ChainstateSnapshot,
};
use open_bitcoin_node::{DurableSyncRuntime, FjallCoinsView, SyncNetwork, SyncRuntimeConfig};

type DurableContext = ManagedRpcContext<FjallChainstateStore, FjallCoinsView>;
type DurableHandle = ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>;

/// Empty index checkpoint over one codec-valid genesis body; CRUD evidence only.
/// Production lifecycle derives its fence, and no reserved lock is seeded by CRUD.
pub(crate) fn durable_context(
    path: &Path,
    active: bool,
) -> (DurableContext, DurableHandle, FjallNodeStore) {
    let store = FjallNodeStore::open(path).expect("store");
    let genesis = build_block(BlockHash::default(), 0, 500_000_000, p2sh_script());
    let position = ChainPosition::new(genesis.header.clone(), 0, 1, i64::from(genesis.header.time));
    if store
        .coins_view()
        .best_block()
        .expect("coins best")
        .is_none()
    {
        store
            .seed_coins_from_snapshot(&ChainstateSnapshot::new(
                vec![position],
                Default::default(),
                Default::default(),
            ))
            .expect("coins and metadata");
        store
            .save_block(&genesis, open_bitcoin_node::PersistMode::Sync)
            .expect("body");
    }
    let runtime = DurableSyncRuntime::open(
        store.clone(),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
    )
    .expect("production runtime");
    let handle = runtime.network_handle();
    if active {
        handle
            .enable_basic_filter_index()
            .expect("trusted host enable");
    }
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &crate::config::RuntimeConfig {
            chain: open_bitcoin_node::core::wallet::AddressNetwork::Regtest,
            ..Default::default()
        },
        handle.clone(),
        Some(store.clone()),
    )
    .expect("durable context");
    drop(runtime);
    (context, handle, store)
}

fn clear_reserved(context: &mut DurableContext) -> Result<Value, crate::RpcFailure> {
    dispatch(
        context,
        MethodCall::ClearPruneLock(ClearPruneLockRequest {
            name: BASIC_INDEX_PRUNE_LOCK.to_owned(),
        }),
    )
}

fn assert_reserved_failure(failure: crate::RpcFailure) {
    let detail = failure.maybe_detail.expect("detail");
    assert_eq!(detail.code, RpcErrorCode::InvalidParameter);
    assert_eq!(
        detail.message,
        "reserved BASIC index prune lock is internally owned"
    );
}

#[test]
fn prune_dispatch_reserved_refuses_absent_active_and_disabled_owners() {
    for mode in 0..3 {
        // Arrange
        let temp = TempStore::new("reserved-dispatch");
        let (mut context, handle, store) = durable_context(temp.path(), mode != 0);
        if mode == 2 {
            handle
                .disable_basic_filter_index()
                .expect("trusted disable");
        }
        let original = store.load_prune_locks().expect("locks");
        let maybe_snapshot = store
            .wallet_scan_chainstate_snapshot()
            .expect("durable chainstate");
        let maybe_best = store.coins_view().best_block().expect("coins best");
        let maybe_body = store.load_block(maybe_best.expect("tip")).expect("body");

        // Act / Assert
        assert_reserved_failure(
            set_lock(&mut context, BASIC_INDEX_PRUNE_LOCK, 0, 10).expect_err("reserved set"),
        );
        assert_reserved_failure(clear_reserved(&mut context).expect_err("reserved clear"));
        assert_eq!(store.load_prune_locks().expect("unchanged"), original);
        drop(context);
        drop(handle);
        drop(store);
        let reopened = FjallNodeStore::open(temp.path()).expect("real reopen");
        assert_eq!(reopened.load_prune_locks().expect("locks"), original);
        assert_eq!(
            reopened
                .wallet_scan_chainstate_snapshot()
                .expect("unchanged metadata and coins"),
            maybe_snapshot
        );
        assert_eq!(
            reopened.coins_view().best_block().expect("best"),
            maybe_best
        );
        assert_eq!(
            reopened.load_block(maybe_best.expect("tip")).expect("body"),
            maybe_body
        );
        let runtime = DurableSyncRuntime::open(
            reopened,
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
        )
        .expect("valid retained lifecycle after reopen");
        assert_eq!(
            runtime.network_handle().list_prune_locks().expect("locks"),
            original
        );
    }
}

#[test]
fn prune_dispatch_ordinary_crud_preserves_active_owner_through_reopen() {
    // Arrange
    let temp = TempStore::new("ordinary-owned-dispatch");
    let (mut context, handle, store) = durable_context(temp.path(), true);
    let reserved = store.load_prune_locks().expect("owned")[0].clone();

    // Act
    set_lock(&mut context, "wallet", 10, 20).expect("set");
    set_lock(&mut context, "wallet", 12, 18).expect("replace");
    let listed = list_locks(&mut context);
    let missing = clear_lock(&mut context, "missing");
    let cleared = clear_lock(&mut context, "wallet");

    // Assert
    assert_eq!(listed.as_array().expect("array").len(), 2);
    assert_eq!(missing, json!({"success":false}));
    assert_eq!(cleared, json!({"success":true}));
    assert_eq!(
        store.load_prune_locks().expect("preserved"),
        vec![reserved.clone()]
    );
    drop(context);
    drop(handle);
    drop(store);
    let reopened = FjallNodeStore::open(temp.path()).expect("reopen");
    assert_eq!(
        reopened.load_prune_locks().expect("preserved owner"),
        vec![reserved]
    );
}
