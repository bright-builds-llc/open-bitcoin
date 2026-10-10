// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::*;
use crate::http::{RpcHttpState, build_http_state_with_shared_context};
use open_bitcoin_node::core::consensus::block_hash;
use open_bitcoin_node::{FjallChainstateStore, FjallCoinsView};
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) mod fixtures;
use fixtures::History;

type DurableState = RpcHttpState<FjallChainstateStore, FjallCoinsView>;

fn durable_state(history: &History) -> DurableState {
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &RuntimeConfig {
            chain: AddressNetwork::Regtest,
            ..Default::default()
        },
        history.runtime.network_handle(),
        Some(history.runtime.store().clone()),
    )
    .expect("same handle context");
    build_http_state(
        RpcAuthConfig::UserPassword {
            username: "alice".into(),
            password: "secret".into(),
        },
        context,
    )
    .expect("state")
}

fn display_hash(hash: open_bitcoin_node::core::primitives::BlockHash) -> String {
    hash.as_bytes()
        .iter()
        .rev()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

async fn invoke<
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
>(
    state: &RpcHttpState<S, V>,
    method: &str,
    params: serde_json::Value,
) -> serde_json::Value {
    let request = json!({"jsonrpc":"2.0", "id": 7, "method":method,"params":params});
    let response = handle_http_request(
        state,
        "/",
        Method::POST,
        &auth_headers("alice", "secret"),
        request.to_string().as_bytes(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    response_json(response).await
}

#[tokio::test]
async fn phase159_filter_rpc_http_auth_before_parse_and_held_context() {
    // Arrange
    let context = Arc::new(Mutex::new(ManagedRpcContext::from_runtime_config(
        &RuntimeConfig::default(),
    )));
    let state = build_http_state_with_shared_context(
        RpcAuthConfig::UserPassword {
            username: "private-user".into(),
            password: "private-password-marker".into(),
        },
        Arc::clone(&context),
    )
    .expect("state");
    // Act / Assert
    for headers in [
        HeaderMap::new(),
        auth_headers("private-user", "wrong-password"),
    ] {
        let guard = context.lock().await;
        let request_state = state.clone();
        let request = tokio::spawn(async move {
            handle_http_request(
                &request_state,
                "/",
                Method::POST,
                &headers,
                br#"{"method":"getblockfilter","params":BROKEN"#,
            )
            .await
        });
        let outcome = tokio::time::timeout(std::time::Duration::from_secs(1), request).await;
        drop(guard);
        let response = outcome
            .expect("auth cannot wait for context")
            .expect("request task");
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers()["www-authenticate"],
            "Basic realm=\"jsonrpc\""
        );
        assert!(
            to_bytes(response.into_body(), 100)
                .await
                .expect("body")
                .is_empty()
        );
    }
    let debug = format!("{state:?}");
    assert!(!debug.contains("private-password-marker"));
    assert!(!debug.contains("private-user"));
}

#[tokio::test]
async fn phase159_filter_rpc_http_initial_available_and_index_summary() {
    // Arrange: initial ordinary turn processes eight of forty genuine accepted blocks.
    let history = History::new(40);
    let state = durable_state(&history);
    // Act
    let found = invoke(
        &state,
        "getblockfilter",
        json!([display_hash(block_hash(&history.blocks[0].header)), null]),
    )
    .await;
    let summary = invoke(&state, "getindexinfo", json!({"index_name":null})).await;
    let unknown = invoke(&state, "getblockfilter", json!(["aa".repeat(32)])).await;
    let missing = invoke(
        &state,
        "getblockfilter",
        json!([display_hash(block_hash(&history.blocks[39].header))]),
    )
    .await;
    // Assert
    assert_eq!(
        unknown["error"],
        json!({"code":-5,"message":"Block not found"})
    );
    assert_eq!(found["result"].as_object().expect("found").len(), 2);
    assert!(found["result"]["filter"].as_str().is_some());
    assert_eq!(
        found["result"]["header"].as_str().expect("header").len(),
        64
    );
    assert_eq!(
        summary["result"],
        json!({"basic block filter index":{"synced":false,"best_block_height":7}})
    );
    assert_eq!(missing["error"]["code"], -1);
    assert_eq!(
        missing["error"]["message"],
        "Filter not found. Block filters are still in the process of being indexed."
    );
    drop(state);
    history.cleanup();
}

#[tokio::test]
async fn phase159_filter_rpc_http_lag_releases_context_summary_and_owner() {
    // Arrange
    let mut history = History::new(2);
    let state = durable_state(&history);
    let hash = history.accept_next();
    let request = json!({"jsonrpc":"2.0","id":"original","method":"getblockfilter","params":[display_hash(hash)]}).to_string();
    let headers = auth_headers("alice", "secret");
    let mut waiting = Box::pin(handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        request.as_bytes(),
    ));
    // Act: poll the actual request until it has registered and yielded.
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(waiting.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    let summary = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        invoke(&state, "getindexinfo", json!([])),
    )
    .await
    .expect("separate request proceeds while filter waits");
    history
        .runtime
        .network_handle()
        .drive_basic_filter_index_turn()
        .expect("independent ordinary owner");
    let response = tokio::time::timeout(std::time::Duration::from_secs(1), waiting)
        .await
        .expect("earned completion");
    let found = response_json(response).await;
    // Assert
    assert_eq!(
        summary["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":1}})
    );
    assert_eq!(found["id"], "original");
    let open_bitcoin_node::BasicFilterQuery::Found(record) = history
        .runtime
        .network_handle()
        .basic_filter_query(hash)
        .expect("same authority")
    else {
        panic!("found");
    };
    assert_eq!(
        found["result"],
        serde_json::to_value(crate::method::GetBlockFilterResult::from_encoded(
            record.encoded_bytes(),
            *record.filter_header().as_bytes()
        ))
        .expect("projection")
    );
    let open_bitcoin_node::core::chainstate::IndexPrefix::Committed(safe) = history
        .runtime
        .store()
        .load_prune_protection()
        .expect("safe")
        .maybe_owner()
        .expect("owner")
        .checkpoint()
        .checkpoint()
        .prefix()
    else {
        panic!("safe prefix");
    };
    assert_eq!(safe.height(), 1);
    assert_eq!(
        invoke(&state, "getindexinfo", json!([])).await["result"]["basic block filter index"]["best_block_height"],
        2
    );
    drop(state);
    history.cleanup();
}

#[tokio::test]
async fn phase159_filter_rpc_http_raw_names_syntax_batches_and_notifications() {
    // Arrange
    let state = state();
    let headers = auth_headers("alice", "secret");
    let batch = br#"[{"jsonrpc":"2.0","id":"dup","method":"getindexinfo","params":{"index_name":null,"index_name":""}},{"method":"getindexinfo","params":null,"id":3},{"jsonrpc":"2.0","method":"getblockfilter","params":[]}]"#;
    // Act
    let response = handle_http_request(&state, "/", Method::POST, &headers, batch).await;
    assert_eq!(response.status(), StatusCode::OK);
    let result = response_json(response).await;
    // Assert
    assert_eq!(result.as_array().expect("batch").len(), 2);
    assert_eq!(result[0]["id"], "dup");
    assert_eq!(result[0]["error"]["code"], -8);
    assert_eq!(result[1], json!({"id":3,"result":{},"error":null}));
    let duplicate_filter = br#"{"jsonrpc":"2.0","id":4,"method":"getblockfilter","params":{"blockhash":null,"blockhash":null}}"#;
    let duplicate = response_json(
        handle_http_request(&state, "/", Method::POST, &headers, duplicate_filter).await,
    )
    .await;
    assert_eq!(duplicate["error"]["code"], -8);
    assert_eq!(
        duplicate["error"]["message"],
        "Parameter blockhash specified multiple times"
    );
    let invalid_envelope = br#"{"jsonrpc":"2.0","id":5,"params":{"index_name":1,"index_name":2}}"#;
    let invalid = response_json(
        handle_http_request(&state, "/", Method::POST, &headers, invalid_envelope).await,
    )
    .await;
    assert_eq!(
        invalid["error"],
        json!({"code":-32600,"message":"Missing method"}),
        "existing envelope validation precedes duplicate semantics"
    );
    for body in [
        br#"{"method":"getindexinfo","params":{"index_name":1,"index_name":2},BROKEN}"#.as_slice(),
        br#"[{"method":"getindexinfo","params":{"index_name":1,"index_name":2}},BROKEN]"#
            .as_slice(),
    ] {
        let response = handle_http_request(&state, "/", Method::POST, &headers, body).await;
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_json(response).await["error"]["code"],
            -32700,
            "whole document syntax wins over semantic duplicates"
        );
    }
    for body in [
        br#"{"jsonrpc":"2.0","method":"getindexinfo"}"#.as_slice(),
        br#"[{"jsonrpc":"2.0","method":"getindexinfo"}]"#.as_slice(),
    ] {
        let response = handle_http_request(&state, "/", Method::POST, &headers, body).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert!(
            to_bytes(response.into_body(), 100)
                .await
                .expect("body")
                .is_empty()
        );
    }
    assert_eq!(
        response_json(handle_http_request(&state, "/", Method::POST, &headers, b"[]").await).await,
        json!([])
    );
}

#[tokio::test]
async fn phase159_filter_rpc_http_exact_precedence_scope_and_versions() {
    // Arrange
    let state = state();
    let headers = auth_headers("alice", "secret");
    for (params, code, message) in [
        (
            json!(["00".repeat(32)]),
            -1,
            "Index is not enabled for filtertype basic",
        ),
        (
            json!({"blockhash":"00".repeat(32),"filtertype":null}),
            -1,
            "Index is not enabled for filtertype basic",
        ),
        (
            json!(["00".repeat(32), "v0"]),
            -1,
            "Index is not enabled for filtertype v0",
        ),
        (json!(["00".repeat(32), "BASIC"]), -5, "Unknown filtertype"),
        (
            json!({"args":["00".repeat(32)],"blockhash":"11".repeat(32)}),
            -8,
            "Parameter blockhash specified twice both as positional and named argument",
        ),
    ] {
        for v2 in [false, true] {
            // Act
            let mut request = json!({"method":"getblockfilter","params":params,"id":null});
            if v2 {
                request["jsonrpc"] = json!("2.0");
            }
            let response = handle_http_request(
                &state,
                "/",
                Method::POST,
                &headers,
                request.to_string().as_bytes(),
            )
            .await;
            // Assert
            assert_eq!(
                response.status(),
                if v2 {
                    StatusCode::OK
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            );
            let result = response_json(response).await;
            assert_eq!(result["id"], serde_json::Value::Null);
            assert_eq!(result["error"]["code"], code);
            assert_eq!(result["error"]["message"], message);
        }
    }
    for method in ["getblockfilter", "getindexinfo"] {
        let params = if method == "getblockfilter" {
            json!(["00".repeat(32)])
        } else {
            json!([])
        };
        let request = json!({"jsonrpc":"2.0","method":method,"params":params,"id":7});
        let result = response_json(
            handle_http_request(
                &state,
                "/wallet/private",
                Method::POST,
                &headers,
                request.to_string().as_bytes(),
            )
            .await,
        )
        .await;
        assert_eq!(result["error"]["code"], -32600);
        assert_eq!(
            result["error"]["message"],
            "Node-scoped RPC methods must be requested through the root URI path."
        );
    }
}

#[tokio::test]
async fn phase159_filter_rpc_http_cookie_success_and_exact_selection() {
    // Arrange
    let history = History::new(2);
    let mut state = durable_state(&history);
    let cookie = temp_cookie_path("phase159-cookie");
    state.auth = super::super::resolve_auth(RpcAuthConfig::Cookie {
        maybe_cookie_file: Some(cookie.clone()),
    })
    .expect("cookie auth");
    let credentials = fs::read_to_string(&cookie).expect("generated cookie");
    let (username, password) = credentials.trim().split_once(':').expect("credentials");
    let headers = auth_headers(username, password);
    // Act / Assert
    for params in [
        json!(null),
        json!([]),
        json!([null]),
        json!([""]),
        json!({"index_name":"basic block filter index"}),
    ] {
        let request = json!({"jsonrpc":"2.0","method":"getindexinfo","params":params,"id":1});
        let response = handle_http_request(
            &state,
            "/",
            Method::POST,
            &headers,
            request.to_string().as_bytes(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response_json(response).await["result"],
            json!({"basic block filter index":{"synced":true,"best_block_height":1}})
        );
    }
    assert_eq!(
        invoke(
            &durable_state(&history),
            "getindexinfo",
            json!(["Basic block filter index"])
        )
        .await["result"],
        json!({})
    );
    assert!(!format!("{state:?}").contains(password));
    drop(state);
    fs::remove_dir_all(cookie.parent().expect("parent")).expect("cookie cleanup");
    history.cleanup();
}

#[tokio::test]
async fn phase159_filter_rpc_http_waiter_terminal_lifecycle_is_redacted() {
    for action in ["stop", "disable", "failure", "reorg"] {
        // Arrange
        let mut history = History::new(2);
        let state = durable_state(&history);
        let hash = history.accept_next();
        let request =
            json!({"jsonrpc":"2.0","method":"getblockfilter","params":[display_hash(hash)],"id":7})
                .to_string();
        let headers = auth_headers("alice", "secret");
        let mut waiting = Box::pin(handle_http_request(
            &state,
            "/",
            Method::POST,
            &headers,
            request.as_bytes(),
        ));
        assert!(
            std::future::poll_fn(|cx| std::task::Poll::Ready(waiting.as_mut().poll(cx)))
                .await
                .is_pending()
        );
        let handle = history.runtime.network_handle();
        // Act
        match action {
            "stop" => handle
                .stop_basic_filter_readiness()
                .expect("existing owner stop"),
            "disable" => handle.disable_basic_filter_index().expect("disable"),
            "failure" => {
                history
                    .runtime
                    .store()
                    .coins_view()
                    .batch_write_with_persist_mode(
                        open_bitcoin_node::core::chainstate::CoinsBatch {
                            entries: Default::default(),
                        },
                        Some(
                            open_bitcoin_node::core::primitives::BlockHash::from_byte_array(
                                [99; 32],
                            ),
                        ),
                        PersistMode::Sync,
                    )
                    .expect("injected unrelated durable coins marker invalidates append authority");
                assert!(handle.drive_basic_filter_index_turn().is_err());
            }
            "reorg" => {
                let snapshot = handle.chainstate_snapshot().expect("genuine accepted undo");
                history
                    .runtime
                    .store()
                    .save_undo(hash, &snapshot.undo_by_block[&hash], PersistMode::Sync)
                    .expect("retain actual disconnected undo without a coins fence");
                let mut replacement = fixtures::next_block(Some(&history.blocks[1]), 2);
                replacement.header.time += 1;
                replacement.header.nonce = (0..=u32::MAX)
                    .find(|nonce| {
                        replacement.header.nonce = *nonce;
                        open_bitcoin_node::core::consensus::check_block_header(&replacement.header)
                            .is_ok()
                    })
                    .expect("easy replacement header");
                handle
                    .reorg_to_branch(
                        &[history.blocks[2].clone()],
                        &[open_bitcoin_node::core::chainstate::AnchoredBlock {
                            block: replacement,
                            chain_work: 100,
                        }],
                        open_bitcoin_mempool::ReorgLifecycleContext::new(
                            open_bitcoin_mempool::PolicyTime::from_unix_seconds(50_000),
                        ),
                        open_bitcoin_node::core::consensus::ScriptVerifyFlags::P2SH,
                        fixtures::params(),
                    )
                    .expect("genuine replacement");
            }
            _ => unreachable!("test cases"),
        }
        let response = tokio::time::timeout(std::time::Duration::from_secs(1), waiting)
            .await
            .expect("terminal settlement");
        let result = response_json(response).await;
        // Assert
        assert_eq!(result["error"]["code"], -32603, "{action}");
        assert_eq!(
            result["error"]["message"], "Block filter index is unavailable",
            "{action}"
        );
        assert!(
            !result
                .to_string()
                .contains(history.path.to_str().expect("path"))
        );
        assert!(!result.to_string().contains("secret"));
        assert!(state.context.try_lock().is_ok());
        drop(handle);
        drop(state);
        history.cleanup();
    }
}

#[tokio::test]
async fn phase159_filter_rpc_http_completion_cannot_change_original_hash() {
    use crate::dispatch::filter_index::{PreparedDispatch, prepare};
    // Arrange
    let mut history = History::new(2);
    let state = durable_state(&history);
    let hash = history.accept_next();
    let call = crate::method::normalize_method_call(
        "getblockfilter",
        crate::method::RequestParameters::Positional(vec![json!(display_hash(hash))]),
    )
    .expect("call");
    let PreparedDispatch::Pending {
        mut request,
        barrier,
    } = prepare(&mut *state.context.lock().await, call).expect("prepare")
    else {
        panic!("pending");
    };
    request.block_hash = block_hash(&history.blocks[0].header);
    history
        .runtime
        .network_handle()
        .drive_basic_filter_index_turn()
        .expect("actual owner");
    // Act
    let failure = super::super::filter_index::finish(
        &state.context,
        history.runtime.network_handle(),
        PreparedDispatch::Pending { request, barrier },
    )
    .await
    .expect_err("original request-bound token");
    // Assert
    assert_eq!(
        failure.maybe_detail.expect("fixed detail"),
        crate::RpcErrorDetail::new(
            crate::RpcErrorCode::InternalError,
            "Block filter index is unavailable"
        )
    );
    drop(state);
    history.cleanup();
}
