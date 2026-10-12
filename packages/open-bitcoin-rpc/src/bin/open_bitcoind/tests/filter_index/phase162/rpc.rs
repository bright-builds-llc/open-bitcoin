// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/index/base.cpp

//! Authenticated actual HTTP dispatch against independently decoded retained history.

use super::{fixtures::Phase162Fixture, oracle_case, oracle_hash, retention};
use axum::{
    body::to_bytes,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
};
use serde_json::{Value, json};

pub(crate) async fn assert_initial_sync(fixture: &Phase162Fixture) {
    assert_summary(fixture, false, 7).await;
    let missing = oracle_case(fixture, "active-1000");
    assert_eq!(
        retention::invoke(
            fixture,
            "getblockfilter",
            json!([missing["block_hash_display"]])
        )
        .await,
        error(
            -1,
            "Filter not found. Block filters are still in the process of being indexed."
        )
    );
}

fn error(code: i32, message: &str) -> Value {
    json!({"jsonrpc":"2.0","id":"phase162","error":{"code":code,"message":message}})
}

async fn assert_summary(fixture: &Phase162Fixture, synced: bool, height: u32) {
    let expected = json!({"jsonrpc":"2.0","id":"phase162","result":{
        "basic block filter index":{"synced":synced,"best_block_height":height}}});
    for selector in [
        json!([]),
        json!(["basic block filter index"]),
        json!({"index_name":"basic block filter index"}),
    ] {
        assert_eq!(
            retention::invoke(fixture, "getindexinfo", selector).await,
            expected
        );
    }
    for selector in ["absent", "Basic block filter index"] {
        assert_eq!(
            retention::invoke(fixture, "getindexinfo", json!([selector])).await,
            json!({"jsonrpc":"2.0","id":"phase162","result":{}})
        );
    }
}

async fn assert_unauthorized(fixture: &Phase162Fixture) {
    for method in ["getblockfilter", "getindexinfo"] {
        for maybe_auth in [None, Some("Basic aW52YWxpZDppbnZhbGlk")] {
            let mut headers = HeaderMap::new();
            if let Some(auth) = maybe_auth {
                headers.insert("authorization", HeaderValue::from_static(auth));
            }
            // Invalid method parameters make authentication-before-dispatch observable.
            let request = json!({"jsonrpc":"2.0","id":"phase162","method":method,"params":[true]});
            let response = open_bitcoin_rpc::http::handle_http_request(
                &fixture.state,
                "/",
                Method::POST,
                &headers,
                request.to_string().as_bytes(),
            )
            .await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
            assert_eq!(
                response.headers()["www-authenticate"],
                "Basic realm=\"jsonrpc\""
            );
            let body = to_bytes(response.into_body(), 1024)
                .await
                .expect("bounded refusal body");
            assert!(!body.windows(6).any(|bytes| bytes == b"result"));
        }
    }
}

pub(crate) async fn assert_phase162_rpc(fixture: &Phase162Fixture) {
    // Arrange: these expectations predate retention and come exclusively from raw blocks.
    assert_eq!(
        fixture.config.prune_mode,
        super::core_types::chainstate::PruneMode::Automatic { target_mib: 550 }
    );
    let before = fixture
        .opened
        .network
        .maybe_basic_index_summary()
        .expect("before RPC");
    // Act / Assert: no credentials or incorrect credentials cannot reach JSON dispatch.
    assert_unauthorized(fixture).await;
    assert_summary(fixture, true, 1002).await;
    for (name, pair_present) in [
        ("active-0", false),
        ("active-20", false),
        ("active-21", false),
        ("active-714", false),
        ("active-715", true),
        ("fork-1000", true),
        ("fork-1002", true),
        ("active-999", true),
        ("active-1000", true),
    ] {
        let row = oracle_case(fixture, name);
        retention::assert_pair(fixture, oracle_hash(row), pair_present);
        let expected = json!({"jsonrpc":"2.0","id":"phase162","result":{
            "filter":row["encoded"],"header":row["header_display"]}});
        for params in [
            json!([row["block_hash_display"]]),
            json!([row["block_hash_display"], "basic"]),
        ] {
            assert_eq!(
                retention::invoke(fixture, "getblockfilter", params).await,
                expected
            );
        }
        retention::assert_pair(fixture, oracle_hash(row), pair_present);
    }
    let hash = &oracle_case(fixture, "active-20")["block_hash_display"];
    assert_eq!(
        retention::invoke(fixture, "getblockfilter", json!([hash, "unsupported"])).await,
        error(-5, "Unknown filtertype")
    );
    assert_eq!(
        fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("after RPC"),
        before
    );
    eprintln!(
        "phase162 retained RPC: authenticated_hashes=9 selectors=2 unauthorized=4 initial_sync=false7_then_true1002 independent_raw=true"
    );
}
