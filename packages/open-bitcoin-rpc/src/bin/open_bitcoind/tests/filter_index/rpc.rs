// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::fixtures::DaemonFixture;
use axum::{
    body::to_bytes,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
};
use base64::Engine;
use open_bitcoin_node::core::{
    chainstate::CoinsView, consensus::block_hash, primitives::BlockHash,
};
use open_bitcoin_rpc::http::handle_http_request;
use serde_json::{Value, json};
use std::{future::Future, time::Duration};

#[path = "rpc/failures.rs"]
mod failures;
#[path = "rpc/retention.rs"]
mod retention;

fn display_hash(hash: BlockHash) -> String {
    hash.as_bytes()
        .iter()
        .rev()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let credentials = base64::engine::general_purpose::STANDARD.encode("phase159:private-password");
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Basic {credentials}")).expect("header"),
    );
    headers
}

async fn response(response: axum::response::Response) -> Value {
    assert_eq!(response.status(), StatusCode::OK);
    serde_json::from_slice(
        &to_bytes(response.into_body(), 1024 * 1024)
            .await
            .expect("body"),
    )
    .expect("JSON")
}

async fn invoke(fixture: &DaemonFixture, method: &str, params: Value) -> Value {
    let request = json!({"jsonrpc":"2.0", "id":"daemon", "method":method, "params":params});
    response(
        handle_http_request(
            &fixture.state,
            "/",
            Method::POST,
            &headers(),
            request.to_string().as_bytes(),
        )
        .await,
    )
    .await
}

async fn filter(fixture: &DaemonFixture, hash: BlockHash) -> Value {
    invoke(fixture, "getblockfilter", json!([display_hash(hash)])).await
}

fn expected(fixture: &DaemonFixture, hash: BlockHash) -> Value {
    let open_bitcoin_node::BasicFilterQuery::Found(record) = fixture
        .opened
        .network
        .basic_filter_query(hash)
        .expect("same owner checked read")
    else {
        panic!("present immutable row")
    };
    assert!(record.work().record_reads <= 2);
    json!({"filter":record.encoded_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>(), "header":display_hash(BlockHash::from_byte_array(*record.filter_header().as_bytes()))})
}

fn historical_oracle(fixture: &DaemonFixture, height: usize) -> Value {
    use open_bitcoin_node::core::{
        chainstate::{BasicFilterInputs, HistoricalBlockUndo},
        consensus::FilterHeaderPredecessor,
    };
    let snapshot = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("genuine accepted facts");
    let mut predecessor = FilterHeaderPredecessor::Genesis;
    let mut result = Value::Null;
    for (offset, block) in fixture.blocks[..=height].iter().enumerate() {
        let position = &snapshot.active_chain[offset];
        let inputs = BasicFilterInputs::from_historical(
            block,
            position,
            snapshot
                .undo_by_block
                .get(&position.block_hash)
                .map(|undo| HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo,
                }),
        )
        .expect("actual retained validated body/undo");
        if offset > 1 {
            assert_eq!(
                inputs
                    .spent_scripts()
                    .map(<[u8]>::to_vec)
                    .collect::<Vec<_>>(),
                vec![
                    fixture.blocks[offset - 1].transactions[0].outputs[0]
                        .script_pubkey
                        .as_bytes()
                        .to_vec(),
                    vec![0x52, 0x75, 0x51]
                ]
            );
        }
        let (filter, header) = inputs
            .generate(predecessor)
            .expect("independent pure historical generation");
        if offset > 1 {
            let outputs = block
                .transactions
                .iter()
                .flat_map(|transaction| &transaction.outputs)
                .map(|output| output.script_pubkey.as_bytes())
                .collect::<Vec<_>>();
            let omitted_history =
                open_bitcoin_node::core::consensus::BasicFilter::from_script_facts(
                    position.block_hash,
                    &outputs,
                    &[],
                )
                .expect("explicit incorrect output-only control");
            assert_ne!(
                filter.encoded_bytes(),
                omitted_history.encoded_bytes(),
                "historical input omission must change the commitment"
            );
        }
        predecessor = FilterHeaderPredecessor::Previous {
            block_hash: position.block_hash,
            height: position.height,
            header,
        };
        result = json!({"filter":filter.encoded_bytes().iter().map(|byte|format!("{byte:02x}")).collect::<String>(),"header":display_hash(BlockHash::from_byte_array(*header.as_bytes()))});
    }
    result
}

fn assert_error(value: &Value, code: i32, message: &str) {
    assert_eq!(
        value,
        &json!({"jsonrpc":"2.0", "id":"daemon", "error":{"code":code,"message":message}})
    );
}

#[tokio::test]
async fn phase159_daemon_rpc_configured_initial_and_exact_shared_results() {
    // Arrange: real continuous acceptance, durable coins fence, then actual config/open.
    let fixture = DaemonFixture::new(40);
    let genesis = block_hash(&fixture.blocks[0].header);
    let available = block_hash(&fixture.blocks[7].header);
    let missing = block_hash(&fixture.blocks[39].header);
    // Act
    let found = filter(&fixture, available).await;
    let summary = invoke(&fixture, "getindexinfo", json!([])).await;
    // Assert
    assert_eq!(
        found,
        json!({"jsonrpc":"2.0","id":"daemon","result":expected(&fixture, available)})
    );
    assert_eq!(found["result"], historical_oracle(&fixture, 7));
    assert_ne!(
        found["result"]["header"],
        filter(&fixture, genesis).await["result"]["header"]
    );
    assert_eq!(
        summary["result"],
        json!({"basic block filter index":{"synced":false,"best_block_height":7}})
    );
    assert_error(
        &filter(&fixture, missing).await,
        -1,
        "Filter not found. Block filters are still in the process of being indexed.",
    );
    assert_error(
        &filter(&fixture, BlockHash::from_byte_array([0xab; 32])).await,
        -5,
        "Block not found",
    );
    fixture.finish();
    assert_eq!(
        filter(&fixture, missing).await["result"],
        expected(&fixture, missing)
    );
    assert_eq!(
        invoke(&fixture, "getindexinfo", json!([])).await["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":39}})
    );
    for selector in [
        json!(null),
        json!([null]),
        json!([""]),
        json!({"index_name":"basic block filter index"}),
    ] {
        assert_eq!(
            invoke(&fixture, "getindexinfo", selector).await["result"],
            json!({"basic block filter index":{"synced":true,"best_block_height":39}})
        );
    }
    assert_eq!(
        invoke(
            &fixture,
            "getindexinfo",
            json!(["Basic block filter index"])
        )
        .await["result"],
        json!({})
    );
    fixture.cleanup();
}

#[tokio::test]
async fn phase159_daemon_rpc_later_lag_same_owner_and_independent_summary() {
    // Arrange
    let mut fixture = DaemonFixture::new(2);
    let hash = fixture.accept_next();
    let body = json!({"jsonrpc":"2.0","id":"daemon","method":"getblockfilter","params":[display_hash(hash)]}).to_string();
    let auth = headers();
    let mut waiting = Box::pin(handle_http_request(
        &fixture.state,
        "/",
        Method::POST,
        &auth,
        body.as_bytes(),
    ));
    // Act
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(waiting.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    let summary = tokio::time::timeout(
        Duration::from_secs(2),
        invoke(&fixture, "getindexinfo", json!([])),
    )
    .await
    .expect("summary while wait owns no context lock");
    fixture
        .opened
        .network
        .drive_basic_filter_index_turn()
        .expect("ordinary same owner scheduled turn");
    let found = response(
        tokio::time::timeout(Duration::from_secs(2), waiting)
            .await
            .expect("owner wakes HTTP"),
    )
    .await;
    // Assert: RPC context and maintenance share this authority, including unflushed state.
    assert_eq!(
        summary["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":1}})
    );
    assert_eq!(found["result"], expected(&fixture, hash));
    assert_eq!(
        invoke(&fixture, "getindexinfo", json!([])).await["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":2}})
    );
    assert_eq!(
        fixture
            .store
            .coins_view()
            .best_block()
            .expect("durable coins"),
        Some(block_hash(&fixture.blocks[1].header))
    );
    fixture.cleanup();
}

#[tokio::test]
async fn phase159_daemon_rpc_actual_maintenance_worker_completes_pending_http() {
    // Arrange: both actual daemon consumers receive clones of the configured handle.
    let mut fixture = DaemonFixture::new(2);
    let hash = fixture.accept_next();
    let body = json!({"jsonrpc":"2.0","id":"daemon","method":"getblockfilter","params":[display_hash(hash)]}).to_string();
    let auth = headers();
    let mut waiting = Box::pin(handle_http_request(
        &fixture.state,
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
    // Act: no request-owned indexing; the unchanged one-second daemon owner ticks.
    let worker = crate::coins_flush::start_coins_flush_worker(
        fixture.opened.network.clone(),
        Some(fixture.store.clone()),
    )
    .expect("actual durable maintenance worker");
    let outcome = tokio::time::timeout(Duration::from_secs(10), waiting).await;
    // HTTP drains before the daemon's ordinary worker shutdown/Always settlement.
    let shutdown = worker.shutdown_always();
    // Assert
    let result = response(outcome.expect("actual maintenance must settle HTTP")).await;
    assert_eq!(result["result"], expected(&fixture, hash));
    shutdown.expect("actual daemon maintenance shutdown");
    assert_eq!(
        invoke(&fixture, "getindexinfo", json!([])).await["result"],
        json!({"basic block filter index":{"synced":true,"best_block_height":2}})
    );
    fixture.cleanup();
}
