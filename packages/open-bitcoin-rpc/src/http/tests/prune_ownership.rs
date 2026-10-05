// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::*;
use crate::dispatch::durable_context;
use open_bitcoin_node::core::chainstate::BASIC_INDEX_PRUNE_LOCK;
use open_bitcoin_node::core::chainstate::CoinsView;
use open_bitcoin_node::{FjallChainstateStore, FjallCoinsView};

async fn call(
    state: &crate::http::RpcHttpState<FjallChainstateStore, FjallCoinsView>,
    headers: &HeaderMap,
    method: &str,
    params: serde_json::Value,
) -> axum::response::Response {
    let request =
        serde_json::to_vec(&json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}))
            .expect("request");
    handle_http_request(state, "/", Method::POST, headers, &request).await
}

#[tokio::test]
async fn prune_http_authentication_precedes_reserved_dispatch_and_parsing() {
    // Arrange
    let path = temp_store_path("prune-auth-order");
    let (context, handle, store) = durable_context(&path, true);
    let original = store.load_prune_locks().expect("locks");
    let state = build_http_state(
        RpcAuthConfig::UserPassword {
            username: "alice".into(),
            password: "secret".into(),
        },
        context,
    )
    .expect("state");

    // Act / Assert
    for headers in [HeaderMap::new(), auth_headers("alice", "wrong")] {
        let response = call(
            &state,
            &headers,
            "clearprunelock",
            json!([BASIC_INDEX_PRUNE_LOCK]),
        )
        .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().contains_key("www-authenticate"));
        let malformed =
            handle_http_request(&state, "/", Method::POST, &headers, b"invalid json").await;
        assert_eq!(malformed.status(), StatusCode::UNAUTHORIZED);
    }
    assert_eq!(store.load_prune_locks().expect("unchanged"), original);
    drop(state);
    drop(handle);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert_eq!(reopened.load_prune_locks().expect("unchanged"), original);
    drop(reopened);
    fs::remove_dir_all(path).expect("cleanup");
}

#[tokio::test]
async fn prune_http_authenticated_reserved_mutations_refuse_all_lifecycle_states() {
    for mode in 0..3 {
        // Arrange
        let path = temp_store_path("prune-auth-reserved");
        let (context, handle, store) = durable_context(&path, mode != 0);
        if mode == 2 {
            handle
                .disable_basic_filter_index()
                .expect("trusted disable");
        }
        let state = build_http_state(
            RpcAuthConfig::UserPassword {
                username: "alice".into(),
                password: "secret".into(),
            },
            context,
        )
        .expect("state");
        let headers = auth_headers("alice", "secret");
        let original = store.load_prune_locks().expect("locks");
        let maybe_snapshot = store.wallet_scan_chainstate_snapshot().expect("chainstate");
        let maybe_best = store.coins_view().best_block().expect("coins best");
        let maybe_saved_body = store.load_block(maybe_best.expect("tip")).expect("body");

        // Act / Assert
        for (method, params) in [
            ("setprunelock", json!([BASIC_INDEX_PRUNE_LOCK, 0, 10])),
            ("clearprunelock", json!([BASIC_INDEX_PRUNE_LOCK])),
        ] {
            let response = call(&state, &headers, method, params).await;
            assert_eq!(response.status(), StatusCode::OK);
            let body = response_json(response).await;
            assert_eq!(body["error"]["code"], json!(-8));
            assert_eq!(
                body["error"]["message"],
                json!("reserved BASIC index prune lock is internally owned")
            );
            assert_eq!(store.load_prune_locks().expect("unchanged"), original);
        }
        let legacy_request = serde_json::to_vec(
            &json!({"id":2,"method":"clearprunelock","params":[BASIC_INDEX_PRUNE_LOCK]}),
        )
        .expect("legacy request");
        let legacy =
            handle_http_request(&state, "/", Method::POST, &headers, &legacy_request).await;
        assert_eq!(legacy.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(response_json(legacy).await["error"]["code"], json!(-8));
        drop(state);
        drop(handle);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("reopen");
        assert_eq!(reopened.load_prune_locks().expect("unchanged"), original);
        assert_eq!(
            reopened
                .wallet_scan_chainstate_snapshot()
                .expect("chainstate"),
            maybe_snapshot
        );
        assert_eq!(
            reopened.coins_view().best_block().expect("coins best"),
            maybe_best
        );
        assert_eq!(
            reopened.load_block(maybe_best.expect("tip")).expect("body"),
            maybe_saved_body
        );
        let runtime = open_bitcoin_node::DurableSyncRuntime::open(
            reopened,
            open_bitcoin_node::SyncRuntimeConfig {
                network: open_bitcoin_node::SyncNetwork::Regtest,
                ..Default::default()
            },
        )
        .expect("real lifecycle reopen");
        assert_eq!(
            runtime.network_handle().list_prune_locks().expect("locks"),
            original
        );
        drop(runtime);
        fs::remove_dir_all(path).expect("cleanup");
    }
}

#[tokio::test]
async fn prune_http_ordinary_authenticated_crud_persists_with_reserved_owner() {
    // Arrange
    let path = temp_store_path("prune-auth-ordinary");
    let (context, handle, store) = durable_context(&path, true);
    let reserved = store.load_prune_locks().expect("owner")[0].clone();
    let state = build_http_state(
        RpcAuthConfig::UserPassword {
            username: "alice".into(),
            password: "secret".into(),
        },
        context,
    )
    .expect("state");
    let headers = auth_headers("alice", "secret");

    // Act
    for params in [json!(["wallet", 10, 20]), json!(["wallet", 12, 18])] {
        let body = response_json(call(&state, &headers, "setprunelock", params).await).await;
        assert!(body["error"].is_null());
    }
    let listed = response_json(call(&state, &headers, "listprunelocks", json!([])).await).await;
    let missing =
        response_json(call(&state, &headers, "clearprunelock", json!(["missing"])).await).await;

    // Assert
    assert_eq!(listed["result"].as_array().expect("locks").len(), 2);
    assert_eq!(missing["result"], json!({"success":false}));
    drop(state);
    drop(handle);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen set/replace");
    let locks = reopened.load_prune_locks().expect("locks");
    assert!(locks.contains(&reserved));
    assert!(
        locks
            .iter()
            .any(|lock| lock.name == "wallet" && lock.height_first == 12 && lock.height_last == 18)
    );
    drop(reopened);
    let (context, handle, store) = durable_context(&path, true);
    let state = build_http_state(
        RpcAuthConfig::UserPassword {
            username: "alice".into(),
            password: "secret".into(),
        },
        context,
    )
    .expect("state");
    let cleared =
        response_json(call(&state, &headers, "clearprunelock", json!(["wallet"])).await).await;
    assert_eq!(cleared["result"], json!({"success":true}));
    drop(state);
    drop(handle);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen clear");
    assert_eq!(reopened.load_prune_locks().expect("owner"), vec![reserved]);
    drop(reopened);
    fs::remove_dir_all(path).expect("cleanup");
}
