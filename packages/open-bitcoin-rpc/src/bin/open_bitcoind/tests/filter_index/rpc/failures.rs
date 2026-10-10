// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::super::fixtures::{History, daemon_block};
use super::*;
use open_bitcoin_node::core::consensus::ScriptVerifyFlags;

#[tokio::test]
async fn phase159_daemon_rpc_failures_known_header_only_is_never_connected() {
    // Arrange
    let fixture = DaemonFixture::new(2);
    let candidate = daemon_block(fixture.blocks.last(), 2);
    let hash = block_hash(&candidate.header);
    fixture
        .opened
        .network
        .connect_outbound_peer(1, 10_000)
        .expect("local header admission fixture");
    // Act: real header validation/admission without body acceptance.
    fixture
        .opened
        .network
        .receive_sync_message(
            1,
            open_bitcoin_network::WireNetworkMessage::Headers(
                open_bitcoin_network::HeadersMessage {
                    headers: vec![candidate.header],
                },
            ),
            1_000_000,
            ScriptVerifyFlags::P2SH,
            open_bitcoin_node::SyncNetwork::Regtest.consensus_params(),
        )
        .expect("genuine header-only insertion");
    // Assert
    assert_error(
        &filter(&fixture, hash).await,
        -5,
        "Filter not found. Block was not connected to active chain.",
    );
    assert_eq!(fixture.store.load_block(hash).expect("no body"), None);
    assert_eq!(
        fixture
            .opened
            .network
            .maybe_chain_tip()
            .expect("unchanged accepted tip")
            .expect("tip")
            .height,
        1
    );
    fixture.cleanup();
}

#[tokio::test]
async fn phase159_daemon_rpc_failures_legacy_raw_seed_is_honestly_unknown() {
    // Arrange: old codec-valid seeded snapshots deliberately grant no acceptance.
    let history = History::new(40);
    drop(history.seed());
    let fixture = DaemonFixture::open(history.path.clone(), history.blocks.clone(), true);
    let hash = block_hash(&history.blocks[39].header);
    // Act / Assert: successful stored genesis wins; missing legacy row is unavailable.
    assert!(filter(&fixture, block_hash(&history.blocks[0].header)).await["result"].is_object());
    assert_error(
        &filter(&fixture, hash).await,
        -32603,
        "Block filter index is unavailable",
    );
    let (path, blocks) = fixture.close();
    let fixture = DaemonFixture::open(path, blocks, true);
    assert_error(
        &filter(&fixture, hash).await,
        -32603,
        "Block filter index is unavailable",
    );
    drop(fixture.close());
    drop(history);
}

#[tokio::test]
async fn phase159_daemon_rpc_failures_pending_lifecycle_and_real_append_fence() {
    for action in ["stop", "disable", "fence", "reorg"] {
        // Arrange
        let mut fixture = DaemonFixture::new(2);
        let hash = fixture.accept_next();
        let body = json!({"jsonrpc":"2.0","id":"daemon","method":"getblockfilter","params":[display_hash(hash)]}).to_string();
        let auth = headers();
        let request_state = fixture.state.clone();
        let mut waiting = Box::pin(handle_http_request(
            &request_state,
            "/",
            Method::POST,
            &auth,
            body.as_bytes(),
        ));
        assert!(
            std::future::poll_fn(|cx| std::task::Poll::Ready(waiting.as_mut().poll(cx)))
                .await
                .is_pending()
        );
        // Act
        match action {
            "stop" => fixture
                .opened
                .network
                .stop_basic_filter_readiness()
                .expect("ordinary owner stop"),
            "disable" => fixture
                .opened
                .network
                .disable_basic_filter_index()
                .expect("real configured owner disable"),
            "reorg" => retention::replace_tip(&mut fixture, 1, 1),
            "fence" => {
                fixture
                    .store
                    .coins_view()
                    .batch_write_with_persist_mode(
                        open_bitcoin_node::core::chainstate::CoinsBatch {
                            entries: Default::default(),
                        },
                        Some(BlockHash::from_byte_array([0x99; 32])),
                        open_bitcoin_node::PersistMode::Sync,
                    )
                    .expect("real raw writer invalidates publication fence");
                assert!(
                    fixture
                        .opened
                        .network
                        .drive_basic_filter_index_turn()
                        .is_err(),
                    "actual production publication refuses the invalidated fence"
                );
            }
            _ => unreachable!("enumerated lifecycle control"),
        }
        let result = response(
            tokio::time::timeout(Duration::from_secs(2), waiting)
                .await
                .expect("terminal event wakes HTTP"),
        )
        .await;
        // Assert: exact redacted fixed failure, no empty successful filter or hang.
        assert_error(&result, -32603, "Block filter index is unavailable");
        assert!(
            !result
                .to_string()
                .contains(fixture.path.to_str().expect("path"))
        );
        assert!(!result.to_string().contains("private-password"));
        assert!(fixture.shared.try_lock().is_ok());
        drop(request_state);
        fixture.cleanup();
    }
}

#[tokio::test]
async fn phase159_daemon_rpc_failures_disabled_selection_and_type_precedence() {
    // Arrange
    let fixture = DaemonFixture::new(2);
    let (path, blocks) = fixture.close();
    let fixture = DaemonFixture::open(path, blocks, false);
    // Act / Assert
    assert_eq!(
        invoke(&fixture, "getindexinfo", json!([])).await,
        json!({"jsonrpc":"2.0","id":"daemon","result":{}})
    );
    assert_error(
        &filter(&fixture, BlockHash::from_byte_array([0xaa; 32])).await,
        -1,
        "Index is not enabled for filtertype basic",
    );
    assert_error(
        &invoke(&fixture, "getblockfilter", json!(["aa".repeat(32), "v0"])).await,
        -1,
        "Index is not enabled for filtertype v0",
    );
    assert_error(
        &invoke(
            &fixture,
            "getblockfilter",
            json!(["aa".repeat(32), "BASIC"]),
        )
        .await,
        -5,
        "Unknown filtertype",
    );
    let typed = invoke(&fixture, "getblockfilter", json!(["bad", 42])).await;
    assert_eq!(
        typed["error"]["code"], -3,
        "framework type check precedes malformed hash"
    );
    fixture.cleanup();
}

#[tokio::test]
async fn phase159_daemon_rpc_failures_configured_auth_cookie_duplicates_and_scope() {
    // Arrange
    let fixture = DaemonFixture::new(2);
    let body = br#"{"jsonrpc":"2.0","id":"daemon","method":"getindexinfo","params":{"index_name":null,"index_name":""}}"#;
    // Act / Assert: malformed unauthorized requests cannot wait for the held context.
    for auth in [HeaderMap::new(), {
        let mut wrong = headers();
        wrong.insert(
            "authorization",
            HeaderValue::from_static("Basic YmFkOmJhZA=="),
        );
        wrong
    }] {
        let guard = fixture.shared.lock().await;
        let result = tokio::time::timeout(
            Duration::from_secs(2),
            handle_http_request(&fixture.state, "/", Method::POST, &auth, b"BROKEN"),
        )
        .await
        .expect("auth precedes JSON/context");
        drop(guard);
        assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
        assert!(
            to_bytes(result.into_body(), 100)
                .await
                .expect("empty 401")
                .is_empty()
        );
    }
    assert_eq!(
        response(handle_http_request(&fixture.state, "/", Method::POST, &headers(), body).await)
            .await["error"],
        json!({"code":-8,"message":"Parameter index_name specified multiple times"})
    );
    let cookie_path = fixture.path.join(".phase159-cookie");
    let cookie_state = open_bitcoin_rpc::http::build_http_state_with_shared_context(
        open_bitcoin_rpc::config::RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(cookie_path.clone()),
        },
        std::sync::Arc::clone(&fixture.shared),
    )
    .expect("existing cookie auth");
    let credentials = std::fs::read_to_string(cookie_path).expect("private local cookie");
    let mut cookie_auth = HeaderMap::new();
    cookie_auth.insert(
        "authorization",
        HeaderValue::from_str(&format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(credentials.trim())
        ))
        .expect("cookie header"),
    );
    let request = br#"{"jsonrpc":"2.0","id":"daemon","method":"getindexinfo"}"#;
    assert_eq!(
        response(
            handle_http_request(&cookie_state, "/", Method::POST, &cookie_auth, request).await
        )
        .await["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":1}})
    );
    assert_eq!(
        response(
            handle_http_request(
                &cookie_state,
                "/wallet/private",
                Method::POST,
                &cookie_auth,
                request
            )
            .await
        )
        .await["error"]["code"],
        -32600
    );
    assert!(!format!("{cookie_state:?}").contains(credentials.trim()));
    drop(cookie_state);
    fixture.cleanup();
}
