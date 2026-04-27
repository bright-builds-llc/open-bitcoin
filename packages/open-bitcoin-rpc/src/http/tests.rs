// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use axum::{
    body::to_bytes,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
};
use base64::Engine as _;
use open_bitcoin_node::{
    FjallNodeStore, PersistMode, WalletRegistry,
    core::wallet::{AddressNetwork, Wallet},
};
use serde_json::json;
use std::{fs, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::{
    ManagedRpcContext, RpcAuthConfig, RuntimeConfig,
    config::DEFAULT_COOKIE_AUTH_USER,
    http::{build_http_state, handle_http_request},
};

fn auth_headers(username: &str, password: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let credentials =
        base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Basic {credentials}")).expect("header"),
    );
    headers
}

fn state() -> crate::http::RpcHttpState {
    let context = ManagedRpcContext::from_runtime_config(&RuntimeConfig::default());
    build_http_state(
        RpcAuthConfig::UserPassword {
            username: "alice".to_string(),
            password: "secret".to_string(),
        },
        context,
    )
    .expect("state")
}

fn state_with_wallet_registry(wallet_names: &[&str]) -> crate::http::RpcHttpState {
    let path = temp_store_path("wallet-registry");
    let store = FjallNodeStore::open(&path).expect("store");
    let mut registry = WalletRegistry::default();
    for wallet_name in wallet_names {
        registry
            .create_wallet(
                &store,
                (*wallet_name).to_string(),
                Wallet::new(AddressNetwork::Regtest),
                PersistMode::Sync,
            )
            .expect("create wallet");
    }
    drop(store);
    let context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(path),
        ..RuntimeConfig::default()
    });
    build_http_state(
        RpcAuthConfig::UserPassword {
            username: "alice".to_string(),
            password: "secret".to_string(),
        },
        context,
    )
    .expect("state")
}

fn temp_cookie_path(test_name: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    std::env::temp_dir()
        .join(format!(
            "open-bitcoin-rpc-{test_name}-{}-{unique}",
            std::process::id()
        ))
        .join(".cookie")
}

fn temp_store_path(test_name: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-rpc-{test_name}-{}-{unique}",
        std::process::id()
    ))
}

async fn response_json(response: axum::response::Response) -> serde_json::Value {
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice(&body).expect("json")
}

#[tokio::test]
async fn legacy_and_json_rpc_v2_status_mapping_matches_phase_8_contract() {
    // Arrange
    let state = state();
    let headers = auth_headers("alice", "secret");

    // Act
    let legacy_not_found = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"method":"invalidmethod","params":[],"id":1}"#,
    )
    .await;
    let legacy_invalid_params = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"method":"deriveaddresses","params":["wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)"],"id":2}"#,
    )
    .await;
    let v2_not_found = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"jsonrpc":"2.0","method":"invalidmethod","params":[],"id":3}"#,
    )
    .await;

    // Assert
    assert_eq!(legacy_not_found.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        response_json(legacy_not_found).await["error"]["code"],
        json!(-32601)
    );
    assert_eq!(
        legacy_invalid_params.status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(v2_not_found.status(), StatusCode::OK);
    assert_eq!(
        response_json(v2_not_found).await["error"]["code"],
        json!(-32601)
    );
}

#[tokio::test]
async fn json_rpc_v2_notifications_return_no_content_and_execute() {
    // Arrange
    let state = state();
    let headers = auth_headers("alice", "secret");

    // Act
    let notification = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"jsonrpc":"2.0","method":"importdescriptors","params":{"requests":[{"desc":"wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)","label":"receive","internal":false,"timestamp":0}]}}"#,
    )
    .await;
    let follow_up = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"jsonrpc":"2.0","method":"importdescriptors","params":{"requests":[{"desc":"wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)","label":"receive","internal":false,"timestamp":0}]},"id":1}"#,
    )
    .await;

    // Assert
    assert_eq!(notification.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        response_json(follow_up).await["result"]["results"][0]["success"],
        json!(false)
    );
}

#[tokio::test]
async fn mixed_version_batches_are_accepted() {
    // Arrange
    let state = state();
    let headers = auth_headers("alice", "secret");

    // Act
    let response = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"[{"method":"getwalletinfo","params":[],"id":1},{"jsonrpc":"2.0","method":"invalidmethod","params":[],"id":2}]"#,
    )
    .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body.as_array().expect("array").len(), 2);
    assert!(body[0].get("jsonrpc").is_none());
    assert_eq!(body[1]["jsonrpc"], json!("2.0"));
}

#[tokio::test]
async fn post_only_transport_rejects_unauthenticated_requests() {
    // Arrange
    let state = state();
    let headers = HeaderMap::new();

    // Act
    let bad_method = handle_http_request(
        &state,
        "/",
        Method::GET,
        &headers,
        br#"{"method":"getwalletinfo","params":[],"id":1}"#,
    )
    .await;
    let missing_auth = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"method":"getwalletinfo","params":[],"id":1}"#,
    )
    .await;
    let wrong_auth = handle_http_request(
        &state,
        "/",
        Method::POST,
        &auth_headers("alice", "wrong"),
        br#"{"method":"getwalletinfo","params":[],"id":1}"#,
    )
    .await;

    // Assert
    assert_eq!(bad_method.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(missing_auth.status(), StatusCode::UNAUTHORIZED);
    assert!(missing_auth.headers().contains_key("www-authenticate"));
    assert_eq!(wrong_auth.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn wallet_routes_select_named_wallet_and_keep_node_methods_rooted() {
    // Arrange
    let state = state_with_wallet_registry(&["alpha", "beta"]);
    let headers = auth_headers("alice", "secret");

    // Act
    let node_response = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}"#,
    )
    .await;
    let root_wallet_failure = handle_http_request(
        &state,
        "/",
        Method::POST,
        &headers,
        br#"{"jsonrpc":"2.0","method":"getwalletinfo","params":[],"id":2}"#,
    )
    .await;
    let scoped_wallet_success = handle_http_request(
        &state,
        "/wallet/alpha",
        Method::POST,
        &headers,
        br#"{"jsonrpc":"2.0","method":"getwalletinfo","params":[],"id":3}"#,
    )
    .await;

    // Assert
    assert_eq!(node_response.status(), StatusCode::OK);
    assert_eq!(
        response_json(node_response).await["result"]["chain"],
        json!("regtest")
    );
    assert_eq!(root_wallet_failure.status(), StatusCode::OK);
    assert_eq!(
        response_json(root_wallet_failure).await["error"]["code"],
        json!(-19)
    );
    assert_eq!(scoped_wallet_success.status(), StatusCode::OK);
    assert_eq!(
        response_json(scoped_wallet_success).await["result"]["network"],
        json!("regtest")
    );
}

#[test]
fn cookie_auth_creates_owner_only_file_with_random_secret() {
    // Arrange
    let path = temp_cookie_path("cookie-auth-creates-owner-only-file");

    // Act
    let password = super::read_or_create_cookie_password(&path).expect("cookie password");

    // Assert
    let contents = fs::read_to_string(&path).expect("cookie contents");
    assert_eq!(contents, format!("{DEFAULT_COOKIE_AUTH_USER}:{password}\n"));
    assert_eq!(password.len(), 64);
    assert!(
        password
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "password must be lowercase hex",
    );
    #[cfg(unix)]
    assert_eq!(
        fs::metadata(&path).expect("metadata").permissions().mode() & 0o777,
        0o600,
    );

    let parent = path.parent().expect("parent");
    let _ = fs::remove_dir_all(parent);
}

#[test]
fn cookie_auth_preserves_existing_cookie_file() {
    // Arrange
    let path = temp_cookie_path("cookie-auth-preserves-existing-file");
    fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
    fs::write(
        &path,
        format!("{DEFAULT_COOKIE_AUTH_USER}:existing-secret\n"),
    )
    .expect("write");

    // Act
    let password = super::read_or_create_cookie_password(&path).expect("cookie password");

    // Assert
    assert_eq!(password, "existing-secret");
    assert_eq!(
        fs::read_to_string(&path).expect("cookie contents"),
        format!("{DEFAULT_COOKIE_AUTH_USER}:existing-secret\n"),
    );

    let parent = path.parent().expect("parent");
    let _ = fs::remove_dir_all(parent);
}
